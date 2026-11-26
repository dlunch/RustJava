use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::String};

// class java.lang.IncompatibleClassChangeError
#[java_class(name = "java/lang/IncompatibleClassChangeError", extends = "java/lang/LinkageError", public)]
pub struct IncompatibleClassChangeError;

#[java_methods]
impl IncompatibleClassChangeError {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.IncompatibleClassChangeError::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/LinkageError", "<init>", "()V", ()).await?;

        Ok(())
    }

    #[method(constructor, public)]
    async fn init_with_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.IncompatibleClassChangeError::<init>({this:?}, {message:?})");

        let _: () = jvm
            .invoke_special(&this, "java/lang/LinkageError", "<init>", "(Ljava/lang/String;)V", (message,))
            .await?;

        Ok(())
    }
}
