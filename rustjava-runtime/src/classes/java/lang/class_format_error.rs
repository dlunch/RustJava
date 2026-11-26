use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::String};

// class java.lang.ClassFormatError
#[java_class(name = "java/lang/ClassFormatError", extends = "java/lang/LinkageError", public)]
pub struct ClassFormatError;

#[java_methods]
impl ClassFormatError {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.ClassFormatError::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/LinkageError", "<init>", "()V", ()).await?;

        Ok(())
    }

    #[method(constructor, public)]
    async fn init_with_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.ClassFormatError::<init>({this:?}, {message:?})");

        let _: () = jvm
            .invoke_special(&this, "java/lang/LinkageError", "<init>", "(Ljava/lang/String;)V", (message,))
            .await?;

        Ok(())
    }
}
