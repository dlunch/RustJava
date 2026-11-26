use alloc::string::ToString;

use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{
        io::{File, InputStream},
        lang::String,
        util::{
            Enumeration,
            jar::{JarEntry, Manifest},
            zip::{ZipEntry, ZipFile},
        },
    },
};

// class java.util.jar.JarFile
#[java_class(name = "java/util/jar/JarFile", extends = "java/util/zip/ZipFile", public)]
pub struct JarFile;

#[java_methods]
impl JarFile {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, file: ClassInstanceRef<File>) -> Result<()> {
        tracing::debug!("java.util.jar.JarFile::<init>({this:?}, {file:?})");

        let _: () = jvm
            .invoke_special(&this, "java/util/zip/ZipFile", "<init>", "(Ljava/io/File;)V", (file,))
            .await?;

        Ok(())
    }

    #[method(constructor, public)]
    async fn init_with_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, name: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.util.jar.JarFile::<init>({this:?}, {name:?})");

        let file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (name,)).await?;

        let _: () = jvm
            .invoke_special(&this, "java/util/jar/JarFile", "<init>", "(Ljava/io/File;)V", (file,))
            .await?;

        Ok(())
    }

    #[method(public)]
    async fn get_jar_entry(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<JarEntry>> {
        tracing::debug!("java.util.jar.JarFile::getJarEntry({this:?}, {name:?})");

        let zip_entry: ClassInstanceRef<ZipEntry> = jvm
            .invoke_virtual(
                &this,
                "java/util/jar/JarFile",
                "getEntry",
                "(Ljava/lang/String;)Ljava/util/zip/ZipEntry;",
                (name,),
            )
            .await?;

        if zip_entry.is_null() {
            return Ok(None.into());
        }

        let jar_entry = jvm
            .new_class("java/util/jar/JarEntry", "(Ljava/util/zip/ZipEntry;)V", (zip_entry,))
            .await?;

        Ok(jar_entry.into())
    }

    #[method(public)]
    async fn entries(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Enumeration>> {
        tracing::debug!("java.util.jar.JarFile::entries({this:?})");

        let zip_entries: ClassInstanceRef<Enumeration> = jvm
            .invoke_special(&this, "java/util/zip/ZipFile", "entries", "()Ljava/util/Enumeration;", ())
            .await?;

        let entries = jvm
            .new_class("java/util/jar/JarFile$Entries", "(Ljava/util/zip/ZipFile$Entries;)V", (zip_entries,))
            .await?;

        Ok(entries.into())
    }

    #[method(public)]
    async fn get_manifest(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Manifest>> {
        tracing::debug!("java.util.jar.JarFile::getManifest({this:?})");

        let manifest_name = JavaLangString::from_rust_string(jvm, "META-INF/MANIFEST.MF").await?;
        let mut manifest_file: ClassInstanceRef<JarEntry> = jvm
            .invoke_virtual(
                &this,
                "java/util/jar/JarFile",
                "getJarEntry",
                "(Ljava/lang/String;)Ljava/util/jar/JarEntry;",
                (manifest_name,),
            )
            .await?;

        if manifest_file.is_null() {
            let name = {
                let zip = ZipFile::get_zip_archive(jvm, &ClassInstanceRef::from(this.instance.clone())).await?;
                let mut manifest_name = None;
                for name in zip.file_names() {
                    let name = match name {
                        Ok(name) => name,
                        Err(err) => return Err(jvm.exception("java/util/zip/ZipException", &err.to_string()).await),
                    };
                    if name.eq_ignore_ascii_case("META-INF/MANIFEST.MF") {
                        manifest_name = Some(name.into_owned());
                        break;
                    }
                }
                manifest_name
            };
            let Some(name) = name else {
                return Ok(None.into());
            };
            let name = JavaLangString::from_rust_string(jvm, &name).await?;
            manifest_file = jvm
                .invoke_virtual(
                    &this,
                    "java/util/jar/JarFile",
                    "getJarEntry",
                    "(Ljava/lang/String;)Ljava/util/jar/JarEntry;",
                    (name,),
                )
                .await?;
        }

        let input_stream: ClassInstanceRef<InputStream> = jvm
            .invoke_virtual(
                &this,
                "java/util/jar/JarFile",
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
