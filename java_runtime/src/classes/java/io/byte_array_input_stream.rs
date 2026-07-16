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
                JavaMethodProto::new("<init>", "([BII)V", Self::init_with_offset_length, Default::default()),
                JavaMethodProto::new("available", "()I", Self::available, Default::default()),
                JavaMethodProto::new("read", "([BII)I", Self::read, Default::default()),
                JavaMethodProto::new("read", "()I", Self::read_byte, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
                JavaMethodProto::new("skip", "(J)J", Self::skip, Default::default()),
                JavaMethodProto::new("mark", "(I)V", Self::mark, Default::default()),
                JavaMethodProto::new("reset", "()V", Self::reset, Default::default()),
                JavaMethodProto::new("markSupported", "()Z", Self::mark_supported, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("buf", "[B", Default::default()),
                JavaFieldProto::new("pos", "I", Default::default()),
                JavaFieldProto::new("count", "I", Default::default()),
                JavaFieldProto::new("mark", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, data: ClassInstanceRef<Array<i8>>) -> Result<()> {
        tracing::debug!("java.io.ByteArrayInputStream::<init>({this:?}, {data:?})");

        let count = jvm.array_length(&data).await?;

        let _: () = jvm
            .invoke_special(&this, "java/io/ByteArrayInputStream", "<init>", "([BII)V", (data, 0, count as i32))
            .await?;

        Ok(())
    }

    async fn init_with_offset_length(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        data: ClassInstanceRef<Array<i8>>,
        offset: i32,
        length: i32,
    ) -> Result<()> {
        tracing::debug!("java.io.ByteArrayInputStream::<init>({this:?}, {data:?}, {offset}, {length})");

        let data_length = jvm.array_length(&data).await? as i32;
        if offset < 0 || length < 0 || offset > data_length {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "Invalid offset or length").await);
        }

        let _: () = jvm.invoke_special(&this, "java/io/InputStream", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "buf", "[B", data).await?;
        jvm.put_field(&mut this, "pos", "I", offset).await?;
        jvm.put_field(&mut this, "count", "I", (offset + length).min(data_length)).await?;
        jvm.put_field(&mut this, "mark", "I", offset).await?;

        Ok(())
    }

    async fn available(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.io.ByteArrayInputStream::available({this:?})");

        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        let pos: i32 = jvm.get_field(&this, "pos", "I").await?;

        Ok((count - pos) as _)
    }

    async fn read(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        b: ClassInstanceRef<Array<i8>>,
        off: i32,
        len: i32,
    ) -> Result<i32> {
        tracing::debug!("java.io.ByteArrayInputStream::read({this:?}, {b:?}, {off}, {len})");

        let buf: ClassInstanceRef<Array<i8>> = jvm.get_field(&this, "buf", "[B").await?;
        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        let pos: i32 = jvm.get_field(&this, "pos", "I").await?;

        let target_length = jvm.array_length(&b).await? as i32;
        if off < 0 || len < 0 || off > target_length - len {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "Invalid offset or length").await);
        }
        if len == 0 {
            return Ok(0);
        }

        let available = count - pos;
        if available <= 0 {
            return Ok(-1);
        }
        let len_to_read = if len > available { available } else { len };

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
        tracing::debug!("java.io.ByteArrayInputStream::readByte({this:?})");

        let buf = jvm.get_field(&this, "buf", "[B").await?;
        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        let pos: i32 = jvm.get_field(&this, "pos", "I").await?;

        if pos >= count {
            return Ok(-1);
        }

        let result: i8 = jvm.load_array(&buf, pos as _, 1).await?[0];

        jvm.put_field(&mut this, "pos", "I", pos + 1).await?;

        Ok(result as u8 as _)
    }

    async fn close(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<ByteArrayInputStream>) -> Result<()> {
        tracing::debug!("java.io.ByteArrayInputStream::close({this:?})");

        Ok(())
    }

    async fn skip(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, n: i64) -> Result<i64> {
        tracing::debug!("java.io.ByteArrayInputStream::skip({this:?}, {n:?})");

        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        let pos: i32 = jvm.get_field(&this, "pos", "I").await?;

        let available = (count - pos) as i64;
        let len_to_skip = n.max(0).min(available);

        jvm.put_field(&mut this, "pos", "I", pos + len_to_skip as i32).await?;

        Ok(len_to_skip)
    }

    async fn mark(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, readlimit: i32) -> Result<()> {
        tracing::debug!("java.io.ByteArrayInputStream::mark({this:?}, {readlimit:?})");

        let pos: i32 = jvm.get_field(&this, "pos", "I").await?;
        jvm.put_field(&mut this, "mark", "I", pos).await?;

        Ok(())
    }

    async fn reset(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.ByteArrayInputStream::reset({this:?})");

        let mark: i32 = jvm.get_field(&this, "mark", "I").await?;
        jvm.put_field(&mut this, "pos", "I", mark).await?;

        Ok(())
    }

    async fn mark_supported(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.io.ByteArrayInputStream::markSupported({this:?})");
        Ok(true)
    }
}
