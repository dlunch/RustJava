use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::String};

// class java.lang.IllegalStateException
#[java_class(name = "java/lang/IllegalStateException", extends = "java/lang/RuntimeException", public)]
pub struct IllegalStateException;

#[java_methods]
impl IllegalStateException {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.IllegalStateException::<init>({this:?})");

        jvm.invoke_special(&this, "java/lang/RuntimeException", "<init>", "()V", ()).await
    }

    #[method(constructor, public)]
    async fn init_with_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.IllegalStateException::<init>({this:?}, {message:?})");

        jvm.invoke_special(&this, "java/lang/RuntimeException", "<init>", "(Ljava/lang/String;)V", (message,))
            .await
    }
}
