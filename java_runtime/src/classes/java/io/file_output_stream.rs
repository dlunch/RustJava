use alloc::vec;

use bytemuck::cast_slice;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{runtime::JavaLangString, Array, ClassInstanceRef, Jvm, Result};

use crate::{
    classes::java::io::{File, FileDescriptor},
    RuntimeClassProto, RuntimeContext,
};

// class java.io.FileOutputStream
pub struct FileOutputStream;

impl FileOutputStream {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/FileOutputStream",
            parent_class: Some("java/io/OutputStream"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/File;)V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/io/FileDescriptor;)V",
                    Self::init_with_file_descriptor,
                    Default::default(),
                ),
                JavaMethodProto::new("write", "([BII)V", Self::write_bytes_offset, Default::default()),
                JavaMethodProto::new("write", "(I)V", Self::write, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("fd", "Ljava/io/FileDescriptor;", Default::default())],
        }
    }

    async fn init(jvm: &Jvm, context: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, file: ClassInstanceRef<File>) -> Result<()> {
        tracing::debug!("java.io.FileOutputStream::<init>({:?}, {:?})", &this, &file);

        let _: () = jvm.invoke_special(&this, "java/io/OutputStream", "<init>", "()V", ()).await?;

        let path = jvm.invoke_virtual(&file, "getPath", "()Ljava/lang/String;", ()).await?;
        let path = JavaLangString::to_rust_string(jvm, &path).await?;

        let file = context.open(&path, true).await.unwrap();
        let fd = FileDescriptor::from_file(jvm, file).await?;

        jvm.put_field(&mut this, "fd", "Ljava/io/FileDescriptor;", fd).await?;

        Ok(())
    }

    async fn init_with_file_descriptor(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        file_descriptor: ClassInstanceRef<File>,
    ) -> Result<()> {
        tracing::debug!("java.io.FileOutputStream::<init>({:?}, {:?})", &this, &file_descriptor);

        let _: () = jvm.invoke_special(&this, "java/io/OutputStream", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "fd", "Ljava/io/FileDescriptor;", file_descriptor).await?;

        Ok(())
    }

    async fn write_bytes_offset(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        buffer: ClassInstanceRef<Array<i8>>,
        offset: i32,
        length: i32,
    ) -> Result<()> {
        tracing::debug!(
            "java.io.FileOutputStream::write({:?}, {:?}, {:?}, {:?})",
            &this,
            &buffer,
            &offset,
            &length
        );

        let fd = jvm.get_field(&this, "fd", "Ljava/io/FileDescriptor;").await?;
        let mut file = FileDescriptor::file(jvm, fd).await?;

        let mut buf = vec![0; length as _];
        jvm.array_raw_buffer(&buffer).await?.read(offset as _, &mut buf).unwrap();

        file.write(cast_slice(&buf)).await.unwrap();

        Ok(())
    }

    async fn write(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, byte: i32) -> Result<()> {
        tracing::debug!("java.io.FileOutputStream::write({:?}, {:?})", &this, &byte);

        let fd = jvm.get_field(&this, "fd", "Ljava/io/FileDescriptor;").await?;
        let mut file = FileDescriptor::file(jvm, fd).await?;

        file.write(&[byte as u8]).await.unwrap();

        Ok(())
    }
}
