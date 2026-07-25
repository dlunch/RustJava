use core::{cmp::min, future::Future};

use alloc::{vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::Reader,
        lang::{Object, String},
    },
};

const DEFAULT_BUFFER_SIZE: usize = 8192;
const INVALIDATED: i32 = -2;
const UNMARKED: i32 = -1;

// class java.io.BufferedReader
pub struct BufferedReader;

impl BufferedReader {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/BufferedReader",
            parent_class: Some("java/io/Reader"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/Reader;)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(Ljava/io/Reader;I)V", Self::init_with_size, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("read", "()I", Self::read_char, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("read", "([CII)I", Self::read, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("readLine", "()Ljava/lang/String;", Self::read_line, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("skip", "(J)J", Self::skip, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("ready", "()Z", Self::ready, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("markSupported", "()Z", Self::mark_supported, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("mark", "(I)V", Self::mark, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("reset", "()V", Self::reset, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("close", "()V", Self::close, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("in", "Ljava/io/Reader;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("cb", "[C", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("nChars", "I", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("nextChar", "I", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("markedChar", "I", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("readAheadLimit", "I", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("skipLF", "Z", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("markedSkipLF", "Z", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, r#in: ClassInstanceRef<Reader>) -> Result<()> {
        tracing::debug!("java.io.BufferedReader::<init>({this:?}, {in:?})", in = &r#in);

        jvm.invoke_special(
            &this,
            "java/io/BufferedReader",
            "<init>",
            "(Ljava/io/Reader;I)V",
            (r#in, DEFAULT_BUFFER_SIZE as i32),
        )
        .await
    }

    async fn init_with_size(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        r#in: ClassInstanceRef<Reader>,
        size: i32,
    ) -> Result<()> {
        tracing::debug!("java.io.BufferedReader::<init>({this:?}, {in:?}, {size})", in = &r#in);

        if r#in.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "reader is null").await);
        }
        if size <= 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "buffer size is not positive").await);
        }

        let _: () = jvm
            .invoke_special(&this, "java/io/Reader", "<init>", "(Ljava/lang/Object;)V", (r#in.clone(),))
            .await?;

        let cb = jvm.instantiate_array("C", size as usize).await?;
        jvm.put_field(&mut this, "in", "Ljava/io/Reader;", r#in).await?;
        jvm.put_field(&mut this, "cb", "[C", cb).await?;
        jvm.put_field(&mut this, "nChars", "I", 0).await?;
        jvm.put_field(&mut this, "nextChar", "I", 0).await?;
        jvm.put_field(&mut this, "markedChar", "I", UNMARKED).await?;
        jvm.put_field(&mut this, "readAheadLimit", "I", 0).await?;
        jvm.put_field(&mut this, "skipLF", "Z", false).await?;
        jvm.put_field(&mut this, "markedSkipLF", "Z", false).await?;
        Ok(())
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
                    tracing::error!(?exit_error, "failed to release BufferedReader lock");
                }
                Err(error)
            }
        }
    }

    async fn fill(jvm: &Jvm, this: &mut ClassInstanceRef<Self>) -> Result<i32> {
        let r#in: ClassInstanceRef<Reader> = jvm.get_field(this, "in", "Ljava/io/Reader;").await?;
        if r#in.is_null() {
            return Err(jvm.exception("java/io/IOException", "stream is closed").await);
        }

        let mut cb: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(this, "cb", "[C").await?;
        let mut destination = 0;
        let marked_char: i32 = jvm.get_field(this, "markedChar", "I").await?;
        if marked_char >= 0 {
            let next_char: i32 = jvm.get_field(this, "nextChar", "I").await?;
            let delta = next_char - marked_char;
            let read_ahead_limit: i32 = jvm.get_field(this, "readAheadLimit", "I").await?;
            if delta >= read_ahead_limit {
                jvm.put_field(this, "markedChar", "I", INVALIDATED).await?;
                jvm.put_field(this, "readAheadLimit", "I", 0).await?;
            } else {
                let capacity = jvm.array_length(&cb).await?;
                if read_ahead_limit as usize > capacity {
                    let expanded_capacity = min(capacity.saturating_mul(2), read_ahead_limit as usize);
                    let expanded = jvm.instantiate_array("C", expanded_capacity).await?;
                    let _: () = jvm
                        .invoke_static(
                            "java/lang/System",
                            "arraycopy",
                            "(Ljava/lang/Object;ILjava/lang/Object;II)V",
                            (cb, marked_char, expanded.clone(), 0, delta),
                        )
                        .await?;
                    cb = expanded.into();
                    jvm.put_field(this, "cb", "[C", cb.clone()).await?;
                } else if delta > 0 {
                    let _: () = jvm
                        .invoke_static(
                            "java/lang/System",
                            "arraycopy",
                            "(Ljava/lang/Object;ILjava/lang/Object;II)V",
                            (cb.clone(), marked_char, cb.clone(), 0, delta),
                        )
                        .await?;
                }

                jvm.put_field(this, "markedChar", "I", 0).await?;
                destination = delta;
            }
        }

        jvm.put_field(this, "nextChar", "I", destination).await?;
        jvm.put_field(this, "nChars", "I", destination).await?;

        let capacity = jvm.array_length(&cb).await? as i32;
        let mut read;
        loop {
            read = jvm
                .invoke_virtual(&r#in, "read", "([CII)I", (cb.clone(), destination, capacity - destination))
                .await?;
            if read != 0 {
                break;
            }
        }
        if read > 0 {
            jvm.put_field(this, "nChars", "I", destination + read).await?;
            jvm.put_field(this, "nextChar", "I", destination).await?;
        }

        Ok(read)
    }

    async fn read_char(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, Self::read_char_locked(jvm, this)).await
    }

    async fn read_char_locked(jvm: &Jvm, mut this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.io.BufferedReader::read({this:?})");

        let r#in: ClassInstanceRef<Reader> = jvm.get_field(&this, "in", "Ljava/io/Reader;").await?;
        if r#in.is_null() {
            return Err(jvm.exception("java/io/IOException", "stream is closed").await);
        }

        loop {
            let mut next_char: i32 = jvm.get_field(&this, "nextChar", "I").await?;
            let mut n_chars: i32 = jvm.get_field(&this, "nChars", "I").await?;
            if next_char >= n_chars {
                if Self::fill(jvm, &mut this).await? <= 0 {
                    return Ok(-1);
                }
                next_char = jvm.get_field(&this, "nextChar", "I").await?;
                n_chars = jvm.get_field(&this, "nChars", "I").await?;
            }

            if jvm.get_field::<bool>(&this, "skipLF", "Z").await? {
                jvm.put_field(&mut this, "skipLF", "Z", false).await?;
                let cb: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "cb", "[C").await?;
                if jvm.load_array::<JavaChar>(&cb, next_char as usize, 1).await?[0] == '\n' as JavaChar {
                    next_char += 1;
                    jvm.put_field(&mut this, "nextChar", "I", next_char).await?;
                    if next_char >= n_chars {
                        continue;
                    }
                }
            }

            let cb: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "cb", "[C").await?;
            let value = jvm.load_array::<JavaChar>(&cb, next_char as usize, 1).await?[0];
            jvm.put_field(&mut this, "nextChar", "I", next_char + 1).await?;
            return Ok(value as i32);
        }
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
        tracing::debug!("java.io.BufferedReader::read({this:?}, {target:?}, {offset}, {length})");

        let r#in: ClassInstanceRef<Reader> = jvm.get_field(&this, "in", "Ljava/io/Reader;").await?;
        if r#in.is_null() {
            return Err(jvm.exception("java/io/IOException", "stream is closed").await);
        }
        if target.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "target is null").await);
        }

        let target_length = jvm.array_length(&target).await? as i32;
        if offset < 0 || length < 0 || offset > target_length || length > target_length - offset {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "invalid offset or length").await);
        }
        if length == 0 {
            return Ok(0);
        }

        let mut total = 0;
        while total < length {
            let mut next_char: i32 = jvm.get_field(&this, "nextChar", "I").await?;
            let mut n_chars: i32 = jvm.get_field(&this, "nChars", "I").await?;
            if next_char >= n_chars {
                if total > 0 && !jvm.invoke_virtual::<_, bool>(&r#in, "ready", "()Z", ()).await? {
                    break;
                }
                if Self::fill(jvm, &mut this).await? <= 0 {
                    break;
                }
                next_char = jvm.get_field(&this, "nextChar", "I").await?;
                n_chars = jvm.get_field(&this, "nChars", "I").await?;
            }

            if jvm.get_field::<bool>(&this, "skipLF", "Z").await? {
                jvm.put_field(&mut this, "skipLF", "Z", false).await?;
                let cb: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "cb", "[C").await?;
                if jvm.load_array::<JavaChar>(&cb, next_char as usize, 1).await?[0] == '\n' as JavaChar {
                    next_char += 1;
                    jvm.put_field(&mut this, "nextChar", "I", next_char).await?;
                    if next_char >= n_chars {
                        continue;
                    }
                }
            }

            let count = min(length - total, n_chars - next_char);
            let cb: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "cb", "[C").await?;
            let _: () = jvm
                .invoke_static(
                    "java/lang/System",
                    "arraycopy",
                    "(Ljava/lang/Object;ILjava/lang/Object;II)V",
                    (cb, next_char, target.clone(), offset + total, count),
                )
                .await?;
            total += count;
            jvm.put_field(&mut this, "nextChar", "I", next_char + count).await?;
        }

        if total == 0 { Ok(-1) } else { Ok(total) }
    }

    async fn read_line(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, Self::read_line_locked(jvm, this)).await
    }

    async fn read_line_locked(jvm: &Jvm, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.io.BufferedReader::readLine({this:?})");

        let r#in: ClassInstanceRef<Reader> = jvm.get_field(&this, "in", "Ljava/io/Reader;").await?;
        if r#in.is_null() {
            return Err(jvm.exception("java/io/IOException", "stream is closed").await);
        }

        let mut omit_lf: bool = jvm.get_field(&this, "skipLF", "Z").await?;
        jvm.put_field(&mut this, "skipLF", "Z", false).await?;
        let mut line = Vec::new();

        loop {
            let mut next_char: i32 = jvm.get_field(&this, "nextChar", "I").await?;
            let mut n_chars: i32 = jvm.get_field(&this, "nChars", "I").await?;
            if next_char >= n_chars {
                if Self::fill(jvm, &mut this).await? <= 0 {
                    if line.is_empty() {
                        return Ok(None.into());
                    }

                    let line_length = line.len();
                    let mut chars = jvm.instantiate_array("C", line_length).await?;
                    jvm.store_array(&mut chars, 0, line).await?;
                    let value = jvm.new_class("java/lang/String", "([CII)V", (chars, 0, line_length as i32)).await?;
                    return Ok(value.into());
                }
                next_char = jvm.get_field(&this, "nextChar", "I").await?;
                n_chars = jvm.get_field(&this, "nChars", "I").await?;
            }

            let cb: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "cb", "[C").await?;
            if omit_lf {
                omit_lf = false;
                if jvm.load_array::<JavaChar>(&cb, next_char as usize, 1).await?[0] == '\n' as JavaChar {
                    next_char += 1;
                    jvm.put_field(&mut this, "nextChar", "I", next_char).await?;
                    if next_char >= n_chars {
                        continue;
                    }
                }
            }

            let buffered: Vec<JavaChar> = jvm.load_array(&cb, next_char as usize, (n_chars - next_char) as usize).await?;
            if let Some(index) = buffered.iter().position(|value| *value == '\n' as JavaChar || *value == '\r' as JavaChar) {
                line.extend_from_slice(&buffered[..index]);
                let terminator = buffered[index];
                jvm.put_field(&mut this, "nextChar", "I", next_char + index as i32 + 1).await?;
                if terminator == '\r' as JavaChar {
                    jvm.put_field(&mut this, "skipLF", "Z", true).await?;
                }

                let line_length = line.len();
                let mut chars = jvm.instantiate_array("C", line_length).await?;
                jvm.store_array(&mut chars, 0, line).await?;
                let value = jvm.new_class("java/lang/String", "([CII)V", (chars, 0, line_length as i32)).await?;
                return Ok(value.into());
            }

            line.extend(buffered);
            jvm.put_field(&mut this, "nextChar", "I", n_chars).await?;
        }
    }

    async fn skip(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, count: i64) -> Result<i64> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, Self::skip_locked(jvm, this, count)).await
    }

    async fn skip_locked(jvm: &Jvm, mut this: ClassInstanceRef<Self>, count: i64) -> Result<i64> {
        tracing::debug!("java.io.BufferedReader::skip({this:?}, {count})");

        let r#in: ClassInstanceRef<Reader> = jvm.get_field(&this, "in", "Ljava/io/Reader;").await?;
        if r#in.is_null() {
            return Err(jvm.exception("java/io/IOException", "stream is closed").await);
        }
        if count < 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "skip value is negative").await);
        }

        let mut remaining = count;
        while remaining > 0 {
            let mut next_char: i32 = jvm.get_field(&this, "nextChar", "I").await?;
            let mut n_chars: i32 = jvm.get_field(&this, "nChars", "I").await?;
            if next_char >= n_chars {
                if Self::fill(jvm, &mut this).await? <= 0 {
                    break;
                }
                next_char = jvm.get_field(&this, "nextChar", "I").await?;
                n_chars = jvm.get_field(&this, "nChars", "I").await?;
            }

            if jvm.get_field::<bool>(&this, "skipLF", "Z").await? {
                jvm.put_field(&mut this, "skipLF", "Z", false).await?;
                let cb: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "cb", "[C").await?;
                if jvm.load_array::<JavaChar>(&cb, next_char as usize, 1).await?[0] == '\n' as JavaChar {
                    next_char += 1;
                    jvm.put_field(&mut this, "nextChar", "I", next_char).await?;
                    if next_char >= n_chars {
                        continue;
                    }
                }
            }

            let skipped = min(remaining, (n_chars - next_char) as i64);
            remaining -= skipped;
            jvm.put_field(&mut this, "nextChar", "I", next_char + skipped as i32).await?;
        }

        Ok(count - remaining)
    }

    async fn ready(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, Self::ready_locked(jvm, this)).await
    }

    async fn ready_locked(jvm: &Jvm, mut this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.io.BufferedReader::ready({this:?})");

        let r#in: ClassInstanceRef<Reader> = jvm.get_field(&this, "in", "Ljava/io/Reader;").await?;
        if r#in.is_null() {
            return Err(jvm.exception("java/io/IOException", "stream is closed").await);
        }

        let mut next_char: i32 = jvm.get_field(&this, "nextChar", "I").await?;
        let mut n_chars: i32 = jvm.get_field(&this, "nChars", "I").await?;
        if jvm.get_field::<bool>(&this, "skipLF", "Z").await? {
            if next_char >= n_chars && jvm.invoke_virtual::<_, bool>(&r#in, "ready", "()Z", ()).await? {
                let _ = Self::fill(jvm, &mut this).await?;
                next_char = jvm.get_field(&this, "nextChar", "I").await?;
                n_chars = jvm.get_field(&this, "nChars", "I").await?;
            }

            if next_char < n_chars {
                let cb: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "cb", "[C").await?;
                if jvm.load_array::<JavaChar>(&cb, next_char as usize, 1).await?[0] == '\n' as JavaChar {
                    next_char += 1;
                    jvm.put_field(&mut this, "nextChar", "I", next_char).await?;
                }
                jvm.put_field(&mut this, "skipLF", "Z", false).await?;
            }
        }

        if next_char < n_chars {
            return Ok(true);
        }
        jvm.invoke_virtual(&r#in, "ready", "()Z", ()).await
    }

    async fn mark_supported(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.io.BufferedReader::markSupported({this:?})");
        Ok(true)
    }

    async fn mark(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, read_ahead_limit: i32) -> Result<()> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, Self::mark_locked(jvm, this, read_ahead_limit)).await
    }

    async fn mark_locked(jvm: &Jvm, mut this: ClassInstanceRef<Self>, read_ahead_limit: i32) -> Result<()> {
        tracing::debug!("java.io.BufferedReader::mark({this:?}, {read_ahead_limit})");

        let r#in: ClassInstanceRef<Reader> = jvm.get_field(&this, "in", "Ljava/io/Reader;").await?;
        if r#in.is_null() {
            return Err(jvm.exception("java/io/IOException", "stream is closed").await);
        }
        if read_ahead_limit < 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "read-ahead limit is negative").await);
        }

        let next_char: i32 = jvm.get_field(&this, "nextChar", "I").await?;
        let skip_lf: bool = jvm.get_field(&this, "skipLF", "Z").await?;
        jvm.put_field(&mut this, "readAheadLimit", "I", read_ahead_limit).await?;
        jvm.put_field(&mut this, "markedChar", "I", next_char).await?;
        jvm.put_field(&mut this, "markedSkipLF", "Z", skip_lf).await?;
        Ok(())
    }

    async fn reset(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, Self::reset_locked(jvm, this)).await
    }

    async fn reset_locked(jvm: &Jvm, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.BufferedReader::reset({this:?})");

        let r#in: ClassInstanceRef<Reader> = jvm.get_field(&this, "in", "Ljava/io/Reader;").await?;
        if r#in.is_null() {
            return Err(jvm.exception("java/io/IOException", "stream is closed").await);
        }

        let marked_char: i32 = jvm.get_field(&this, "markedChar", "I").await?;
        if marked_char < 0 {
            let message = if marked_char == INVALIDATED {
                "mark invalid"
            } else {
                "stream not marked"
            };
            return Err(jvm.exception("java/io/IOException", message).await);
        }

        let marked_skip_lf: bool = jvm.get_field(&this, "markedSkipLF", "Z").await?;
        jvm.put_field(&mut this, "nextChar", "I", marked_char).await?;
        jvm.put_field(&mut this, "skipLF", "Z", marked_skip_lf).await?;
        Ok(())
    }

    async fn close(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        Self::with_lock(jvm, &lock, Self::close_locked(jvm, this)).await
    }

    async fn close_locked(jvm: &Jvm, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.BufferedReader::close({this:?})");

        let r#in: ClassInstanceRef<Reader> = jvm.get_field(&this, "in", "Ljava/io/Reader;").await?;
        if r#in.is_null() {
            return Ok(());
        }

        let result = jvm.invoke_virtual(&r#in, "close", "()V", ()).await;
        let closed: ClassInstanceRef<Reader> = None.into();
        jvm.put_field(&mut this, "in", "Ljava/io/Reader;", closed).await?;
        let released: ClassInstanceRef<Array<JavaChar>> = None.into();
        jvm.put_field(&mut this, "cb", "[C", released).await?;
        result
    }
}
