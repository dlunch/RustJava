use alloc::{boxed::Box, vec};

use jvm::{Array, ClassInstance, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    FileOpenOptions, RuntimeContext,
    classes::java::io::{File, FileDescriptor},
};

// class java.io.FileInputStream
#[java_class(name = "java/io/FileInputStream", extends = "java/io/InputStream", public)]
pub struct FileInputStream {
    #[field(private, final)]
    fd: ClassInstanceRef<FileDescriptor>,
}

#[java_methods]
impl FileInputStream {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, file: ClassInstanceRef<File>) -> Result<()> {
        tracing::debug!("java.io.FileInputStream::<init>({this:?}, {file:?})");
        if file.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "argument is null").await);
        }

        let path: Box<dyn ClassInstance> = jvm.invoke_virtual(&file, "java/io/File", "getPath", "()Ljava/lang/String;", ()).await?;
        let path = JavaLangString::to_rust_string(jvm, &path).await?;

        let fd = context
            .open(
                &path,
                FileOpenOptions {
                    read: true,
                    ..Default::default()
                },
            )
            .await;
        if fd.is_err() {
            return Err(jvm.exception("java/io/FileNotFoundException", "File not found").await);
        }

        let fd = FileDescriptor::from_fd(jvm, fd.unwrap()).await?;

        let _: () = jvm
            .invoke_special(&this, "java/io/FileInputStream", "<init>", "(Ljava/io/FileDescriptor;)V", (fd,))
            .await?;

        Ok(())
    }

    #[method(constructor, public)]
    async fn init_with_file_descriptor(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        fd: ClassInstanceRef<FileDescriptor>,
    ) -> Result<()> {
        tracing::debug!("java.io.FileInputStream::<init>({this:?}, {fd:?})");
        if fd.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "argument is null").await);
        }

        let _: () = jvm.invoke_special(&this, "java/io/InputStream", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "java/io/FileInputStream", "fd", "Ljava/io/FileDescriptor;", fd)
            .await?;

        Ok(())
    }

    #[method(public)]
    async fn available(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.io.FileInputStream::available({this:?})");

        let fd = jvm.get_field(&this, "java/io/FileInputStream", "fd", "Ljava/io/FileDescriptor;").await?;
        let rust_file = FileDescriptor::file(jvm, context, fd).await?;

        // TODO get os buffer size
        let Ok(stat) = rust_file.metadata().await else {
            return Err(jvm.exception("java/io/IOException", "I/O error").await);
        };
        let Ok(tell) = rust_file.tell().await else {
            return Err(jvm.exception("java/io/IOException", "I/O error").await);
        };

        let available = stat.size - tell;

        Ok(available as _)
    }

    #[method(name = "read", public)]
    async fn read_array(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mut buf: ClassInstanceRef<Array<i8>>,
        offset: i32,
        length: i32,
    ) -> Result<i32> {
        tracing::debug!("java.io.FileInputStream::read({this:?}, {buf:?}, {offset:?}, {length:?})");
        if buf.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "argument is null").await);
        }

        let array_length = jvm.array_length(&buf).await? as i32;
        if offset < 0 || length < 0 || offset > array_length - length {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "Invalid offset or length").await);
        }

        let fd = jvm.get_field(&this, "java/io/FileInputStream", "fd", "Ljava/io/FileDescriptor;").await?;
        let mut rust_file = FileDescriptor::file(jvm, context, fd).await?;

        let mut rust_buf = vec![0; length as _];
        let Ok(read) = rust_file.read(&mut rust_buf).await else {
            return Err(jvm.exception("java/io/IOException", "I/O error").await);
        };
        if read == 0 {
            return Ok(-1);
        }

        jvm.array_raw_buffer_mut(&mut buf).await?.write(offset as _, &rust_buf[..read])?;

        Ok(read as _)
    }

    #[method(name = "read", public)]
    async fn read_byte(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.io.FileInputStream::read({this:?})");

        let fd = jvm.get_field(&this, "java/io/FileInputStream", "fd", "Ljava/io/FileDescriptor;").await?;
        let mut rust_file = FileDescriptor::file(jvm, context, fd).await?;

        let mut buf = [0; 1];
        let Ok(read) = rust_file.read(&mut buf).await else {
            return Err(jvm.exception("java/io/IOException", "I/O error").await);
        };
        if read == 0 {
            return Ok(-1);
        }

        Ok(buf[0] as i32)
    }

    #[method(public)]
    async fn close(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.FileInputStream::close({this:?})");

        let fd = jvm.get_field(&this, "java/io/FileInputStream", "fd", "Ljava/io/FileDescriptor;").await?;
        FileDescriptor::close(jvm, context, fd).await?;

        Ok(())
    }
}
