use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::lang::{String, Throwable},
};

// class java.lang.RuntimeException
#[java_class(name = "java/lang/RuntimeException", extends = "java/lang/Exception", public)]
pub struct RuntimeException;

#[java_methods]
impl RuntimeException {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.RuntimeException::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Exception", "<init>", "()V", ()).await?;

        Ok(())
    }

    #[method(constructor, public)]
    async fn init_with_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.RuntimeException::<init>({this:?}, {message:?})");

        let _: () = jvm
            .invoke_special(&this, "java/lang/Exception", "<init>", "(Ljava/lang/String;)V", (message,))
            .await?;

        Ok(())
    }

    #[method(constructor, public)]
    async fn init_with_cause(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, cause: ClassInstanceRef<Throwable>) -> Result<()> {
        tracing::debug!("java.lang.RuntimeException::<init>({this:?}, {cause:?})");

        let _: () = jvm
            .invoke_special(&this, "java/lang/Exception", "<init>", "(Ljava/lang/Throwable;)V", (cause,))
            .await?;

        Ok(())
    }

    #[method(constructor, public)]
    async fn init_with_message_and_cause(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        message: ClassInstanceRef<String>,
        cause: ClassInstanceRef<Throwable>,
    ) -> Result<()> {
        tracing::debug!("java.lang.RuntimeException::<init>({this:?}, {message:?}, {cause:?})");

        let _: () = jvm
            .invoke_special(
                &this,
                "java/lang/Exception",
                "<init>",
                "(Ljava/lang/String;Ljava/lang/Throwable;)V",
                (message, cause),
            )
            .await?;

        Ok(())
    }
}
