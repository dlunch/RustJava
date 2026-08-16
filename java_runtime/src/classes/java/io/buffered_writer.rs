use core::future::Future;

use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::Writer,
        lang::{Object, String},
    },
};

const DEFAULT_CHAR_BUFFER_SIZE: i32 = 8192;

// class java.io.BufferedWriter
pub struct BufferedWriter;

impl BufferedWriter {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/BufferedWriter",
            parent_class: Some("java/io/Writer"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/Writer;)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(Ljava/io/Writer;I)V", Self::init_with_size, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("write", "(I)V", Self::write_char, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("write", "([CII)V", Self::write_chars, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("write", "(Ljava/lang/String;II)V", Self::write_string, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("newLine", "()V", Self::new_line, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("flush", "()V", Self::flush, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("close", "()V", Self::close, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("out", "Ljava/io/Writer;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("cb", "[C", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("nChars", "I", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("nextChar", "I", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("lineSeparator", "Ljava/lang/String;", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, out: ClassInstanceRef<Writer>) -> Result<()> {
        tracing::debug!("java.io.BufferedWriter::<init>({this:?}, {out:?})");
        jvm.invoke_special(
            &this,
            "java/io/BufferedWriter",
            "<init>",
            "(Ljava/io/Writer;I)V",
            (out, DEFAULT_CHAR_BUFFER_SIZE),
        )
        .await
    }

    async fn init_with_size(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        out: ClassInstanceRef<Writer>,
        size: i32,
    ) -> Result<()> {
        tracing::debug!("java.io.BufferedWriter::<init>({this:?}, {out:?}, {size})");

        if out.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "writer is null").await);
        }
        if size <= 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "Buffer size <= 0").await);
        }

        let _: () = jvm
            .invoke_special(&this, "java/io/Writer", "<init>", "(Ljava/lang/Object;)V", (out.clone(),))
            .await?;
        let buffer = jvm.instantiate_array("C", size as usize).await?;
        let key = JavaLangString::from_rust_string(jvm, "line.separator").await?;
        let mut line_separator: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/System", "getProperty", "(Ljava/lang/String;)Ljava/lang/String;", (key,))
            .await?;
        if line_separator.is_null() {
            line_separator = JavaLangString::from_rust_string(jvm, "\n").await?.into();
        }

        jvm.put_field(&mut this, "out", "Ljava/io/Writer;", out).await?;
        jvm.put_field(&mut this, "cb", "[C", buffer).await?;
        jvm.put_field(&mut this, "nChars", "I", size).await?;
        jvm.put_field(&mut this, "nextChar", "I", 0).await?;
        jvm.put_field(&mut this, "lineSeparator", "Ljava/lang/String;", line_separator).await
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
                    tracing::error!(?exit_error, "failed to release BufferedWriter lock");
                }
                Err(error)
            }
        }
    }

    async fn flush_buffer(jvm: &Jvm, this: &mut ClassInstanceRef<Self>) -> Result<()> {
        let out: ClassInstanceRef<Writer> = jvm.get_field(this, "out", "Ljava/io/Writer;").await?;
        let buffer: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(this, "cb", "[C").await?;
        if out.is_null() || buffer.is_null() {
            return Err(jvm.exception("java/io/IOException", "Stream closed").await);
        }
        let next_char: i32 = jvm.get_field(this, "nextChar", "I").await?;
        if next_char > 0 {
            let _: () = jvm
                .invoke_virtual(&out, "java/io/Writer", "write", "([CII)V", (buffer, 0, next_char))
                .await?;
            jvm.put_field(this, "nextChar", "I", 0).await?;
        }
        Ok(())
    }

    async fn write_char(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, Self::write_char_locked(jvm, this, value)).await
    }

    async fn write_char_locked(jvm: &Jvm, mut this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        tracing::debug!("java.io.BufferedWriter::write({this:?}, {value})");

        let out: ClassInstanceRef<Writer> = jvm.get_field(&this, "out", "Ljava/io/Writer;").await?;
        let mut buffer: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "cb", "[C").await?;
        if out.is_null() || buffer.is_null() {
            return Err(jvm.exception("java/io/IOException", "Stream closed").await);
        }
        let n_chars: i32 = jvm.get_field(&this, "nChars", "I").await?;
        let mut next_char: i32 = jvm.get_field(&this, "nextChar", "I").await?;
        if next_char >= n_chars {
            Self::flush_buffer(jvm, &mut this).await?;
            next_char = 0;
            buffer = jvm.get_field(&this, "cb", "[C").await?;
        }
        jvm.store_array(&mut buffer, next_char as usize, [value as JavaChar]).await?;
        jvm.put_field(&mut this, "nextChar", "I", next_char + 1).await
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
        tracing::debug!("java.io.BufferedWriter::write({this:?}, {chars:?}, {offset}, {length})");

        let out: ClassInstanceRef<Writer> = jvm.get_field(&this, "out", "Ljava/io/Writer;").await?;
        let buffer: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "cb", "[C").await?;
        if out.is_null() || buffer.is_null() {
            return Err(jvm.exception("java/io/IOException", "Stream closed").await);
        }
        if chars.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "character array is null").await);
        }
        let source_length = jvm.array_length(&chars).await? as i32;
        if offset < 0 || length < 0 || offset > source_length - length {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "Invalid offset or length").await);
        }
        if length == 0 {
            return Ok(());
        }
        let n_chars: i32 = jvm.get_field(&this, "nChars", "I").await?;
        if length >= n_chars {
            Self::flush_buffer(jvm, &mut this).await?;
            return jvm
                .invoke_virtual(&out, "java/io/Writer", "write", "([CII)V", (chars, offset, length))
                .await;
        }

        let mut source_position = offset;
        let end = offset + length;
        while source_position < end {
            let mut next_char: i32 = jvm.get_field(&this, "nextChar", "I").await?;
            let copied = (n_chars - next_char).min(end - source_position);
            let buffer: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "cb", "[C").await?;
            let _: () = jvm
                .invoke_static(
                    "java/lang/System",
                    "arraycopy",
                    "(Ljava/lang/Object;ILjava/lang/Object;II)V",
                    (chars.clone(), source_position, buffer, next_char, copied),
                )
                .await?;
            source_position += copied;
            next_char += copied;
            jvm.put_field(&mut this, "nextChar", "I", next_char).await?;
            if next_char >= n_chars {
                Self::flush_buffer(jvm, &mut this).await?;
            }
        }
        Ok(())
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
        tracing::debug!("java.io.BufferedWriter::write({this:?}, {value:?}, {offset}, {length})");

        let out: ClassInstanceRef<Writer> = jvm.get_field(&this, "out", "Ljava/io/Writer;").await?;
        let buffer: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "cb", "[C").await?;
        if out.is_null() || buffer.is_null() {
            return Err(jvm.exception("java/io/IOException", "Stream closed").await);
        }
        if value.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "string is null").await);
        }
        let string_length: i32 = jvm.invoke_virtual(&value, "java/lang/String", "length", "()I", ()).await?;
        if offset < 0 || length < 0 || offset > string_length - length {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "Invalid offset or length").await);
        }
        let n_chars: i32 = jvm.get_field(&this, "nChars", "I").await?;
        let mut source_position = offset;
        let end = offset + length;
        while source_position < end {
            let mut next_char: i32 = jvm.get_field(&this, "nextChar", "I").await?;
            let copied = (n_chars - next_char).min(end - source_position);
            let buffer: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "cb", "[C").await?;
            let _: () = jvm
                .invoke_virtual(
                    &value,
                    "java/lang/String",
                    "getChars",
                    "(II[CI)V",
                    (source_position, source_position + copied, buffer, next_char),
                )
                .await?;
            source_position += copied;
            next_char += copied;
            jvm.put_field(&mut this, "nextChar", "I", next_char).await?;
            if next_char >= n_chars {
                Self::flush_buffer(jvm, &mut this).await?;
            }
        }
        Ok(())
    }

    async fn new_line(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, Self::new_line_locked(jvm, this)).await
    }

    async fn new_line_locked(jvm: &Jvm, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.BufferedWriter::newLine({this:?})");

        let line_separator: ClassInstanceRef<String> = jvm.get_field(&this, "lineSeparator", "Ljava/lang/String;").await?;
        let length: i32 = jvm.invoke_virtual(&line_separator, "java/lang/String", "length", "()I", ()).await?;
        Self::write_string_locked(jvm, this, line_separator, 0, length).await
    }

    async fn flush(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, Self::flush_locked(jvm, this)).await
    }

    async fn flush_locked(jvm: &Jvm, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.BufferedWriter::flush({this:?})");

        Self::flush_buffer(jvm, &mut this).await?;
        let out: ClassInstanceRef<Writer> = jvm.get_field(&this, "out", "Ljava/io/Writer;").await?;
        jvm.invoke_virtual(&out, "java/io/Writer", "flush", "()V", ()).await
    }

    async fn close(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, Self::close_locked(jvm, this)).await
    }

    async fn close_locked(jvm: &Jvm, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.BufferedWriter::close({this:?})");

        let out: ClassInstanceRef<Writer> = jvm.get_field(&this, "out", "Ljava/io/Writer;").await?;
        if out.is_null() {
            return Ok(());
        }
        let flush_result = Self::flush_buffer(jvm, &mut this).await;
        let close_result: Result<()> = jvm.invoke_virtual(&out, "java/io/Writer", "close", "()V", ()).await;
        let null_writer: ClassInstanceRef<Writer> = None.into();
        let null_buffer: ClassInstanceRef<Array<JavaChar>> = None.into();
        let clear_out_result = jvm.put_field(&mut this, "out", "Ljava/io/Writer;", null_writer).await;
        let clear_buffer_result = jvm.put_field(&mut this, "cb", "[C", null_buffer).await;

        flush_result?;
        close_result?;
        clear_out_result?;
        clear_buffer_result
    }
}
