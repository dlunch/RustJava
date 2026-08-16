use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::util::{Date, TimerTask, TimerTaskQueue, TimerThread},
};

// class java.util.Timer
pub struct Timer;

impl Timer {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Timer",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("schedule", "(Ljava/util/TimerTask;J)V", Self::schedule_once, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "schedule",
                    "(Ljava/util/TimerTask;Ljava/util/Date;)V",
                    Self::schedule_date,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "schedule",
                    "(Ljava/util/TimerTask;JJ)V",
                    Self::schedule_repeated,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "schedule",
                    "(Ljava/util/TimerTask;Ljava/util/Date;J)V",
                    Self::schedule_date_repeated,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "scheduleAtFixedRate",
                    "(Ljava/util/TimerTask;JJ)V",
                    Self::schedule_at_fixed_rate,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("cancel", "()V", Self::cancel, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![JavaFieldProto::new(
                "thread",
                "Ljava/util/Timer$TimerThread;",
                FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL,
            )],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Timer::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        let queue = jvm.new_class("java/util/Timer$TaskQueue", "()V", ()).await?;
        let timer_thread = jvm
            .new_class("java/util/Timer$TimerThread", "(Ljava/util/Timer$TaskQueue;)V", (queue,))
            .await?;
        jvm.put_field(&mut this, "thread", "Ljava/util/Timer$TimerThread;", timer_thread.clone())
            .await?;
        let _: () = jvm
            .invoke_virtual(&timer_thread, "java/util/Timer$TimerThread", "start", "()V", ())
            .await?;
        Ok(())
    }

