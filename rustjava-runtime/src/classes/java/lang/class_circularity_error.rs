use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::String};

// class java.lang.ClassCircularityError
#[java_class(name = "java/lang/ClassCircularityError", extends = "java/lang/LinkageError", public)]
pub struct ClassCircularityError;

#[java_methods]
impl ClassCircularityError {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.ClassCircularityError::<init>({this:?})");

        jvm.invoke_special(&this, "java/lang/LinkageError", "<init>", "()V", ()).await
    }

    #[method(constructor, public)]
    async fn init_with_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.ClassCircularityError::<init>({this:?}, {message:?})");

        jvm.invoke_special(&this, "java/lang/LinkageError", "<init>", "(Ljava/lang/String;)V", (message,))
            .await
    }
}
