use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{runtime::JavaLangString, ClassInstanceRef, Jvm, Result};

use crate::{
    classes::java::{
        io::InputStream,
        lang::{Object, String},
        net::URL,
        util::jar::JarFile,
    },
    RuntimeClassProto, RuntimeContext,
};

// class rustjava.net.JarURLConnection
pub struct JarURLConnection;

impl JarURLConnection {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "org/rustjava/net/JarURLConnection",
            parent_class: Some("java/net/JarURLConnection"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "(Ljava/net/URL;)V", Self::init, Default::default()),
                JavaMethodProto::new("getJarFile", "()Ljava/util/jar/JarFile;", Self::get_jar_file, Default::default()),
                JavaMethodProto::new("getInputStream", "()Ljava/io/InputStream;", Self::get_input_stream, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("openedFiles", "Ljava/util/Hashtable;", FieldAccessFlags::STATIC)],
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        tracing::debug!("org.rustjava.net.JarURLConnection::<clinit>()");

        let map = jvm.new_class("java/util/Hashtable", "()V", ()).await?;
        jvm.put_static_field("org/rustjava/net/JarURLConnection", "openedFiles", "Ljava/util/Hashtable;", map)
            .await?;

        Ok(())
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, url: ClassInstanceRef<URL>) -> Result<()> {
        tracing::debug!("org.rustjava.net.JarURLConnection::<init>({:?}, {:?})", &this, &url);

        let _: () = jvm
            .invoke_special(&this, "java/net/JarURLConnection", "<init>", "(Ljava/net/URL;)V", (url.clone(),))
            .await?;

        Ok(())
    }

    async fn get_jar_file(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<JarFile>> {
        tracing::debug!("org.rustjava.net.JarURLConnection::getJarFile({:?})", &this);

        let url = jvm.invoke_virtual(&this, "getJarFileURL", "()Ljava/net/URL;", ()).await?;
        let protocol = jvm.invoke_virtual(&url, "getProtocol", "()Ljava/lang/String;", ()).await?;
        let protocol = JavaLangString::to_rust_string(jvm, &protocol).await?;

        if protocol == "file" {
            let name: ClassInstanceRef<String> = jvm.invoke_virtual(&url, "getFile", "()Ljava/lang/String;", ()).await?;

            let opened_files = jvm
                .get_static_field("org/rustjava/net/JarURLConnection", "openedFiles", "Ljava/util/Hashtable;")
                .await?;
            let cache: ClassInstanceRef<JarFile> = jvm
                .invoke_virtual(&opened_files, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (name.clone(),))
                .await?;

            if !cache.is_null() {
                Ok(cache)
            } else {
                let file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (name.clone(),)).await?;
                let jar_file = jvm.new_class("java/util/jar/JarFile", "(Ljava/io/File;)V", (file,)).await?;

                let _: ClassInstanceRef<Object> = jvm
                    .invoke_virtual(
                        &opened_files,
                        "put",
                        "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                        (name, jar_file.clone()),
                    )
                    .await?;

                Ok(jar_file.into())
            }
        } else {
            Err(jvm.exception("java/net/MalformedURLException", "unsupported protocol").await)
        }
    }

    async fn get_input_stream(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<InputStream>> {
        tracing::debug!("org.rustjava.net.JarURLConnection::getInputStream({:?})", &this);

        let entry: ClassInstanceRef<String> = jvm.invoke_virtual(&this, "getEntryName", "()Ljava/lang/String;", ()).await?;
        let jar_file = jvm.invoke_virtual(&this, "getJarFile", "()Ljava/util/jar/JarFile;", ()).await?;

        let jar_entry: ClassInstanceRef<JarFile> = jvm
            .invoke_virtual(&jar_file, "getJarEntry", "(Ljava/lang/String;)Ljava/util/jar/JarEntry;", (entry,))
            .await?;

        if jar_entry.is_null() {
            return Err(jvm.exception("java/io/FileNotFoundException", "entry not found").await);
        }

        let jar_input_stream = jvm
            .invoke_virtual(
                &jar_file,
                "getInputStream",
                "(Ljava/util/zip/ZipEntry;)Ljava/io/InputStream;",
                (jar_entry,),
            )
            .await?;

        Ok(jar_input_stream)
    }
}

