use alloc::{vec, vec::Vec};
use core::time::Duration;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::util::Vector};

// class java.util.Timer$TimerThread
pub struct TimerThread;

impl TimerThread {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Timer$TimerThread",
            parent_class: Some("java/lang/Thread"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/util/Vector;)V", Self::init, Default::default()),
                JavaMethodProto::new("run", "()V", Self::run, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("tasks", "Ljava/util/Vector;", Default::default())],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, tasks: ClassInstanceRef<Vector>) -> Result<()> {
        tracing::debug!("java.util.Timer$TimerThread::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/lang/Thread", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "tasks", "Ljava/util/Vector;", tasks).await?;

        Ok(())
    }

    async fn run(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Timer$TimerThread::run({:?})", &this);

        let java_tasks = jvm.get_field(&this, "tasks", "Ljava/util/Vector;").await?;

        loop {
            // TODO: we need to wait for new tasks to arrive
            context.sleep(Duration::from_millis(16)).await;

            let tasks_size: i32 = jvm.invoke_virtual(&java_tasks, "size", "()I", ()).await?;
            if tasks_size == 0 {
                continue;
            }

            // get all tasks. removing from tasks vector to avoid some concurrency issue
            let mut tasks = Vec::with_capacity(tasks_size as _);
            for _ in 0..tasks_size {
                let task = jvm.invoke_virtual(&java_tasks, "remove", "(I)Ljava/lang/Object;", (0,)).await?;
                tasks.push(task);
            }

            // execute tasks
            let now = context.now() as i64;
            let mut next_tasks = Vec::new();
            for mut task in tasks {
                let next_execution_time: i64 = jvm.get_field(&task, "nextExecutionTime", "J").await?;

                if next_execution_time < now {
                    let _: () = jvm.invoke_virtual(&task, "run", "()V", ()).await?;

                    let period: i64 = jvm.get_field(&task, "period", "J").await?;
                    if period > 0 {
                        let next_execution_time = now + period;
                        jvm.put_field(&mut task, "nextExecutionTime", "J", next_execution_time).await?;
                        next_tasks.push(task);
                    }
                } else {
                    next_tasks.push(task);
                }
            }

            // add pending tasks
            for task in next_tasks {
                let _: () = jvm.invoke_virtual(&java_tasks, "addElement", "(Ljava/lang/Object;)V", (task,)).await?;
            }
        }
    }
}
