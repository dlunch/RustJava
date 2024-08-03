use alloc::{format, vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{runtime::JavaLangString, Array, ClassInstanceRef, Jvm, Result};

use crate::{
    classes::java::{
        lang::{Class, ClassLoader, String},
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
                JavaMethodProto::new("<init>", "([Ljava/net/URL;Ljava/lang/ClassLoader;)V", Self::init, Default::default()),
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

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        urls: ClassInstanceRef<Array<URL>>,
        parent: ClassInstanceRef<ClassLoader>,
    ) -> Result<()> {
        tracing::debug!("java.net.URLClassLoader::<init>({:?}, {:?}, {:?})", &this, &urls, &parent);

        let _: () = jvm
            .invoke_special(&this, "java/lang/ClassLoader", "<init>", "(Ljava/lang/ClassLoader;)V", (parent,))
            .await?;

        jvm.put_field(&mut this, "urls", "[Ljava/net/URL;", urls).await?;

        Ok(())
    }

    async fn find_class(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Class>> {
        tracing::debug!("java.net.URLClassLoader::findClass({:?}, {:?})", &this, name);

        let name_str = JavaLangString::to_rust_string(jvm, &name).await?;
        let resource_name = format!("{}.class", name_str.replace('.', "/"));
        let resource_name = JavaLangString::from_rust_string(jvm, &resource_name).await?;

        let resource: ClassInstanceRef<URL> = jvm
            .invoke_virtual(&this, "findResource", "(Ljava/lang/String;)Ljava/net/URL;", (resource_name,))
            .await?;
        if resource.is_null() {
            return Ok(None.into());
        }

        let stream = jvm.invoke_virtual(&resource, "openStream", "()Ljava/io/InputStream;", ()).await?;

        // TODO can we use ByteArrayOutputStream?
        let mut buf = jvm.instantiate_array("B", 1024).await?;
        let mut read = 0;
        loop {
            let temp = jvm.instantiate_array("B", 1024).await?;
            let cur: i32 = jvm.invoke_virtual(&stream, "read", "([B)I", (temp.clone(),)).await?;
            if cur == -1 {
                break;
            }

            if (jvm.array_length(&buf).await? as i32) < read + cur {
                let new_buf = jvm.instantiate_array("B", (read + cur) as _).await?;
                let _: () = jvm
                    .invoke_static(
                        "java/lang/System",
                        "arraycopy",
                        "(Ljava/lang/Object;ILjava/lang/Object;II)V",
                        (buf.clone(), 0, new_buf.clone(), 0, read),
                    )
                    .await?;
                buf = new_buf;
            }

            let _: () = jvm
                .invoke_static(
                    "java/lang/System",
                    "arraycopy",
                    "(Ljava/lang/Object;ILjava/lang/Object;II)V",
                    (temp, 0, buf.clone(), read, cur),
                )
                .await?;

            read += cur;
        }

        let class = jvm
            .invoke_virtual(&this, "defineClass", "(Ljava/lang/String;[BII)Ljava/lang/Class;", (name, buf, 0, read))
            .await?;

        Ok(class)
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
            if file.ends_with('/') || file.is_empty() {
                // directory
                let final_path = if file.ends_with('/') {
                    format!("{}{}", file, name_str)
                } else {
                    name_str.clone()
                };

                if runtime.stat(&final_path).await.is_ok() {
                    let new_url = jvm
                        .new_class(
                            "java/net/URL",
                            "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
                            (
                                JavaLangString::from_rust_string(jvm, "file").await?,
                                JavaLangString::from_rust_string(jvm, "").await?,
                                JavaLangString::from_rust_string(jvm, &final_path).await?,
                            ),
                        )
                        .await?;

                    return Ok(new_url.into());
                }
            } else {
                // treat as jar
                let name_str = name_str.trim_start_matches('/');

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

    use jvm::{runtime::JavaLangString, ClassInstanceRef, Result};

    use crate::{classes::java::net::URL, test::test_jvm_filesystem};

    #[tokio::test]
    async fn test_jar_loading() -> Result<()> {
        let jar = include_bytes!("../../../../../test_data/test.jar");
        let filesystem = [("test.jar".into(), jar.to_vec())].into_iter().collect();
        let jvm = test_jvm_filesystem(filesystem).await?;

        let url_str = JavaLangString::from_rust_string(&jvm, "file:test.jar").await?;
        let url = jvm.new_class("java/net/URL", "(Ljava/lang/String;)V", (url_str,)).await?;
        let mut urls = jvm.instantiate_array("Ljava/net/URL;", 1).await?;
        jvm.store_array(&mut urls, 0, vec![url]).await?;

        let class_loader = jvm
            .new_class("java/net/URLClassLoader", "([Ljava/net/URL;Ljava/lang/ClassLoader;)V", (urls, None))
            .await?;

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

    #[tokio::test]
    async fn test_jar_loading_with_slash() -> Result<()> {
        let jar = include_bytes!("../../../../../test_data/test.jar");
        let filesystem = [("test.jar".into(), jar.to_vec())].into_iter().collect();
        let jvm = test_jvm_filesystem(filesystem).await?;

        let url_str = JavaLangString::from_rust_string(&jvm, "file:test.jar").await?;
        let url = jvm.new_class("java/net/URL", "(Ljava/lang/String;)V", (url_str,)).await?;
        let mut urls = jvm.instantiate_array("Ljava/net/URL;", 1).await?;
        jvm.store_array(&mut urls, 0, vec![url]).await?;

        let class_loader = jvm
            .new_class("java/net/URLClassLoader", "([Ljava/net/URL;Ljava/lang/ClassLoader;)V", (urls, None))
            .await?;

        let resource_name = JavaLangString::from_rust_string(&jvm, "/test.txt").await?;
        let stream = jvm
            .invoke_virtual(
                &class_loader,
                "getResourceAsStream",
                "(Ljava/lang/String;)Ljava/io/InputStream;",
                (resource_name,),
            )
            .await?;

        let buf = jvm.instantiate_array("B", 17).await?;
        let len: i32 = jvm.invoke_virtual(&stream, "read", "([B)I", (buf.clone(),)).await?;

        let data = jvm.load_byte_array(&buf, 0, len as _).await?;

        assert_eq!(cast_vec::<i8, u8>(data), b"test content\n");

        Ok(())
    }

    #[tokio::test]
    async fn test_load_from_dir() -> Result<()> {
        let filesystem = [("test.txt".into(), b"test content\n".to_vec())].into_iter().collect();
        let jvm = crate::test::test_jvm_filesystem(filesystem).await?;

        let url_str = JavaLangString::from_rust_string(&jvm, "file:.").await?;
        let url = jvm.new_class("java/net/URL", "(Ljava/lang/String;)V", (url_str,)).await?;
        let mut urls = jvm.instantiate_array("Ljava/net/URL;", 1).await?;
        jvm.store_array(&mut urls, 0, vec![url]).await?;

        let class_loader = jvm
            .new_class("java/net/URLClassLoader", "([Ljava/net/URL;Ljava/lang/ClassLoader;)V", (urls, None))
            .await?;

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

    #[tokio::test]
    async fn test_jar_loading_no_file() -> Result<()> {
        let jar = include_bytes!("../../../../../test_data/test.jar");
        let filesystem = [("test.jar".into(), jar.to_vec())].into_iter().collect();
        let jvm = test_jvm_filesystem(filesystem).await?;

        let url_str = JavaLangString::from_rust_string(&jvm, "file:test.jar").await?;
        let url = jvm.new_class("java/net/URL", "(Ljava/lang/String;)V", (url_str,)).await?;
        let mut urls = jvm.instantiate_array("Ljava/net/URL;", 1).await?;
        jvm.store_array(&mut urls, 0, vec![url]).await?;

        let class_loader = jvm
            .new_class("java/net/URLClassLoader", "([Ljava/net/URL;Ljava/lang/ClassLoader;)V", (urls, None))
            .await?;

        let resource_name = JavaLangString::from_rust_string(&jvm, "does_not_exists.txt").await?;
        let resource: ClassInstanceRef<URL> = jvm
            .invoke_virtual(&class_loader, "findResource", "(Ljava/lang/String;)Ljava/net/URL;", (resource_name,))
            .await?;

        assert!(resource.is_null());

        Ok(())
    }

    #[tokio::test]
    async fn test_load_from_dir_no_file() -> Result<()> {
        let filesystem = [("test.txt".into(), b"test content\n".to_vec())].into_iter().collect();
        let jvm = crate::test::test_jvm_filesystem(filesystem).await?;

        let url_str = JavaLangString::from_rust_string(&jvm, "file:.").await?;
        let url = jvm.new_class("java/net/URL", "(Ljava/lang/String;)V", (url_str,)).await?;
        let mut urls = jvm.instantiate_array("Ljava/net/URL;", 1).await?;
        jvm.store_array(&mut urls, 0, vec![url]).await?;

        let class_loader = jvm
            .new_class("java/net/URLClassLoader", "([Ljava/net/URL;Ljava/lang/ClassLoader;)V", (urls, None))
            .await?;

        let resource_name = JavaLangString::from_rust_string(&jvm, "does_not_exists.txt").await?;
        let resource: ClassInstanceRef<URL> = jvm
            .invoke_virtual(&class_loader, "findResource", "(Ljava/lang/String;)Ljava/net/URL;", (resource_name,))
            .await?;
        assert!(resource.is_null());

        Ok(())
    }
}
