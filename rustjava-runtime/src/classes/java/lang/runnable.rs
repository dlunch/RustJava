use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::RuntimeContext;

// interface java.lang.Runnable
#[java_class(name = "java/lang/Runnable", interface)]
pub struct Runnable;

#[java_methods]
impl Runnable {
    #[method(public)]
    async fn run(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()>;
}
