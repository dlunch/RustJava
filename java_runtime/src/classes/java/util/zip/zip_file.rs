use alloc::{string::ToString, sync::Arc, vec, vec::Vec};
use core::iter;

// XXX for zip..
extern crate std;
use std::io::{Cursor, Read};

use parking_lot::Mutex;
use zip::ZipArchive;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::{File, InputStream},
        lang::String,
        util::{Enumeration, zip::ZipEntry},
    },
};

type JavaZipArchive = Arc<Mutex<ZipArchive<Cursor<Vec<u8>>>>>;

// class java.util.zip.ZipFile
pub struct ZipFile;

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
            fields: vec![JavaFieldProto::new("zip", "[B", Default::default())],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, file: ClassInstanceRef<File>) -> Result<()> {
        tracing::debug!("java.util.zip.ZipFile::<init>({:?}, {:?})", &this, &file,);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        let length: i64 = jvm.invoke_virtual(&file, "length", "()J", ()).await?;
        let is = jvm.new_class("java/io/FileInputStream", "(Ljava/io/File;)V", (file,)).await?;

        let buf = jvm.instantiate_array("B", length as _).await?;
        let _: i32 = jvm.invoke_virtual(&is, "read", "([B)I", (buf.clone(),)).await?;

        let mut rust_buf = vec![0; length as _];
        jvm.array_raw_buffer(&buf).await?.read(0, &mut rust_buf).unwrap();
        let zip = Arc::new(Mutex::new(ZipArchive::new(Cursor::new(rust_buf)).unwrap()));
        jvm.put_rust_object_field(&mut this, "zip", zip).await?;

        Ok(())
    }

    async fn get_entry(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<ZipEntry>> {
        tracing::debug!("java.util.zip.ZipFile::getEntry({:?}, {:?})", &this, &name);

        let entry = jvm.new_class("java/util/zip/ZipEntry", "(Ljava/lang/String;)V", (name.clone(),)).await?;
        let name = JavaLangString::to_rust_string(jvm, &name).await?;

        let zip: JavaZipArchive = jvm.get_rust_object_field(&this, "zip").await?;
        let file_size = zip.lock().by_name(&name).map(|x| x.size());

        if let Ok(x) = file_size {
            let _: () = jvm.invoke_virtual(&entry, "setSize", "(J)V", (x as i64,)).await?;

            Ok(entry.into())
        } else {
            Ok(None.into())
        }
    }

    async fn entries(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Enumeration>> {
        tracing::debug!("java.util.zip.ZipFile::entries({:?})", &this);

        let zip: JavaZipArchive = jvm.get_rust_object_field(&this, "zip").await?;
        let names = zip.lock().file_names().map(|x| x.to_string()).collect::<Vec<_>>();

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

        let entry_name = jvm.invoke_virtual(&entry, "getName", "()Ljava/lang/String;", ()).await?;
        let entry_name = JavaLangString::to_rust_string(jvm, &entry_name).await?;

        let data = {
            let zip: JavaZipArchive = jvm.get_rust_object_field(&this, "zip").await?;

            let mut zip = zip.lock();
            let mut file = zip.by_name(&entry_name).unwrap();

            let mut buf = Vec::new();
            file.read_to_end(&mut buf).unwrap();

            buf
        };

        // TODO do we have to use InflaterInputStream?
        let mut java_buf = jvm.instantiate_array("B", data.len() as _).await?;
        jvm.array_raw_buffer_mut(&mut java_buf).await?.write(0, &data).unwrap();

        let input_stream = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (java_buf,)).await?;

        Ok(input_stream.into())
    }
}
