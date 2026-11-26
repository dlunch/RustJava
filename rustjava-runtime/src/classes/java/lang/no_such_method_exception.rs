use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::String};

// class java.lang.NoSuchMethodException
#[java_class(name = "java/lang/NoSuchMethodException", extends = "java/lang/Exception", public)]
pub struct NoSuchMethodException;

#[java_methods]
impl NoSuchMethodException {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.NoSuchMethodException::<init>({this:?})");

        jvm.invoke_special(&this, "java/lang/Exception", "<init>", "()V", ()).await
    }

    #[method(constructor, public)]
    async fn init_with_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.NoSuchMethodException::<init>({this:?}, {message:?})");

        jvm.invoke_special(&this, "java/lang/Exception", "<init>", "(Ljava/lang/String;)V", (message,))
            .await
    }
}