    async fn schedule_once(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        task: ClassInstanceRef<TimerTask>,
        delay: i64,
    ) -> Result<()> {
        tracing::debug!("java.util.Timer::schedule({this:?}, {task:?}, {delay:?})");

        if task.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "task").await);
        }
        if delay < 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "negative delay").await);
        }
        let Some(time) = (context.now() as i64).checked_add(delay) else {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "illegal execution time").await);
        };
        if time < 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "illegal execution time").await);
        }
        Self::sched(jvm, this, task, time, 0).await
    }

    async fn schedule_date(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        task: ClassInstanceRef<TimerTask>,
        date: ClassInstanceRef<Date>,
    ) -> Result<()> {
        tracing::debug!("java.util.Timer::schedule({this:?}, {task:?}, {date:?})");

        if task.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "task").await);
        }
        if date.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "time").await);
        }
        let time: i64 = jvm.invoke_virtual(&date, "java/util/Date", "getTime", "()J", ()).await?;
        if time < 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "illegal execution time").await);
        }
        Self::sched(jvm, this, task, time, 0).await
    }

    async fn schedule_repeated(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        task: ClassInstanceRef<TimerTask>,
        delay: i64,
        period: i64,
    ) -> Result<()> {
        tracing::debug!("java.util.Timer::schedule({this:?}, {task:?}, {delay:?}, {period:?})");

        if task.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "task").await);
        }
        if delay < 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "negative delay").await);
        }
        if period <= 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "non-positive period").await);
        }
        let Some(time) = (context.now() as i64).checked_add(delay) else {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "illegal execution time").await);
        };
        if time < 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "illegal execution time").await);
        }
        Self::sched(jvm, this, task, time, -period).await
    }

    async fn schedule_date_repeated(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        task: ClassInstanceRef<TimerTask>,
        date: ClassInstanceRef<Date>,
        period: i64,
    ) -> Result<()> {
        tracing::debug!("java.util.Timer::schedule({this:?}, {task:?}, {date:?}, {period:?})");

        if task.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "task").await);
        }
        if date.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "time").await);
        }
        if period <= 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "non-positive period").await);
        }
        let time: i64 = jvm.invoke_virtual(&date, "java/util/Date", "getTime", "()J", ()).await?;
        if time < 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "illegal execution time").await);
        }
        Self::sched(jvm, this, task, time, -period).await
    }

    async fn schedule_at_fixed_rate(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        task: ClassInstanceRef<TimerTask>,
        delay: i64,
        period: i64,
    ) -> Result<()> {
        tracing::debug!("java.util.Timer::scheduleAtFixedRate({this:?}, {task:?}, {delay:?}, {period:?})");

        if task.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "task").await);
        }
        if delay < 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "negative delay").await);
        }
        if period <= 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "non-positive period").await);
        }
        let Some(time) = (context.now() as i64).checked_add(delay) else {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "illegal execution time").await);
        };
        if time < 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "illegal execution time").await);
        }
        Self::sched(jvm, this, task, time, period).await
    }

    async fn sched(jvm: &Jvm, this: ClassInstanceRef<Self>, mut task: ClassInstanceRef<TimerTask>, time: i64, period: i64) -> Result<()> {
        let period = if !(-(i64::MAX >> 1)..=(i64::MAX >> 1)).contains(&period) {
            period >> 1
        } else {
            period
        };
        if period != 0 {
            let period_magnitude = if period < 0 {
                let Some(period_magnitude) = period.checked_neg() else {
                    return Err(jvm.exception("java/lang/IllegalArgumentException", "period overflow").await);
                };
                period_magnitude
            } else {
                period
            };
            if time.checked_add(period_magnitude).is_none() {
                return Err(jvm.exception("java/lang/IllegalArgumentException", "period overflow").await);
            }
        }

        let thread: ClassInstanceRef<TimerThread> = jvm.get_field(&this, "thread", "Ljava/util/Timer$TimerThread;").await?;
        let mut queue: ClassInstanceRef<TimerTaskQueue> = jvm.get_field(&thread, "queue", "Ljava/util/Timer$TaskQueue;").await?;
        let lock: ClassInstanceRef<crate::classes::java::lang::Object> = jvm.get_field(&task, "lock", "Ljava/lang/Object;").await?;

        jvm.monitor_enter(&queue).await?;
        let schedule_result = async {
            if !jvm.get_field::<bool>(&thread, "newTasksMayBeScheduled", "Z").await? {
                return Err(jvm.exception("java/lang/IllegalStateException", "timer already cancelled").await);
            }

            jvm.monitor_enter(&lock).await?;
            let task_result = async {
                if jvm.get_field::<i32>(&task, "state", "I").await? != TimerTask::VIRGIN {
                    return Err(jvm
                        .exception("java/lang/IllegalStateException", "task already scheduled or cancelled")
                        .await);
                }
                jvm.put_field(&mut task, "nextExecutionTime", "J", time).await?;
                jvm.put_field(&mut task, "period", "J", period).await?;
                jvm.put_field(&mut task, "state", "I", TimerTask::SCHEDULED).await?;
                TimerTaskQueue::add(jvm, &mut queue, task.clone()).await
            }
            .await;
            let task_exit_result = jvm.monitor_exit(&lock).await;
            let new_first = match task_result {
                Ok(new_first) => {
                    task_exit_result?;
                    new_first
                }
                Err(error) => {
                    task_exit_result?;
                    return Err(error);
                }
            };
            if new_first {
                jvm.object_notify(&queue, 1).await?;
            }
            Ok(())
        }
        .await;
        let queue_exit_result = jvm.monitor_exit(&queue).await;
        match schedule_result {
            Ok(()) => queue_exit_result,
            Err(error) => {
                queue_exit_result?;
                Err(error)
            }
        }
    }

    async fn cancel(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Timer::cancel({this:?})");

        let mut thread: ClassInstanceRef<TimerThread> = jvm.get_field(&this, "thread", "Ljava/util/Timer$TimerThread;").await?;
        let mut queue: ClassInstanceRef<TimerTaskQueue> = jvm.get_field(&thread, "queue", "Ljava/util/Timer$TaskQueue;").await?;
        jvm.monitor_enter(&queue).await?;
        let cancel_result = async {
            jvm.put_field(&mut thread, "newTasksMayBeScheduled", "Z", false).await?;
            TimerTaskQueue::clear(jvm, &mut queue).await?;
            jvm.object_notify(&queue, usize::MAX).await
        }
        .await;
        let exit_result = jvm.monitor_exit(&queue).await;
        match cancel_result {
            Ok(()) => exit_result,
            Err(error) => {
                exit_result?;
                Err(error)
            }
        }
    }
}
