use alloc::vec::Vec;

use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};
use jvm_types::modified_utf8;

use crate::{
    RuntimeContext,
    classes::java::{
        io::{DataInput, InputStream},
        lang::String,
    },
};

// class java.io.DataInputStream
#[java_class(
    name = "java/io/DataInputStream",
    extends = "java/io/FilterInputStream",
    implements("java/io/DataInput"),
    public
)]
pub struct DataInputStream;

#[java_methods]
impl DataInputStream {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, r#in: ClassInstanceRef<InputStream>) -> Result<()> {
        tracing::debug!("java.io.DataInputStream::<init>({this:?}, {:?})", &r#in);

        let _: () = jvm
            .invoke_special(&this, "java/io/FilterInputStream", "<init>", "(Ljava/io/InputStream;)V", (r#in,))
            .await?;

        Ok(())
    }

    #[method(public, final)]
    async fn read_byte(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i8> {
        tracing::debug!("java.io.DataInputStream::readByte({this:?})");

        Ok(Self::read_required_byte(jvm, &this).await? as i8)
    }

    #[method(public, final)]
    async fn read_boolean(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.io.DataInputStream::readBoolean({this:?})");

        Ok(Self::read_required_byte(jvm, &this).await? != 0)
    }

    #[method(public, final)]
    async fn read_char(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<JavaChar> {
        tracing::debug!("java.io.DataInputStream::readChar({this:?})");

        let byte1 = Self::read_required_byte(jvm, &this).await?;
        let byte2 = Self::read_required_byte(jvm, &this).await?;
        Ok(JavaChar::from_be_bytes([byte1, byte2]))
    }

    #[method(public, final)]
    async fn read_short(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i16> {
        tracing::debug!("java.io.DataInputStream::readShort({this:?})");

        let byte1 = Self::read_required_byte(jvm, &this).await?;
        let byte2 = Self::read_required_byte(jvm, &this).await?;
        Ok(i16::from_be_bytes([byte1, byte2]))
    }

    #[method(public, final)]
    async fn read_unsigned_byte(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.io.DataInputStream::readUnsignedByte({this:?})");
        Ok(Self::read_required_byte(jvm, &this).await? as i32)
    }

    #[method(public, final)]
    async fn read_unsigned_short(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.io.DataInputStream::readUnsignedShort({this:?})");

        let byte1 = Self::read_required_byte(jvm, &this).await?;
        let byte2 = Self::read_required_byte(jvm, &this).await?;
        Ok(u16::from_be_bytes([byte1, byte2]) as i32)
    }

    #[method(public, final)]
    async fn read_int(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.io.DataInputStream::readInt({this:?})");

        Ok(i32::from_be_bytes([
            Self::read_required_byte(jvm, &this).await?,
            Self::read_required_byte(jvm, &this).await?,
            Self::read_required_byte(jvm, &this).await?,
            Self::read_required_byte(jvm, &this).await?,
        ]))
    }

    #[method(public, final)]
    async fn read_long(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        tracing::debug!("java.io.DataInputStream::readLong({this:?})");

        Ok(i64::from_be_bytes([
            Self::read_required_byte(jvm, &this).await?,
            Self::read_required_byte(jvm, &this).await?,
            Self::read_required_byte(jvm, &this).await?,
            Self::read_required_byte(jvm, &this).await?,
            Self::read_required_byte(jvm, &this).await?,
            Self::read_required_byte(jvm, &this).await?,
            Self::read_required_byte(jvm, &this).await?,
            Self::read_required_byte(jvm, &this).await?,
        ]))
    }

    #[method(public, final)]
    async fn read_float(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        tracing::debug!("java.io.DataInputStream::readFloat({this:?})");

        Ok(f32::from_be_bytes([
            Self::read_required_byte(jvm, &this).await?,
            Self::read_required_byte(jvm, &this).await?,
            Self::read_required_byte(jvm, &this).await?,
            Self::read_required_byte(jvm, &this).await?,
        ]))
    }

    #[method(public, final)]
    async fn read_double(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f64> {
        tracing::debug!("java.io.DataInputStream::readDouble({this:?})");

        Ok(f64::from_be_bytes([
            Self::read_required_byte(jvm, &this).await?,
            Self::read_required_byte(jvm, &this).await?,
            Self::read_required_byte(jvm, &this).await?,
            Self::read_required_byte(jvm, &this).await?,
            Self::read_required_byte(jvm, &this).await?,
            Self::read_required_byte(jvm, &this).await?,
            Self::read_required_byte(jvm, &this).await?,
            Self::read_required_byte(jvm, &this).await?,
        ]))
    }

    #[method(name = "readUTF", public, final)]
    async fn read_utf(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.io.DataInputStream::readUTF({this:?})");

        let length: i32 = jvm
            .invoke_virtual(&this, "java/io/DataInputStream", "readUnsignedShort", "()I", ())
            .await?;
        let java_array = jvm.instantiate_array("B", length as usize).await?;
        let _: () = jvm
            .invoke_virtual(&this, "java/io/DataInputStream", "readFully", "([BII)V", (java_array.clone(), 0, length))
            .await?;
        let bytes: Vec<i8> = jvm.load_array(&java_array, 0, length as usize).await?;
        let bytes: Vec<u8> = bytes.into_iter().map(|value| value as u8).collect();

        let Some(chars) = modified_utf8::decode(&bytes) else {
            return Err(jvm.exception("java/io/UTFDataFormatException", "malformed modified UTF-8").await);
        };

        Ok(JavaLangString::from_utf16(jvm, chars).await?.into())
    }

    #[method(name = "readUTF", static)]
    async fn read_utf_from_input(jvm: &Jvm, _: &mut RuntimeContext, input: ClassInstanceRef<DataInput>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.io.DataInputStream::readUTF({input:?})");
        if input.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "argument is null").await);
        }
        jvm.invoke_virtual(&input, &input.class_definition().name(), "readUTF", "()Ljava/lang/String;", ())
            .await
    }

    #[method(public, final)]
    async fn read_fully(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, b: ClassInstanceRef<Array<i8>>) -> Result<()> {
        tracing::debug!("java.io.DataInputStream::readFully({this:?}, {b:?})");
        if b.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "argument is null").await);
        }

        let length = jvm.array_length(&b).await?;

        let _: () = jvm
            .invoke_virtual(&this, "java/io/DataInputStream", "readFully", "([BII)V", (b.clone(), 0, length as i32))
            .await?;

        Ok(())
    }

