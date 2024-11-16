use alloc::vec;

use bytemuck::cast_vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{runtime::JavaLangString, Array, ClassInstanceRef, Jvm, Result};

use crate::{
    classes::java::{
        io::{File, FileDescriptor},
        lang::String,
    },
    RuntimeClassProto, RuntimeContext,
};

// class java.io.RandomAccessFile
pub struct RandomAccessFile;

impl RandomAccessFile {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/RandomAccessFile",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/String;Ljava/lang/String;)V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/io/File;Ljava/lang/String;)V", Self::init_with_file, Default::default()),
                JavaMethodProto::new("read", "([B)I", Self::read, Default::default()),
                JavaMethodProto::new("read", "([BII)I", Self::read_offset_length, Default::default()),
                JavaMethodProto::new("write", "([B)V", Self::write, Default::default()),
                JavaMethodProto::new("write", "([BII)V", Self::write_offset_length, Default::default()),
                JavaMethodProto::new("length", "()J", Self::length, Default::default()),
                JavaMethodProto::new("getFilePointer", "()J", Self::get_file_pointer, Default::default()),
                JavaMethodProto::new("seek", "(J)V", Self::seek, Default::default()),
                JavaMethodProto::new("setLength", "(J)V", Self::set_length, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("fd", "Ljava/io/FileDescriptor;", Default::default())],
        }
    }

    async fn init(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
        mode: ClassInstanceRef<String>,
    ) -> Result<()> {
        tracing::debug!("java.io.RandomAccessFile::<init>({:?}, {:?}, {:?})", &this, &name, &mode);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        let name = JavaLangString::to_rust_string(jvm, &name).await?;
        let mode = JavaLangString::to_rust_string(jvm, &mode).await?;

        let write = mode.contains('w');

        let rust_file = context.open(&name, write).await;
        if rust_file.is_err() {
            // TODO correct error handling
            return Err(jvm.exception("java/io/FileNotFoundException", "File not found").await);
        }
        let fd = FileDescriptor::from_file(jvm, rust_file.unwrap()).await?;
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
        tracing::debug!("java.io.RandomAccessFile::<init>({:?}, {:?}, {:?})", &this, &file, &mode);

        let name: ClassInstanceRef<String> = jvm.invoke_virtual(&file, "getPath", "()Ljava/lang/String;", ()).await?;

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
        tracing::debug!("java.io.RandomAccessFile::read({:?}, {:?})", &this, &buf);

        let length = jvm.array_length(&buf).await?;
        let read = jvm.invoke_virtual(&this, "read", "([BII)I", (buf, 0, length as i32)).await?;

        Ok(read)
    }

    async fn read_offset_length(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mut buf: ClassInstanceRef<Array<i8>>,
        offset: i32,
        length: i32,
    ) -> Result<i32> {
        tracing::debug!(
            "java.io.RandomAccessFile::read_offset_length({:?}, {:?}, {:?}, {:?})",
            &this,
            &buf,
            &offset,
            &length
        );

        let fd = jvm.get_field(&this, "fd", "Ljava/io/FileDescriptor;").await?;
        let mut rust_file = FileDescriptor::file(jvm, fd).await?;

        let mut rust_buf = vec![0; length as usize];
        let read = rust_file.read(&mut rust_buf).await.unwrap();

        jvm.array_raw_buffer_mut(&mut buf).await?.write(offset as _, &rust_buf)?;

        Ok(read as i32)
    }

    async fn write(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, buf: ClassInstanceRef<Array<i8>>) -> Result<()> {
        tracing::debug!("java.io.RandomAccessFile::write({:?}, {:?})", &this, &buf);

        let length = jvm.array_length(&buf).await?;
        let _: () = jvm.invoke_virtual(&this, "write", "([BII)V", (buf, 0, length as i32)).await?;

        Ok(())
    }

    async fn write_offset_length(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        buf: ClassInstanceRef<Array<i8>>,
        offset: i32,
        length: i32,
    ) -> Result<()> {
        tracing::debug!(
            "java.io.RandomAccessFile::write_offset_length({:?}, {:?}, {:?}, {:?})",
            &this,
            &buf,
            &offset,
            &length
        );

        let fd = jvm.get_field(&this, "fd", "Ljava/io/FileDescriptor;").await?;
        let mut rust_file = FileDescriptor::file(jvm, fd).await?;

        let mut rust_buf = vec![0; length as usize];
        jvm.array_raw_buffer(&buf).await?.read(offset as _, &mut rust_buf).unwrap();
        rust_file.write(&cast_vec(rust_buf)).await.unwrap();

        Ok(())
    }

    async fn seek(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, pos: i64) -> Result<()> {
        tracing::debug!("java.io.RandomAccessFile::seek({:?}, {:?})", &this, &pos);

        let fd = jvm.get_field(&this, "fd", "Ljava/io/FileDescriptor;").await?;
        let mut rust_file = FileDescriptor::file(jvm, fd).await?;

        rust_file.seek(pos as _).await.unwrap();

        Ok(())
    }

    async fn set_length(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, new_length: i64) -> Result<()> {
        tracing::debug!("java.io.RandomAccessFile::set_length({:?}, {:?})", &this, &new_length);

        let fd = jvm.get_field(&this, "fd", "Ljava/io/FileDescriptor;").await?;
        let mut rust_file = FileDescriptor::file(jvm, fd).await?;

        rust_file.set_len(new_length as _).await.unwrap();

        Ok(())
    }

    async fn length(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        tracing::debug!("java.io.RandomAccessFile::length({:?})", &this);

        let fd = jvm.get_field(&this, "fd", "Ljava/io/FileDescriptor;").await?;
        let rust_file = FileDescriptor::file(jvm, fd).await?;

        let len = rust_file.metadata().await.unwrap().size;

        Ok(len as i64)
    }

    async fn get_file_pointer(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        tracing::debug!("java.io.RandomAccessFile::getFilePointer({:?})", &this);

        let fd = jvm.get_field(&this, "fd", "Ljava/io/FileDescriptor;").await?;
        let rust_file = FileDescriptor::file(jvm, fd).await?;

        let pos = rust_file.tell().await.unwrap();

        Ok(pos as i64)
    }

    async fn close(_jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::warn!("stub java.io.RandomAccessFile::close({:?})", &this);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use alloc::vec;

    use jvm::{runtime::JavaLangString, Result};

    use crate::test::test_jvm_filesystem;

    #[tokio::test]
    async fn test_random_access_file() -> Result<()> {
        let filesystem = [("test.txt".into(), b"hello world".to_vec())];
        let jvm = test_jvm_filesystem(filesystem.into_iter().collect()).await?;

        let file = JavaLangString::from_rust_string(&jvm, "test.txt").await?;
        let mode = JavaLangString::from_rust_string(&jvm, "r").await?;

        let raf = jvm
            .new_class("java/io/RandomAccessFile", "(Ljava/lang/String;Ljava/lang/String;)V", (file, mode))
            .await?;

        let buf = jvm.instantiate_array("B", 11).await?;
        let read: i32 = jvm.invoke_virtual(&raf, "read", "([B)I", (buf.clone(),)).await?;
        assert_eq!(read, 11);

        let mut rust_buf = vec![0; 11];
        jvm.array_raw_buffer(&buf).await?.read(0, &mut rust_buf).unwrap();
        assert_eq!(rust_buf, vec![104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100]);

        Ok(())
    }
}
