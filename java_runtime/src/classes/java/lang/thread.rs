use alloc::{boxed::Box, format, string::String, vec};
use core::time::Duration;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::MethodAccessFlags;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{classes::java::lang::Runnable, RuntimeClassProto, RuntimeContext, SpawnCallback};

// class java.lang.Thread
pub struct Thread {}

impl Thread {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/Runnable;)V", Self::init, Default::default()),
                JavaMethodProto::new("start", "()V", Self::start, Default::default()),
                JavaMethodProto::new("sleep", "(J)V", Self::sleep, MethodAccessFlags::NATIVE | MethodAccessFlags::STATIC),
                JavaMethodProto::new("yield", "()V", Self::r#yield, MethodAccessFlags::NATIVE | MethodAccessFlags::STATIC),
                JavaMethodProto::new("setPriority", "(I)V", Self::set_priority, Default::default()),
                JavaMethodProto::new(
                    "getCurrentThread",
                    "()Ljava/lang/Thread;",
                    Self::get_current_thread,
                    MethodAccessFlags::STATIC,
                ),
                // rustjava internal
                JavaMethodProto::new("<init>", "(Z)V", Self::init_internal, Default::default()),
                JavaMethodProto::new("currentThreadId", "()J", Self::current_thread_id, MethodAccessFlags::STATIC),
            ],
            fields: vec![
                JavaFieldProto::new("id", "J", Default::default()),
                JavaFieldProto::new("target", "Ljava/lang/Runnable;", Default::default()),
            ],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, target: ClassInstanceRef<Runnable>) -> Result<()> {
        tracing::debug!("Thread::<init>({:?}, {:?})", &this, &target);

        jvm.put_field(&mut this, "target", "Ljava/lang/Runnable;", target).await?;

        Ok(())
    }

    async fn init_internal(jvm: &Jvm, _context: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, internal: bool) -> Result<()> {
        tracing::debug!("Thread::<init>({:?}, {:?})", &this, internal);

        let id: i64 = jvm.invoke_static("java/lang/Thread", "currentThreadId", "()J", []).await?;
        jvm.put_field(&mut this, "id", "J", id).await?;

        Ok(())
    }

    async fn start(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("Thread::start({:?})", &this);

        struct ThreadStartProxy {
            jvm: Jvm,
            thread_id: String,
            runnable: ClassInstanceRef<Runnable>,
        }

        #[async_trait::async_trait]
        impl SpawnCallback for ThreadStartProxy {
            #[tracing::instrument(name = "thread", fields(thread = self.thread_id), skip_all)]
            async fn call(&self) {
                tracing::trace!("Thread start");

                self.jvm.invoke_virtual(&self.runnable, "run", "()V", []).await.unwrap()
            }
        }

        let runnable = jvm.get_field(&this, "target", "Ljava/lang/Runnable;").await?;

        context.spawn(Box::new(ThreadStartProxy {
            jvm: jvm.clone(),
            thread_id: format!("{:?}", &runnable),
            runnable,
        }));

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

    async fn get_current_thread(_jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Thread>> {
        tracing::debug!("Thread::getCurrentThread()");

        Ok(None.into()) // TODO
    }

    async fn current_thread_id(_jvm: &Jvm, context: &mut RuntimeContext) -> Result<i64> {
        tracing::debug!("Thread::currentThreadId()");

        let id = context.current_task_id();

        Ok(id as _)
    }
}
