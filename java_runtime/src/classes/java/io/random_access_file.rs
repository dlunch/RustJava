use alloc::vec;

use bytemuck::cast_vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{runtime::JavaLangString, Array, ClassInstanceRef, Jvm, Result};

use crate::{
    classes::java::{io::FileDescriptor, lang::String},
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
                JavaMethodProto::new("read", "([B)I", Self::read, Default::default()),
                JavaMethodProto::new("write", "([B)V", Self::write, Default::default()),
                JavaMethodProto::new("seek", "(J)V", Self::seek, Default::default()),
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

        let rust_file = context.open(&name).await.unwrap();
        let fd = FileDescriptor::from_file(jvm, rust_file).await?;
        jvm.put_field(&mut this, "fd", "Ljava/io/FileDescriptor;", fd).await?;

        Ok(())
    }

    async fn read(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, mut buf: ClassInstanceRef<Array<i8>>) -> Result<i32> {
        tracing::debug!("java.io.RandomAccessFile::read({:?}, {:?})", &this, &buf);

        let fd = jvm.get_field(&this, "fd", "Ljava/io/FileDescriptor;").await?;
        let mut rust_file = FileDescriptor::file(jvm, fd).await?;

        let mut rust_buf = vec![0; jvm.array_length(&buf).await?];
        let read = rust_file.read(&mut rust_buf).await.unwrap();

        jvm.store_byte_array(&mut buf, 0, cast_vec(rust_buf)).await?;

        Ok(read as i32)
    }

    async fn write(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, buf: ClassInstanceRef<Array<i8>>) -> Result<()> {
        tracing::debug!("java.io.RandomAccessFile::write({:?}, {:?})", &this, &buf);

        let fd = jvm.get_field(&this, "fd", "Ljava/io/FileDescriptor;").await?;
        let mut rust_file = FileDescriptor::file(jvm, fd).await?;

        let buf_len = jvm.array_length(&buf).await?;
        let rust_buf = jvm.load_byte_array(&buf, 0, buf_len).await?;
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

        let buf = jvm.load_byte_array(&buf, 0, 11).await?;
        assert_eq!(buf, vec![104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100]);

        Ok(())
    }
}
