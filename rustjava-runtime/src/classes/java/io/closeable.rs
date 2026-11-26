use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::RuntimeContext;

// public interface java.io.Closeable
#[java_class(name = "java/io/Closeable", public, interface, abstract)]
pub struct Closeable;

#[java_methods]
impl Closeable {
    #[method(public)]
    async fn close(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()>;
}
