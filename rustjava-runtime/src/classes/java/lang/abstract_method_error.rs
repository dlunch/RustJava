use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::String};

// class java.lang.AbstractMethodError
#[java_class(name = "java/lang/AbstractMethodError", extends = "java/lang/IncompatibleClassChangeError", public)]
pub struct AbstractMethodError;

#[java_methods]
impl AbstractMethodError {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.AbstractMethodError::<init>({this:?})");

        let _: () = jvm
            .invoke_special(&this, "java/lang/IncompatibleClassChangeError", "<init>", "()V", ())
            .await?;

        Ok(())
    }

    #[method(constructor, public)]
    async fn init_with_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.AbstractMethodError::<init>({this:?}, {message:?})");

        let _: () = jvm
            .invoke_special(
                &this,
                "java/lang/IncompatibleClassChangeError",
                "<init>",
                "(Ljava/lang/String;)V",
                (message,),
            )
            .await?;

        Ok(())
    }
}
