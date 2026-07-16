use alloc::{vec, vec::Vec};

use java_class_proto::JavaMethodProto;
use java_constants::MethodAccessFlags;
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::{DataInput, InputStream},
        lang::String,
    },
};

// class java.io.DataInputStream
pub struct DataInputStream;

impl DataInputStream {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/DataInputStream",
            parent_class: Some("java/io/FilterInputStream"),
            interfaces: vec!["java/io/DataInput"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/InputStream;)V", Self::init, Default::default()),
                JavaMethodProto::new("readBoolean", "()Z", Self::read_boolean, Default::default()),
                JavaMethodProto::new("readByte", "()B", Self::read_byte, Default::default()),
                JavaMethodProto::new("readChar", "()C", Self::read_char, Default::default()),
                JavaMethodProto::new("readDouble", "()D", Self::read_double, Default::default()),
                JavaMethodProto::new("readFloat", "()F", Self::read_float, Default::default()),
                JavaMethodProto::new("readFully", "([B)V", Self::read_fully, Default::default()),
                JavaMethodProto::new("readFully", "([BII)V", Self::read_fully_offset_length, Default::default()),
                JavaMethodProto::new("readInt", "()I", Self::read_int, Default::default()),
                JavaMethodProto::new("readLong", "()J", Self::read_long, Default::default()),
                JavaMethodProto::new("readShort", "()S", Self::read_short, Default::default()),
                JavaMethodProto::new("readUnsignedByte", "()I", Self::read_unsigned_byte, Default::default()),
                JavaMethodProto::new("readUnsignedShort", "()I", Self::read_unsigned_short, Default::default()),
                JavaMethodProto::new("readUTF", "()Ljava/lang/String;", Self::read_utf, Default::default()),
                JavaMethodProto::new(
                    "readUTF",
                    "(Ljava/io/DataInput;)Ljava/lang/String;",
                    Self::read_utf_from_input,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("skipBytes", "(I)I", Self::skip_bytes, Default::default()),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, r#in: ClassInstanceRef<InputStream>) -> Result<()> {
        tracing::debug!("java.io.DataInputStream::<init>({this:?}, {:?})", &r#in);

        let _: () = jvm
            .invoke_special(&this, "java/io/FilterInputStream", "<init>", "(Ljava/io/InputStream;)V", (r#in,))
            .await?;

        Ok(())
    }

    async fn read_byte(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i8> {
        tracing::debug!("java.io.DataInputStream::readByte({this:?})");

        Ok(Self::read_required_byte(jvm, &this).await? as i8)
    }

    async fn read_boolean(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.io.DataInputStream::readBoolean({this:?})");

        Ok(Self::read_required_byte(jvm, &this).await? != 0)
    }

    async fn read_char(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<JavaChar> {
        tracing::debug!("java.io.DataInputStream::readChar({this:?})");

        let byte1 = Self::read_required_byte(jvm, &this).await?;
        let byte2 = Self::read_required_byte(jvm, &this).await?;
        Ok(JavaChar::from_be_bytes([byte1, byte2]))
    }

    async fn read_short(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i16> {
        tracing::debug!("java.io.DataInputStream::readShort({this:?})");

        let byte1 = Self::read_required_byte(jvm, &this).await?;
        let byte2 = Self::read_required_byte(jvm, &this).await?;
        Ok(i16::from_be_bytes([byte1, byte2]))
    }

    async fn read_unsigned_byte(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.io.DataInputStream::readUnsignedByte({this:?})");
        Ok(Self::read_required_byte(jvm, &this).await? as i32)
    }

    async fn read_unsigned_short(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.io.DataInputStream::readUnsignedShort({this:?})");

        let byte1 = Self::read_required_byte(jvm, &this).await?;
        let byte2 = Self::read_required_byte(jvm, &this).await?;
        Ok(u16::from_be_bytes([byte1, byte2]) as i32)
    }

    async fn read_int(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.io.DataInputStream::readInt({this:?})");

        Ok(i32::from_be_bytes([
            Self::read_required_byte(jvm, &this).await?,
            Self::read_required_byte(jvm, &this).await?,
            Self::read_required_byte(jvm, &this).await?,
            Self::read_required_byte(jvm, &this).await?,
        ]))
    }

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

    async fn read_float(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        tracing::debug!("java.io.DataInputStream::readFloat({this:?})");

        Ok(f32::from_be_bytes([
            Self::read_required_byte(jvm, &this).await?,
            Self::read_required_byte(jvm, &this).await?,
            Self::read_required_byte(jvm, &this).await?,
            Self::read_required_byte(jvm, &this).await?,
        ]))
    }

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

    async fn read_utf(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.io.DataInputStream::readUTF({this:?})");

        let length: i32 = jvm.invoke_virtual(&this, "readUnsignedShort", "()I", ()).await?;
        let mut java_array = jvm.instantiate_array("B", length as usize).await?;
        let _: () = jvm.invoke_virtual(&this, "readFully", "([BII)V", (java_array.clone(), 0, length)).await?;
        let bytes: Vec<i8> = jvm.load_array(&java_array, 0, length as usize).await?;
        let bytes: Vec<u8> = bytes.into_iter().map(|value| value as u8).collect();

        let mut chars = Vec::with_capacity(length as usize);
        let mut index = 0;
        while index < bytes.len() {
            let first = bytes[index];
            match first >> 4 {
                0..=7 => {
                    chars.push(first as JavaChar);
                    index += 1;
                }
                12 | 13 => {
                    if index + 1 >= bytes.len() || bytes[index + 1] & 0xc0 != 0x80 {
                        return Err(jvm.exception("java/io/UTFDataFormatException", "malformed modified UTF-8").await);
                    }
                    chars.push((((first & 0x1f) as JavaChar) << 6) | ((bytes[index + 1] & 0x3f) as JavaChar));
                    index += 2;
                }
                14 => {
                    if index + 2 >= bytes.len() || bytes[index + 1] & 0xc0 != 0x80 || bytes[index + 2] & 0xc0 != 0x80 {
                        return Err(jvm.exception("java/io/UTFDataFormatException", "malformed modified UTF-8").await);
                    }
                    chars.push(
                        (((first & 0x0f) as JavaChar) << 12)
                            | (((bytes[index + 1] & 0x3f) as JavaChar) << 6)
                            | ((bytes[index + 2] & 0x3f) as JavaChar),
                    );
                    index += 3;
                }
                _ => return Err(jvm.exception("java/io/UTFDataFormatException", "malformed modified UTF-8").await),
            }
        }

        java_array = jvm.instantiate_array("C", chars.len()).await?;
        jvm.store_array(&mut java_array, 0, chars).await?;
        Ok(jvm.new_class("java/lang/String", "([C)V", (java_array,)).await?.into())
    }

    async fn read_utf_from_input(jvm: &Jvm, _: &mut RuntimeContext, input: ClassInstanceRef<DataInput>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.io.DataInputStream::readUTF({input:?})");
        jvm.invoke_virtual(&input, "readUTF", "()Ljava/lang/String;", ()).await
    }

    async fn read_fully(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, b: ClassInstanceRef<Array<i8>>) -> Result<()> {
        tracing::debug!("java.io.DataInputStream::readFully({this:?}, {b:?})");

        let length = jvm.array_length(&b).await?;

        let _: () = jvm.invoke_virtual(&this, "readFully", "([BII)V", (b.clone(), 0, length as i32)).await?;

        Ok(())
    }

    async fn read_fully_offset_length(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        b: ClassInstanceRef<Array<i8>>,
        off: i32,
        len: i32,
    ) -> Result<()> {
        tracing::debug!("java.io.DataInputStream::readFully({this:?}, {b:?}, {off}, {len})");

        let mut read = 0;
        while read < len {
            let r: i32 = jvm.invoke_virtual(&this, "read", "([BII)I", (b.clone(), off + read, len - read)).await?;
            if r == -1 {
                return Err(jvm.exception("java/io/EOFException", "End of stream reached before reading fully").await);
            }
            read += r;
        }

        Ok(())
    }

    async fn skip_bytes(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, n: i32) -> Result<i32> {
        tracing::debug!("java.io.DataInputStream::skipBytes({this:?}, {n:?})");

        let r#in = jvm.get_field(&this, "in", "Ljava/io/InputStream;").await?;
        let skipped: i64 = jvm.invoke_virtual(&r#in, "skip", "(J)J", (n as i64,)).await?;

        Ok(skipped as _)
    }

    async fn read_required_byte(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<u8> {
        let r#in = jvm.get_field(this, "in", "Ljava/io/InputStream;").await?;
        let value: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;
        if value == -1 {
            return Err(jvm.exception("java/io/EOFException", "End of stream").await);
        }

        Ok(value as u8)
    }
}
