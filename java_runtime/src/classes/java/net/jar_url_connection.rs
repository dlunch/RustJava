use alloc::{vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{lang::String, net::URL, util::jar::Attributes},
};

// abstract class java.net.JarURLConnection
pub struct JarURLConnection;

impl JarURLConnection {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/net/JarURLConnection",
            parent_class: Some("java/net/URLConnection"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/net/URL;)V", Self::init, MethodAccessFlags::PROTECTED),
                JavaMethodProto::new_abstract("getJarFile", "()Ljava/util/jar/JarFile;", MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getEntryName", "()Ljava/lang/String;", Self::get_entry_name, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getJarFileURL", "()Ljava/net/URL;", Self::get_jar_file_url, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "getJarEntry",
                    "()Ljava/util/jar/JarEntry;",
                    Self::get_jar_entry,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "getMainAttributes",
                    "()Ljava/util/jar/Attributes;",
                    Self::get_main_attributes,
                    MethodAccessFlags::PUBLIC,
                ),
            ],
            fields: vec![
                JavaFieldProto::new("fileUrl", "Ljava/net/URL;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("entry", "Ljava/lang/String;", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::ABSTRACT,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, url: ClassInstanceRef<URL>) -> Result<()> {
        tracing::debug!("java.net.JarURLConnection::<init>({this:?}, {url:?})");

        let _: () = jvm
            .invoke_special(&this, "java/net/URLConnection", "<init>", "(Ljava/net/URL;)V", (url.clone(),))
            .await?;

        let file = jvm.invoke_virtual(&url, "java/net/URL", "getFile", "()Ljava/lang/String;", ()).await?;
        let file = JavaLangString::to_rust_string(jvm, &file).await?;
        let split = file.splitn(2, "!/").collect::<Vec<_>>();

        let file_url = JavaLangString::from_rust_string(jvm, split[0]).await?;
        let file_url = jvm.new_class("java/net/URL", "(Ljava/lang/String;)V", (file_url,)).await?;
        jvm.put_field(&mut this, "fileUrl", "Ljava/net/URL;", file_url).await?;

        let entry = JavaLangString::from_rust_string(jvm, split[1]).await?;
        jvm.put_field(&mut this, "entry", "Ljava/lang/String;", entry).await?;

        Ok(())
    }

    async fn get_entry_name(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.net.JarURLConnection::getEntryName({this:?})");

        let entry = jvm.get_field(&this, "entry", "Ljava/lang/String;").await?;

        Ok(entry)
    }

    async fn get_jar_file_url(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<URL>> {
        tracing::debug!("java.net.JarURLConnection::getJarFileURL({this:?})");

        let file_url = jvm.get_field(&this, "fileUrl", "Ljava/net/URL;").await?;

        Ok(file_url)
    }

    async fn get_jar_entry(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<URL>> {
        tracing::debug!("java.net.JarURLConnection::getJarEntry({this:?})");

        let jar_file = jvm
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

    async fn get_main_attributes(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Attributes>> {
        tracing::debug!("java.net.JarURLConnection::getMainAttributes({this:?})");

        let jar_file = jvm
            .invoke_virtual(&this, "java/net/JarURLConnection", "getJarFile", "()Ljava/util/jar/JarFile;", ())
            .await?;
        let manifest = jvm
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
