use alloc::{boxed::Box, vec};

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{classes::java::util::TimerTask, RuntimeClassProto, RuntimeContext, SpawnCallback};

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

#[cfg(test)]
mod test {
    use alloc::{boxed::Box, collections::BTreeMap, vec};

    use java_class_proto::{JavaFieldProto, JavaMethodProto};
    use jvm::{ClassInstanceRef, Jvm, Result};
    use jvm_rust::ClassDefinitionImpl;

    use crate::{runtime::test::TestRuntime, test::create_test_jvm, RuntimeClassProto, RuntimeContext};

    struct TestClass;
    impl TestClass {
        pub fn as_proto() -> RuntimeClassProto {
            RuntimeClassProto {
                name: "TestClass",
                parent_class: Some("java/lang/Object"),
                interfaces: vec!["java/lang/Runnable"],
                methods: vec![
                    JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                    JavaMethodProto::new("run", "()V", Self::run, Default::default()),
                ],
                fields: vec![JavaFieldProto::new("ran", "Z", Default::default())],
            }
        }

        async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
            jvm.put_field(&mut this, "ran", "Z", false).await?;

            let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

            Ok(())
        }

        async fn run(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
            jvm.put_field(&mut this, "ran", "Z", true).await?;

            Ok(())
        }
    }

    #[tokio::test]
    async fn test_timer() -> Result<()> {
        let runtime = TestRuntime::new(BTreeMap::new());
        let jvm = create_test_jvm(runtime.clone()).await?;

        let class = Box::new(ClassDefinitionImpl::from_class_proto(
            TestClass::as_proto(),
            Box::new(runtime.clone()) as Box<_>,
        ));
        jvm.register_class(class, None).await?;

        let test_class = jvm.new_class("TestClass", "()V", ()).await?;

        let timer = jvm.new_class("java/util/Timer", "()V", ()).await?;
        let _: () = jvm
            .invoke_virtual(&timer, "schedule", "(Ljava/util/TimerTask;JJ)V", (test_class.clone(), 0i64, 100i64))
            .await?;

        let _: () = jvm.invoke_static("java/lang/Thread", "sleep", "(J)V", (200i64,)).await?;

        let ran: bool = jvm.get_field(&test_class, "ran", "Z").await?;
        assert!(ran);

        Ok(())
    }
}
