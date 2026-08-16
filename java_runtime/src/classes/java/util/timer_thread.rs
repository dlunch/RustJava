use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, JavaError, Jvm, Result};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::util::{TimerTask, TimerTaskQueue},
};

enum WorkerAction {
    Continue,
    Run(ClassInstanceRef<TimerTask>),
    Stop,
}

// class java.util.Timer$TimerThread
pub struct TimerThread;

impl TimerThread {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Timer$TimerThread",
            parent_class: Some("java/lang/Thread"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/util/Timer$TaskQueue;)V", Self::init, MethodAccessFlags::empty()),
                JavaMethodProto::new("run", "()V", Self::run, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("queue", "Ljava/util/Timer$TaskQueue;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("newTasksMayBeScheduled", "Z", FieldAccessFlags::empty()),
            ],
            access_flags: ClassAccessFlags::empty(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, queue: ClassInstanceRef<TimerTaskQueue>) -> Result<()> {
        tracing::debug!("java.util.Timer$TimerThread::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Thread", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "queue", "Ljava/util/Timer$TaskQueue;", queue).await?;
        jvm.put_field(&mut this, "newTasksMayBeScheduled", "Z", true).await?;
        Ok(())
    }

    async fn run(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Timer$TimerThread::run({this:?})");

        let result = Self::main_loop(jvm, context, &this).await;
        let mut thread = this;
        let mut queue: ClassInstanceRef<TimerTaskQueue> = jvm.get_field(&thread, "queue", "Ljava/util/Timer$TaskQueue;").await?;
        jvm.monitor_enter(&queue).await?;
        let cleanup_result = async {
            jvm.put_field(&mut thread, "newTasksMayBeScheduled", "Z", false).await?;
            TimerTaskQueue::clear(jvm, &mut queue).await?;
            jvm.object_notify(&queue, usize::MAX).await
        }
        .await;
        let exit_result = jvm.monitor_exit(&queue).await;

        match result {
            Ok(()) => {
                cleanup_result?;
                exit_result
            }
            Err(error) => {
                if let Err(cleanup_error) = cleanup_result {
                    exit_result?;
                    return Err(cleanup_error);
                }
                exit_result?;
                Err(error)
            }
        }
    }

    async fn main_loop(jvm: &Jvm, context: &mut RuntimeContext, this: &ClassInstanceRef<Self>) -> Result<()> {
        let mut queue: ClassInstanceRef<TimerTaskQueue> = jvm.get_field(this, "queue", "Ljava/util/Timer$TaskQueue;").await?;

        loop {
            jvm.monitor_enter(&queue).await?;
            let action_result = async {
                loop {
                    let size: i32 = jvm.get_field(&queue, "size", "I").await?;
                    if size == 0 {
                        if !jvm.get_field::<bool>(this, "newTasksMayBeScheduled", "Z").await? {
                            return Ok(WorkerAction::Stop);
                        }
                        let wait_result: Result<()> = jvm.invoke_virtual(&queue, "java/lang/Object", "wait", "()V", ()).await;
                        if let Err(error) = wait_result
                            && !matches!(
                                &error,
                                JavaError::JavaException(exception)
                                    if exception.class_definition().name() == "java/lang/InterruptedException"
                            )
                        {
                            return Err(error);
                        }
                        continue;
                    }

                    let heap: ClassInstanceRef<Array<TimerTask>> = jvm.get_field(&queue, "queue", "[Ljava/util/TimerTask;").await?;
                    let mut task: ClassInstanceRef<TimerTask> = jvm.load_array(&heap, 1, 1).await?.remove(0);
                    let lock: ClassInstanceRef<crate::classes::java::lang::Object> = jvm.get_field(&task, "lock", "Ljava/lang/Object;").await?;
                    jvm.monitor_enter(&lock).await?;
                    let task_result = async {
                        let state: i32 = jvm.get_field(&task, "state", "I").await?;
                        if state == TimerTask::CANCELLED {
                            TimerTaskQueue::remove_min(jvm, &mut queue).await?;
                            return Ok(WorkerAction::Continue);
                        }

                        let now = context.now() as i64;
                        let execution_time: i64 = jvm.get_field(&task, "nextExecutionTime", "J").await?;
                        if execution_time > now {
                            return Ok(WorkerAction::Continue);
                        }

                        let period: i64 = jvm.get_field(&task, "period", "J").await?;
                        jvm.put_field(&mut task, "lastScheduledExecutionTime", "J", execution_time).await?;
                        if period == 0 {
                            TimerTaskQueue::remove_min(jvm, &mut queue).await?;
                            jvm.put_field(&mut task, "state", "I", TimerTask::EXECUTED).await?;
                        } else {
                            let next_execution_time = if period < 0 {
                                now.checked_sub(period)
                            } else {
                                execution_time.checked_add(period)
                            };
                            if let Some(next_execution_time) = next_execution_time {
                                TimerTaskQueue::reschedule_min(jvm, &mut queue, next_execution_time).await?;
                            } else {
                                TimerTaskQueue::remove_min(jvm, &mut queue).await?;
                                jvm.put_field(&mut task, "state", "I", TimerTask::EXECUTED).await?;
                            }
                        }
                        Ok(WorkerAction::Run(task.clone()))
                    }
                    .await;
                    let task_exit_result = jvm.monitor_exit(&lock).await;
                    let action = match task_result {
                        Ok(action) => {
                            task_exit_result?;
                            action
                        }
                        Err(error) => {
                            task_exit_result?;
                            return Err(error);
                        }
                    };

                    if matches!(action, WorkerAction::Run(_)) {
                        return Ok(action);
                    }
                    let next_size: i32 = jvm.get_field(&queue, "size", "I").await?;
                    if next_size == 0 {
                        continue;
                    }
                    let next_heap: ClassInstanceRef<Array<TimerTask>> = jvm.get_field(&queue, "queue", "[Ljava/util/TimerTask;").await?;
                    let next_task: ClassInstanceRef<TimerTask> = jvm.load_array(&next_heap, 1, 1).await?.remove(0);
                    let next_execution_time: i64 = jvm.get_field(&next_task, "nextExecutionTime", "J").await?;
                    let now = context.now() as i64;
                    if next_execution_time <= now {
                        continue;
                    }
                    let wait_result: Result<()> = jvm
                        .invoke_virtual(&queue, "java/lang/Object", "wait", "(J)V", (next_execution_time - now,))
                        .await;
                    if let Err(error) = wait_result
                        && !matches!(
                            &error,
                            JavaError::JavaException(exception)
                                if exception.class_definition().name() == "java/lang/InterruptedException"
                        )
                    {
                        return Err(error);
                    }
                }
            }
            .await;
            let queue_exit_result = jvm.monitor_exit(&queue).await;
            let action = match action_result {
                Ok(action) => {
                    queue_exit_result?;
                    action
                }
                Err(error) => {
                    queue_exit_result?;
                    return Err(error);
                }
            };

            match action {
                WorkerAction::Continue => {}
                WorkerAction::Run(task) => {
                    let _: () = jvm.invoke_virtual(&task, "java/util/TimerTask", "run", "()V", ()).await?;
                }
                WorkerAction::Stop => return Ok(()),
            }
        }
    }
}