#[cfg(test)]
mod test {
    use bytemuck::cast_vec;

    use jvm::{runtime::JavaLangString, ClassInstanceRef, Result};

    use crate::{classes::java::util::jar::JarFile, test::test_jvm_filesystem};

    #[tokio::test]
    async fn test_jar_entry() -> Result<()> {
        let jar = include_bytes!("../../../../../../test_data/test.jar");
        let filesystem = [("test.jar".into(), jar.to_vec())].into_iter().collect();
        let jvm = test_jvm_filesystem(filesystem).await?;

        let url_spec = JavaLangString::from_rust_string(&jvm, "jar:file:test.jar!/test.txt").await?;
        let url = jvm.new_class("java/net/URL", "(Ljava/lang/String;)V", (url_spec,)).await?;

        let connection = jvm.invoke_virtual(&url, "openConnection", "()Ljava/net/URLConnection;", ()).await?;

        let stream = jvm.invoke_virtual(&connection, "getInputStream", "()Ljava/io/InputStream;", ()).await?;

        let buf = jvm.instantiate_array("B", 17).await?;
        let len: i32 = jvm.invoke_virtual(&stream, "read", "([B)I", (buf.clone(),)).await?;

        let data = jvm.load_byte_array(&buf, 0, len as _).await?;

        assert_eq!(cast_vec::<i8, u8>(data), b"test content\n");

        Ok(())
    }

    #[tokio::test]
    async fn test_jar_file() -> Result<()> {
        let jar = include_bytes!("../../../../../../test_data/test.jar");
        let filesystem = [("test.jar".into(), jar.to_vec())].into_iter().collect();
        let jvm = test_jvm_filesystem(filesystem).await?;

        let url_spec = JavaLangString::from_rust_string(&jvm, "jar:file:test.jar!/").await?;
        let url = jvm.new_class("java/net/URL", "(Ljava/lang/String;)V", (url_spec,)).await?;

        let connection = jvm.invoke_virtual(&url, "openConnection", "()Ljava/net/URLConnection;", ()).await?;

        let attributes = jvm
            .invoke_virtual(&connection, "getMainAttributes", "()Ljava/util/jar/Attributes;", ())
            .await?;

        let key = JavaLangString::from_rust_string(&jvm, "Main-Class").await?;
        let value = jvm
            .invoke_virtual(&attributes, "getValue", "(Ljava/lang/String;)Ljava/lang/String;", (key,))
            .await?;

        assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, "JarTest");

        Ok(())
    }

    #[tokio::test]
    async fn test_jar_cache() -> Result<()> {
        let jar = include_bytes!("../../../../../../test_data/test.jar");
        let filesystem = [("test.jar".into(), jar.to_vec())].into_iter().collect();
        let jvm = test_jvm_filesystem(filesystem).await?;

        let url_spec = JavaLangString::from_rust_string(&jvm, "jar:file:test.jar!/").await?;
        let url = jvm.new_class("java/net/URL", "(Ljava/lang/String;)V", (url_spec,)).await?;

        let connection = jvm.invoke_virtual(&url, "openConnection", "()Ljava/net/URLConnection;", ()).await?;

        let jar_file = jvm.invoke_virtual(&connection, "getJarFile", "()Ljava/util/jar/JarFile;", ()).await?;
        let jar_file2: ClassInstanceRef<JarFile> = jvm.invoke_virtual(&connection, "getJarFile", "()Ljava/util/jar/JarFile;", ()).await?;

        let equals: bool = jvm.invoke_virtual(&jar_file, "equals", "(Ljava/lang/Object;)Z", (jar_file2,)).await?;

        assert!(equals);

        Ok(())
    }
}
