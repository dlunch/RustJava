use alloc::boxed::Box;

use crate::{class::Class, class_instance::ClassInstance, jvm::Jvm, JvmResult};

pub struct JavaLangClass {}

impl JavaLangClass {
    pub fn to_rust_class(jvm: &Jvm, instance: Box<dyn ClassInstance>) -> JvmResult<Box<dyn Class>> {
        let rust_class = jvm.get_rust_object_field(&instance, "raw")?;

        Ok(rust_class)
    }
}
