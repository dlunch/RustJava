use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::util::Formatter};

// public interface java.util.Formattable
#[java_class(name = "java/util/Formattable", public, interface, abstract)]
pub struct Formattable;

#[java_methods]
impl Formattable {
    #[method(public)]
    async fn format_to(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        formatter: ClassInstanceRef<Formatter>,
        flags: i32,
        width: i32,
        precision: i32,
    ) -> Result<()>;
}
