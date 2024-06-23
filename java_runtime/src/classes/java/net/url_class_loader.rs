use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{
    classes::java::{
        lang::{Class, String},
        net::URL,
    },
    RuntimeClassProto, RuntimeContext,
};

// class java.net.URLClassLoader
pub struct URLClassLoader {}

impl URLClassLoader {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/ClassLoader"), // TODO java.security.SecureClassLoader
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "([Ljava/net/URL;)V", Self::init, Default::default()),
                JavaMethodProto::new("findClass", "(Ljava/lang/String;)Ljava/lang/Class;", Self::find_class, Default::default()),
                JavaMethodProto::new(
                    "findResource",
                    "(Ljava/lang/String;)Ljava/net/URL;",
                    Self::find_resource,
                    Default::default(),
                ),
            ],
            fields: vec![JavaFieldProto::new("urls", "[Ljava/net/URL;", Default::default())],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, urls: ClassInstanceRef<Array<URL>>) -> Result<()> {
        tracing::debug!("java.net.URLClassLoader::<init>({:?}, {:?})", &this, &urls);

        jvm.invoke_special(&this, "java/lang/ClassLoader", "<init>", "(Ljava/lang/ClassLoader;)V", (None,))
            .await?;

        Ok(())
    }

    async fn find_class(
        _jvm: &Jvm,
        _runtime: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Class>> {
        tracing::debug!("java.net.URLClassLoader::findClass({:?}, {:?})", &this, name);

        Ok(None.into())
    }

    async fn find_resource(
        _jvm: &Jvm,
        _runtime: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<URL>> {
        tracing::debug!("java.net.URLClassLoader::findResource({:?}, {:?})", &this, name);

        Ok(None.into())
    }
}

#[cfg(test)]
mod test {
    use alloc::vec;

    use bytemuck::cast_vec;

    use jvm::{runtime::JavaLangString, Result};

    use crate::test::test_jvm_filesystem;

    // #[futures_test::test]
    async fn test_jar_loading() -> Result<()> {
        let jar = include_bytes!("../../../../../test_data/test.jar");
        let filesystem = [("test.jar".into(), jar.to_vec())].into_iter().collect();
        let jvm = test_jvm_filesystem(filesystem).await?;

        let url = JavaLangString::from_rust_string(&jvm, "file:test.jar").await?;
        let mut urls = jvm.instantiate_array("Ljava/net/URL;", 1).await?;
        jvm.store_array(&mut urls, 0, vec![url]).await?;

        let class_loader = jvm.new_class("java/net/URLClassLoader", "([Ljava/net/URL;)V", (urls,)).await?;

        let resource_name = JavaLangString::from_rust_string(&jvm, "test.txt").await?;
        let resource = jvm
            .invoke_virtual(&class_loader, "findResource", "(Ljava/lang/String;)Ljava/net/URL;", (resource_name,))
            .await?;

        let stream = jvm.invoke_virtual(&resource, "openStream", "()Ljava/io/InputStream;", ()).await?;

        let buf = jvm.instantiate_array("B", 17).await?;
        let len: i32 = jvm.invoke_virtual(&stream, "read", "([B)I", (buf.clone(),)).await?;

        let data = jvm.load_byte_array(&buf, 0, len as _).await?;

        assert_eq!(cast_vec::<i8, u8>(data), b"test content\n");

        Ok(())
    }
}
