use alloc::vec;

use bytemuck::cast_vec;
use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{runtime::JavaLangString, ClassInstanceRef, Jvm, Result};
use zip::ZipArchive;

use crate::{
    classes::java::{io::File, lang::String, util::zip::ZipEntry},
    RuntimeClassProto, RuntimeContext,
};

// class java.util.zip.ZipFile
pub struct ZipFile {}

impl ZipFile {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/File;)V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "getEntry",
                    "(Ljava/lang/String;)Ljava/util/zip/ZipEntry;",
                    Self::get_entry,
                    Default::default(),
                ),
            ],
            fields: vec![JavaFieldProto::new("buf", "[B", Default::default())],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, file: ClassInstanceRef<File>) -> Result<()> {
        tracing::debug!("java.util.zip.ZipFile::<init>({:?}, {:?})", &this, &file,);

        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        let length: i64 = jvm.invoke_virtual(&file, "length", "()J", ()).await?;
        let is = jvm.new_class("java/io/FileInputStream", "(Ljava/io/File;)V", (file,)).await?;

        let buf = jvm.instantiate_array("B", length as _).await?;
        let _: i32 = jvm.invoke_virtual(&is, "read", "([B)I", (buf.clone(),)).await?;

        jvm.put_field(&mut this, "buf", "[B", buf).await?;

        Ok(())
    }

    async fn get_entry(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<ZipEntry>> {
        tracing::debug!("java.util.zip.ZipFile::getEntry({:?}, {:?})", &this, &name);

        let buf = jvm.get_field(&this, "buf", "[B").await?;
        let buf = jvm.load_byte_array(&buf, 0, jvm.array_length(&buf).await?).await?;

        let entry = jvm.new_class("java/util/zip/ZipEntry", "(Ljava/lang/String;)V", (name.clone(),)).await?;
        let name = JavaLangString::to_rust_string(jvm, &name).await?;

        let file_size = {
            // XXX
            extern crate std;
            use std::io::Cursor;

            let mut zip = ZipArchive::new(Cursor::new(cast_vec(buf))).unwrap();
            let file = zip.by_name(&name).unwrap();

            file.size()
        };

        jvm.invoke_virtual(&entry, "setSize", "(J)V", (file_size as i64,)).await?;

        Ok(entry.into())
    }
}

#[cfg(test)]
mod test {
    use jvm::{runtime::JavaLangString, Result};

    use crate::test::test_jvm_filesystem;

    #[futures_test::test]
    async fn test_zip() -> Result<()> {
        let jar = include_bytes!("../../../../../../test_data/test.jar");
        let filesystem = [("test.jar".into(), jar.to_vec())].into_iter().collect();
        let jvm = test_jvm_filesystem(filesystem).await?;

        let name = JavaLangString::from_rust_string(&jvm, "test.jar").await?;
        let file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (name,)).await?;
        let zip = jvm.new_class("java/util/zip/ZipFile", "(Ljava/io/File;)V", (file,)).await?;

        let entry_name = JavaLangString::from_rust_string(&jvm, "test.txt").await?;
        let entry = jvm
            .invoke_virtual(&zip, "getEntry", "(Ljava/lang/String;)Ljava/util/zip/ZipEntry;", (entry_name,))
            .await?;

        let size: i64 = jvm.invoke_virtual(&entry, "getSize", "()J", ()).await?;

        assert_eq!(size, 13);

        Ok(())
    }
}
