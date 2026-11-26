use alloc::{boxed::Box, vec};

use bytemuck::cast_slice;

use jvm::{Array, ClassInstance, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    FileOpenOptions, RuntimeContext,
    classes::java::io::{File, FileDescriptor},
};

// class java.io.FileOutputStream
#[java_class(name = "java/io/FileOutputStream", extends = "java/io/OutputStream", public)]
pub struct FileOutputStream {
    #[field(private, final)]
    fd: ClassInstanceRef<FileDescriptor>,
    #[field(private, final)]
    append: bool,
}

#[java_methods]
impl FileOutputStream {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, file: ClassInstanceRef<File>) -> Result<()> {
        tracing::debug!("java.io.FileOutputStream::<init>({this:?}, {file:?})");
        jvm.invoke_special(&this, "java/io/FileOutputStream", "<init>", "(Ljava/io/File;Z)V", (file, false))
            .await
    }

    #[method(constructor, public)]
    async fn init_with_append(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        file: ClassInstanceRef<File>,
        append: bool,
    ) -> Result<()> {
        tracing::debug!("java.io.FileOutputStream::<init>({this:?}, {file:?}, {append})");
        if file.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "argument is null").await);
        }

        let path: Box<dyn ClassInstance> = jvm.invoke_virtual(&file, "java/io/File", "getPath", "()Ljava/lang/String;", ()).await?;
        let path = JavaLangString::to_rust_string(jvm, &path).await?;

        let fd = context
            .open(
                &path,
                FileOpenOptions {
                    write: true,
                    append,
                    truncate: !append,
                    create: true,
                    ..Default::default()
                },
            )
            .await;
        if fd.is_err() {
            return Err(jvm.exception("java/io/FileNotFoundException", "File not found").await);
        }

        let fd = FileDescriptor::from_fd(jvm, fd.unwrap()).await?;
        let _: () = jvm.invoke_special(&this, "java/io/OutputStream", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/io/FileOutputStream", "fd", "Ljava/io/FileDescriptor;", fd)
            .await?;
        jvm.put_field(&mut this, "java/io/FileOutputStream", "append", "Z", append).await
    }

    #[method(constructor, public)]
    async fn init_with_file_descriptor(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        file_descriptor: ClassInstanceRef<FileDescriptor>,
    ) -> Result<()> {
        tracing::debug!("java.io.FileOutputStream::<init>({this:?}, {file_descriptor:?})");
        if file_descriptor.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "argument is null").await);
        }

        let _: () = jvm.invoke_special(&this, "java/io/OutputStream", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "java/io/FileOutputStream", "fd", "Ljava/io/FileDescriptor;", file_descriptor)
            .await?;
        jvm.put_field(&mut this, "java/io/FileOutputStream", "append", "Z", false).await?;

        Ok(())
    }

    #[method(name = "write", public)]
    async fn write_bytes_offset(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        buffer: ClassInstanceRef<Array<i8>>,
        offset: i32,
        length: i32,
    ) -> Result<()> {
        tracing::debug!("java.io.FileOutputStream::write({this:?}, {buffer:?}, {offset:?}, {length:?})");
        if buffer.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "argument is null").await);
        }

        let fd = jvm.get_field(&this, "java/io/FileOutputStream", "fd", "Ljava/io/FileDescriptor;").await?;
        let mut file = FileDescriptor::file(jvm, context, fd).await?;

        let mut buf = vec![0; length as _];
        jvm.array_raw_buffer(&buffer).await?.read(offset as _, &mut buf)?;

        let bytes = cast_slice(&buf);
        let mut written = 0;
        while written < bytes.len() {
            match file.write(&bytes[written..]).await {
                Ok(0) | Err(_) => return Err(jvm.exception("java/io/IOException", "I/O error").await),
                Ok(length) if length > bytes.len() - written => return Err(jvm.exception("java/io/IOException", "I/O error").await),
                Ok(length) => written += length,
            }
        }

        Ok(())
    }

    #[method(public)]
    async fn write(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, byte: i32) -> Result<()> {
        tracing::debug!("java.io.FileOutputStream::write({this:?}, {byte:?})");

        let fd = jvm.get_field(&this, "java/io/FileOutputStream", "fd", "Ljava/io/FileDescriptor;").await?;
        let mut file = FileDescriptor::file(jvm, context, fd).await?;

        if !matches!(file.write(&[byte as u8]).await, Ok(1)) {
            return Err(jvm.exception("java/io/IOException", "I/O error").await);
        }

        Ok(())
    }

    #[method(public)]
    async fn close(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.FileOutputStream::close({this:?})");

        let fd = jvm.get_field(&this, "java/io/FileOutputStream", "fd", "Ljava/io/FileDescriptor;").await?;
        FileDescriptor::close(jvm, context, fd).await?;

        Ok(())
    }
}
