use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::String};

// class java.lang.UnsupportedClassVersionError
#[java_class(name = "java/lang/UnsupportedClassVersionError", extends = "java/lang/ClassFormatError", public)]
pub struct UnsupportedClassVersionError;

#[java_methods]
impl UnsupportedClassVersionError {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.UnsupportedClassVersionError::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/ClassFormatError", "<init>", "()V", ()).await?;

        Ok(())
    }

    #[method(constructor, public)]
    async fn init_with_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.UnsupportedClassVersionError::<init>({this:?}, {message:?})");

        let _: () = jvm
            .invoke_special(&this, "java/lang/ClassFormatError", "<init>", "(Ljava/lang/String;)V", (message,))
            .await?;

        Ok(())
    }
}
