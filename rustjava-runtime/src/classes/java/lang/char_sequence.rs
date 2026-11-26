use jvm::{ClassInstanceRef, JavaChar, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::String};

// public interface java.lang.CharSequence
#[java_class(name = "java/lang/CharSequence", public, interface, abstract)]
pub struct CharSequence;

#[java_methods]
impl CharSequence {
    #[method(public)]
    async fn length(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32>;

    #[method(public)]
    async fn char_at(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<JavaChar>;

    #[method(public)]
    async fn sub_sequence(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, start: i32, end: i32) -> Result<ClassInstanceRef<Self>>;

    #[method(public)]
    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>>;
}
