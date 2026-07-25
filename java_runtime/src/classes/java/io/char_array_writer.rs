use core::future::Future;

use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::Writer,
        lang::{Object, String},
    },
};

const DEFAULT_INITIAL_BUFFER_SIZE: i32 = 32;

// class java.io.CharArrayWriter
pub struct CharArrayWriter;

impl CharArrayWriter {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/CharArrayWriter",
            parent_class: Some("java/io/Writer"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(I)V", Self::init_with_size, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("write", "(I)V", Self::write_char, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("write", "([CII)V", Self::write_chars, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("write", "(Ljava/lang/String;II)V", Self::write_string, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("writeTo", "(Ljava/io/Writer;)V", Self::write_to, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("reset", "()V", Self::reset, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("toCharArray", "()[C", Self::to_char_array, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("size", "()I", Self::size, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("flush", "()V", Self::flush, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("close", "()V", Self::close, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("buf", "[C", FieldAccessFlags::PROTECTED),
                JavaFieldProto::new("count", "I", FieldAccessFlags::PROTECTED),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.CharArrayWriter::<init>({this:?})");
        jvm.invoke_special(&this, "java/io/CharArrayWriter", "<init>", "(I)V", (DEFAULT_INITIAL_BUFFER_SIZE,))
            .await
    }

    async fn init_with_size(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, size: i32) -> Result<()> {
        tracing::debug!("java.io.CharArrayWriter::<init>({this:?}, {size})");

        if size < 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "Negative initial size").await);
        }
        let _: () = jvm.invoke_special(&this, "java/io/Writer", "<init>", "()V", ()).await?;
        let buffer = jvm.instantiate_array("C", size as usize).await?;
        jvm.put_field(&mut this, "buf", "[C", buffer).await?;
        jvm.put_field(&mut this, "count", "I", 0).await
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
                    tracing::error!(?exit_error, "failed to release CharArrayWriter lock");
                }
                Err(error)
            }
        }
    }

    async fn ensure_capacity(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, minimum: i32) -> Result<()> {
        let buffer: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(this, "buf", "[C").await?;
        let current = jvm.array_length(&buffer).await? as i32;
        if minimum > current {
            let new_length = current.saturating_mul(2).max(minimum).max(1);
            let new_buffer = jvm.instantiate_array("C", new_length as usize).await?;
            let count: i32 = jvm.get_field(this, "count", "I").await?;
            let _: () = jvm
                .invoke_static(
                    "java/lang/System",
                    "arraycopy",
                    "(Ljava/lang/Object;ILjava/lang/Object;II)V",
                    (buffer, 0, new_buffer.clone(), 0, count),
                )
                .await?;
            jvm.put_field(this, "buf", "[C", new_buffer).await?;
        }
        Ok(())
    }

    async fn write_char(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, Self::write_char_locked(jvm, this, value)).await
    }

    async fn write_char_locked(jvm: &Jvm, mut this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        tracing::debug!("java.io.CharArrayWriter::write({this:?}, {value})");

        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        Self::ensure_capacity(jvm, &mut this, count + 1).await?;
        let mut buffer: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "buf", "[C").await?;
        jvm.store_array(&mut buffer, count as usize, [value as JavaChar]).await?;
        jvm.put_field(&mut this, "count", "I", count + 1).await
    }

    async fn write_chars(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        chars: ClassInstanceRef<Array<JavaChar>>,
        offset: i32,
        length: i32,
    ) -> Result<()> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, Self::write_chars_locked(jvm, this, chars, offset, length)).await
    }

    async fn write_chars_locked(
        jvm: &Jvm,
        mut this: ClassInstanceRef<Self>,
        chars: ClassInstanceRef<Array<JavaChar>>,
        offset: i32,
        length: i32,
    ) -> Result<()> {
        tracing::debug!("java.io.CharArrayWriter::write({this:?}, {chars:?}, {offset}, {length})");

        if chars.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "character array is null").await);
        }
        let source_length = jvm.array_length(&chars).await? as i32;
        if offset < 0 || length < 0 || offset > source_length - length {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "Invalid offset or length").await);
        }
        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        Self::ensure_capacity(jvm, &mut this, count + length).await?;
        let buffer: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "buf", "[C").await?;
        let _: () = jvm
            .invoke_static(
                "java/lang/System",
                "arraycopy",
                "(Ljava/lang/Object;ILjava/lang/Object;II)V",
                (chars, offset, buffer, count, length),
            )
            .await?;
        jvm.put_field(&mut this, "count", "I", count + length).await
    }

    async fn write_string(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        value: ClassInstanceRef<String>,
        offset: i32,
        length: i32,
    ) -> Result<()> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, Self::write_string_locked(jvm, this, value, offset, length)).await
    }

    async fn write_string_locked(
        jvm: &Jvm,
        mut this: ClassInstanceRef<Self>,
        value: ClassInstanceRef<String>,
        offset: i32,
        length: i32,
    ) -> Result<()> {
        tracing::debug!("java.io.CharArrayWriter::write({this:?}, {value:?}, {offset}, {length})");

        if value.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "string is null").await);
        }
        let source_length: i32 = jvm.invoke_virtual(&value, "length", "()I", ()).await?;
        if offset < 0 || length < 0 || offset > source_length - length {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "Invalid offset or length").await);
        }
        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        Self::ensure_capacity(jvm, &mut this, count + length).await?;
        let buffer: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "buf", "[C").await?;
        let _: () = jvm
            .invoke_virtual(&value, "getChars", "(II[CI)V", (offset, offset + length, buffer, count))
            .await?;
        jvm.put_field(&mut this, "count", "I", count + length).await
    }

    async fn write_to(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, out: ClassInstanceRef<Writer>) -> Result<()> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, Self::write_to_locked(jvm, this, out)).await
    }

    async fn write_to_locked(jvm: &Jvm, this: ClassInstanceRef<Self>, out: ClassInstanceRef<Writer>) -> Result<()> {
        tracing::debug!("java.io.CharArrayWriter::writeTo({this:?}, {out:?})");

        if out.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "writer is null").await);
        }
        let buffer: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "buf", "[C").await?;
        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        jvm.invoke_virtual(&out, "write", "([CII)V", (buffer, 0, count)).await
    }

    async fn reset(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, Self::reset_locked(jvm, this)).await
    }

    async fn reset_locked(jvm: &Jvm, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.CharArrayWriter::reset({this:?})");
        jvm.put_field(&mut this, "count", "I", 0).await
    }

    async fn to_char_array(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Array<JavaChar>>> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, Self::to_char_array_locked(jvm, this)).await
    }

    async fn to_char_array_locked(jvm: &Jvm, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Array<JavaChar>>> {
        tracing::debug!("java.io.CharArrayWriter::toCharArray({this:?})");

        let buffer: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "buf", "[C").await?;
        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        let copy = jvm.instantiate_array("C", count as usize).await?;
        let _: () = jvm
            .invoke_static(
                "java/lang/System",
                "arraycopy",
                "(Ljava/lang/Object;ILjava/lang/Object;II)V",
                (buffer, 0, copy.clone(), 0, count),
            )
            .await?;
        Ok(copy.into())
    }

    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, Self::size_locked(jvm, this)).await
    }

    async fn size_locked(jvm: &Jvm, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.io.CharArrayWriter::size({this:?})");
        jvm.get_field(&this, "count", "I").await
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, Self::to_string_locked(jvm, this)).await
    }

    async fn to_string_locked(jvm: &Jvm, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.io.CharArrayWriter::toString({this:?})");

        let buffer: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "buf", "[C").await?;
        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        Ok(jvm.new_class("java/lang/String", "([CII)V", (buffer, 0, count)).await?.into())
    }

    async fn flush(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.CharArrayWriter::flush({this:?})");
        Ok(())
    }

    async fn close(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.CharArrayWriter::close({this:?})");
        Ok(())
    }
}
