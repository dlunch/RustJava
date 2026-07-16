use alloc::{vec, vec::Vec};

use bytemuck::cast_vec;

use java_class_proto::JavaMethodProto;
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{io::OutputStream, lang::String},
};

// class java.io.DataOutputStream
pub struct DataOutputStream;

impl DataOutputStream {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/DataOutputStream",
            parent_class: Some("java/io/FilterOutputStream"),
            interfaces: vec!["java/io/DataOutput"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/OutputStream;)V", Self::init, Default::default()),
                JavaMethodProto::new("write", "(I)V", Self::write, Default::default()),
                JavaMethodProto::new("writeByte", "(I)V", Self::write_byte, Default::default()),
                JavaMethodProto::new("writeBoolean", "(Z)V", Self::write_boolean, Default::default()),
                JavaMethodProto::new("writeInt", "(I)V", Self::write_int, Default::default()),
                JavaMethodProto::new("writeShort", "(I)V", Self::write_short, Default::default()),
                JavaMethodProto::new("writeChar", "(I)V", Self::write_char, Default::default()),
                JavaMethodProto::new("writeLong", "(J)V", Self::write_long, Default::default()),
                JavaMethodProto::new("writeFloat", "(F)V", Self::write_float, Default::default()),
                JavaMethodProto::new("writeDouble", "(D)V", Self::write_double, Default::default()),
                JavaMethodProto::new("writeBytes", "(Ljava/lang/String;)V", Self::write_bytes, Default::default()),
                JavaMethodProto::new("writeChars", "(Ljava/lang/String;)V", Self::write_chars, Default::default()),
                JavaMethodProto::new("writeUTF", "(Ljava/lang/String;)V", Self::write_utf, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
                JavaMethodProto::new("flush", "()V", Self::flush, Default::default()),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, out: ClassInstanceRef<OutputStream>) -> Result<()> {
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
        tracing::debug!("java.io.DataOutputStream::writeShort({this:?}, {s:?})");

        let bytes = (s as i16).to_be_bytes();
        let mut byte_array = jvm.instantiate_array("B", bytes.len() as _).await?;
        jvm.store_array(&mut byte_array, 0, cast_vec::<u8, i8>(bytes.to_vec())).await?;

        let out = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
        let _: () = jvm.invoke_virtual(&out, "write", "([B)V", (byte_array,)).await?;

        Ok(())
    }

    async fn write_char(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        tracing::debug!("java.io.DataOutputStream::writeChar({this:?}, {value:?})");
        jvm.invoke_virtual(&this, "writeShort", "(I)V", (value & 0xffff,)).await
    }

    async fn write_int(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, i: i32) -> Result<()> {
        tracing::debug!("java.io.DataOutputStream::writeInt({this:?}, {i:?})");

        let bytes = i.to_be_bytes();
        let mut byte_array = jvm.instantiate_array("B", bytes.len() as _).await?;
        jvm.store_array(&mut byte_array, 0, cast_vec::<u8, i8>(bytes.to_vec())).await?;

        let out = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
        let _: () = jvm.invoke_virtual(&out, "write", "([B)V", (byte_array,)).await?;

        Ok(())
    }

    async fn write_long(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, l: i64) -> Result<()> {
        tracing::debug!("java.io.DataOutputStream::writeLong({this:?}, {l:?})");

        let bytes = l.to_be_bytes();
        let mut byte_array = jvm.instantiate_array("B", bytes.len() as _).await?;
        jvm.store_array(&mut byte_array, 0, cast_vec::<u8, i8>(bytes.to_vec())).await?;

        let out = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
        let _: () = jvm.invoke_virtual(&out, "write", "([B)V", (byte_array,)).await?;

        Ok(())
    }

    async fn write_float(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: f32) -> Result<()> {
        tracing::debug!("java.io.DataOutputStream::writeFloat({this:?}, {value:?})");
        jvm.invoke_virtual(&this, "writeInt", "(I)V", (value.to_bits() as i32,)).await
    }

    async fn write_double(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: f64) -> Result<()> {
        tracing::debug!("java.io.DataOutputStream::writeDouble({this:?}, {value:?})");
        jvm.invoke_virtual(&this, "writeLong", "(J)V", (value.to_bits() as i64,)).await
    }

    async fn write_bytes(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, s: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.io.DataOutputStream::writeBytes({this:?}, {s:?})");

        let chars: ClassInstanceRef<Array<JavaChar>> = jvm.invoke_virtual(&s, "toCharArray", "()[C", ()).await?;
        let length = jvm.array_length(&chars).await?;
        let chars: Vec<JavaChar> = jvm.load_array(&chars, 0, length).await?;
        let mut bytes = jvm.instantiate_array("B", chars.len()).await?;
        jvm.store_array(&mut bytes, 0, chars.into_iter().map(|value| value as i8)).await?;

        let out = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
        jvm.invoke_virtual(&out, "write", "([B)V", (bytes,)).await
    }

    async fn write_chars(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, s: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.io.DataOutputStream::writeChars({this:?}, {s:?})");

        let chars: ClassInstanceRef<Array<JavaChar>> = jvm.invoke_virtual(&s, "toCharArray", "()[C", ()).await?;
        let length = jvm.array_length(&chars).await?;
        let chars: Vec<JavaChar> = jvm.load_array(&chars, 0, length).await?;
        let mut data = Vec::with_capacity(chars.len() * 2);
        for value in chars {
            data.push((value >> 8) as i8);
            data.push(value as i8);
        }

        let mut bytes = jvm.instantiate_array("B", data.len()).await?;
        jvm.store_array(&mut bytes, 0, data).await?;

        let out = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
        jvm.invoke_virtual(&out, "write", "([B)V", (bytes,)).await
    }

    async fn write_utf(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, s: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.io.DataOutputStream::writeUTF({this:?}, {s:?})");

        let chars: ClassInstanceRef<Array<JavaChar>> = jvm.invoke_virtual(&s, "toCharArray", "()[C", ()).await?;
        let length = jvm.array_length(&chars).await?;
        let chars: Vec<JavaChar> = jvm.load_array(&chars, 0, length).await?;
        let mut data = Vec::new();
        for value in chars {
            if (0x0001..=0x007f).contains(&value) {
                data.push(value as i8);
            } else if value <= 0x07ff {
                data.push((0xc0 | ((value >> 6) & 0x1f)) as i8);
                data.push((0x80 | (value & 0x3f)) as i8);
            } else {
                data.push((0xe0 | ((value >> 12) & 0x0f)) as i8);
                data.push((0x80 | ((value >> 6) & 0x3f)) as i8);
                data.push((0x80 | (value & 0x3f)) as i8);
            }
        }

        if data.len() > u16::MAX as usize {
            return Err(jvm.exception("java/io/UTFDataFormatException", "encoded string is too long").await);
        }

        let _: () = jvm.invoke_virtual(&this, "writeShort", "(I)V", (data.len() as i32,)).await?;

        let mut bytes = jvm.instantiate_array("B", data.len()).await?;
        jvm.store_array(&mut bytes, 0, data).await?;

        let out = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
        jvm.invoke_virtual(&out, "write", "([B)V", (bytes,)).await
    }

    async fn close(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.DataOutputStream::close({this:?})");

        let out = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
        let _: () = jvm.invoke_virtual(&out, "close", "()V", []).await?;

        Ok(())
    }

    async fn flush(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.DataOutputStream::flush({this:?})");

        let out = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
        let _: () = jvm.invoke_virtual(&out, "flush", "()V", []).await?;

        Ok(())
    }
}
