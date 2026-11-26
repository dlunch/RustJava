use alloc::{boxed::Box, vec::Vec};

use jvm::{ClassInstance, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{
        lang::String,
        net::URL,
        util::jar::{Attributes, JarFile},
    },
};

// abstract class java.net.JarURLConnection
#[java_class(name = "java/net/JarURLConnection", extends = "java/net/URLConnection", abstract)]
pub struct JarURLConnection {
    #[field(private)]
    fileUrl: ClassInstanceRef<URL>,
    #[field(private)]
    entry: ClassInstanceRef<String>,
}

#[java_methods]
impl JarURLConnection {
    #[method(constructor, protected)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, url: ClassInstanceRef<URL>) -> Result<()> {
        tracing::debug!("java.net.JarURLConnection::<init>({this:?}, {url:?})");

        if url.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "url is null").await);
        }

        let _: () = jvm
            .invoke_special(&this, "java/net/URLConnection", "<init>", "(Ljava/net/URL;)V", (url.clone(),))
            .await?;

        let file: Box<dyn ClassInstance> = jvm.invoke_virtual(&url, "java/net/URL", "getFile", "()Ljava/lang/String;", ()).await?;
        let file = JavaLangString::to_rust_string(jvm, &file).await?;
        let split = file.splitn(2, "!/").collect::<Vec<_>>();

        let file_url = JavaLangString::from_rust_string(jvm, split[0]).await?;
        let file_url = jvm.new_class("java/net/URL", "(Ljava/lang/String;)V", (file_url,)).await?;
        jvm.put_field(&mut this, "java/net/JarURLConnection", "fileUrl", "Ljava/net/URL;", file_url)
            .await?;

        let entry = JavaLangString::from_rust_string(jvm, split[1]).await?;
        let entry: ClassInstanceRef<String> = jvm
            .invoke_static("org/rustjava/net/URLPath", "decode", "(Ljava/lang/String;)Ljava/lang/String;", (entry,))
            .await?;
        jvm.put_field(&mut this, "java/net/JarURLConnection", "entry", "Ljava/lang/String;", entry)
            .await?;

        Ok(())
    }

    #[method(public)]
    async fn get_jar_file(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<JarFile>>;

    #[method(public)]
    async fn get_entry_name(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.net.JarURLConnection::getEntryName({this:?})");

        let entry = jvm.get_field(&this, "java/net/JarURLConnection", "entry", "Ljava/lang/String;").await?;

        Ok(entry)
    }

    #[method(name = "getJarFileURL", public)]
    async fn get_jar_file_url(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<URL>> {
        tracing::debug!("java.net.JarURLConnection::getJarFileURL({this:?})");

        let file_url = jvm.get_field(&this, "java/net/JarURLConnection", "fileUrl", "Ljava/net/URL;").await?;

        Ok(file_url)
    }

    #[method(descriptor = "()Ljava/util/jar/JarEntry;", public)]
    async fn get_jar_entry(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<URL>> {
        tracing::debug!("java.net.JarURLConnection::getJarEntry({this:?})");

        let jar_file: Box<dyn ClassInstance> = jvm
            .invoke_virtual(&this, "java/net/JarURLConnection", "getJarFile", "()Ljava/util/jar/JarFile;", ())
            .await?;
        let entry_name: ClassInstanceRef<String> = jvm
            .invoke_virtual(&this, "java/net/JarURLConnection", "getEntryName", "()Ljava/lang/String;", ())
            .await?;

        let entry = jvm
            .invoke_virtual(
                &jar_file,
                "java/util/jar/JarFile",
                "getJarEntry",
                "(Ljava/lang/String;)Ljava/util/jar/JarEntry;",
                (entry_name,),
            )
            .await?;

        Ok(entry)
    }

    #[method(public)]
    async fn get_main_attributes(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Attributes>> {
        tracing::debug!("java.net.JarURLConnection::getMainAttributes({this:?})");

        let jar_file: Box<dyn ClassInstance> = jvm
            .invoke_virtual(&this, "java/net/JarURLConnection", "getJarFile", "()Ljava/util/jar/JarFile;", ())
            .await?;
        let manifest: Box<dyn ClassInstance> = jvm
            .invoke_virtual(&jar_file, "java/util/jar/JarFile", "getManifest", "()Ljava/util/jar/Manifest;", ())
            .await?;
        let main_attributes = jvm
            .invoke_virtual(
                &manifest,
                "java/util/jar/Manifest",
                "getMainAttributes",
                "()Ljava/util/jar/Attributes;",
                (),
            )
            .await?;

        Ok(main_attributes)
    }
}
