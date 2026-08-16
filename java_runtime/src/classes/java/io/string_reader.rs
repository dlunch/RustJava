use core::future::Future;

use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::lang::{Object, String},
};

// class java.io.StringReader
pub struct StringReader;

impl StringReader {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/StringReader",
            parent_class: Some("java/io/Reader"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init, MethodAccessFlags::PUBLIC),
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
                JavaFieldProto::new("str", "Ljava/lang/String;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("length", "I", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("next", "I", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("mark", "I", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.io.StringReader::<init>({this:?}, {value:?})");

        if value.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "string is null").await);
        }
        let _: () = jvm.invoke_special(&this, "java/io/Reader", "<init>", "()V", ()).await?;
        let length: i32 = jvm.invoke_virtual(&value, "java/lang/String", "length", "()I", ()).await?;
        jvm.put_field(&mut this, "str", "Ljava/lang/String;", value).await?;
        jvm.put_field(&mut this, "length", "I", length).await?;
        jvm.put_field(&mut this, "next", "I", 0).await?;
        jvm.put_field(&mut this, "mark", "I", 0).await
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
                    tracing::error!(?exit_error, "failed to release StringReader lock");
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
        tracing::debug!("java.io.StringReader::read({this:?})");

        let value: ClassInstanceRef<String> = jvm.get_field(&this, "str", "Ljava/lang/String;").await?;
        if value.is_null() {
            return Err(jvm.exception("java/io/IOException", "Stream closed").await);
        }
        let next: i32 = jvm.get_field(&this, "next", "I").await?;
        let length: i32 = jvm.get_field(&this, "length", "I").await?;
        if next >= length {
            return Ok(-1);
        }
        let result: JavaChar = jvm.invoke_virtual(&value, "java/lang/String", "charAt", "(I)C", (next,)).await?;
        jvm.put_field(&mut this, "next", "I", next + 1).await?;
        Ok(result as i32)
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
        tracing::debug!("java.io.StringReader::read({this:?}, {target:?}, {offset}, {length})");

        let value: ClassInstanceRef<String> = jvm.get_field(&this, "str", "Ljava/lang/String;").await?;
        if value.is_null() {
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

        let next: i32 = jvm.get_field(&this, "next", "I").await?;
        let source_length: i32 = jvm.get_field(&this, "length", "I").await?;
        if next >= source_length {
            return Ok(-1);
        }
        let copied = length.min(source_length - next);
        let _: () = jvm
            .invoke_virtual(&value, "java/lang/String", "getChars", "(II[CI)V", (next, next + copied, target, offset))
            .await?;
        jvm.put_field(&mut this, "next", "I", next + copied).await?;
        Ok(copied)
    }

    async fn skip(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, amount: i64) -> Result<i64> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, Self::skip_locked(jvm, this, amount)).await
    }

    async fn skip_locked(jvm: &Jvm, mut this: ClassInstanceRef<Self>, amount: i64) -> Result<i64> {
        tracing::debug!("java.io.StringReader::skip({this:?}, {amount})");

        let value: ClassInstanceRef<String> = jvm.get_field(&this, "str", "Ljava/lang/String;").await?;
        if value.is_null() {
            return Err(jvm.exception("java/io/IOException", "Stream closed").await);
        }
        if amount <= 0 {
            return Ok(0);
        }
        let next: i32 = jvm.get_field(&this, "next", "I").await?;
        let length: i32 = jvm.get_field(&this, "length", "I").await?;
        let skipped = amount.min((length - next) as i64);
        jvm.put_field(&mut this, "next", "I", next + skipped as i32).await?;
        Ok(skipped)
    }

    async fn ready(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, Self::ready_locked(jvm, this)).await
    }

    async fn ready_locked(jvm: &Jvm, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.io.StringReader::ready({this:?})");

        let value: ClassInstanceRef<String> = jvm.get_field(&this, "str", "Ljava/lang/String;").await?;
        if value.is_null() {
            return Err(jvm.exception("java/io/IOException", "Stream closed").await);
        }
        Ok(true)
    }

    async fn mark_supported(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.io.StringReader::markSupported({this:?})");
        Ok(true)
    }

    async fn mark(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, read_ahead_limit: i32) -> Result<()> {
        tracing::debug!("java.io.StringReader::mark({this:?}, {read_ahead_limit})");

        if read_ahead_limit < 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "Read-ahead limit < 0").await);
        }
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, Self::mark_locked(jvm, this)).await
    }

    async fn mark_locked(jvm: &Jvm, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let value: ClassInstanceRef<String> = jvm.get_field(&this, "str", "Ljava/lang/String;").await?;
        if value.is_null() {
            return Err(jvm.exception("java/io/IOException", "Stream closed").await);
        }
        let next: i32 = jvm.get_field(&this, "next", "I").await?;
        jvm.put_field(&mut this, "mark", "I", next).await
    }

    async fn reset(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, Self::reset_locked(jvm, this)).await
    }

    async fn reset_locked(jvm: &Jvm, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.StringReader::reset({this:?})");

        let value: ClassInstanceRef<String> = jvm.get_field(&this, "str", "Ljava/lang/String;").await?;
        if value.is_null() {
            return Err(jvm.exception("java/io/IOException", "Stream closed").await);
        }
        let mark: i32 = jvm.get_field(&this, "mark", "I").await?;
        jvm.put_field(&mut this, "next", "I", mark).await
    }

    async fn close(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, Self::close_locked(jvm, this)).await
    }

    async fn close_locked(jvm: &Jvm, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.StringReader::close({this:?})");

        let null_string: ClassInstanceRef<String> = None.into();
        jvm.put_field(&mut this, "str", "Ljava/lang/String;", null_string).await
    }
}
