use alloc::vec;

use bytemuck::cast_vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{Array, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::io::{File, FileDescriptor},
};

// class java.io.FileInputStream
pub struct FileInputStream;

impl FileInputStream {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/FileInputStream",
            parent_class: Some("java/io/InputStream"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/File;)V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/io/FileDescriptor;)V",
                    Self::init_with_file_descriptor,
                    Default::default(),
                ),
                JavaMethodProto::new("read", "()I", Self::read_byte, Default::default()),
                JavaMethodProto::new("read", "([BII)I", Self::read_array, Default::default()),
                JavaMethodProto::new("available", "()I", Self::available, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("fd", "Ljava/io/FileDescriptor;", Default::default())],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, file: ClassInstanceRef<File>) -> Result<()> {
        tracing::debug!("java.io.FileInputStream::<init>({:?}, {:?})", &this, &file);

        let path = jvm.invoke_virtual(&file, "getPath", "()Ljava/lang/String;", ()).await?;
        let path = JavaLangString::to_rust_string(jvm, &path).await?;

        let rust_file = context.open(&path, false).await;
        if rust_file.is_err() {
            // TODO correct error handling
            return Err(jvm.exception("java/io/FileNotFoundException", "File not found").await);
        }

        let fd = FileDescriptor::from_file(jvm, rust_file.unwrap()).await?;

        let _: () = jvm
            .invoke_special(&this, "java/io/FileInputStream", "<init>", "(Ljava/io/FileDescriptor;)V", (fd,))
            .await?;

        Ok(())
    }

    async fn init_with_file_descriptor(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        fd: ClassInstanceRef<FileDescriptor>,
    ) -> Result<()> {
        tracing::debug!("java.io.FileInputStream::<init>({:?}, {:?})", &this, &fd);

        let _: () = jvm.invoke_special(&this, "java/io/InputStream", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "fd", "Ljava/io/FileDescriptor;", fd).await?;

        Ok(())
    }

    async fn available(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.io.FileInputStream::available({:?})", &this);

        let fd = jvm.get_field(&this, "fd", "Ljava/io/FileDescriptor;").await?;
        let rust_file = FileDescriptor::file(jvm, fd).await?;

        // TODO get os buffer size
        let stat = rust_file.metadata().await.unwrap();
        let tell = rust_file.tell().await.unwrap();

        let available = stat.size - tell;

        Ok(available as _)
    }

    async fn read_array(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mut buf: ClassInstanceRef<Array<i8>>,
        offset: i32,
        length: i32,
    ) -> Result<i32> {
        tracing::debug!("java.io.FileInputStream::read({this:?}, {buf:?}, {offset:?}, {length:?})");

        let fd = jvm.get_field(&this, "fd", "Ljava/io/FileDescriptor;").await?;
        let mut rust_file = FileDescriptor::file(jvm, fd).await?;

        let mut rust_buf = vec![0; length as _];
        let read = rust_file.read(&mut rust_buf).await.unwrap();
        if read == 0 {
            return Ok(-1);
        }

        jvm.store_array(&mut buf, offset as _, cast_vec::<u8, i8>(rust_buf)).await?;

        Ok(read as _)
    }

    async fn read_byte(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.io.FileInputStream::read({:?})", &this);

        let fd = jvm.get_field(&this, "fd", "Ljava/io/FileDescriptor;").await?;
        let mut rust_file = FileDescriptor::file(jvm, fd).await?;

        let mut buf = [0; 1];
        let read = rust_file.read(&mut buf).await.unwrap();
        if read == 0 {
            return Ok(-1);
        }

        Ok(buf[0] as i32)
    }

    async fn close(_jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("stub java.io.FileInputStream::close({:?})", &this);

        Ok(())
    }
}
