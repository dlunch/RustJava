use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::String};

// class java.lang.LinkageError
#[java_class(name = "java/lang/LinkageError", extends = "java/lang/Error", public)]
pub struct LinkageError;

#[java_methods]
impl LinkageError {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.LinkageError::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Error", "<init>", "()V", ()).await?;

        Ok(())
    }

    #[method(constructor, public)]
    async fn init_with_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.LinkageError::<init>({this:?}, {message:?})");

        let _: () = jvm
            .invoke_special(&this, "java/lang/Error", "<init>", "(Ljava/lang/String;)V", (message,))
            .await?;

        Ok(())
    }
}
