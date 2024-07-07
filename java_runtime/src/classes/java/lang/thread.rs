use alloc::{boxed::Box, format, string::String, vec};
use core::time::Duration;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::MethodAccessFlags;
use jvm::{ClassInstanceRef, JavaValue, Jvm, JvmCallback, Result};

use crate::{classes::java::lang::Runnable, RuntimeClassProto, RuntimeContext};

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
                JavaMethodProto::new("attach", "()Ljava/lang/Thread;", Self::attach, MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "getCurrentNativeThreadId",
                    "()J",
                    Self::get_current_native_thread_id,
                    MethodAccessFlags::STATIC,
                ),
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

    async fn start(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("Thread::start({:?})", &this);

        struct ThreadStartProxy {
            thread_id: String,
            runnable: ClassInstanceRef<Runnable>,
        }

        #[async_trait::async_trait]
        impl JvmCallback for ThreadStartProxy {
            #[tracing::instrument(name = "thread", fields(thread = self.thread_id), skip_all)]
            async fn call(&self, jvm: &Jvm, _: Box<[JavaValue]>) -> Result<JavaValue> {
                tracing::trace!("Thread start");

                jvm.invoke_virtual(&self.runnable, "run", "()V", []).await?;

                Ok(JavaValue::Void)
            }
        }

        let runnable = jvm.get_field(&this, "target", "Ljava/lang/Runnable;").await?;

        context.spawn(Box::new(ThreadStartProxy {
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

    async fn attach(_jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("Thread::attach()");

        Ok(None.into()) // TODO
    }

    async fn get_current_native_thread_id(_: &Jvm, runtime: &mut RuntimeContext) -> Result<i64> {
        tracing::debug!("Thread::getCurrentNativeThreadId()");

        Ok(runtime.current_task_id() as _)
    }
}
