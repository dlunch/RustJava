use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::io::OutputStream};

const DEFAULT_BUFFER_SIZE: i32 = 8192;

// class java.io.BufferedOutputStream
pub struct BufferedOutputStream;

impl BufferedOutputStream {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/BufferedOutputStream",
            parent_class: Some("java/io/FilterOutputStream"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/OutputStream;)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(Ljava/io/OutputStream;I)V", Self::init_with_size, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("write", "(I)V", Self::write, MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED),
                JavaMethodProto::new(
                    "write",
                    "([BII)V",
                    Self::write_bytes,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new("flush", "()V", Self::flush, MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED),
            ],
            fields: vec![
                JavaFieldProto::new("buf", "[B", FieldAccessFlags::PROTECTED),
                JavaFieldProto::new("count", "I", FieldAccessFlags::PROTECTED),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, out: ClassInstanceRef<OutputStream>) -> Result<()> {
        tracing::debug!("java.io.BufferedOutputStream::<init>({this:?}, {out:?})");
        jvm.invoke_special(
            &this,
            "java/io/BufferedOutputStream",
            "<init>",
            "(Ljava/io/OutputStream;I)V",
            (out, DEFAULT_BUFFER_SIZE),
        )
        .await
    }

    async fn init_with_size(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        out: ClassInstanceRef<OutputStream>,
        size: i32,
    ) -> Result<()> {
        tracing::debug!("java.io.BufferedOutputStream::<init>({this:?}, {out:?}, {size})");

        if out.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "output is null").await);
        }
        if size <= 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "Buffer size <= 0").await);
        }

        let _: () = jvm
            .invoke_special(&this, "java/io/FilterOutputStream", "<init>", "(Ljava/io/OutputStream;)V", (out,))
            .await?;
        let buffer = jvm.instantiate_array("B", size as usize).await?;
        jvm.put_field(&mut this, "buf", "[B", buffer).await?;
        jvm.put_field(&mut this, "count", "I", 0).await
    }

    async fn flush_buffer(jvm: &Jvm, this: &mut ClassInstanceRef<Self>) -> Result<()> {
        let out: ClassInstanceRef<OutputStream> = jvm.get_field(this, "out", "Ljava/io/OutputStream;").await?;
        let buffer: ClassInstanceRef<Array<i8>> = jvm.get_field(this, "buf", "[B").await?;
        if out.is_null() || buffer.is_null() {
            return Err(jvm.exception("java/io/IOException", "Stream closed").await);
        }
        let count: i32 = jvm.get_field(this, "count", "I").await?;
        if count > 0 {
            let _: () = jvm.invoke_virtual(&out, "write", "([BII)V", (buffer, 0, count)).await?;
            jvm.put_field(this, "count", "I", 0).await?;
        }
        Ok(())
    }

    async fn write(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        tracing::debug!("java.io.BufferedOutputStream::write({this:?}, {value})");

        let out: ClassInstanceRef<OutputStream> = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
        let mut buffer: ClassInstanceRef<Array<i8>> = jvm.get_field(&this, "buf", "[B").await?;
        if out.is_null() || buffer.is_null() {
            return Err(jvm.exception("java/io/IOException", "Stream closed").await);
        }
        let mut count: i32 = jvm.get_field(&this, "count", "I").await?;
        if count >= jvm.array_length(&buffer).await? as i32 {
            Self::flush_buffer(jvm, &mut this).await?;
            count = 0;
            buffer = jvm.get_field(&this, "buf", "[B").await?;
        }
        jvm.store_array(&mut buffer, count as usize, [value as i8]).await?;
        jvm.put_field(&mut this, "count", "I", count + 1).await
    }

    async fn write_bytes(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        bytes: ClassInstanceRef<Array<i8>>,
        offset: i32,
        length: i32,
    ) -> Result<()> {
        tracing::debug!("java.io.BufferedOutputStream::write({this:?}, {bytes:?}, {offset}, {length})");

        let source_length = jvm.array_length(&bytes).await? as i32;
        if offset < 0 || length < 0 || offset > source_length - length {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "Invalid offset or length").await);
        }
        let out: ClassInstanceRef<OutputStream> = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
        let mut buffer: ClassInstanceRef<Array<i8>> = jvm.get_field(&this, "buf", "[B").await?;
        if out.is_null() || buffer.is_null() {
            return Err(jvm.exception("java/io/IOException", "Stream closed").await);
        }
        if length == 0 {
            return Ok(());
        }

        let buffer_length = jvm.array_length(&buffer).await? as i32;
        if length >= buffer_length {
            Self::flush_buffer(jvm, &mut this).await?;
            return jvm.invoke_virtual(&out, "write", "([BII)V", (bytes, offset, length)).await;
        }

        let mut count: i32 = jvm.get_field(&this, "count", "I").await?;
        if length > buffer_length - count {
            Self::flush_buffer(jvm, &mut this).await?;
            count = 0;
            buffer = jvm.get_field(&this, "buf", "[B").await?;
        }
        let _: () = jvm
            .invoke_static(
                "java/lang/System",
                "arraycopy",
                "(Ljava/lang/Object;ILjava/lang/Object;II)V",
                (bytes, offset, buffer, count, length),
            )
            .await?;
        jvm.put_field(&mut this, "count", "I", count + length).await
    }

    async fn flush(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.BufferedOutputStream::flush({this:?})");

        Self::flush_buffer(jvm, &mut this).await?;
        let out: ClassInstanceRef<OutputStream> = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
        jvm.invoke_virtual(&out, "flush", "()V", ()).await
    }
}
