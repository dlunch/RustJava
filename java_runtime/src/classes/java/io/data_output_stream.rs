use alloc::vec;

use bytemuck::cast_vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result};

use crate::{classes::java::io::InputStream, RuntimeClassProto, RuntimeContext};

// class java.io.DataOutputStream
pub struct DataOutputStream;

impl DataOutputStream {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/DataOutputStream",
            parent_class: Some("java/io/OutputStream"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/OutputStream;)V", Self::init, Default::default()),
                JavaMethodProto::new("write", "(I)V", Self::write, Default::default()),
                JavaMethodProto::new("writeByte", "(I)V", Self::write, Default::default()),
                JavaMethodProto::new("writeInt", "(I)V", Self::write_int, Default::default()),
                JavaMethodProto::new("writeLong", "(J)V", Self::write_long, Default::default()),
                JavaMethodProto::new("writeChars", "(Ljava/lang/String;)V", Self::write_chars, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
                JavaMethodProto::new("flush", "()V", Self::flush, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("out", "Ljava/io/OutputStream;", Default::default())],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, r#in: ClassInstanceRef<InputStream>) -> Result<()> {
        tracing::debug!("java.io.DataOutputStream::<init>({:?}, {:?})", &this, &r#in);

        let _: () = jvm.invoke_special(&this, "java/io/OutputStream", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "out", "Ljava/io/OutputStream;", r#in).await?;

        Ok(())
    }

    async fn write(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, b: i32) -> Result<()> {
        tracing::debug!("java.io.DataOutputStream::write({:?}, {:?})", &this, b);

        let out = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
        let _: () = jvm.invoke_virtual(&out, "write", "(I)V", [b.into()]).await?;

        Ok(())
    }

    async fn write_int(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, i: i32) -> Result<()> {
        tracing::debug!("java.io.DataOutputStream::writeInt({:?}, {:?})", &this, i);

        let bytes = i.to_be_bytes();
        let mut byte_array = jvm.instantiate_array("B", bytes.len() as _).await?;
        jvm.store_array(&mut byte_array, 0, cast_vec::<u8, i8>(bytes.to_vec())).await?;

        let out = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
        let _: () = jvm.invoke_virtual(&out, "write", "([B)V", (byte_array,)).await?;

        Ok(())
    }

    async fn write_long(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, l: i64) -> Result<()> {
        tracing::debug!("java.io.DataOutputStream::writeLong({:?}, {:?})", &this, l);

        let bytes = l.to_be_bytes();
        let mut byte_array = jvm.instantiate_array("B", bytes.len() as _).await?;
        jvm.store_array(&mut byte_array, 0, cast_vec::<u8, i8>(bytes.to_vec())).await?;

        let out = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
        let _: () = jvm.invoke_virtual(&out, "write", "([B)V", (byte_array,)).await?;

        Ok(())
    }

    async fn write_chars(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, s: ClassInstanceRef<JavaChar>) -> Result<()> {
        tracing::debug!("java.io.DataOutputStream::writeChars({:?}, {:?})", &this, &s);

        let bytes: ClassInstanceRef<Array<i8>> = jvm.invoke_virtual(&s, "getBytes", "()[B", ()).await?;

        let out = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
        let _: () = jvm.invoke_virtual(&out, "write", "([B)V", (bytes,)).await?;

        Ok(())
    }

    async fn close(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.DataInputStream::close({:?})", &this);

        let out = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
        let _: () = jvm.invoke_virtual(&out, "close", "()V", []).await?;

        Ok(())
    }

    async fn flush(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.DataInputStream::flush({:?})", &this);

        let out = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
        let _: () = jvm.invoke_virtual(&out, "flush", "()V", []).await?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use alloc::vec;

    use jvm::{runtime::JavaLangString, Result};

    use crate::test::test_jvm;

    #[tokio::test]
    async fn test_data_output_stream() -> Result<()> {
        let jvm = test_jvm().await?;

        let stream = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?;
        let data_output_stream = jvm
            .new_class("java/io/DataOutputStream", "(Ljava/io/OutputStream;)V", (stream.clone(),))
            .await?;

        let _: () = jvm.invoke_virtual(&data_output_stream, "write", "(I)V", (1,)).await?;
        let string = JavaLangString::from_rust_string(&jvm, "hello, world").await?;
        let _: () = jvm
            .invoke_virtual(&data_output_stream, "writeChars", "(Ljava/lang/String;)V", (string,))
            .await?;
        let _: () = jvm.invoke_virtual(&data_output_stream, "writeInt", "(I)V", (12341234,)).await?;
        let _: () = jvm.invoke_virtual(&data_output_stream, "writeLong", "(J)V", (123412341324i64,)).await?;

        let bytes = jvm.invoke_virtual(&stream, "toByteArray", "()[B", ()).await?;

        let length = jvm.array_length(&bytes).await?;
        let mut buf = vec![0; length];
        jvm.array_raw_buffer(&bytes).await?.read(0, &mut buf)?;

        assert_eq!(
            buf,
            vec![
                1, b'h', b'e', b'l', b'l', b'o', b',', b' ', b'w', b'o', b'r', b'l', b'd', 0, 0xbc, 0x4f, 0xf2, 0, 0, 0, 0x1c, 0xbb, 0xf2, 0xe2, 0x4c
            ]
        );

        Ok(())
    }
}
