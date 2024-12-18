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
