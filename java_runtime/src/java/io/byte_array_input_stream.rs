use alloc::vec;

use java_runtime_base::{Array, JavaFieldAccessFlag, JavaFieldProto, JavaMethodFlag, JavaMethodProto, JavaResult, JvmClassInstanceHandle};
use jvm::Jvm;

use crate::{RuntimeClassProto, RuntimeContext};

// class java.io.ByteArrayInputStream
pub struct ByteArrayInputStream {}

impl ByteArrayInputStream {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/io/InputStream"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "([B)V", Self::init, JavaMethodFlag::NONE),
                JavaMethodProto::new("available", "()I", Self::available, JavaMethodFlag::NONE),
                JavaMethodProto::new("read", "([BII)I", Self::read, JavaMethodFlag::NONE),
                JavaMethodProto::new("read", "()I", Self::read_byte, JavaMethodFlag::NONE),
                JavaMethodProto::new("close", "()V", Self::close, JavaMethodFlag::NONE),
            ],
            fields: vec![
                JavaFieldProto::new("buf", "[B", JavaFieldAccessFlag::NONE),
                JavaFieldProto::new("pos", "I", JavaFieldAccessFlag::NONE),
            ],
        }
    }

    async fn init(
        jvm: &mut Jvm,
        _: &mut RuntimeContext,
        mut this: JvmClassInstanceHandle<Self>,
        data: JvmClassInstanceHandle<Array<i8>>,
    ) -> JavaResult<()> {
        tracing::debug!("java.lang.ByteArrayInputStream::<init>({:?}, {:?})", &this, &data);

        jvm.put_field(&mut this, "buf", "[B", data)?;
        jvm.put_field(&mut this, "pos", "I", 0)?;

        Ok(())
    }

    async fn available(jvm: &mut Jvm, _: &mut RuntimeContext, this: JvmClassInstanceHandle<Self>) -> JavaResult<i32> {
        tracing::debug!("java.lang.ByteArrayInputStream::available({:?})", &this);

        let buf = jvm.get_field(&this, "buf", "[B")?;
        let pos: i32 = jvm.get_field(&this, "pos", "I")?;
        let buf_length = jvm.array_length(&buf)? as i32;

        Ok((buf_length - pos) as _)
    }

    async fn read(
        jvm: &mut Jvm,
        _: &mut RuntimeContext,
        mut this: JvmClassInstanceHandle<Self>,
        b: JvmClassInstanceHandle<Array<i8>>,
        off: i32,
        len: i32,
    ) -> JavaResult<i32> {
        tracing::debug!("java.lang.ByteArrayInputStream::read({:?}, {:?}, {}, {})", &this, &b, off, len);

        let buf = jvm.get_field(&this, "buf", "[B")?;
        let buf_length = jvm.array_length(&buf)?;
        let pos: i32 = jvm.get_field(&this, "pos", "I")?;

        let available = (buf_length as i32 - pos) as _;
        let len_to_read = if len > available { available } else { len };
        if len_to_read == 0 {
            return Ok(0);
        }

        jvm.invoke_static(
            "java/lang/System",
            "arraycopy",
            "(Ljava/lang/Object;ILjava/lang/Object;II)V",
            (buf, pos, b, off, len_to_read),
        )
        .await?;

        jvm.put_field(&mut this, "pos", "I", pos + len)?;

        Ok(len)
    }

    async fn read_byte(jvm: &mut Jvm, _: &mut RuntimeContext, mut this: JvmClassInstanceHandle<Self>) -> JavaResult<i8> {
        tracing::debug!("java.lang.ByteArrayInputStream::readByte({:?})", &this);

        let buf = jvm.get_field(&this, "buf", "[B")?;
        let buf_length = jvm.array_length(&buf)?;
        let pos: i32 = jvm.get_field(&this, "pos", "I")?;

        if pos as usize >= buf_length {
            return Ok(-1);
        }

        let result = jvm.load_byte_array(&buf, pos as _, 1)?[0];

        jvm.put_field(&mut this, "pos", "I", pos + 1)?;

        Ok(result)
    }

    async fn close(_: &mut Jvm, _: &mut RuntimeContext, this: JvmClassInstanceHandle<ByteArrayInputStream>) -> JavaResult<()> {
        tracing::debug!("java.lang.ByteArrayInputStream::close({:?})", &this);

        Ok(())
    }
}
