use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

// class java.io.ByteArrayInputStream
pub struct ByteArrayInputStream;

impl ByteArrayInputStream {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/ByteArrayInputStream",
            parent_class: Some("java/io/InputStream"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "([B)V", Self::init, Default::default()),
                JavaMethodProto::new("available", "()I", Self::available, Default::default()),
                JavaMethodProto::new("read", "([BII)I", Self::read, Default::default()),
                JavaMethodProto::new("read", "()I", Self::read_byte, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
                JavaMethodProto::new("skip", "(J)J", Self::skip, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("buf", "[B", Default::default()),
                JavaFieldProto::new("pos", "I", Default::default()),
            ],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, data: ClassInstanceRef<Array<i8>>) -> Result<()> {
        tracing::debug!("java.lang.ByteArrayInputStream::<init>({:?}, {:?})", &this, &data);

        jvm.put_field(&mut this, "buf", "[B", data).await?;
        jvm.put_field(&mut this, "pos", "I", 0).await?;

        Ok(())
    }

    async fn available(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.lang.ByteArrayInputStream::available({:?})", &this);

        let buf = jvm.get_field(&this, "buf", "[B").await?;
        let pos: i32 = jvm.get_field(&this, "pos", "I").await?;
        let buf_length = jvm.array_length(&buf).await? as i32;

        Ok((buf_length - pos) as _)
    }

    async fn read(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        b: ClassInstanceRef<Array<i8>>,
        off: i32,
        len: i32,
    ) -> Result<i32> {
        tracing::debug!("java.lang.ByteArrayInputStream::read({:?}, {:?}, {}, {})", &this, &b, off, len);

        let buf = jvm.get_field(&this, "buf", "[B").await?;
        let buf_length = jvm.array_length(&buf).await?;
        let pos: i32 = jvm.get_field(&this, "pos", "I").await?;

        let available = (buf_length as i32 - pos) as _;
        let len_to_read = if len > available { available } else { len };
        if len_to_read == 0 {
            return Ok(-1);
        }

        let _: () = jvm
            .invoke_static(
                "java/lang/System",
                "arraycopy",
                "(Ljava/lang/Object;ILjava/lang/Object;II)V",
                (buf, pos, b, off, len_to_read),
            )
            .await?;

        jvm.put_field(&mut this, "pos", "I", pos + len_to_read).await?;

        Ok(len_to_read)
    }

    async fn read_byte(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.io.ByteArrayInputStream::readByte({:?})", &this);

        let buf = jvm.get_field(&this, "buf", "[B").await?;
        let buf_length = jvm.array_length(&buf).await?;
        let pos: i32 = jvm.get_field(&this, "pos", "I").await?;

        if pos as usize >= buf_length {
            return Ok(-1);
        }

        let result: i8 = jvm.load_array(&buf, pos as _, 1).await?[0];

        jvm.put_field(&mut this, "pos", "I", pos + 1).await?;

        Ok(result as u8 as _)
    }

    async fn close(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<ByteArrayInputStream>) -> Result<()> {
        tracing::debug!("java.io.ByteArrayInputStream::close({:?})", &this);

        Ok(())
    }

    async fn skip(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, n: i64) -> Result<i64> {
        tracing::debug!("java.io.ByteArrayInputStream::skip({:?}, {:?})", &this, n);

        let buf = jvm.get_field(&this, "buf", "[B").await?;
        let buf_length = jvm.array_length(&buf).await?;
        let pos: i32 = jvm.get_field(&this, "pos", "I").await?;

        let available = (buf_length as i32 - pos) as i64;
        let len_to_skip = if n > available { available } else { n };

        jvm.put_field(&mut this, "pos", "I", pos + len_to_skip as i32).await?;

        Ok(len_to_skip)
    }
}
