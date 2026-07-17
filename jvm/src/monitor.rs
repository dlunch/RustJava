use alloc::{collections::VecDeque, sync::Arc};

use event_listener::{Event, EventListener};
use parking_lot::Mutex;

pub(crate) struct Monitor {
    state: Mutex<MonitorState>,
    entry_event: Event,
}

struct MonitorState {
    owner: Option<u64>,
    depth: usize,
    next_waiter_id: u64,
    waiters: VecDeque<MonitorWaiter>,
}

struct MonitorWaiter {
    id: u64,
    event: Arc<Event>,
}

pub struct MonitorWait {
    monitor: Arc<Monitor>,
    listener: EventListener,
    depth: usize,
    thread_id: u64,
}

#[derive(Clone)]
pub struct MonitorWaitTimeout {
    monitor: Arc<Monitor>,
    waiter_id: u64,
    event: Arc<Event>,
}

#[derive(Debug)]
pub(crate) enum MonitorError {
    NotOwner,
}

impl Monitor {
    pub(crate) fn new() -> Self {
        Self {
            state: Mutex::new(MonitorState {
                owner: None,
                depth: 0,
                next_waiter_id: 0,
                waiters: VecDeque::new(),
            }),
            entry_event: Event::new(),
        }
    }

    pub(crate) async fn enter(self: &Arc<Self>, thread_id: u64) {
        loop {
            let listener = self.entry_event.listen();
            {
                let mut state = self.state.lock();
                match state.owner {
                    None => {
                        state.owner = Some(thread_id);
                        state.depth = 1;
                        return;
                    }
                    Some(owner) if owner == thread_id => {
                        state.depth += 1;
                        return;
                    }
                    Some(_) => {}
                }
            }
            listener.await;
        }
    }

    pub(crate) fn exit(&self, thread_id: u64) -> core::result::Result<(), MonitorError> {
        let released = {
            let mut state = self.state.lock();
            if state.owner != Some(thread_id) {
                return Err(MonitorError::NotOwner);
            }

            state.depth -= 1;
            if state.depth == 0 {
                state.owner = None;
                true
            } else {
                false
            }
        };

        if released {
            self.entry_event.notify(1);
        }
        Ok(())
    }

    pub(crate) fn prepare_wait(self: &Arc<Self>, thread_id: u64) -> core::result::Result<(MonitorWait, MonitorWaitTimeout), MonitorError> {
        let event = Arc::new(Event::new());
        let listener = event.listen();

        let (waiter_id, depth) = {
            let mut state = self.state.lock();
            if state.owner != Some(thread_id) {
                return Err(MonitorError::NotOwner);
            }

            let depth = state.depth;
            let waiter_id = state.next_waiter_id;
            state.next_waiter_id = state.next_waiter_id.wrapping_add(1);
            state.waiters.push_back(MonitorWaiter {
                id: waiter_id,
                event: event.clone(),
            });
            state.owner = None;
            state.depth = 0;

            (waiter_id, depth)
        };

        self.entry_event.notify(1);

        Ok((
            MonitorWait {
                monitor: self.clone(),
                listener,
                depth,
                thread_id,
            },
            MonitorWaitTimeout {
                monitor: self.clone(),
                waiter_id,
                event,
            },
        ))
    }

    pub(crate) fn notify(&self, thread_id: u64, count: usize) -> core::result::Result<(), MonitorError> {
        let events = {
            let mut state = self.state.lock();
            if state.owner != Some(thread_id) {
                return Err(MonitorError::NotOwner);
            }

            let count = count.min(state.waiters.len());
            (0..count)
                .filter_map(|_| state.waiters.pop_front())
                .map(|waiter| waiter.event)
                .collect::<alloc::vec::Vec<_>>()
        };

        for event in events {
            event.notify(1);
        }
        Ok(())
    }
}

impl MonitorWait {
    pub(crate) async fn wait(self) {
        self.listener.await;
        self.monitor.enter(self.thread_id).await;
        self.monitor.state.lock().depth = self.depth;
    }
}

