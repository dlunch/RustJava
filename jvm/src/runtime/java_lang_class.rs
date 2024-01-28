use alloc::boxed::Box;

use crate::{class::Class, class_instance::ClassInstance, jvm::Jvm, JvmResult};

pub struct JavaLangClass {}

impl JavaLangClass {
    pub fn to_rust_class(jvm: &Jvm, this: Box<dyn ClassInstance>) -> JvmResult<Box<dyn Class>> {
        let rust_class = jvm.get_rust_object_field(&this, "raw")?;

        Ok(rust_class)
    }

    pub async fn from_rust_class(
        jvm: &Jvm,
        rust_class: Box<dyn Class>,
        class_loader: Option<Box<dyn ClassInstance>>,
    ) -> JvmResult<Box<dyn ClassInstance>> {
        let mut java_class = jvm.new_class("java/lang/Class", "()V", ()).await?;

        jvm.put_rust_object_field(&mut java_class, "raw", rust_class).await?;
        jvm.put_field(&mut java_class, "classLoader", "Ljava/lang/ClassLoader;", class_loader)?;

        Ok(java_class)
    }
}
