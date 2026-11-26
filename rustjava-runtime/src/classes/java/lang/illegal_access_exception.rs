use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::String};

// class java.lang.IllegalAccessException
#[java_class(name = "java/lang/IllegalAccessException", extends = "java/lang/Exception", public)]
pub struct IllegalAccessException;

#[java_methods]
impl IllegalAccessException {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.IllegalAccessException::<init>({this:?})");
        jvm.invoke_special(&this, "java/lang/Exception", "<init>", "()V", ()).await
    }

    #[method(constructor, public)]
    async fn init_with_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.IllegalAccessException::<init>({this:?}, {message:?})");
        jvm.invoke_special(&this, "java/lang/Exception", "<init>", "(Ljava/lang/String;)V", (message,))
            .await
    }
}
