use alloc::{boxed::Box, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{
    runtime::{JavaLangClassLoader, JavaLangString},
    ClassDefinition, ClassInstanceRef, Jvm, Result,
};

use crate::{
    classes::java::{
        io::InputStream,
        lang::{ClassLoader, String},
    },
    RuntimeClassProto, RuntimeContext,
};

// class java.lang.Class
pub struct Class {}

impl Class {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Class",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getName", "()Ljava/lang/String;", Self::get_name, Default::default()),
                JavaMethodProto::new(
                    "getResourceAsStream",
                    "(Ljava/lang/String;)Ljava/io/InputStream;",
                    Self::get_resource_as_stream,
                    Default::default(),
                ),
            ],
            fields: vec![
                JavaFieldProto::new("raw", "[B", Default::default()), // raw rust pointer of Box<dyn Class>
                JavaFieldProto::new("classLoader", "Ljava/lang/ClassLoader;", Default::default()),
            ],
        }
    }

    async fn init(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.Class::<init>({:?})", &this);

        Ok(())
    }

    async fn get_name(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.Class::getName({:?})", &this);

        let rust_class: Box<dyn ClassDefinition> = jvm.get_rust_object_field(&this, "raw").await?;
        let result = JavaLangString::from_rust_string(jvm, &rust_class.name()).await?;

        Ok(result.into())
    }

    async fn get_resource_as_stream(
        jvm: &Jvm,
        _context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<InputStream>> {
        tracing::debug!("java.lang.Class::getResourceAsStream({:?}, {:?})", &this, &name);

        let class_loader: ClassInstanceRef<ClassLoader> = jvm.get_field(&this, "classLoader", "Ljava/lang/ClassLoader;").await?;

        let class_loader = if class_loader.is_null() {
            // TODO ClassLoader.getSystemResourceAsStream?
            JavaLangClassLoader::get_system_class_loader(jvm).await?
        } else {
            class_loader.into()
        };

        jvm.invoke_virtual(&class_loader, "getResourceAsStream", "(Ljava/lang/String;)Ljava/io/InputStream;", (name,))
            .await
    }
}

#[cfg(test)]
mod test {
    use jvm::{runtime::JavaLangClass, Result};

    use crate::test::test_jvm;

    #[tokio::test]
    async fn test_class() -> Result<()> {
        let jvm = test_jvm().await?;

        let java_class = jvm.resolve_class("java/lang/String").await?.java_class(&jvm).await?;

        let rust_class = JavaLangClass::to_rust_class(&jvm, &java_class).await?;
        assert_eq!(rust_class.name(), "java/lang/String");

        // try call to_rust_class twice to test if box is not dropped
        let rust_class = JavaLangClass::to_rust_class(&jvm, &java_class).await?;
        assert_eq!(rust_class.name(), "java/lang/String");

        Ok(())
    }
}
