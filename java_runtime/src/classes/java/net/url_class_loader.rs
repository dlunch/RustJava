use alloc::{format, vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{runtime::JavaLangString, Array, ClassInstanceRef, Jvm, Result};

use crate::{
    classes::java::{
        lang::{Class, String},
        net::{JarURLConnection, URL},
        util::jar::JarEntry,
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

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, urls: ClassInstanceRef<Array<URL>>) -> Result<()> {
        tracing::debug!("java.net.URLClassLoader::<init>({:?}, {:?})", &this, &urls);

        jvm.invoke_special(&this, "java/lang/ClassLoader", "<init>", "(Ljava/lang/ClassLoader;)V", (None,))
            .await?;

        jvm.put_field(&mut this, "urls", "[Ljava/net/URL;", urls).await?;

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
        jvm: &Jvm,
        runtime: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<URL>> {
        tracing::debug!("java.net.URLClassLoader::findResource({:?}, {:?})", &this, name);

        // TODO cache

        let name_str = JavaLangString::to_rust_string(jvm, &name).await?;

        let urls = jvm.get_field(&this, "urls", "[Ljava/net/URL;").await?;
        let urls: Vec<ClassInstanceRef<URL>> = jvm.load_array(&urls, 0, jvm.array_length(&urls).await? as _).await?;

        for url in urls {
            let file = jvm.invoke_virtual(&url, "getFile", "()Ljava/lang/String;", ()).await?;
            let file = JavaLangString::to_rust_string(jvm, &file).await?;
            if file.ends_with('/') || file == "." {
                // directory
                let final_path = if file.ends_with('/') {
                    format!("file:{}{}", file, name_str)
                } else {
                    format!("file:{}", name_str)
                };

                if runtime.stat(&final_path).await.is_ok() {
                    let new_url = jvm
                        .new_class(
                            "java/net/URL",
                            "(Ljava/lang/String;)V",
                            (JavaLangString::from_rust_string(jvm, &final_path).await?,),
                        )
                        .await?;

                    return Ok(new_url.into());
                }
            } else {
                // treat as jar
                let jar_url_str = format!("jar:file:{}!/{}", file, name_str); // TODO url might not be file
                let jar_url = JavaLangString::from_rust_string(jvm, &jar_url_str).await?;
                let jar_url = jvm.new_class("java/net/URL", "(Ljava/lang/String;)V", (jar_url,)).await?;
                let connection: ClassInstanceRef<JarURLConnection> =
                    jvm.invoke_virtual(&jar_url, "openConnection", "()Ljava/net/URLConnection;", ()).await?;

                let entry: ClassInstanceRef<JarEntry> = jvm.invoke_virtual(&connection, "getJarEntry", "()Ljava/util/jar/JarEntry;", ()).await?;

                if !entry.is_null() {
                    return Ok(jar_url.into());
                }
            }
        }

        Ok(None.into())
    }
}

#[cfg(test)]
mod test {
    use alloc::vec;

    use bytemuck::cast_vec;

    use jvm::{runtime::JavaLangString, Result};

    use crate::test::test_jvm_filesystem;

    #[futures_test::test]
    async fn test_jar_loading() -> Result<()> {
        let jar = include_bytes!("../../../../../test_data/test.jar");
        let filesystem = [("test.jar".into(), jar.to_vec())].into_iter().collect();
        let jvm = test_jvm_filesystem(filesystem).await?;

        let url_str = JavaLangString::from_rust_string(&jvm, "file:test.jar").await?;
        let url = jvm.new_class("java/net/URL", "(Ljava/lang/String;)V", (url_str,)).await?;
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
