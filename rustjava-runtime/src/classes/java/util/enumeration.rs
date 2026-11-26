use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::Object};

// interface java.util.Enumeration
#[java_class(name = "java/util/Enumeration", interface)]
pub struct Enumeration;

#[java_methods]
impl Enumeration {
    #[method(public)]
    async fn has_more_elements(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool>;

    #[method(public)]
    async fn next_element(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>>;
}
