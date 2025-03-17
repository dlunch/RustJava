use alloc::boxed::Box;

use crate::{Result, class_instance::ClassInstance, jvm::Jvm};

use super::JavaLangString;

pub struct JavaLangClassLoader;

impl JavaLangClassLoader {
    pub async fn get_system_class_loader(jvm: &Jvm) -> Result<Box<dyn ClassInstance>> {
        let system_class_loader = jvm
            .invoke_static("java/lang/ClassLoader", "getSystemClassLoader", "()Ljava/lang/ClassLoader;", ())
            .await?;

        Ok(system_class_loader)
    }

    #[allow(clippy::borrowed_box)]
    pub async fn load_class(jvm: &Jvm, this: &Box<dyn ClassInstance>, class_name: &str) -> Result<Option<Box<dyn ClassInstance>>> {
        let java_class_name = JavaLangString::from_rust_string(jvm, class_name).await?;

        let java_class: Option<Box<dyn ClassInstance>> = jvm
            .invoke_virtual(this, "loadClass", "(Ljava/lang/String;)Ljava/lang/Class;", (java_class_name,))
            .await?;

        Ok(java_class)
    }

    #[allow(clippy::borrowed_box)]
    pub async fn get_resource_as_stream(jvm: &Jvm, this: &Box<dyn ClassInstance>, name: &str) -> Result<Option<Box<dyn ClassInstance>>> {
        let java_name = JavaLangString::from_rust_string(jvm, name).await?;

        let java_resource: Option<Box<dyn ClassInstance>> = jvm
            .invoke_virtual(this, "getResourceAsStream", "(Ljava/lang/String;)Ljava/io/InputStream;", (java_name,))
            .await?;

        Ok(java_resource)
    }
}
