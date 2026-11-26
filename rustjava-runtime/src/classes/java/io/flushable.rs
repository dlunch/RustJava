use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::RuntimeContext;

// public interface java.io.Flushable
#[java_class(name = "java/io/Flushable", public, interface, abstract)]
pub struct Flushable;

#[java_methods]
impl Flushable {
    #[method(public)]
    async fn flush(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()>;
}
