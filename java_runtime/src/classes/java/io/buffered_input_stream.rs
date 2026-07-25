use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::io::InputStream};

const DEFAULT_BUFFER_SIZE: i32 = 8192;

// class java.io.BufferedInputStream
pub struct BufferedInputStream;

impl BufferedInputStream {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/BufferedInputStream",
            parent_class: Some("java/io/FilterInputStream"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/InputStream;)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(Ljava/io/InputStream;I)V", Self::init_with_size, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "read",
                    "()I",
                    Self::read_byte,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new("read", "([BII)I", Self::read, MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED),
                JavaMethodProto::new("skip", "(J)J", Self::skip, MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED),
                JavaMethodProto::new(
                    "available",
                    "()I",
                    Self::available,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new("mark", "(I)V", Self::mark, MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED),
                JavaMethodProto::new("reset", "()V", Self::reset, MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED),
                JavaMethodProto::new("markSupported", "()Z", Self::mark_supported, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("close", "()V", Self::close, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("buf", "[B", FieldAccessFlags::PROTECTED | FieldAccessFlags::VOLATILE),
                JavaFieldProto::new("count", "I", FieldAccessFlags::PROTECTED),
                JavaFieldProto::new("pos", "I", FieldAccessFlags::PROTECTED),
                JavaFieldProto::new("markpos", "I", FieldAccessFlags::PROTECTED),
                JavaFieldProto::new("marklimit", "I", FieldAccessFlags::PROTECTED),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, r#in: ClassInstanceRef<InputStream>) -> Result<()> {
        tracing::debug!("java.io.BufferedInputStream::<init>({this:?}, {in:?})", in = &r#in);

        jvm.invoke_special(
            &this,
            "java/io/BufferedInputStream",
            "<init>",
            "(Ljava/io/InputStream;I)V",
            (r#in, DEFAULT_BUFFER_SIZE),
        )
        .await
    }

    async fn init_with_size(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        r#in: ClassInstanceRef<InputStream>,
        size: i32,
    ) -> Result<()> {
        tracing::debug!("java.io.BufferedInputStream::<init>({this:?}, {in:?}, {size})", in = &r#in);

        if r#in.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "input is null").await);
        }
        if size <= 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "Buffer size <= 0").await);
        }

        let _: () = jvm
            .invoke_special(&this, "java/io/FilterInputStream", "<init>", "(Ljava/io/InputStream;)V", (r#in,))
            .await?;
        let buffer = jvm.instantiate_array("B", size as usize).await?;
        jvm.put_field(&mut this, "buf", "[B", buffer).await?;
        jvm.put_field(&mut this, "count", "I", 0).await?;
        jvm.put_field(&mut this, "pos", "I", 0).await?;
        jvm.put_field(&mut this, "markpos", "I", -1).await?;
        jvm.put_field(&mut this, "marklimit", "I", 0).await
    }

    async fn fill(jvm: &Jvm, this: &mut ClassInstanceRef<Self>) -> Result<i32> {
        let r#in: ClassInstanceRef<InputStream> = jvm.get_field(this, "in", "Ljava/io/InputStream;").await?;
        let mut buffer: ClassInstanceRef<Array<i8>> = jvm.get_field(this, "buf", "[B").await?;
        if r#in.is_null() || buffer.is_null() {
            return Err(jvm.exception("java/io/IOException", "Stream closed").await);
        }

        let mut position: i32 = jvm.get_field(this, "pos", "I").await?;
        let mut mark_position: i32 = jvm.get_field(this, "markpos", "I").await?;
        let mark_limit: i32 = jvm.get_field(this, "marklimit", "I").await?;
        let mut buffer_length = jvm.array_length(&buffer).await? as i32;

        if mark_position < 0 {
            position = 0;
        } else if position >= buffer_length {
            if mark_position > 0 {
                let preserved = position - mark_position;
                let _: () = jvm
                    .invoke_static(
                        "java/lang/System",
                        "arraycopy",
                        "(Ljava/lang/Object;ILjava/lang/Object;II)V",
                        (buffer.clone(), mark_position, buffer.clone(), 0, preserved),
                    )
                    .await?;
                position = preserved;
                mark_position = 0;
            } else if buffer_length >= mark_limit {
                mark_position = -1;
                position = 0;
            } else {
                let new_length = (buffer_length.saturating_mul(2)).min(mark_limit).max(buffer_length + 1);
                let new_buffer: ClassInstanceRef<Array<i8>> = jvm.instantiate_array("B", new_length as usize).await?.into();
                let _: () = jvm
                    .invoke_static(
                        "java/lang/System",
                        "arraycopy",
                        "(Ljava/lang/Object;ILjava/lang/Object;II)V",
                        (buffer, 0, new_buffer.clone(), 0, position),
                    )
                    .await?;
                buffer = new_buffer;
                buffer_length = new_length;
                jvm.put_field(this, "buf", "[B", buffer.clone()).await?;
            }
        }

        jvm.put_field(this, "pos", "I", position).await?;
        jvm.put_field(this, "markpos", "I", mark_position).await?;
        jvm.put_field(this, "count", "I", position).await?;

        let read: i32 = jvm
            .invoke_virtual(&r#in, "read", "([BII)I", (buffer, position, buffer_length - position))
            .await?;
        if read > 0 {
            jvm.put_field(this, "count", "I", position + read).await?;
        }
        Ok(read)
    }

    async fn read_byte(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.io.BufferedInputStream::read({this:?})");

        let r#in: ClassInstanceRef<InputStream> = jvm.get_field(&this, "in", "Ljava/io/InputStream;").await?;
        let buffer: ClassInstanceRef<Array<i8>> = jvm.get_field(&this, "buf", "[B").await?;
        if r#in.is_null() || buffer.is_null() {
            return Err(jvm.exception("java/io/IOException", "Stream closed").await);
        }

        let mut position: i32 = jvm.get_field(&this, "pos", "I").await?;
        let mut count: i32 = jvm.get_field(&this, "count", "I").await?;
        if position >= count {
            if Self::fill(jvm, &mut this).await? == -1 {
                return Ok(-1);
            }
            position = jvm.get_field(&this, "pos", "I").await?;
            count = jvm.get_field(&this, "count", "I").await?;
            if position >= count {
                return Ok(-1);
            }
        }

        let buffer: ClassInstanceRef<Array<i8>> = jvm.get_field(&this, "buf", "[B").await?;
        let value = jvm.load_array::<i8>(&buffer, position as usize, 1).await?[0];
        jvm.put_field(&mut this, "pos", "I", position + 1).await?;
        Ok(value as u8 as i32)
    }

    async fn read(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        target: ClassInstanceRef<Array<i8>>,
        offset: i32,
        length: i32,
    ) -> Result<i32> {
        tracing::debug!("java.io.BufferedInputStream::read({this:?}, {target:?}, {offset}, {length})");

        let r#in: ClassInstanceRef<InputStream> = jvm.get_field(&this, "in", "Ljava/io/InputStream;").await?;
        let buffer: ClassInstanceRef<Array<i8>> = jvm.get_field(&this, "buf", "[B").await?;
        if r#in.is_null() || buffer.is_null() {
            return Err(jvm.exception("java/io/IOException", "Stream closed").await);
        }
        if target.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "target is null").await);
        }
        let target_length = jvm.array_length(&target).await? as i32;
        if offset < 0 || length < 0 || offset > target_length - length {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "Invalid offset or length").await);
        }
        if length == 0 {
            return Ok(0);
        }

        let mut total = 0;
        while total < length {
            let mut position: i32 = jvm.get_field(&this, "pos", "I").await?;
            let mut count: i32 = jvm.get_field(&this, "count", "I").await?;
            if position >= count {
                if Self::fill(jvm, &mut this).await? == -1 {
                    break;
                }
                position = jvm.get_field(&this, "pos", "I").await?;
                count = jvm.get_field(&this, "count", "I").await?;
                if position >= count {
                    break;
                }
            }

            let copied = (count - position).min(length - total);
            let buffer: ClassInstanceRef<Array<i8>> = jvm.get_field(&this, "buf", "[B").await?;
            let _: () = jvm
                .invoke_static(
                    "java/lang/System",
                    "arraycopy",
                    "(Ljava/lang/Object;ILjava/lang/Object;II)V",
                    (buffer, position, target.clone(), offset + total, copied),
                )
                .await?;
            jvm.put_field(&mut this, "pos", "I", position + copied).await?;
            total += copied;

            if total < length {
                let available: i32 = jvm.invoke_virtual(&r#in, "available", "()I", ()).await?;
                if available == 0 {
                    break;
                }
            }
        }

        if total == 0 { Ok(-1) } else { Ok(total) }
    }

    async fn skip(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, amount: i64) -> Result<i64> {
        tracing::debug!("java.io.BufferedInputStream::skip({this:?}, {amount})");

        let r#in: ClassInstanceRef<InputStream> = jvm.get_field(&this, "in", "Ljava/io/InputStream;").await?;
        let buffer: ClassInstanceRef<Array<i8>> = jvm.get_field(&this, "buf", "[B").await?;
        if r#in.is_null() || buffer.is_null() {
            return Err(jvm.exception("java/io/IOException", "Stream closed").await);
        }
        if amount <= 0 {
            return Ok(0);
        }

        let mut position: i32 = jvm.get_field(&this, "pos", "I").await?;
        let mut count: i32 = jvm.get_field(&this, "count", "I").await?;
        if position >= count {
            let mark_position: i32 = jvm.get_field(&this, "markpos", "I").await?;
            if mark_position < 0 {
                return jvm.invoke_virtual(&r#in, "skip", "(J)J", (amount,)).await;
            }
            if Self::fill(jvm, &mut this).await? == -1 {
                return Ok(0);
            }
            position = jvm.get_field(&this, "pos", "I").await?;
            count = jvm.get_field(&this, "count", "I").await?;
        }

        let skipped = amount.min((count - position) as i64);
        jvm.put_field(&mut this, "pos", "I", position + skipped as i32).await?;
        Ok(skipped)
    }

    async fn available(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.io.BufferedInputStream::available({this:?})");

        let r#in: ClassInstanceRef<InputStream> = jvm.get_field(&this, "in", "Ljava/io/InputStream;").await?;
        let buffer: ClassInstanceRef<Array<i8>> = jvm.get_field(&this, "buf", "[B").await?;
        if r#in.is_null() || buffer.is_null() {
            return Err(jvm.exception("java/io/IOException", "Stream closed").await);
        }
        let position: i32 = jvm.get_field(&this, "pos", "I").await?;
        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        let underlying: i32 = jvm.invoke_virtual(&r#in, "available", "()I", ()).await?;
        Ok((count - position).saturating_add(underlying))
    }

    async fn mark(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, read_limit: i32) -> Result<()> {
        tracing::debug!("java.io.BufferedInputStream::mark({this:?}, {read_limit})");

        let position: i32 = jvm.get_field(&this, "pos", "I").await?;
        jvm.put_field(&mut this, "marklimit", "I", read_limit).await?;
        jvm.put_field(&mut this, "markpos", "I", position).await
    }

    async fn reset(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.BufferedInputStream::reset({this:?})");

        let r#in: ClassInstanceRef<InputStream> = jvm.get_field(&this, "in", "Ljava/io/InputStream;").await?;
        let buffer: ClassInstanceRef<Array<i8>> = jvm.get_field(&this, "buf", "[B").await?;
        if r#in.is_null() || buffer.is_null() {
            return Err(jvm.exception("java/io/IOException", "Stream closed").await);
        }
        let mark_position: i32 = jvm.get_field(&this, "markpos", "I").await?;
        if mark_position < 0 {
            return Err(jvm.exception("java/io/IOException", "Resetting to invalid mark").await);
        }
        jvm.put_field(&mut this, "pos", "I", mark_position).await
    }

    async fn mark_supported(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.io.BufferedInputStream::markSupported({this:?})");
        Ok(true)
    }

    async fn close(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.BufferedInputStream::close({this:?})");

        let r#in: ClassInstanceRef<InputStream> = jvm.get_field(&this, "in", "Ljava/io/InputStream;").await?;
        if r#in.is_null() {
            return Ok(());
        }
        let null_input: ClassInstanceRef<InputStream> = None.into();
        let null_buffer: ClassInstanceRef<Array<i8>> = None.into();
        jvm.put_field(&mut this, "in", "Ljava/io/InputStream;", null_input).await?;
        jvm.put_field(&mut this, "buf", "[B", null_buffer).await?;
        jvm.invoke_virtual(&r#in, "close", "()V", ()).await
    }
}
