use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result};

use crate::{classes::java::io::InputStream, RuntimeClassProto, RuntimeContext};

// class java.io.DataInputStream
pub struct DataInputStream {}

impl DataInputStream {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/io/InputStream"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/InputStream;)V", Self::init, Default::default()),
                JavaMethodProto::new("available", "()I", Self::available, Default::default()),
                JavaMethodProto::new("read", "([BII)I", Self::read, Default::default()),
                JavaMethodProto::new("readBoolean", "()Z", Self::read_boolean, Default::default()),
                JavaMethodProto::new("readByte", "()B", Self::read_byte, Default::default()),
                JavaMethodProto::new("readChar", "()C", Self::read_char, Default::default()),
                JavaMethodProto::new("readDouble", "()D", Self::read_double, Default::default()),
                JavaMethodProto::new("readFloat", "()F", Self::read_float, Default::default()),
                JavaMethodProto::new("readInt", "()I", Self::read_int, Default::default()),
                JavaMethodProto::new("readLong", "()J", Self::read_long, Default::default()),
                JavaMethodProto::new("readShort", "()S", Self::read_short, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("in", "Ljava/io/InputStream;", Default::default())],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, r#in: ClassInstanceRef<InputStream>) -> Result<()> {
        tracing::debug!("java.lang.DataInputStream::<init>({:?}, {:?})", &this, &r#in);

        jvm.put_field(&mut this, "in", "Ljava/io/InputStream;", r#in)?;

        Ok(())
    }

    async fn available(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.lang.DataInputStream::available({:?})", &this);

        let r#in = jvm.get_field(&this, "in", "Ljava/io/InputStream;")?;
        let available: i32 = jvm.invoke_virtual(&r#in, "available", "()I", []).await?;

        Ok(available)
    }

    async fn read(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        b: ClassInstanceRef<Array<i8>>,
        off: i32,
        len: i32,
    ) -> Result<i32> {
        tracing::debug!("java.lang.DataInputStream::read({:?}, {:?}, {}, {})", &this, &b, off, len);

        let r#in = jvm.get_field(&this, "in", "Ljava/io/InputStream;")?;
        let result: i32 = jvm.invoke_virtual(&r#in, "read", "([BII)I", (b, off, len)).await?;

        Ok(result)
    }

    async fn read_byte(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i8> {
        tracing::debug!("java.lang.DataInputStream::readByte({:?})", &this);

        let r#in = jvm.get_field(&this, "in", "Ljava/io/InputStream;")?;
        let result: i32 = jvm.invoke_virtual(&r#in, "read", "()I", []).await?;

        Ok(result as _)
    }

    async fn read_boolean(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.lang.DataInputStream::readBoolean({:?})", &this);

        let byte: i8 = jvm.invoke_virtual(&this, "readByte", "()B", []).await?;

        Ok(byte != 0)
    }

    async fn read_char(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<JavaChar> {
        tracing::debug!("java.lang.DataInputStream::readChar({:?})", &this);

        let byte1: i8 = jvm.invoke_virtual(&this, "readByte", "()B", []).await?;
        let byte2: i8 = jvm.invoke_virtual(&this, "readByte", "()B", []).await?;

        Ok(((byte1 as u16) << 8 | (byte2 as u16)) as JavaChar)
    }

    async fn read_short(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i16> {
        tracing::debug!("java.lang.DataInputStream::readShort({:?})", &this);

        let byte1: i8 = jvm.invoke_virtual(&this, "readByte", "()B", []).await?;
        let byte2: i8 = jvm.invoke_virtual(&this, "readByte", "()B", []).await?;

        Ok(((byte1 as i16) << 8 | (byte2 as i16)) as i16)
    }

    async fn read_int(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.lang.DataInputStream::readInt({:?})", &this);

        let byte1: i8 = jvm.invoke_virtual(&this, "readByte", "()B", []).await?;
        let byte2: i8 = jvm.invoke_virtual(&this, "readByte", "()B", []).await?;
        let byte3: i8 = jvm.invoke_virtual(&this, "readByte", "()B", []).await?;
        let byte4: i8 = jvm.invoke_virtual(&this, "readByte", "()B", []).await?;

        Ok(((byte1 as i32) << 24 | (byte2 as i32) << 16 | (byte3 as i32) << 8 | (byte4 as i32)) as i32)
    }

    async fn read_long(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        tracing::debug!("java.lang.DataInputStream::readLong({:?})", &this);

        let byte1: i8 = jvm.invoke_virtual(&this, "readByte", "()B", []).await?;
        let byte2: i8 = jvm.invoke_virtual(&this, "readByte", "()B", []).await?;
        let byte3: i8 = jvm.invoke_virtual(&this, "readByte", "()B", []).await?;
        let byte4: i8 = jvm.invoke_virtual(&this, "readByte", "()B", []).await?;
        let byte5: i8 = jvm.invoke_virtual(&this, "readByte", "()B", []).await?;
        let byte6: i8 = jvm.invoke_virtual(&this, "readByte", "()B", []).await?;
        let byte7: i8 = jvm.invoke_virtual(&this, "readByte", "()B", []).await?;
        let byte8: i8 = jvm.invoke_virtual(&this, "readByte", "()B", []).await?;

        Ok(((byte1 as i64) << 56
            | (byte2 as i64) << 48
            | (byte3 as i64) << 40
            | (byte4 as i64) << 32
            | (byte5 as i64) << 24
            | (byte6 as i64) << 16
            | (byte7 as i64) << 8
            | (byte8 as i64)) as i64)
    }

    async fn read_float(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        tracing::debug!("java.lang.DataInputStream::readFloat({:?})", &this);

        let byte1: i8 = jvm.invoke_virtual(&this, "readByte", "()B", []).await?;
        let byte2: i8 = jvm.invoke_virtual(&this, "readByte", "()B", []).await?;
        let byte3: i8 = jvm.invoke_virtual(&this, "readByte", "()B", []).await?;
        let byte4: i8 = jvm.invoke_virtual(&this, "readByte", "()B", []).await?;

        Ok(f32::from_be_bytes([byte1 as u8, byte2 as u8, byte3 as u8, byte4 as u8]))
    }

    async fn read_double(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f64> {
        tracing::debug!("java.lang.DataInputStream::readDouble({:?})", &this);

        let byte1: i8 = jvm.invoke_virtual(&this, "readByte", "()B", []).await?;
        let byte2: i8 = jvm.invoke_virtual(&this, "readByte", "()B", []).await?;
        let byte3: i8 = jvm.invoke_virtual(&this, "readByte", "()B", []).await?;
        let byte4: i8 = jvm.invoke_virtual(&this, "readByte", "()B", []).await?;
        let byte5: i8 = jvm.invoke_virtual(&this, "readByte", "()B", []).await?;
        let byte6: i8 = jvm.invoke_virtual(&this, "readByte", "()B", []).await?;
        let byte7: i8 = jvm.invoke_virtual(&this, "readByte", "()B", []).await?;
        let byte8: i8 = jvm.invoke_virtual(&this, "readByte", "()B", []).await?;

        Ok(f64::from_be_bytes([
            byte1 as u8,
            byte2 as u8,
            byte3 as u8,
            byte4 as u8,
            byte5 as u8,
            byte6 as u8,
            byte7 as u8,
            byte8 as u8,
        ]))
    }

    async fn close(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.DataInputStream::close({:?})", &this);

        let r#in = jvm.get_field(&this, "in", "Ljava/io/InputStream;")?;
        jvm.invoke_virtual(&r#in, "close", "()V", []).await?;

        Ok(())
    }
}