    #[method(name = "readFully", public, final)]
    async fn read_fully_offset_length(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        b: ClassInstanceRef<Array<i8>>,
        off: i32,
        len: i32,
    ) -> Result<()> {
        tracing::debug!("java.io.DataInputStream::readFully({this:?}, {b:?}, {off}, {len})");
        if b.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "argument is null").await);
        }

        let mut read = 0;
        while read < len {
            let r: i32 = jvm
                .invoke_virtual(&this, "java/io/DataInputStream", "read", "([BII)I", (b.clone(), off + read, len - read))
                .await?;
            if r == -1 {
                return Err(jvm.exception("java/io/EOFException", "End of stream reached before reading fully").await);
            }
            read += r;
        }

        Ok(())
    }

    #[method(public, final)]
    async fn skip_bytes(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, n: i32) -> Result<i32> {
        tracing::debug!("java.io.DataInputStream::skipBytes({this:?}, {n:?})");

        if n <= 0 {
            return Ok(0);
        }
        let r#in: ClassInstanceRef<InputStream> = jvm.get_field(&this, "java/io/DataInputStream", "in", "Ljava/io/InputStream;").await?;
        if r#in.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "underlying stream is null").await);
        }
        let skipped: i64 = jvm.invoke_virtual(&r#in, "java/io/InputStream", "skip", "(J)J", (n as i64,)).await?;

        Ok(skipped as _)
    }

    async fn read_required_byte(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<u8> {
        let r#in: ClassInstanceRef<InputStream> = jvm.get_field(this, "java/io/DataInputStream", "in", "Ljava/io/InputStream;").await?;
        if r#in.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "underlying stream is null").await);
        }
        let value: i32 = jvm.invoke_virtual(&r#in, "java/io/InputStream", "read", "()I", ()).await?;
        if value == -1 {
            return Err(jvm.exception("java/io/EOFException", "End of stream").await);
        }

        Ok(value as u8)
    }
}
