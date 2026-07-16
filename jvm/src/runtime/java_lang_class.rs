use alloc::{boxed::Box, string::String, vec::Vec};

use bytemuck::cast_vec;

use crate::{Array, ClassInstanceRef, Result, class_definition::ClassDefinition, class_instance::ClassInstance, jvm::Jvm};

pub struct JavaLangClass;

impl JavaLangClass {
    pub async fn from_rust_primitive(jvm: &Jvm, name: &str) -> Result<Box<dyn ClassInstance>> {
        let mut java_class = jvm.new_class("java/lang/Class", "()V", ()).await?;
        let mut name_bytes = jvm.instantiate_array("B", name.len()).await?;
        let bytes: Vec<i8> = cast_vec(name.as_bytes().to_vec());
        jvm.store_array(&mut name_bytes, 0, bytes).await?;
        jvm.put_field(&mut java_class, "nameBytes", "[B", name_bytes).await?;

        Ok(java_class)
    }

    #[allow(clippy::borrowed_box)]
    pub async fn name(jvm: &Jvm, this: &Box<dyn ClassInstance>) -> Result<String> {
        let name_bytes: ClassInstanceRef<Array<i8>> = jvm.get_field(this, "nameBytes", "[B").await?;
        let len = jvm.array_length(&name_bytes).await?;
        let name_bytes_vec: Vec<i8> = jvm.load_array(&name_bytes, 0, len).await?;
        match String::from_utf8(cast_vec(name_bytes_vec)) {
            Ok(name) => Ok(name),
            Err(_) => Err(jvm.exception("java/lang/NoClassDefFoundError", "invalid class name").await),
        }
    }

    #[allow(clippy::borrowed_box)]
    pub async fn to_rust_class(jvm: &Jvm, this: &Box<dyn ClassInstance>) -> Result<Box<dyn ClassDefinition>> {
        let class_name = Self::name(jvm, this).await?;
        if let Some(class) = jvm.get_class(&class_name) {
            Ok(class.definition)
        } else {
            Err(jvm.exception("java/lang/NoClassDefFoundError", &class_name).await)
        }
    }

    pub async fn from_rust_class(
        jvm: &Jvm,
        rust_class: Box<dyn ClassDefinition>,
        class_loader: Option<Box<dyn ClassInstance>>,
    ) -> Result<Box<dyn ClassInstance>> {
        let mut java_class = jvm.new_class("java/lang/Class", "()V", ()).await?;

        let class_name = rust_class.name();
        let mut name_bytes = jvm.instantiate_array("B", class_name.len()).await?;
        let bytes: Vec<i8> = cast_vec(class_name.into_bytes());
        jvm.store_array(&mut name_bytes, 0, bytes).await?;
        jvm.put_field(&mut java_class, "nameBytes", "[B", name_bytes).await?;

        jvm.put_field(&mut java_class, "classLoader", "Ljava/lang/ClassLoader;", class_loader)
            .await?;

        Ok(java_class)
    }

    #[allow(clippy::borrowed_box)]
    pub async fn class_loader(jvm: &Jvm, this: &Box<dyn ClassInstance>) -> Result<Option<Box<dyn ClassInstance>>> {
        jvm.get_field(this, "classLoader", "Ljava/lang/ClassLoader;").await
    }
}