impl MonitorWaitTimeout {
    pub fn notify(self) {
        let event = {
            let mut state = self.monitor.state.lock();
            state
                .waiters
                .iter()
                .position(|waiter| waiter.id == self.waiter_id)
                .and_then(|position| state.waiters.remove(position))
                .map(|waiter| waiter.event)
        };

        if let Some(event) = event {
            debug_assert!(Arc::ptr_eq(&event, &self.event));
            event.notify(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::sync::Arc;
    use core::{
        sync::atomic::{AtomicBool, Ordering},
        time::Duration,
    };

    use super::Monitor;

    #[tokio::test]
    async fn monitor_is_reentrant_and_excludes_other_threads() {
        let monitor = Arc::new(Monitor::new());
        monitor.enter(1).await;
        monitor.enter(1).await;

        let entered = Arc::new(AtomicBool::new(false));
        let contender = {
            let monitor = monitor.clone();
            let entered = entered.clone();
            tokio::spawn(async move {
                monitor.enter(2).await;
                entered.store(true, Ordering::SeqCst);
                monitor.exit(2).unwrap();
            })
        };

        tokio::time::sleep(Duration::from_millis(10)).await;
        assert!(!entered.load(Ordering::SeqCst));
        monitor.exit(1).unwrap();
        tokio::time::sleep(Duration::from_millis(10)).await;
        assert!(!entered.load(Ordering::SeqCst));
        monitor.exit(1).unwrap();

        tokio::time::timeout(Duration::from_secs(1), contender).await.unwrap().unwrap();
        assert!(entered.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn wait_releases_and_restores_the_full_reentrancy_depth() {
        let monitor = Arc::new(Monitor::new());
        monitor.enter(1).await;
        monitor.enter(1).await;
        let (wait, _) = monitor.prepare_wait(1).unwrap();

        monitor.enter(2).await;
        monitor.notify(2, 1).unwrap();
        monitor.exit(2).unwrap();
        wait.wait().await;

        monitor.exit(1).unwrap();
        let entered = Arc::new(AtomicBool::new(false));
        let contender = {
            let monitor = monitor.clone();
            let entered = entered.clone();
            tokio::spawn(async move {
                monitor.enter(3).await;
                entered.store(true, Ordering::SeqCst);
                monitor.exit(3).unwrap();
            })
        };
        tokio::time::sleep(Duration::from_millis(10)).await;
        assert!(!entered.load(Ordering::SeqCst));

        monitor.exit(1).unwrap();
        tokio::time::timeout(Duration::from_secs(1), contender).await.unwrap().unwrap();
        assert!(entered.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn notify_one_and_notify_all_remove_the_expected_waiters() {
        let monitor = Arc::new(Monitor::new());
        monitor.enter(1).await;
        let (first_wait, _) = monitor.prepare_wait(1).unwrap();
        monitor.enter(2).await;
        let (second_wait, _) = monitor.prepare_wait(2).unwrap();

        monitor.enter(3).await;
        monitor.notify(3, 1).unwrap();
        assert_eq!(monitor.state.lock().waiters.len(), 1);
        monitor.exit(3).unwrap();
        first_wait.wait().await;
        monitor.exit(1).unwrap();

        monitor.enter(3).await;
        monitor.notify(3, usize::MAX).unwrap();
        assert!(monitor.state.lock().waiters.is_empty());
        monitor.exit(3).unwrap();
        second_wait.wait().await;
        monitor.exit(2).unwrap();
    }

    #[tokio::test]
    async fn a_stale_timeout_cannot_consume_a_later_notification() {
        let monitor = Arc::new(Monitor::new());
        monitor.enter(1).await;
        let (first_wait, first_timeout) = monitor.prepare_wait(1).unwrap();
        first_timeout.clone().notify();
        first_wait.wait().await;
        monitor.exit(1).unwrap();

        monitor.enter(2).await;
        let (second_wait, _) = monitor.prepare_wait(2).unwrap();
        first_timeout.notify();
        assert_eq!(monitor.state.lock().waiters.len(), 1);

        monitor.enter(3).await;
        monitor.notify(3, 1).unwrap();
        monitor.exit(3).unwrap();
        second_wait.wait().await;
        monitor.exit(2).unwrap();
    }
}
