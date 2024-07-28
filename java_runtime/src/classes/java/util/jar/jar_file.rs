use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{runtime::JavaLangString, ClassInstanceRef, Jvm, Result};

use crate::{
    classes::java::{
        io::{File, InputStream},
        lang::String,
        util::{
            jar::{JarEntry, Manifest},
            zip::ZipEntry,
        },
    },
    RuntimeClassProto, RuntimeContext,
};

// class java.util.jar.JarFile
pub struct JarFile {}

impl JarFile {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/util/zip/ZipFile"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/File;)V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "getJarEntry",
                    "(Ljava/lang/String;)Ljava/util/jar/JarEntry;",
                    Self::get_jar_entry,
                    Default::default(),
                ),
                JavaMethodProto::new("getManifest", "()Ljava/util/jar/Manifest;", Self::get_manifest, Default::default()),
            ],
            fields: vec![],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, file: ClassInstanceRef<File>) -> Result<()> {
        tracing::debug!("java.util.jar.JarFile::<init>({:?}, {:?})", &this, &file,);

        let _: () = jvm
            .invoke_special(&this, "java/util/zip/ZipFile", "<init>", "(Ljava/io/File;)V", (file,))
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
            .invoke_virtual(&this, "getEntry", "(Ljava/lang/String;)Ljava/util/zip/ZipEntry;", (name.clone(),))
            .await?;

        if zip_entry.is_null() {
            return Ok(None.into());
        }

        let jar_entry = jvm.new_class("java/util/jar/JarEntry", "(Ljava/lang/String;)V", (name,)).await?;

        Ok(jar_entry.into())
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

#[cfg(test)]
mod test {
    use jvm::{runtime::JavaLangString, Result};

    use crate::test::test_jvm_filesystem;

    #[tokio::test]
    async fn test_jar_manifest() -> Result<()> {
        let jar = include_bytes!("../../../../../../test_data/test.jar");
        let filesystem = [("test.jar".into(), jar.to_vec())].into_iter().collect();
        let jvm = test_jvm_filesystem(filesystem).await?;

        let name = JavaLangString::from_rust_string(&jvm, "test.jar").await?;
        let file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (name,)).await?;
        let jar = jvm.new_class("java/util/jar/JarFile", "(Ljava/io/File;)V", (file,)).await?;

        let manifest = jvm.invoke_virtual(&jar, "getManifest", "()Ljava/util/jar/Manifest;", ()).await?;

        let main_attributes = jvm
            .invoke_virtual(&manifest, "getMainAttributes", "()Ljava/util/jar/Attributes;", ())
            .await?;

        let key = JavaLangString::from_rust_string(&jvm, "Main-Class").await?;
        let value = jvm
            .invoke_virtual(&main_attributes, "getValue", "(Ljava/lang/String;)Ljava/lang/String;", (key,))
            .await?;

        assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, "JarTest");

        Ok(())
    }
}
