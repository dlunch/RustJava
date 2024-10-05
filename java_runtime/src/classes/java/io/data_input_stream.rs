use alloc::{string::String as RustString, vec};

use bytemuck::cast_vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{runtime::JavaLangString, Array, ClassInstanceRef, JavaChar, Jvm, Result};

use crate::{
    classes::java::{io::InputStream, lang::String},
    RuntimeClassProto, RuntimeContext,
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

        Ok((byte1 as JavaChar) << 8 | (byte2 as JavaChar))
    }

    async fn read_short(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i16> {
        tracing::debug!("java.io.DataInputStream::readShort({:?})", &this);

        let r#in = jvm.get_field(&this, "in", "Ljava/io/InputStream;").await?;

        let byte1: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;
        let byte2: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;

        Ok((byte1 as i16) << 8 | (byte2 as i16))
    }

    async fn read_unsigned_short(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.io.DataInputStream::readUnsignedShort({:?})", &this);

        let r#in = jvm.get_field(&this, "in", "Ljava/io/InputStream;").await?;

        let byte1: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;
        let byte2: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;

        Ok((byte1 << 8 | byte2) & 0xffff)
    }

    async fn read_int(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.io.DataInputStream::readInt({:?})", &this);

        let r#in = jvm.get_field(&this, "in", "Ljava/io/InputStream;").await?;

        let byte1: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;
        let byte2: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;
        let byte3: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;
        let byte4: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;

        Ok(byte1 << 24 | byte2 << 16 | byte3 << 8 | byte4)
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

        Ok((byte1 as i64) << 56
            | (byte2 as i64) << 48
            | (byte3 as i64) << 40
            | (byte4 as i64) << 32
            | (byte5 as i64) << 24
            | (byte6 as i64) << 16
            | (byte7 as i64) << 8
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
        let _: i32 = jvm
            .invoke_virtual(&this, "read", "([BII)I", (java_array.clone(), 0, length as i32))
            .await?;

        let bytes = jvm.load_byte_array(&java_array, 0, length as _).await?;

        // TODO handle modified utf-8
        let string = RustString::from_utf8(cast_vec(bytes)).unwrap();

        Ok(JavaLangString::from_rust_string(jvm, &string).await?.into())
    }

    async fn close(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.DataInputStream::close({:?})", &this);

        let r#in = jvm.get_field(&this, "in", "Ljava/io/InputStream;").await?;
        let _: () = jvm.invoke_virtual(&r#in, "close", "()V", ()).await?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use alloc::vec;

    use bytemuck::cast_vec;

    use jvm::Result;

    use crate::test::test_jvm;

    #[tokio::test]
    async fn test_data_input_stream() -> Result<()> {
        let jvm = test_jvm().await?;

        let data = vec![
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
            0x18, 0x19, 0x1a, 0x1b,
        ];
        let data_len = data.len();

        let mut data_array = jvm.instantiate_array("B", data_len).await?;
        jvm.store_byte_array(&mut data_array, 0, data).await?;

        let input_stream = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (data_array,)).await?;
        let data_input_stream = jvm
            .new_class("java/io/DataInputStream", "(Ljava/io/InputStream;)V", (input_stream,))
            .await?;

        let available: i32 = jvm.invoke_virtual(&data_input_stream, "available", "()I", ()).await?;
        assert_eq!(available, data_len as i32);

        let byte: i8 = jvm.invoke_virtual(&data_input_stream, "readByte", "()B", ()).await?;
        assert_eq!(byte, 0x01);

        let short: i16 = jvm.invoke_virtual(&data_input_stream, "readShort", "()S", ()).await?;
        assert_eq!(short, 0x0203);

        let int: i32 = jvm.invoke_virtual(&data_input_stream, "readInt", "()I", ()).await?;
        assert_eq!(int, 0x04050607);

        let long: i64 = jvm.invoke_virtual(&data_input_stream, "readLong", "()J", ()).await?;
        assert_eq!(long, 0x08090a0b0c0d0e0f);

        let float: f32 = jvm.invoke_virtual(&data_input_stream, "readFloat", "()F", ()).await?;
        assert_eq!(float, f32::from_be_bytes([0x10, 0x11, 0x12, 0x13]));

        let double: f64 = jvm.invoke_virtual(&data_input_stream, "readDouble", "()D", ()).await?;
        assert_eq!(double, f64::from_be_bytes([0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b]));

        Ok(())
    }

    #[tokio::test]
    async fn test_data_input_stream_high_bit() -> Result<()> {
        let jvm = test_jvm().await?;

        let data = cast_vec(vec![
            0x81u8, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96,
            0x97, 0x98, 0x99, 0x9a, 0x9b,
        ]);
        let data_len = data.len();

        let mut data_array = jvm.instantiate_array("B", data_len).await?;
        jvm.store_byte_array(&mut data_array, 0, data).await?;

        let input_stream = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (data_array,)).await?;
        let data_input_stream = jvm
            .new_class("java/io/DataInputStream", "(Ljava/io/InputStream;)V", (input_stream,))
            .await?;

        let available: i32 = jvm.invoke_virtual(&data_input_stream, "available", "()I", ()).await?;
        assert_eq!(available, data_len as i32);

        let byte: i8 = jvm.invoke_virtual(&data_input_stream, "readByte", "()B", ()).await?;
        assert_eq!(byte, i8::from_be_bytes([0x81]));

        let short: i16 = jvm.invoke_virtual(&data_input_stream, "readShort", "()S", ()).await?;
        assert_eq!(short, i16::from_be_bytes([0x82, 0x83]));

        let int: i32 = jvm.invoke_virtual(&data_input_stream, "readInt", "()I", ()).await?;
        assert_eq!(int, i32::from_be_bytes([0x84, 0x85, 0x86, 0x87]));

        let long: i64 = jvm.invoke_virtual(&data_input_stream, "readLong", "()J", ()).await?;
        assert_eq!(long, i64::from_be_bytes([0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f]));

        let float: f32 = jvm.invoke_virtual(&data_input_stream, "readFloat", "()F", ()).await?;
        assert_eq!(float, f32::from_be_bytes([0x90, 0x91, 0x92, 0x93]));

        let double: f64 = jvm.invoke_virtual(&data_input_stream, "readDouble", "()D", ()).await?;
        assert_eq!(double, f64::from_be_bytes([0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b]));

        Ok(())
    }
}
