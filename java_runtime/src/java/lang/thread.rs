use alloc::{boxed::Box, format, string::String, vec};
use core::time::Duration;

use java_runtime_base::{JavaError, JavaFieldAccessFlag, JavaFieldProto, JavaMethodFlag, JavaMethodProto, JavaResult, JvmClassInstanceHandle};
use jvm::{JavaValue, Jvm, JvmCallback};

use crate::{java::lang::Runnable, JavaClassProto, JavaContext};

// class java.lang.Thread
pub struct Thread {}

impl Thread {
    pub fn as_proto() -> JavaClassProto {
        JavaClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/Runnable;)V", Self::init, JavaMethodFlag::NONE),
                JavaMethodProto::new("start", "()V", Self::start, JavaMethodFlag::NONE),
                JavaMethodProto::new("sleep", "(J)V", Self::sleep, JavaMethodFlag::NATIVE),
                JavaMethodProto::new("yield", "()V", Self::r#yield, JavaMethodFlag::NATIVE),
                JavaMethodProto::new("setPriority", "(I)V", Self::set_priority, JavaMethodFlag::NONE),
            ],
            fields: vec![JavaFieldProto::new("target", "Ljava/lang/Runnable;", JavaFieldAccessFlag::NONE)],
        }
    }

    async fn init(
        jvm: &mut Jvm,
        _: &JavaContext,
        mut this: JvmClassInstanceHandle<Self>,
        target: JvmClassInstanceHandle<Runnable>,
    ) -> JavaResult<()> {
        tracing::debug!("Thread::<init>({:?}, {:?})", &this, &target);

        jvm.put_field(&mut this, "target", "Ljava/lang/Runnable;", target)?;

        Ok(())
    }

    async fn start(jvm: &mut Jvm, context: &JavaContext, this: JvmClassInstanceHandle<Self>) -> JavaResult<()> {
        tracing::debug!("Thread::start({:?})", &this);

        struct ThreadStartProxy {
            thread_id: String,
            runnable: JvmClassInstanceHandle<Runnable>,
        }

        #[async_trait::async_trait(?Send)]
        impl JvmCallback for ThreadStartProxy {
            #[tracing::instrument(name = "thread", fields(thread = self.thread_id), skip_all)]
            async fn call(&self, jvm: &mut Jvm, _: Box<[JavaValue]>) -> Result<JavaValue, JavaError> {
                tracing::trace!("Thread start");

                jvm.invoke_virtual(&self.runnable, "java/lang/Runnable", "run", "()V", []).await?;

                Ok(JavaValue::Void)
            }
        }

        let runnable = jvm.get_field(&this, "target", "Ljava/lang/Runnable;")?;

        context.spawn(Box::new(ThreadStartProxy {
            thread_id: format!("{:?}", &runnable),
            runnable,
        }));

        Ok(())
    }

    async fn sleep(_: &mut Jvm, context: &JavaContext, duration: i64) -> JavaResult<i32> {
        tracing::debug!("Thread::sleep({:?})", duration);

        context.sleep(Duration::from_millis(duration as _)).await;

        Ok(0)
    }

    async fn r#yield(_: &mut Jvm, context: &JavaContext) -> JavaResult<i32> {
        tracing::debug!("Thread::yield()");
        context.r#yield().await;

        Ok(0)
    }

    async fn set_priority(_: &mut Jvm, _: &JavaContext, this: JvmClassInstanceHandle<Thread>, new_priority: i32) -> JavaResult<()> {
        tracing::warn!("stub java.lang.Thread::setPriority({:?}, {:?})", &this, new_priority);

        Ok(())
    }
}
