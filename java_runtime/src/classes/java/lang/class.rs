use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{
    runtime::{JavaLangClass, JavaLangClassLoader, JavaLangString},
    ClassInstanceRef, Jvm, Result,
};

use crate::{
    classes::java::{
        io::InputStream,
        lang::{ClassLoader, String},
    },
    RuntimeClassProto, RuntimeContext,
};

// class java.lang.Class
pub struct Class;

impl Class {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Class",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getName", "()Ljava/lang/String;", Self::get_name, Default::default()),
                JavaMethodProto::new("isAssignableFrom", "(Ljava/lang/Class;)Z", Self::is_assignable_from, Default::default()),
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

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.Class::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn get_name(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.Class::getName({:?})", &this);

        let rust_class = JavaLangClass::to_rust_class(jvm, &this).await?;
        let result = JavaLangString::from_rust_string(jvm, &rust_class.name()).await?;

        Ok(result.into())
    }

    async fn is_assignable_from(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.lang.Class::isAssignableFrom({:?}, {:?})", &this, &other);

        let rust_class = JavaLangClass::to_rust_class(jvm, &this).await?;
        let other_rust_class = JavaLangClass::to_rust_class(jvm, &other).await?;

        Ok(jvm.is_inherited_from(&*other_rust_class, &rust_class.name()))
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

        let java_class = jvm.resolve_class("java/lang/String").await?.java_class();

        let rust_class = JavaLangClass::to_rust_class(&jvm, &java_class).await?;
        assert_eq!(rust_class.name(), "java/lang/String");

        // try call to_rust_class twice to test if box is not dropped
        let rust_class = JavaLangClass::to_rust_class(&jvm, &java_class).await?;
        assert_eq!(rust_class.name(), "java/lang/String");

        Ok(())
    }

    #[tokio::test]
    async fn test_is_assignable_from() -> Result<()> {
        let jvm = test_jvm().await?;

        let string_class = jvm.resolve_class("java/lang/String").await?.java_class();
        let object_class = jvm.resolve_class("java/lang/Object").await?.java_class();

        let result: bool = jvm
            .invoke_virtual(&object_class, "isAssignableFrom", "(Ljava/lang/Class;)Z", (string_class.clone(),))
            .await?;
        assert!(result);

        let thread_class = jvm.resolve_class("java/lang/Thread").await?.java_class();

        let result: bool = jvm
            .invoke_virtual(&string_class, "isAssignableFrom", "(Ljava/lang/Class;)Z", (thread_class,))
            .await?;
        assert!(!result);

        Ok(())
    }
}
