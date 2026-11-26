use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::String};

// class java.lang.Error
#[java_class(name = "java/lang/Error", extends = "java/lang/Throwable", public)]
pub struct Error;

#[java_methods]
impl Error {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.Error::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Throwable", "<init>", "()V", ()).await?;

        Ok(())
    }

    #[method(constructor, public)]
    async fn init_with_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.Error::<init>({this:?}, {message:?})");

        let _: () = jvm
            .invoke_special(&this, "java/lang/Throwable", "<init>", "(Ljava/lang/String;)V", (message,))
            .await?;

        Ok(())
    }
}
