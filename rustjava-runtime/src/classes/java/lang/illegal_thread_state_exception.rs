use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::String};

// class java.lang.IllegalThreadStateException
#[java_class(name = "java/lang/IllegalThreadStateException", extends = "java/lang/IllegalArgumentException", public)]
pub struct IllegalThreadStateException;

#[java_methods]
impl IllegalThreadStateException {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.IllegalThreadStateException::<init>({this:?})");
        jvm.invoke_special(&this, "java/lang/IllegalArgumentException", "<init>", "()V", ()).await
    }

    #[method(constructor, public)]
    async fn init_with_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.IllegalThreadStateException::<init>({this:?}, {message:?})");
        jvm.invoke_special(&this, "java/lang/IllegalArgumentException", "<init>", "(Ljava/lang/String;)V", (message,))
            .await
    }
}
