use core::iter;

use alloc::{string::ToString, vec, vec::Vec};

use bytemuck::cast_vec;
use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{runtime::JavaLangString, ClassInstanceRef, Jvm, Result};
use zip::ZipArchive;

use crate::{
    classes::java::{
        io::{File, InputStream},
        lang::String,
        util::{zip::ZipEntry, Enumeration},
    },
    RuntimeClassProto, RuntimeContext,
};

// class java.util.zip.ZipFile
pub struct ZipFile {}

impl ZipFile {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/zip/ZipFile",
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
                JavaMethodProto::new(
                    "getInputStream",
                    "(Ljava/util/zip/ZipEntry;)Ljava/io/InputStream;",
                    Self::get_input_stream,
                    Default::default(),
                ),
                JavaMethodProto::new("entries", "()Ljava/util/Enumeration;", Self::entries, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("buf", "[B", Default::default())],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, file: ClassInstanceRef<File>) -> Result<()> {
        tracing::debug!("java.util.zip.ZipFile::<init>({:?}, {:?})", &this, &file,);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

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
            let file = zip.by_name(&name);

            file.map(|x| x.size())
        };

        if let Ok(x) = file_size {
            let _: () = jvm.invoke_virtual(&entry, "setSize", "(J)V", (x as i64,)).await?;

            Ok(entry.into())
        } else {
            Ok(None.into())
        }
    }

    async fn entries(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Enumeration>> {
        tracing::debug!("java.util.zip.ZipFile::entries({:?})", &this);

        let buf = jvm.get_field(&this, "buf", "[B").await?;
        let buf = jvm.load_byte_array(&buf, 0, jvm.array_length(&buf).await?).await?;

        let names = {
            // XXX
            extern crate std;
            use std::io::Cursor;

            let zip = ZipArchive::new(Cursor::new(cast_vec(buf))).unwrap();
            zip.file_names().map(|x| x.to_string()).collect::<Vec<_>>()
        };

        let mut name_array = jvm.instantiate_array("Ljava/lang/String;", names.len() as _).await?;
        for (i, name) in names.iter().enumerate() {
            let name = JavaLangString::from_rust_string(jvm, name).await?;
            jvm.store_array(&mut name_array, i as _, iter::once(name)).await?;
        }

        let entries = jvm
            .new_class(
                "java/util/zip/ZipFile$Entries",
                "(Ljava/util/zip/ZipFile;[Ljava/lang/String;)V",
                (this, name_array),
            )
            .await?;

        Ok(entries.into())
    }

    async fn get_input_stream(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        entry: ClassInstanceRef<ZipEntry>,
    ) -> Result<ClassInstanceRef<InputStream>> {
        tracing::debug!("java.util.zip.ZipFile::getInputStream({:?}, {:?})", &this, &entry);

        let buf = jvm.get_field(&this, "buf", "[B").await?;
        let buf = jvm.load_byte_array(&buf, 0, jvm.array_length(&buf).await?).await?;

        let entry_name = jvm.invoke_virtual(&entry, "getName", "()Ljava/lang/String;", ()).await?;
        let entry_name = JavaLangString::to_rust_string(jvm, &entry_name).await?;

        let data = {
            // XXX
            extern crate std;
            use std::io::{Cursor, Read};

            let mut zip = ZipArchive::new(Cursor::new(cast_vec(buf))).unwrap();
            let mut file = zip.by_name(&entry_name).unwrap();

            let mut buf = Vec::new();
            file.read_to_end(&mut buf).unwrap();

            buf
        };
        // TODO do we have to use InflaterInputStream?

        let mut java_buf = jvm.instantiate_array("B", data.len() as _).await?;
        jvm.store_byte_array(&mut java_buf, 0, cast_vec(data)).await?;

        let input_stream = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (java_buf,)).await?;

        Ok(input_stream.into())
    }
}

#[cfg(test)]
mod test {
    use bytemuck::cast_vec;

    use jvm::{runtime::JavaLangString, Result};

    use crate::test::test_jvm_filesystem;

    #[tokio::test]
    async fn test_zip_entry() -> Result<()> {
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

        let is = jvm
            .invoke_virtual(&zip, "getInputStream", "(Ljava/util/zip/ZipEntry;)Ljava/io/InputStream;", (entry,))
            .await?;

        let buf = jvm.instantiate_array("B", size as _).await?;
        let _: i32 = jvm.invoke_virtual(&is, "read", "([B)I", (buf.clone(),)).await?;

        let data = jvm.load_byte_array(&buf, 0, size as _).await?;
        assert_eq!(cast_vec::<i8, u8>(data), b"test content\n".to_vec());

        Ok(())
    }
}
