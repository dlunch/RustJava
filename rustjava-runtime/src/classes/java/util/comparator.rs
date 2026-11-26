use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::Object};

// interface java.util.Comparator
#[java_class(name = "java/util/Comparator", public, interface, abstract)]
pub struct Comparator;

#[java_methods]
impl Comparator {
    #[method(public)]
    async fn compare(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        o1: ClassInstanceRef<Object>,
        o2: ClassInstanceRef<Object>,
    ) -> Result<i32>;

    #[method(public)]
    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool>;
}
