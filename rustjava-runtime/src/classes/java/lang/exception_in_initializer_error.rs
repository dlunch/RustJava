use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::lang::{String, Throwable},
};

// class java.lang.ExceptionInInitializerError
#[java_class(name = "java/lang/ExceptionInInitializerError", extends = "java/lang/LinkageError", public)]
pub struct ExceptionInInitializerError;

#[java_methods]
impl ExceptionInInitializerError {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.ExceptionInInitializerError::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/LinkageError", "<init>", "()V", ()).await?;

        Ok(())
    }

    #[method(constructor, public)]
    async fn init_with_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.ExceptionInInitializerError::<init>({this:?}, {message:?})");

        let _: () = jvm
            .invoke_special(&this, "java/lang/LinkageError", "<init>", "(Ljava/lang/String;)V", (message,))
            .await?;

        Ok(())
    }

    #[method(constructor, public)]
    async fn init_with_cause(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, cause: ClassInstanceRef<Throwable>) -> Result<()> {
        tracing::debug!("java.lang.ExceptionInInitializerError::<init>({this:?}, {cause:?})");

        // unlike Throwable(Throwable), this keeps detailMessage null so toString is just the class name
        let _: () = jvm.invoke_special(&this, "java/lang/LinkageError", "<init>", "()V", ()).await?;

        jvm.put_field(
            &mut this,
            "java/lang/ExceptionInInitializerError",
            "cause",
            "Ljava/lang/Throwable;",
            cause,
        )
        .await?;

        Ok(())
    }

    #[method(public)]
    async fn get_exception(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Throwable>> {
        tracing::debug!("java.lang.ExceptionInInitializerError::getException({this:?})");

        jvm.get_field(&this, "java/lang/ExceptionInInitializerError", "cause", "Ljava/lang/Throwable;")
            .await
    }
}
