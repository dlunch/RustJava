use alloc::vec::Vec;

use bytemuck::cast_vec;

use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};
use jvm_types::modified_utf8;

use crate::{
    RuntimeContext,
    classes::java::{io::OutputStream, lang::String},
};

// class java.io.DataOutputStream
#[java_class(
    name = "java/io/DataOutputStream",
    extends = "java/io/FilterOutputStream",
    implements("java/io/DataOutput"),
    public
)]
pub struct DataOutputStream;

#[java_methods]
impl DataOutputStream {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, out: ClassInstanceRef<OutputStream>) -> Result<()> {
        tracing::debug!("java.io.DataOutputStream::<init>({this:?}, {out:?})");

        let _: () = jvm
            .invoke_special(&this, "java/io/FilterOutputStream", "<init>", "(Ljava/io/OutputStream;)V", (out,))
            .await?;

        Ok(())
    }

    #[method(public, synchronized)]
    async fn write(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, b: i32) -> Result<()> {
        tracing::debug!("java.io.DataOutputStream::write({this:?}, {b:?})");

        let out: ClassInstanceRef<OutputStream> = jvm.get_field(&this, "java/io/DataOutputStream", "out", "Ljava/io/OutputStream;").await?;
        if out.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "underlying stream is null").await);
        }
        let _: () = jvm.invoke_virtual(&out, "java/io/OutputStream", "write", "(I)V", (b,)).await?;

        Ok(())
    }

    #[method(public, final)]
    async fn write_byte(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, v: i32) -> Result<()> {
        tracing::debug!("java.io.DataOutputStream::writeByte({this:?}, {v:?})");

        let out: ClassInstanceRef<OutputStream> = jvm.get_field(&this, "java/io/DataOutputStream", "out", "Ljava/io/OutputStream;").await?;
        if out.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "underlying stream is null").await);
        }
        let _: () = jvm.invoke_virtual(&out, "java/io/OutputStream", "write", "(I)V", (v,)).await?;

        Ok(())
    }

    #[method(public, final)]
    async fn write_boolean(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, v: bool) -> Result<()> {
        tracing::debug!("java.io.DataOutputStream::writeBoolean({this:?}, {v:?})");

        let _: () = jvm
            .invoke_virtual(&this, "java/io/DataOutputStream", "writeByte", "(I)V", (v as i32,))
            .await?;

        Ok(())
    }

    #[method(public, final)]
    async fn write_short(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, s: i32) -> Result<()> {
        tracing::debug!("java.io.DataOutputStream::writeShort({this:?}, {s:?})");

        let bytes = (s as i16).to_be_bytes();
        let mut byte_array = jvm.instantiate_array("B", bytes.len() as _).await?;
        jvm.store_array(&mut byte_array, 0, cast_vec::<u8, i8>(bytes.to_vec())).await?;

        let out: ClassInstanceRef<OutputStream> = jvm.get_field(&this, "java/io/DataOutputStream", "out", "Ljava/io/OutputStream;").await?;
        if out.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "underlying stream is null").await);
        }
        let _: () = jvm.invoke_virtual(&out, "java/io/OutputStream", "write", "([B)V", (byte_array,)).await?;

        Ok(())
    }

    #[method(public, final)]
    async fn write_char(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        tracing::debug!("java.io.DataOutputStream::writeChar({this:?}, {value:?})");
        jvm.invoke_virtual(&this, "java/io/DataOutputStream", "writeShort", "(I)V", (value & 0xffff,))
            .await
    }

    #[method(public, final)]
    async fn write_int(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, i: i32) -> Result<()> {
        tracing::debug!("java.io.DataOutputStream::writeInt({this:?}, {i:?})");

        let bytes = i.to_be_bytes();
        let mut byte_array = jvm.instantiate_array("B", bytes.len() as _).await?;
        jvm.store_array(&mut byte_array, 0, cast_vec::<u8, i8>(bytes.to_vec())).await?;

        let out: ClassInstanceRef<OutputStream> = jvm.get_field(&this, "java/io/DataOutputStream", "out", "Ljava/io/OutputStream;").await?;
        if out.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "underlying stream is null").await);
        }
        let _: () = jvm.invoke_virtual(&out, "java/io/OutputStream", "write", "([B)V", (byte_array,)).await?;

        Ok(())
    }

    #[method(public, final)]
    async fn write_long(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, l: i64) -> Result<()> {
        tracing::debug!("java.io.DataOutputStream::writeLong({this:?}, {l:?})");

        let bytes = l.to_be_bytes();
        let mut byte_array = jvm.instantiate_array("B", bytes.len() as _).await?;
        jvm.store_array(&mut byte_array, 0, cast_vec::<u8, i8>(bytes.to_vec())).await?;

        let out: ClassInstanceRef<OutputStream> = jvm.get_field(&this, "java/io/DataOutputStream", "out", "Ljava/io/OutputStream;").await?;
        if out.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "underlying stream is null").await);
        }
        let _: () = jvm.invoke_virtual(&out, "java/io/OutputStream", "write", "([B)V", (byte_array,)).await?;

        Ok(())
    }

    #[method(public, final)]
    async fn write_float(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: f32) -> Result<()> {
        tracing::debug!("java.io.DataOutputStream::writeFloat({this:?}, {value:?})");
        jvm.invoke_virtual(&this, "java/io/DataOutputStream", "writeInt", "(I)V", (value.to_bits() as i32,))
            .await
    }

    #[method(public, final)]
    async fn write_double(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: f64) -> Result<()> {
        tracing::debug!("java.io.DataOutputStream::writeDouble({this:?}, {value:?})");
        jvm.invoke_virtual(&this, "java/io/DataOutputStream", "writeLong", "(J)V", (value.to_bits() as i64,))
            .await
    }

    #[method(public, final)]
    async fn write_bytes(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, s: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.io.DataOutputStream::writeBytes({this:?}, {s:?})");
        if s.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "argument is null").await);
        }

        let chars: ClassInstanceRef<Array<JavaChar>> = jvm.invoke_virtual(&s, "java/lang/String", "toCharArray", "()[C", ()).await?;
        let length = jvm.array_length(&chars).await?;
        let chars: Vec<JavaChar> = jvm.load_array(&chars, 0, length).await?;
        if chars.is_empty() {
            return Ok(());
        }
        let mut bytes = jvm.instantiate_array("B", chars.len()).await?;
        jvm.store_array(&mut bytes, 0, chars.into_iter().map(|value| value as i8)).await?;

        let out: ClassInstanceRef<OutputStream> = jvm.get_field(&this, "java/io/DataOutputStream", "out", "Ljava/io/OutputStream;").await?;
        if out.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "underlying stream is null").await);
        }
        jvm.invoke_virtual(&out, "java/io/OutputStream", "write", "([B)V", (bytes,)).await
    }

    #[method(public, final)]
    async fn write_chars(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, s: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.io.DataOutputStream::writeChars({this:?}, {s:?})");
        if s.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "argument is null").await);
        }

        let chars: ClassInstanceRef<Array<JavaChar>> = jvm.invoke_virtual(&s, "java/lang/String", "toCharArray", "()[C", ()).await?;
        let length = jvm.array_length(&chars).await?;
        let chars: Vec<JavaChar> = jvm.load_array(&chars, 0, length).await?;
        if chars.is_empty() {
            return Ok(());
        }
        let mut data = Vec::with_capacity(chars.len() * 2);
        for value in chars {
            data.push((value >> 8) as i8);
            data.push(value as i8);
        }

        let mut bytes = jvm.instantiate_array("B", data.len()).await?;
        jvm.store_array(&mut bytes, 0, data).await?;

        let out: ClassInstanceRef<OutputStream> = jvm.get_field(&this, "java/io/DataOutputStream", "out", "Ljava/io/OutputStream;").await?;
        if out.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "underlying stream is null").await);
        }
        jvm.invoke_virtual(&out, "java/io/OutputStream", "write", "([B)V", (bytes,)).await
    }

    #[method(name = "writeUTF", public, final)]
    async fn write_utf(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, s: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.io.DataOutputStream::writeUTF({this:?}, {s:?})");
        if s.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "argument is null").await);
        }

        let chars: ClassInstanceRef<Array<JavaChar>> = jvm.invoke_virtual(&s, "java/lang/String", "toCharArray", "()[C", ()).await?;
        let length = jvm.array_length(&chars).await?;
        let chars: Vec<JavaChar> = jvm.load_array(&chars, 0, length).await?;
        let data: Vec<i8> = cast_vec(modified_utf8::encode(&chars));

        if data.len() > u16::MAX as usize {
            return Err(jvm.exception("java/io/UTFDataFormatException", "encoded string is too long").await);
        }

        let _: () = jvm
            .invoke_virtual(&this, "java/io/DataOutputStream", "writeShort", "(I)V", (data.len() as i32,))
            .await?;

        let mut bytes = jvm.instantiate_array("B", data.len()).await?;
        jvm.store_array(&mut bytes, 0, data).await?;

        let out: ClassInstanceRef<OutputStream> = jvm.get_field(&this, "java/io/DataOutputStream", "out", "Ljava/io/OutputStream;").await?;
        if out.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "underlying stream is null").await);
        }
        jvm.invoke_virtual(&out, "java/io/OutputStream", "write", "([B)V", (bytes,)).await
    }

    #[method(public)]
    async fn close(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.DataOutputStream::close({this:?})");

        let out: ClassInstanceRef<OutputStream> = jvm.get_field(&this, "java/io/DataOutputStream", "out", "Ljava/io/OutputStream;").await?;
        if out.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "underlying stream is null").await);
        }
        let _: () = jvm.invoke_virtual(&out, "java/io/OutputStream", "close", "()V", []).await?;

        Ok(())
    }

    #[method(public)]
    async fn flush(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.DataOutputStream::flush({this:?})");

        let out: ClassInstanceRef<OutputStream> = jvm.get_field(&this, "java/io/DataOutputStream", "out", "Ljava/io/OutputStream;").await?;
        if out.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "underlying stream is null").await);
        }
        let _: () = jvm.invoke_virtual(&out, "java/io/OutputStream", "flush", "()V", []).await?;

        Ok(())
    }
}
