use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::String};

// class java.lang.IllegalAccessError
#[java_class(name = "java/lang/IllegalAccessError", extends = "java/lang/IncompatibleClassChangeError", public)]
pub struct IllegalAccessError;

#[java_methods]
impl IllegalAccessError {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.IllegalAccessError::<init>({this:?})");

        jvm.invoke_special(&this, "java/lang/IncompatibleClassChangeError", "<init>", "()V", ())
            .await
    }

    #[method(constructor, public)]
    async fn init_with_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.IllegalAccessError::<init>({this:?}, {message:?})");

        jvm.invoke_special(
            &this,
            "java/lang/IncompatibleClassChangeError",
            "<init>",
            "(Ljava/lang/String;)V",
            (message,),
        )
        .await
    }
}
