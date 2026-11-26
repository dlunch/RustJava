use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::Object};

// interface java.lang.Comparable
#[java_class(name = "java/lang/Comparable", public, interface, abstract)]
pub struct Comparable;

#[java_methods]
impl Comparable {
    #[method(public)]
    async fn compare_to(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<i32>;
}
