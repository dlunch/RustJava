use core::future::Future;

use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

// class java.io.CharArrayReader
pub struct CharArrayReader;

impl CharArrayReader {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/CharArrayReader",
            parent_class: Some("java/io/Reader"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "([C)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "([CII)V", Self::init_with_range, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("read", "()I", Self::read_char, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("read", "([CII)I", Self::read, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("skip", "(J)J", Self::skip, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("ready", "()Z", Self::ready, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("markSupported", "()Z", Self::mark_supported, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("mark", "(I)V", Self::mark, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("reset", "()V", Self::reset, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("close", "()V", Self::close, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("buf", "[C", FieldAccessFlags::PROTECTED),
                JavaFieldProto::new("pos", "I", FieldAccessFlags::PROTECTED),
                JavaFieldProto::new("markedPos", "I", FieldAccessFlags::PROTECTED),
                JavaFieldProto::new("count", "I", FieldAccessFlags::PROTECTED),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, buffer: ClassInstanceRef<Array<JavaChar>>) -> Result<()> {
        tracing::debug!("java.io.CharArrayReader::<init>({this:?}, {buffer:?})");

        if buffer.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "buffer is null").await);
        }
        let length = jvm.array_length(&buffer).await? as i32;
        jvm.invoke_special(&this, "java/io/CharArrayReader", "<init>", "([CII)V", (buffer, 0, length))
            .await
    }

    async fn init_with_range(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        buffer: ClassInstanceRef<Array<JavaChar>>,
        offset: i32,
        length: i32,
    ) -> Result<()> {
        tracing::debug!("java.io.CharArrayReader::<init>({this:?}, {buffer:?}, {offset}, {length})");

        if buffer.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "buffer is null").await);
        }
        let buffer_length = jvm.array_length(&buffer).await? as i32;
        let end = offset as i64 + length as i64;
        if offset < 0 || offset > buffer_length || length < 0 || end < 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "Invalid offset or length").await);
        }

        let _: () = jvm.invoke_special(&this, "java/io/Reader", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "buf", "[C", buffer).await?;
        jvm.put_field(&mut this, "pos", "I", offset).await?;
        jvm.put_field(&mut this, "markedPos", "I", offset).await?;
        jvm.put_field(&mut this, "count", "I", end.min(buffer_length as i64) as i32).await
    }

    async fn with_lock<T, F>(jvm: &Jvm, lock: &ClassInstanceRef<Object>, operation: F) -> Result<T>
    where
        F: Future<Output = Result<T>>,
    {
        jvm.monitor_enter(lock).await?;
        match operation.await {
            Ok(value) => {
                jvm.monitor_exit(lock).await?;
                Ok(value)
            }
            Err(error) => {
                if let Err(exit_error) = jvm.monitor_exit(lock).await {
                    tracing::error!(?exit_error, "failed to release CharArrayReader lock");
                }
                Err(error)
            }
        }
    }

    async fn read_char(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, Self::read_char_locked(jvm, this)).await
    }

    async fn read_char_locked(jvm: &Jvm, mut this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.io.CharArrayReader::read({this:?})");

        let buffer: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "buf", "[C").await?;
        if buffer.is_null() {
            return Err(jvm.exception("java/io/IOException", "Stream closed").await);
        }
        let position: i32 = jvm.get_field(&this, "pos", "I").await?;
        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        if position >= count {
            return Ok(-1);
        }
        let value = jvm.load_array::<JavaChar>(&buffer, position as usize, 1).await?[0];
        jvm.put_field(&mut this, "pos", "I", position + 1).await?;
        Ok(value as i32)
    }

    async fn read(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        target: ClassInstanceRef<Array<JavaChar>>,
        offset: i32,
        length: i32,
    ) -> Result<i32> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, Self::read_locked(jvm, this, target, offset, length)).await
    }

    async fn read_locked(
        jvm: &Jvm,
        mut this: ClassInstanceRef<Self>,
        target: ClassInstanceRef<Array<JavaChar>>,
        offset: i32,
        length: i32,
    ) -> Result<i32> {
        tracing::debug!("java.io.CharArrayReader::read({this:?}, {target:?}, {offset}, {length})");

        let buffer: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "buf", "[C").await?;
        if buffer.is_null() {
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
        let position: i32 = jvm.get_field(&this, "pos", "I").await?;
        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        if position >= count {
            return Ok(-1);
        }
        let copied = length.min(count - position);
        let _: () = jvm
            .invoke_static(
                "java/lang/System",
                "arraycopy",
                "(Ljava/lang/Object;ILjava/lang/Object;II)V",
                (buffer, position, target, offset, copied),
            )
            .await?;
        jvm.put_field(&mut this, "pos", "I", position + copied).await?;
        Ok(copied)
    }

    async fn skip(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, amount: i64) -> Result<i64> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, Self::skip_locked(jvm, this, amount)).await
    }

    async fn skip_locked(jvm: &Jvm, mut this: ClassInstanceRef<Self>, amount: i64) -> Result<i64> {
        tracing::debug!("java.io.CharArrayReader::skip({this:?}, {amount})");

        let buffer: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "buf", "[C").await?;
        if buffer.is_null() {
            return Err(jvm.exception("java/io/IOException", "Stream closed").await);
        }
        if amount <= 0 {
            return Ok(0);
        }
        let position: i32 = jvm.get_field(&this, "pos", "I").await?;
        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        let skipped = amount.min((count - position) as i64);
        jvm.put_field(&mut this, "pos", "I", position + skipped as i32).await?;
        Ok(skipped)
    }

    async fn ready(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, Self::ready_locked(jvm, this)).await
    }

    async fn ready_locked(jvm: &Jvm, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.io.CharArrayReader::ready({this:?})");

        let buffer: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "buf", "[C").await?;
        if buffer.is_null() {
            return Err(jvm.exception("java/io/IOException", "Stream closed").await);
        }
        let position: i32 = jvm.get_field(&this, "pos", "I").await?;
        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        Ok(count - position > 0)
    }

    async fn mark_supported(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.io.CharArrayReader::markSupported({this:?})");
        Ok(true)
    }

    async fn mark(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, read_ahead_limit: i32) -> Result<()> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, Self::mark_locked(jvm, this, read_ahead_limit)).await
    }

    async fn mark_locked(jvm: &Jvm, mut this: ClassInstanceRef<Self>, read_ahead_limit: i32) -> Result<()> {
        tracing::debug!("java.io.CharArrayReader::mark({this:?}, {read_ahead_limit})");

        let buffer: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "buf", "[C").await?;
        if buffer.is_null() {
            return Err(jvm.exception("java/io/IOException", "Stream closed").await);
        }
        let position: i32 = jvm.get_field(&this, "pos", "I").await?;
        jvm.put_field(&mut this, "markedPos", "I", position).await
    }

    async fn reset(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, Self::reset_locked(jvm, this)).await
    }

    async fn reset_locked(jvm: &Jvm, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.CharArrayReader::reset({this:?})");

        let buffer: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "buf", "[C").await?;
        if buffer.is_null() {
            return Err(jvm.exception("java/io/IOException", "Stream closed").await);
        }
        let marked_position: i32 = jvm.get_field(&this, "markedPos", "I").await?;
        jvm.put_field(&mut this, "pos", "I", marked_position).await
    }

    async fn close(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, Self::close_locked(jvm, this)).await
    }

    async fn close_locked(jvm: &Jvm, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.CharArrayReader::close({this:?})");

        let null_buffer: ClassInstanceRef<Array<JavaChar>> = None.into();
        jvm.put_field(&mut this, "buf", "[C", null_buffer).await
    }
}
