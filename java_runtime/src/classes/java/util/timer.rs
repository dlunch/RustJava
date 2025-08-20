use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::util::{TimerTask, Vector},
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
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("schedule", "(Ljava/util/TimerTask;JJ)V", Self::schedule, Default::default()),
                JavaMethodProto::new(
                    "scheduleAtFixedRate",
                    "(Ljava/util/TimerTask;JJ)V",
                    Self::schedule_at_fixed_rate,
                    Default::default(),
                ),
            ],
            fields: vec![
                JavaFieldProto::new("tasks", "Ljava/util/Vector;", Default::default()),
                JavaFieldProto::new("thread", "Ljava/lang/Thread;", Default::default()),
            ],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Timer::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        let tasks = jvm.new_class("java/util/Vector", "()V", ()).await?;
        let timer_thread = jvm
            .new_class("java/util/Timer$TimerThread", "(Ljava/util/Vector;)V", (tasks.clone(),))
            .await?;

        jvm.put_field(&mut this, "tasks", "Ljava/util/Vector;", tasks).await?;
        jvm.put_field(&mut this, "thread", "Ljava/lang/Thread;", timer_thread.clone()).await?;

        let _: () = jvm.invoke_virtual(&timer_thread, "start", "()V", ()).await?;

        Ok(())
    }

    async fn schedule(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        task: ClassInstanceRef<TimerTask>,
        delay: i64,
        period: i64,
    ) -> Result<()> {
        tracing::debug!("java.util.Timer::schedule({:?}, {:?}, {:?}, {:?})", &this, &task, delay, period);

        let now: i64 = context.now() as i64;
        let next_execution_time = now + delay;

        Self::do_schedule(jvm, this, task, next_execution_time, period).await
    }

    async fn schedule_at_fixed_rate(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        task: ClassInstanceRef<TimerTask>,
        delay: i64,
        period: i64,
    ) -> Result<()> {
        tracing::debug!(
            "java.util.Timer::scheduleAtFixedRate({:?}, {:?}, {:?}, {:?})",
            &this,
            &task,
            delay,
            period
        );
        // FIXME: fixed rate is not different from normal rate

        let now: i64 = context.now() as i64;
        let next_execution_time = now + delay;

        Self::do_schedule(jvm, this, task, next_execution_time, period).await
    }

    async fn do_schedule(
        jvm: &Jvm,
        this: ClassInstanceRef<Self>,
        mut task: ClassInstanceRef<TimerTask>,
        next_execution_time: i64,
        period: i64,
    ) -> Result<()> {
        jvm.put_field(&mut task, "nextExecutionTime", "J", next_execution_time).await?;
        jvm.put_field(&mut task, "period", "J", period).await?;

        let tasks: ClassInstanceRef<Vector> = jvm.get_field(&this, "tasks", "Ljava/util/Vector;").await?;
        let _: bool = jvm.invoke_virtual(&tasks, "add", "(Ljava/lang/Object;)Z", (task,)).await?;

        Ok(())
    }
}
