use jvm::{ClassInstanceRef, JavaChar, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::CharSequence};

// public interface java.lang.Appendable
#[java_class(name = "java/lang/Appendable", public, interface, abstract)]
pub struct Appendable;

#[java_methods]
impl Appendable {
    #[method(name = "append", public)]
    async fn append_char_sequence(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        sequence: ClassInstanceRef<CharSequence>,
    ) -> Result<ClassInstanceRef<Self>>;

    #[method(name = "append", public)]
    async fn append_char_sequence_range(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        sequence: ClassInstanceRef<CharSequence>,
        start: i32,
        end: i32,
    ) -> Result<ClassInstanceRef<Self>>;

    #[method(name = "append", public)]
    async fn append_char(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, character: JavaChar) -> Result<ClassInstanceRef<Self>>;
}
