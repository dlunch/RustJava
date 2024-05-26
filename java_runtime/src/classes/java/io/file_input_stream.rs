use alloc::vec;

use bytemuck::cast_vec;
use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{runtime::JavaLangString, Array, ClassInstanceRef, Jvm, Result};

use crate::{
    classes::java::io::{File, FileDescriptor},
    RuntimeClassProto, RuntimeContext,
};

// class java.io.FileInputStream
pub struct FileInputStream {}

impl FileInputStream {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/io/InputStream"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/File;)V", Self::init, Default::default()),
                JavaMethodProto::new("read", "([B)I", Self::read, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("fd", "Ljava/io/FileDescriptor;", Default::default())],
        }
    }

    async fn init(jvm: &Jvm, context: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, file: ClassInstanceRef<File>) -> Result<()> {
        tracing::debug!("java.io.FileInputStream::<init>({:?}, {:?})", &this, &file);

        let path = jvm.invoke_virtual(&file, "getPath", "()Ljava/lang/String;", ()).await?;
        let path = JavaLangString::to_rust_string(jvm, &path).await?;

        let rust_file = context.open(&path).await.unwrap();

        let fd = FileDescriptor::from_file(jvm, rust_file).await?;

        jvm.put_field(&mut this, "fd", "Ljava/io/FileDescriptor;", fd).await?;

        Ok(())
    }

    async fn read(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, mut buf: ClassInstanceRef<Array<i8>>) -> Result<i32> {
        tracing::debug!("java.io.FileInputStream::read({:?}, {:?})", &this, &buf);

        let length = jvm.array_length(&buf).await?;

        let fd = jvm.get_field(&this, "fd", "Ljava/io/FileDescriptor;").await?;
        let mut rust_file = FileDescriptor::file(jvm, fd).await?;

        let mut rust_buf = vec![0; length as usize];
        let read = rust_file.read(&mut rust_buf).await.unwrap();

        jvm.store_byte_array(&mut buf, 0, cast_vec(rust_buf)).await?;

        Ok(read as _)
    }
}
