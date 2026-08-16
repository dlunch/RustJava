use alloc::vec;

use bytemuck::cast_vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{
    FileOpenOptions, RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::{File, FileDescriptor},
        lang::String,
    },
};

// class java.io.RandomAccessFile
pub struct RandomAccessFile;

impl RandomAccessFile {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/RandomAccessFile",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/io/DataInput", "java/io/DataOutput"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/String;Ljava/lang/String;)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/io/File;Ljava/lang/String;)V",
                    Self::init_with_file,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("read", "([B)I", Self::read, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("read", "([BII)I", Self::read_offset_length, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("write", "([B)V", Self::write, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("write", "([BII)V", Self::write_offset_length, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("length", "()J", Self::length, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getFilePointer", "()J", Self::get_file_pointer, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "getFD",
                    "()Ljava/io/FileDescriptor;",
                    Self::get_fd,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::FINAL,
                ),
                JavaMethodProto::new("seek", "(J)V", Self::seek, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("setLength", "(J)V", Self::set_length, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("close", "()V", Self::close, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![JavaFieldProto::new(
                "fd",
                "Ljava/io/FileDescriptor;",
                FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL,
            )],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
        mode: ClassInstanceRef<String>,
    ) -> Result<()> {
        tracing::debug!("java.io.RandomAccessFile::<init>({this:?}, {name:?}, {mode:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        let name = JavaLangString::to_rust_string(jvm, &name).await?;
        let mode = JavaLangString::to_rust_string(jvm, &mode).await?;

        let write = mode.contains('w');

        let fd_id = context
            .open(
                &name,
                FileOpenOptions {
                    read: true,
                    write,
                    create: write,
                    ..Default::default()
                },
            )
            .await;
        if fd_id.is_err() {
            return Err(jvm.exception("java/io/FileNotFoundException", "File not found").await);
        }
        let fd = FileDescriptor::from_fd(jvm, fd_id.unwrap()).await?;
        jvm.put_field(&mut this, "fd", "Ljava/io/FileDescriptor;", fd).await?;

        Ok(())
    }

    async fn init_with_file(
        jvm: &Jvm,
        _context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        file: ClassInstanceRef<File>,
        mode: ClassInstanceRef<String>,
    ) -> Result<()> {
        tracing::debug!("java.io.RandomAccessFile::<init>({this:?}, {file:?}, {mode:?})");

        let name: ClassInstanceRef<String> = jvm.invoke_virtual(&file, "java/io/File", "getPath", "()Ljava/lang/String;", ()).await?;

        let _: () = jvm
            .invoke_special(
                &this,
                "java/io/RandomAccessFile",
                "<init>",
                "(Ljava/lang/String;Ljava/lang/String;)V",
                (name, mode),
            )
            .await?;

        Ok(())
    }

    async fn read(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, buf: ClassInstanceRef<Array<i8>>) -> Result<i32> {
        tracing::debug!("java.io.RandomAccessFile::read({this:?}, {buf:?})");

        let length = jvm.array_length(&buf).await?;
        let read = jvm
            .invoke_virtual(&this, "java/io/RandomAccessFile", "read", "([BII)I", (buf, 0, length as i32))
            .await?;

        Ok(read)
    }

    async fn read_offset_length(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mut buf: ClassInstanceRef<Array<i8>>,
        offset: i32,
        length: i32,
    ) -> Result<i32> {
        tracing::debug!("java.io.RandomAccessFile::read({this:?}, {buf:?}, {offset:?}, {length:?})");

        let fd = jvm.get_field(&this, "fd", "Ljava/io/FileDescriptor;").await?;
        let mut rust_file = FileDescriptor::file(jvm, context, fd).await?;

        let mut rust_buf = vec![0; length as usize];
        let Ok(read) = rust_file.read(&mut rust_buf).await else {
            return Err(jvm.exception("java/io/IOException", "I/O error").await);
        };

        jvm.array_raw_buffer_mut(&mut buf).await?.write(offset as _, &rust_buf)?;

        Ok(read as i32)
    }

    async fn write(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, buf: ClassInstanceRef<Array<i8>>) -> Result<()> {
        tracing::debug!("java.io.RandomAccessFile::write({this:?}, {buf:?})");

        let length = jvm.array_length(&buf).await?;
        let _: () = jvm
            .invoke_virtual(&this, "java/io/RandomAccessFile", "write", "([BII)V", (buf, 0, length as i32))
            .await?;

        Ok(())
    }

    async fn write_offset_length(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        buf: ClassInstanceRef<Array<i8>>,
        offset: i32,
        length: i32,
    ) -> Result<()> {
        tracing::debug!("java.io.RandomAccessFile::write({this:?}, {buf:?}, {offset:?}, {length:?})");

        let fd = jvm.get_field(&this, "fd", "Ljava/io/FileDescriptor;").await?;
        let mut rust_file = FileDescriptor::file(jvm, context, fd).await?;

        let mut rust_buf = vec![0; length as usize];
        jvm.array_raw_buffer(&buf).await?.read(offset as _, &mut rust_buf)?;
        let rust_buf = cast_vec(rust_buf);
        let mut written = 0;
        while written < rust_buf.len() {
            match rust_file.write(&rust_buf[written..]).await {
                Ok(0) | Err(_) => return Err(jvm.exception("java/io/IOException", "I/O error").await),
                Ok(length) if length > rust_buf.len() - written => return Err(jvm.exception("java/io/IOException", "I/O error").await),
                Ok(length) => written += length,
            }
        }

        Ok(())
    }

    async fn seek(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, pos: i64) -> Result<()> {
        tracing::debug!("java.io.RandomAccessFile::seek({this:?}, {pos:?})");

        let fd = jvm.get_field(&this, "fd", "Ljava/io/FileDescriptor;").await?;
        let mut rust_file = FileDescriptor::file(jvm, context, fd).await?;

        if rust_file.seek(pos as _).await.is_err() {
            return Err(jvm.exception("java/io/IOException", "I/O error").await);
        }

        Ok(())
    }

    async fn set_length(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, new_length: i64) -> Result<()> {
        tracing::debug!("java.io.RandomAccessFile::setLength({this:?}, {new_length:?})");

        let fd = jvm.get_field(&this, "fd", "Ljava/io/FileDescriptor;").await?;
        let mut rust_file = FileDescriptor::file(jvm, context, fd).await?;

        if rust_file.set_len(new_length as _).await.is_err() {
            return Err(jvm.exception("java/io/IOException", "I/O error").await);
        }

        Ok(())
    }

    async fn length(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        tracing::debug!("java.io.RandomAccessFile::length({this:?})");

        let fd = jvm.get_field(&this, "fd", "Ljava/io/FileDescriptor;").await?;
        let rust_file = FileDescriptor::file(jvm, context, fd).await?;

        let Ok(metadata) = rust_file.metadata().await else {
            return Err(jvm.exception("java/io/IOException", "I/O error").await);
        };
        let len = metadata.size;

        Ok(len as i64)
    }

    async fn get_file_pointer(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        tracing::debug!("java.io.RandomAccessFile::getFilePointer({this:?})");

        let fd = jvm.get_field(&this, "fd", "Ljava/io/FileDescriptor;").await?;
        let rust_file = FileDescriptor::file(jvm, context, fd).await?;

        let Ok(pos) = rust_file.tell().await else {
            return Err(jvm.exception("java/io/IOException", "I/O error").await);
        };

        Ok(pos as i64)
    }

    async fn get_fd(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<File>> {
        tracing::debug!("java.io.RandomAccessFile::getFD({this:?})");

        let fd = jvm.get_field(&this, "fd", "Ljava/io/FileDescriptor;").await?;

        Ok(fd)
    }

    async fn close(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.RandomAccessFile::close({this:?})");

        let fd = jvm.get_field(&this, "fd", "Ljava/io/FileDescriptor;").await?;
        FileDescriptor::close(jvm, context, fd).await?;

        Ok(())
    }
}
