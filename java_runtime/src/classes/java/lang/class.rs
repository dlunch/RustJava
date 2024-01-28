use alloc::{boxed::Box, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto, JavaResult};
use jvm::{Class as JvmClass, ClassInstanceRef, Jvm};

use crate::{
    classes::java::{io::InputStream, lang::String},
    RuntimeClassProto, RuntimeContext,
};

// class java.lang.Class
pub struct Class {}

impl Class {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
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

    async fn init(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> JavaResult<()> {
        tracing::debug!("java.lang.Class::<init>({:?})", &this);

        Ok(())
    }

    async fn get_resource_as_stream(
        jvm: &Jvm,
        _context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> JavaResult<ClassInstanceRef<InputStream>> {
        tracing::debug!("java.lang.Class::getResourceAsStream({:?}, {:?})", &this, &name);

        let class_loader = jvm.get_field(&this, "classLoader", "Ljava/lang/ClassLoader;")?;

        jvm.invoke_virtual(&class_loader, "getResourceAsStream", "(Ljava/lang/String;)Ljava/io/InputStream;", (name,))
            .await
    }

    pub fn to_rust_class(jvm: &Jvm, java_class: ClassInstanceRef<Self>) -> JavaResult<Box<dyn JvmClass>> {
        jvm.get_rust_object_field(&java_class, "raw")
    }
}

#[cfg(test)]
mod test {
    use crate::test::test_jvm;

    use super::Class;

    #[futures_test::test]
    async fn test_class() -> anyhow::Result<()> {
        let jvm = test_jvm().await?;

        let java_class = jvm.get_java_class("java/lang/String").await?.unwrap();

        let rust_class = Class::to_rust_class(&jvm, java_class.clone().into())?;
        assert_eq!(rust_class.name(), "java/lang/String");

        // try call to_rust_class twice to test if box is not dropped
        let rust_class = Class::to_rust_class(&jvm, java_class.into())?;
        assert_eq!(rust_class.name(), "java/lang/String");

        Ok(())
    }
}
