use alloc::{vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

// class java.io.ByteArrayOutputStream
pub struct ByteArrayOutputStream;

impl ByteArrayOutputStream {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/ByteArrayOutputStream",
            parent_class: Some("java/io/OutputStream"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(I)V", Self::init_with_size, Default::default()),
                JavaMethodProto::new("write", "(I)V", Self::write, Default::default()),
                JavaMethodProto::new("toByteArray", "()[B", Self::to_byte_array, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("buf", "[B", Default::default()),
                JavaFieldProto::new("pos", "I", Default::default()),
            ],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.ByteArrayOutputStream::<init>({:?})", &this);

        let _: () = jvm
            .invoke_special(&this, "java/io/ByteArrayOutputStream", "<init>", "(I)V", (1024,))
            .await?;

        Ok(())
    }

    async fn init_with_size(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, size: i32) -> Result<()> {
        tracing::debug!("java.io.ByteArrayOutputStream::<init>({:?}, {:?})", &this, size);

        let _: () = jvm.invoke_special(&this, "java/io/OutputStream", "<init>", "()V", ()).await?;

        let array = jvm.instantiate_array("B", 1024).await?;

        jvm.put_field(&mut this, "buf", "[B", array).await?;
        jvm.put_field(&mut this, "pos", "I", 0).await?;

        Ok(())
    }

    async fn write(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, b: i32) -> Result<()> {
        tracing::debug!("java.io.ByteArrayOutputStream::write({:?}, {:?})", &this, b);

        let pos: i32 = jvm.get_field(&this, "pos", "I").await?;
        Self::ensure_capacity(jvm, &mut this, (pos + 1) as _).await?;

        let mut buf = jvm.get_field(&this, "buf", "[B").await?;
        jvm.store_byte_array(&mut buf, pos as _, vec![b as i8]).await?;

        jvm.put_field(&mut this, "pos", "I", pos + 1).await?;

        Ok(())
    }

    async fn to_byte_array(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Array<i8>>> {
        tracing::debug!("java.io.ByteArrayOutputStream::to_byte_array({:?})", &this);

        let buf = jvm.get_field(&this, "buf", "[B").await?;
        let pos: i32 = jvm.get_field(&this, "pos", "I").await?;

        let bytes = jvm.load_byte_array(&buf, 0, pos as _).await?;
        let mut array = jvm.instantiate_array("B", bytes.len()).await?;
        jvm.store_byte_array(&mut array, 0, bytes).await?;

        Ok(array.into())
    }

    async fn ensure_capacity(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, capacity: usize) -> Result<()> {
        let old_buf = jvm.get_field(this, "buf", "[B").await?;
        let current_capacity = jvm.array_length(&old_buf).await?;

        if current_capacity < capacity {
            let old_values: Vec<i8> = jvm.load_byte_array(&old_buf, 0, current_capacity).await?;
            let new_capacity = capacity * 2;

            let mut new_buf = jvm.instantiate_array("B", new_capacity).await?;
            jvm.put_field(this, "buf", "[B", new_buf.clone()).await?;
            jvm.store_byte_array(&mut new_buf, 0, old_values).await?;
            jvm.destroy(old_buf)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {

    use bytemuck::cast_vec;

    use jvm::Result;

    use crate::test::test_jvm;

    #[tokio::test]
    async fn test_byte_array_output_stream() -> Result<()> {
        let jvm = test_jvm().await?;

        let stream = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?;
        let _: () = jvm.invoke_virtual(&stream, "write", "(I)V", (b'H' as i32,)).await?;
        let _: () = jvm.invoke_virtual(&stream, "write", "(I)V", (b'e' as i32,)).await?;
        let _: () = jvm.invoke_virtual(&stream, "write", "(I)V", (b'l' as i32,)).await?;
        let _: () = jvm.invoke_virtual(&stream, "write", "(I)V", (b'l' as i32,)).await?;
        let _: () = jvm.invoke_virtual(&stream, "write", "(I)V", (b'o' as i32,)).await?;

        let buf = jvm.invoke_virtual(&stream, "toByteArray", "()[B", ()).await?;

        let bytes = jvm.load_byte_array(&buf, 0, 5).await?;
        assert_eq!(bytes, cast_vec(b"Hello".to_vec()));

        Ok(())
    }
}
