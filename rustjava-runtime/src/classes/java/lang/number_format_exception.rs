use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::String};

// class java/lang/NumberFormatException
#[java_class(name = "java/lang/NumberFormatException", extends = "java/lang/IllegalArgumentException", public)]
pub struct NumberFormatException;

#[java_methods]
impl NumberFormatException {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.NumberFormatException::<init>({this:?})");

        let _: () = jvm
            .invoke_special(&this, "java/lang/IllegalArgumentException", "<init>", "()V", ())
            .await?;

        Ok(())
    }

    #[method(constructor, public)]
    async fn init_with_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.NumberFormatException::<init>({this:?}, {message:?})");

        let _: () = jvm
            .invoke_special(&this, "java/lang/IllegalArgumentException", "<init>", "(Ljava/lang/String;)V", (message,))
            .await?;

        Ok(())
    }
}
