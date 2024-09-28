use alloc::{boxed::Box, format, string::String, sync::Arc, vec};
use core::time::Duration;

use event_listener::Event;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::MethodAccessFlags;
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{classes::java::lang::Runnable, RuntimeClassProto, RuntimeContext, SpawnCallback};

// class java.lang.Thread
pub struct Thread;

impl Thread {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Thread",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/Runnable;)V", Self::init_with_runnable, Default::default()),
                JavaMethodProto::new("start", "()V", Self::start, Default::default()),
                JavaMethodProto::new("join", "()V", Self::join, Default::default()),
                JavaMethodProto::new("sleep", "(J)V", Self::sleep, MethodAccessFlags::NATIVE | MethodAccessFlags::STATIC),
                JavaMethodProto::new("yield", "()V", Self::r#yield, MethodAccessFlags::NATIVE | MethodAccessFlags::STATIC),
                JavaMethodProto::new("setPriority", "(I)V", Self::set_priority, Default::default()),
                JavaMethodProto::new("currentThread", "()Ljava/lang/Thread;", Self::current_thread, MethodAccessFlags::STATIC),
                // rustjava internal
                JavaMethodProto::new("<init>", "(Z)V", Self::init_internal, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("id", "J", Default::default()),
                JavaFieldProto::new("target", "Ljava/lang/Runnable;", Default::default()),
                JavaFieldProto::new("joinEvent", "[B", Default::default()),
            ],
        }
    }

    async fn init_with_runnable(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        target: ClassInstanceRef<Runnable>,
    ) -> Result<()> {
        tracing::debug!("Thread::<init>({:?}, {:?})", &this, &target);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "target", "Ljava/lang/Runnable;", target).await?;

        Ok(())
    }

    async fn init_internal(jvm: &Jvm, _context: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, internal: bool) -> Result<()> {
        tracing::debug!("Thread::<init>({:?}, {:?})", &this, internal);

        let id: i64 = jvm.invoke_static("java/lang/Thread", "currentThreadId", "()J", []).await?;
        jvm.put_field(&mut this, "id", "J", id).await?;

        Ok(())
    }

    async fn start(jvm: &Jvm, context: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("Thread::start({:?})", &this);

        struct ThreadStartProxy {
            jvm: Jvm,
            thread_id: String,
            join_event: Arc<Event>,
            runnable: ClassInstanceRef<Runnable>,
        }

        #[async_trait::async_trait]
        impl SpawnCallback for ThreadStartProxy {
            #[tracing::instrument(name = "thread", fields(thread = self.thread_id), skip_all)]
            async fn call(&self) -> Result<()> {
                tracing::trace!("Thread start");

                self.jvm.attach_thread().await?;

                let _: () = self.jvm.invoke_virtual(&self.runnable, "run", "()V", []).await?;

                self.jvm.detach_thread().await?;

                self.join_event.notify(usize::MAX);

                Ok(())
            }
        }

        let join_event = Arc::new(Event::new());
        jvm.put_rust_object_field(&mut this, "joinEvent", join_event.clone()).await?;

        let runnable = jvm.get_field(&this, "target", "Ljava/lang/Runnable;").await?;

        context.spawn(
            jvm,
            Box::new(ThreadStartProxy {
                jvm: jvm.clone(),
                thread_id: format!("{:?}", &runnable),
                join_event,
                runnable,
            }),
        );

        Ok(())
    }

    async fn join(jvm: &Jvm, _context: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("Thread::join({:?})", &this);

        // TODO we don't have get same field twice
        let raw_join_event: ClassInstanceRef<Array<i8>> = jvm.get_field(&this, "joinEvent", "[B").await?;
        if raw_join_event.is_null() {
            return Ok(()); // already joined or not started
        }

        let join_event: Arc<Event> = jvm.get_rust_object_field(&this, "joinEvent").await?;
        join_event.listen().await;

        jvm.put_field(&mut this, "joinEvent", "[B", None).await?;

        Ok(())
    }

    async fn sleep(_: &Jvm, context: &mut RuntimeContext, duration: i64) -> Result<i32> {
        tracing::debug!("Thread::sleep({:?})", duration);

        context.sleep(Duration::from_millis(duration as _)).await;

        Ok(0)
    }

    async fn r#yield(_: &Jvm, context: &mut RuntimeContext) -> Result<i32> {
        tracing::debug!("Thread::yield()");
        context.r#yield().await;

        Ok(0)
    }

    async fn set_priority(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Thread>, new_priority: i32) -> Result<()> {
        tracing::warn!("stub java.lang.Thread::setPriority({:?}, {:?})", &this, new_priority);

        Ok(())
    }

    async fn current_thread(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        tracing::warn!("stub Thread::currentThread()");

        let thread = jvm.new_class("java/lang/Thread", "(Z)V", (true,)).await?;

        Ok(thread.into())
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
                parent_class: Some("java/lang/Runnable"),
                interfaces: vec![],
                methods: vec![
                    JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                    JavaMethodProto::new("run", "()V", Self::run, Default::default()),
                ],
                fields: vec![JavaFieldProto::new("ran", "Z", Default::default())],
            }
        }

        async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
            jvm.put_field(&mut this, "ran", "Z", false).await?;

            Ok(())
        }

        async fn run(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
            jvm.put_field(&mut this, "ran", "Z", true).await?;

            Ok(())
        }
    }

    #[tokio::test]
    async fn test_thread() -> Result<()> {
        let runtime = TestRuntime::new(BTreeMap::new());
        let jvm = create_test_jvm(runtime.clone()).await?;

        let class = Box::new(ClassDefinitionImpl::from_class_proto(
            TestClass::as_proto(),
            Box::new(runtime.clone()) as Box<_>,
        ));
        jvm.register_class(class, None).await?;

        let test_class = jvm.new_class("TestClass", "()V", ()).await?;

        let thread = jvm
            .new_class("java/lang/Thread", "(Ljava/lang/Runnable;)V", (test_class.clone(),))
            .await?;
        let _: () = jvm.invoke_virtual(&thread, "start", "()V", []).await?;

        let _: () = jvm.invoke_virtual(&thread, "join", "()V", []).await?;

        let ran: bool = jvm.get_field(&test_class, "ran", "Z").await?;
        assert!(ran);

        Ok(())
    }
}
