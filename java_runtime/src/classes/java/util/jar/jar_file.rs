use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::{File, InputStream},
        lang::String,
        util::{
            Enumeration,
            jar::{JarEntry, Manifest},
            zip::ZipEntry,
        },
    },
};

// class java.util.jar.JarFile
pub struct JarFile;

impl JarFile {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/jar/JarFile",
            parent_class: Some("java/util/zip/ZipFile"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/File;)V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_with_string, Default::default()),
                JavaMethodProto::new(
                    "getJarEntry",
                    "(Ljava/lang/String;)Ljava/util/jar/JarEntry;",
                    Self::get_jar_entry,
                    Default::default(),
                ),
                JavaMethodProto::new("entries", "()Ljava/util/Enumeration;", Self::entries, Default::default()),
                JavaMethodProto::new("getManifest", "()Ljava/util/jar/Manifest;", Self::get_manifest, Default::default()),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, file: ClassInstanceRef<File>) -> Result<()> {
        tracing::debug!("java.util.jar.JarFile::<init>({:?}, {:?})", &this, &file,);

        let _: () = jvm
            .invoke_special(&this, "java/util/zip/ZipFile", "<init>", "(Ljava/io/File;)V", (file,))
            .await?;

        Ok(())
    }

    async fn init_with_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, name: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.util.jar.JarFile::<init>({:?}, {:?})", &this, &name,);

        let file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (name,)).await?;

        let _: () = jvm
            .invoke_special(&this, "java/util/jar/JarFile", "<init>", "(Ljava/io/File;)V", (file,))
            .await?;

        Ok(())
    }

    async fn get_jar_entry(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<JarEntry>> {
        tracing::debug!("java.util.jar.JarFile::getJarEntry({:?}, {:?})", &this, &name);

        let zip_entry: ClassInstanceRef<ZipEntry> = jvm
            .invoke_virtual(&this, "getEntry", "(Ljava/lang/String;)Ljava/util/zip/ZipEntry;", (name,))
            .await?;

        if zip_entry.is_null() {
            return Ok(None.into());
        }

        let jar_entry = jvm
            .new_class("java/util/jar/JarEntry", "(Ljava/util/zip/ZipEntry;)V", (zip_entry,))
            .await?;

        Ok(jar_entry.into())
    }

    async fn entries(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Enumeration>> {
        tracing::debug!("java.util.jar.JarFile::entries({:?})", &this);

        let zip_entries: ClassInstanceRef<Enumeration> = jvm
            .invoke_special(&this, "java/util/zip/ZipFile", "entries", "()Ljava/util/Enumeration;", ())
            .await?;

        let entries = jvm
            .new_class("java/util/jar/JarFile$Entries", "(Ljava/util/zip/ZipFile$Entries;)V", (zip_entries,))
            .await?;

        Ok(entries.into())
    }

    async fn get_manifest(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Manifest>> {
        tracing::debug!("java.util.jar.JarFile::getManifest({:?})", &this);

        let manifest_name = JavaLangString::from_rust_string(jvm, "META-INF/MANIFEST.MF").await?;
        let manifest_file: ClassInstanceRef<JarEntry> = jvm
            .invoke_virtual(&this, "getJarEntry", "(Ljava/lang/String;)Ljava/util/jar/JarEntry;", (manifest_name,))
            .await?;

        let input_stream: ClassInstanceRef<InputStream> = jvm
            .invoke_virtual(
                &this,
                "getInputStream",
                "(Ljava/util/zip/ZipEntry;)Ljava/io/InputStream;",
                (manifest_file,),
            )
            .await?;

        let manifest = jvm
            .new_class("java/util/jar/Manifest", "(Ljava/io/InputStream;)V", (input_stream,))
            .await?;

        Ok(manifest.into())
    }
}
