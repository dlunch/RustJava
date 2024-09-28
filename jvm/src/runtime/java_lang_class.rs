use alloc::boxed::Box;

use crate::{class_definition::ClassDefinition, class_instance::ClassInstance, jvm::Jvm, Result};

pub struct JavaLangClass;

impl JavaLangClass {
    #[allow(clippy::borrowed_box)]
    pub async fn to_rust_class(jvm: &Jvm, this: &Box<dyn ClassInstance>) -> Result<Box<dyn ClassDefinition>> {
        let rust_class = jvm.get_rust_object_field(this, "raw").await?;

        Ok(rust_class)
    }

    pub async fn from_rust_class(
        jvm: &Jvm,
        rust_class: Box<dyn ClassDefinition>,
        class_loader: Option<Box<dyn ClassInstance>>,
    ) -> Result<Box<dyn ClassInstance>> {
        let mut java_class = jvm.new_class("java/lang/Class", "()V", ()).await?;

        jvm.put_rust_object_field(&mut java_class, "raw", rust_class).await?;
        jvm.put_field(&mut java_class, "classLoader", "Ljava/lang/ClassLoader;", class_loader)
            .await?;

        Ok(java_class)
    }

    #[allow(clippy::borrowed_box)]
    pub async fn class_loader(jvm: &Jvm, this: &Box<dyn ClassInstance>) -> Result<Option<Box<dyn ClassInstance>>> {
        jvm.get_field(this, "classLoader", "Ljava/lang/ClassLoader;").await
    }
}
