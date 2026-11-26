use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::RuntimeContext;

// class java.lang.ThreadDeath
#[java_class(name = "java/lang/ThreadDeath", extends = "java/lang/Error", public)]
pub struct ThreadDeath;

#[java_methods]
impl ThreadDeath {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.ThreadDeath::<init>({this:?})");

        jvm.invoke_special(&this, "java/lang/Error", "<init>", "()V", ()).await
    }
}
