use alloc::boxed::Box;

use jvm::{ClassInstance, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{
        io::InputStream,
        lang::{Object, String},
        net::URL,
        util::{Hashtable, jar::JarFile},
    },
};

// class rustjava.net.JarURLConnection
#[java_class(name = "org/rustjava/net/JarURLConnection", extends = "java/net/JarURLConnection")]
pub struct JarURLConnection {
    #[field(static)]
    openedFiles: ClassInstanceRef<Hashtable>,
}

#[java_methods]
impl JarURLConnection {
    #[method(static_initializer)]
    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        tracing::debug!("org.rustjava.net.JarURLConnection::<clinit>()");

        let map = jvm.new_class("java/util/Hashtable", "()V", ()).await?;
        jvm.put_static_field("org/rustjava/net/JarURLConnection", "openedFiles", "Ljava/util/Hashtable;", map)
            .await?;

        Ok(())
    }

    #[method(constructor)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, url: ClassInstanceRef<URL>) -> Result<()> {
        tracing::debug!("org.rustjava.net.JarURLConnection::<init>({this:?}, {url:?})");

        let _: () = jvm
            .invoke_special(&this, "java/net/JarURLConnection", "<init>", "(Ljava/net/URL;)V", (url.clone(),))
            .await?;

        Ok(())
    }

    #[method(public)]
    async fn get_jar_file(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<JarFile>> {
        tracing::debug!("org.rustjava.net.JarURLConnection::getJarFile({this:?})");

        let url: Box<dyn ClassInstance> = jvm
            .invoke_virtual(&this, "org/rustjava/net/JarURLConnection", "getJarFileURL", "()Ljava/net/URL;", ())
            .await?;
        let protocol: Box<dyn ClassInstance> = jvm
            .invoke_virtual(&url, "java/net/URL", "getProtocol", "()Ljava/lang/String;", ())
            .await?;
        let protocol = JavaLangString::to_rust_string(jvm, &protocol).await?;

        if protocol == "file" {
            let name: ClassInstanceRef<String> = jvm.invoke_virtual(&url, "java/net/URL", "getFile", "()Ljava/lang/String;", ()).await?;

            let opened_files: Box<dyn ClassInstance> = jvm
                .get_static_field("org/rustjava/net/JarURLConnection", "openedFiles", "Ljava/util/Hashtable;")
                .await?;
            let cache: ClassInstanceRef<JarFile> = jvm
                .invoke_virtual(
                    &opened_files,
                    "java/util/Hashtable",
                    "get",
                    "(Ljava/lang/Object;)Ljava/lang/Object;",
                    (name.clone(),),
                )
                .await?;

            if !cache.is_null() {
                Ok(cache)
            } else {
                let path: ClassInstanceRef<String> = jvm
                    .invoke_static(
                        "org/rustjava/net/URLPath",
                        "decode",
                        "(Ljava/lang/String;)Ljava/lang/String;",
                        (name.clone(),),
                    )
                    .await?;
                let file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (path,)).await?;
                let jar_file = jvm.new_class("java/util/jar/JarFile", "(Ljava/io/File;)V", (file,)).await?;

                let _: ClassInstanceRef<Object> = jvm
                    .invoke_virtual(
                        &opened_files,
                        "java/util/Hashtable",
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

    #[method(public)]
    async fn get_input_stream(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<InputStream>> {
        tracing::debug!("org.rustjava.net.JarURLConnection::getInputStream({this:?})");

        let entry: ClassInstanceRef<String> = jvm
            .invoke_virtual(&this, "org/rustjava/net/JarURLConnection", "getEntryName", "()Ljava/lang/String;", ())
            .await?;
        let jar_file: Box<dyn ClassInstance> = jvm
            .invoke_virtual(&this, "org/rustjava/net/JarURLConnection", "getJarFile", "()Ljava/util/jar/JarFile;", ())
            .await?;

        let jar_entry: ClassInstanceRef<JarFile> = jvm
            .invoke_virtual(
                &jar_file,
                "java/util/jar/JarFile",
                "getJarEntry",
                "(Ljava/lang/String;)Ljava/util/jar/JarEntry;",
                (entry,),
            )
            .await?;

        if jar_entry.is_null() {
            return Err(jvm.exception("java/io/FileNotFoundException", "entry not found").await);
        }

        let jar_input_stream = jvm
            .invoke_virtual(
                &jar_file,
                "java/util/jar/JarFile",
                "getInputStream",
                "(Ljava/util/zip/ZipEntry;)Ljava/io/InputStream;",
                (jar_entry,),
            )
            .await?;

        Ok(jar_input_stream)
    }
}
