use alloc::vec;

use bytemuck::cast_vec;

use java_class_proto::JavaMethodProto;
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::io::InputStream};

// class java.io.DataOutputStream
pub struct DataOutputStream;

impl DataOutputStream {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/DataOutputStream",
            parent_class: Some("java/io/FilterOutputStream"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/OutputStream;)V", Self::init, Default::default()),
                JavaMethodProto::new("write", "(I)V", Self::write, Default::default()),
                JavaMethodProto::new("writeByte", "(I)V", Self::write_byte, Default::default()),
                JavaMethodProto::new("writeBoolean", "(Z)V", Self::write_boolean, Default::default()),
                JavaMethodProto::new("writeInt", "(I)V", Self::write_int, Default::default()),
                JavaMethodProto::new("writeShort", "(I)V", Self::write_short, Default::default()),
                JavaMethodProto::new("writeLong", "(J)V", Self::write_long, Default::default()),
                JavaMethodProto::new("writeChars", "(Ljava/lang/String;)V", Self::write_chars, Default::default()),
                JavaMethodProto::new("writeUTF", "(Ljava/lang/String;)V", Self::write_utf, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
                JavaMethodProto::new("flush", "()V", Self::flush, Default::default()),
            ],
            fields: vec![],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, out: ClassInstanceRef<InputStream>) -> Result<()> {
        tracing::debug!("java.io.DataOutputStream::<init>({this:?}, {out:?})");

        let _: () = jvm
            .invoke_special(&this, "java/io/FilterOutputStream", "<init>", "(Ljava/io/OutputStream;)V", (out,))
            .await?;

        Ok(())
    }

    async fn write(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, b: i32) -> Result<()> {
        tracing::debug!("java.io.DataOutputStream::write({this:?}, {b:?})");

        let out = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
        let _: () = jvm.invoke_virtual(&out, "write", "(I)V", (b,)).await?;

        Ok(())
    }

    async fn write_byte(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, v: i32) -> Result<()> {
        tracing::debug!("java.io.DataOutputStream::writeByte({this:?}, {v:?})");

        let out = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
        let _: () = jvm.invoke_virtual(&out, "write", "(I)V", (v,)).await?;

        Ok(())
    }

    async fn write_boolean(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, v: bool) -> Result<()> {
        tracing::debug!("java.io.DataOutputStream::writeBoolean({this:?}, {v:?})");

        let _: () = jvm.invoke_virtual(&this, "writeByte", "(I)V", (v as i32,)).await?;

        Ok(())
    }

    async fn write_short(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, s: i32) -> Result<()> {
        tracing::debug!("java.io.DataOutputStream::writeShort({:?}, {:?})", &this, s);

        let bytes = (s as i16).to_be_bytes();
        let mut byte_array = jvm.instantiate_array("B", bytes.len() as _).await?;
        jvm.store_array(&mut byte_array, 0, cast_vec::<u8, i8>(bytes.to_vec())).await?;

        let out = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
        let _: () = jvm.invoke_virtual(&out, "write", "([B)V", (byte_array,)).await?;

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

    async fn write_utf(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, s: ClassInstanceRef<JavaChar>) -> Result<()> {
        tracing::debug!("java.io.DataOutputStream::writeUTF({:?}, {:?})", &this, &s);

        // TODO handle modified utf-8
        let bytes: ClassInstanceRef<Array<i8>> = jvm.invoke_virtual(&s, "getBytes", "()[B", ()).await?;
        let length = jvm.array_length(&bytes).await?;

        let _: () = jvm.invoke_virtual(&this, "writeShort", "(I)V", (length as i32,)).await?;

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
