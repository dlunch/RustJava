use alloc::{string::String as RustString, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{io::InputStream, lang::String},
};

// class java.io.DataInputStream
pub struct DataInputStream;

impl DataInputStream {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/DataInputStream",
            parent_class: Some("java/io/InputStream"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/InputStream;)V", Self::init, Default::default()),
                JavaMethodProto::new("available", "()I", Self::available, Default::default()),
                JavaMethodProto::new("read", "()I", Self::read_byte_int, Default::default()),
                JavaMethodProto::new("read", "([BII)I", Self::read, Default::default()),
                JavaMethodProto::new("readBoolean", "()Z", Self::read_boolean, Default::default()),
                JavaMethodProto::new("readByte", "()B", Self::read_byte, Default::default()),
                JavaMethodProto::new("readChar", "()C", Self::read_char, Default::default()),
                JavaMethodProto::new("readDouble", "()D", Self::read_double, Default::default()),
                JavaMethodProto::new("readFloat", "()F", Self::read_float, Default::default()),
                JavaMethodProto::new("readInt", "()I", Self::read_int, Default::default()),
                JavaMethodProto::new("readLong", "()J", Self::read_long, Default::default()),
                JavaMethodProto::new("readShort", "()S", Self::read_short, Default::default()),
                JavaMethodProto::new("readUnsignedShort", "()I", Self::read_unsigned_short, Default::default()),
                JavaMethodProto::new("readUTF", "()Ljava/lang/String;", Self::read_utf, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("in", "Ljava/io/InputStream;", Default::default())],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, r#in: ClassInstanceRef<InputStream>) -> Result<()> {
        tracing::debug!("java.io.DataInputStream::<init>({:?}, {:?})", &this, &r#in);

        let _: () = jvm.invoke_special(&this, "java/io/InputStream", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "in", "Ljava/io/InputStream;", r#in).await?;

        Ok(())
    }

    async fn available(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.io.DataInputStream::available({:?})", &this);

        let r#in = jvm.get_field(&this, "in", "Ljava/io/InputStream;").await?;
        let available: i32 = jvm.invoke_virtual(&r#in, "available", "()I", ()).await?;

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
        tracing::debug!("java.io.DataInputStream::read({:?}, {:?}, {}, {})", &this, &b, off, len);

        let r#in = jvm.get_field(&this, "in", "Ljava/io/InputStream;").await?;
        let result: i32 = jvm.invoke_virtual(&r#in, "read", "([BII)I", (b, off, len)).await?;

        Ok(result)
    }

    async fn read_byte_int(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.io.DataInputStream::read({:?})", &this);

        let r#in = jvm.get_field(&this, "in", "Ljava/io/InputStream;").await?;
        let result: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;

        Ok(result)
    }

    async fn read_byte(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i8> {
        tracing::debug!("java.io.DataInputStream::readByte({:?})", &this);

        let r#in = jvm.get_field(&this, "in", "Ljava/io/InputStream;").await?;
        let result: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;

        Ok(result as _)
    }

    async fn read_boolean(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.io.DataInputStream::readBoolean({:?})", &this);

        let r#in = jvm.get_field(&this, "in", "Ljava/io/InputStream;").await?;
        let byte: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;

        Ok(byte != 0)
    }

    async fn read_char(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<JavaChar> {
        tracing::debug!("java.io.DataInputStream::readChar({:?})", &this);

        let r#in = jvm.get_field(&this, "in", "Ljava/io/InputStream;").await?;

        let byte1: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;
        let byte2: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;

        Ok(((byte1 as JavaChar) << 8) | (byte2 as JavaChar))
    }

    async fn read_short(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i16> {
        tracing::debug!("java.io.DataInputStream::readShort({:?})", &this);

        let r#in = jvm.get_field(&this, "in", "Ljava/io/InputStream;").await?;

        let byte1: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;
        let byte2: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;

        Ok(((byte1 as i16) << 8) | (byte2 as i16))
    }

    async fn read_unsigned_short(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.io.DataInputStream::readUnsignedShort({:?})", &this);

        let r#in = jvm.get_field(&this, "in", "Ljava/io/InputStream;").await?;

        let byte1: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;
        let byte2: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;

        Ok(((byte1 << 8) | byte2) & 0xffff)
    }

    async fn read_int(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.io.DataInputStream::readInt({:?})", &this);

        let r#in = jvm.get_field(&this, "in", "Ljava/io/InputStream;").await?;

        let byte1: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;
        let byte2: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;
        let byte3: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;
        let byte4: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;

        Ok((byte1 << 24) | (byte2 << 16) | (byte3 << 8) | byte4)
    }

    async fn read_long(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        tracing::debug!("java.io.DataInputStream::readLong({:?})", &this);

        let r#in = jvm.get_field(&this, "in", "Ljava/io/InputStream;").await?;

        let byte1: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;
        let byte2: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;
        let byte3: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;
        let byte4: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;
        let byte5: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;
        let byte6: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;
        let byte7: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;
        let byte8: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;

        Ok(((byte1 as i64) << 56)
            | ((byte2 as i64) << 48)
            | ((byte3 as i64) << 40)
            | ((byte4 as i64) << 32)
            | ((byte5 as i64) << 24)
            | ((byte6 as i64) << 16)
            | ((byte7 as i64) << 8)
            | (byte8 as i64))
    }

    async fn read_float(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        tracing::debug!("java.io.DataInputStream::readFloat({:?})", &this);

        let r#in = jvm.get_field(&this, "in", "Ljava/io/InputStream;").await?;

        let byte1: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;
        let byte2: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;
        let byte3: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;
        let byte4: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;

        Ok(f32::from_be_bytes([byte1 as u8, byte2 as u8, byte3 as u8, byte4 as u8]))
    }

    async fn read_double(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f64> {
        tracing::debug!("java.io.DataInputStream::readDouble({:?})", &this);

        let r#in = jvm.get_field(&this, "in", "Ljava/io/InputStream;").await?;

        let byte1: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;
        let byte2: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;
        let byte3: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;
        let byte4: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;
        let byte5: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;
        let byte6: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;
        let byte7: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;
        let byte8: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;

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

    async fn read_utf(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.io.DataInputStream::readUTF({:?})", &this);

        let length: i32 = jvm.invoke_virtual(&this, "readUnsignedShort", "()I", ()).await?;
        let java_array = jvm.instantiate_array("B", length as _).await?;
        let _: i32 = jvm.invoke_virtual(&this, "read", "([BII)I", (java_array.clone(), 0, length)).await?;

        let mut buf = vec![0; length as _];
        jvm.array_raw_buffer(&java_array).await?.read(0, &mut buf)?;

        // TODO handle modified utf-8
        let string = RustString::from_utf8(buf).unwrap();

        Ok(JavaLangString::from_rust_string(jvm, &string).await?.into())
    }

    async fn close(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.DataInputStream::close({:?})", &this);

        let r#in = jvm.get_field(&this, "in", "Ljava/io/InputStream;").await?;
        let _: () = jvm.invoke_virtual(&r#in, "close", "()V", ()).await?;

        Ok(())
    }
}
