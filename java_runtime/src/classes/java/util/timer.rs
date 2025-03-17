use alloc::{boxed::Box, vec};

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, SpawnCallback, classes::java::util::TimerTask};

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
            ],
            fields: vec![],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Timer::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

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

        // TODO we should not spawn new thread every time

        struct TimerProxy {
            jvm: Jvm,
            task: ClassInstanceRef<TimerTask>,
            delay: i64,
            period: i64,
        }

        #[async_trait::async_trait]
        impl SpawnCallback for TimerProxy {
            #[tracing::instrument(name = "timer", skip_all)]
            async fn call(&self) -> Result<()> {
                self.jvm.attach_thread()?;

                let _: () = self.jvm.invoke_static("java/lang/Thread", "sleep", "(J)V", (self.delay,)).await?;

                loop {
                    let _: () = self.jvm.invoke_virtual(&self.task, "run", "()V", ()).await?;

                    let _: () = self.jvm.invoke_static("java/lang/Thread", "sleep", "(J)V", (self.period,)).await?;
                }
            }
        }

        context.spawn(
            jvm,
            Box::new(TimerProxy {
                jvm: jvm.clone(),
                task,
                delay,
                period,
            }),
        );

        Ok(())
    }
}
