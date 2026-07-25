use alloc::{vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{io::OutputStream, lang::String},
};

// class java.io.ByteArrayOutputStream
pub struct ByteArrayOutputStream;

impl ByteArrayOutputStream {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/ByteArrayOutputStream",
            parent_class: Some("java/io/OutputStream"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(I)V", Self::init_with_size, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("write", "(I)V", Self::write, MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED),
                JavaMethodProto::new(
                    "write",
                    "([BII)V",
                    Self::write_bytes,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "writeTo",
                    "(Ljava/io/OutputStream;)V",
                    Self::write_to,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "toByteArray",
                    "()[B",
                    Self::to_byte_array,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "toString",
                    "()Ljava/lang/String;",
                    Self::to_string,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "toString",
                    "(Ljava/lang/String;)Ljava/lang/String;",
                    Self::to_string_with_encoding,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new("size", "()I", Self::size, MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED),
                JavaMethodProto::new("reset", "()V", Self::reset, MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED),
                JavaMethodProto::new("close", "()V", Self::close, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("buf", "[B", FieldAccessFlags::PROTECTED),
                JavaFieldProto::new("count", "I", FieldAccessFlags::PROTECTED),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.ByteArrayOutputStream::<init>({this:?})");

        let _: () = jvm
            .invoke_special(&this, "java/io/ByteArrayOutputStream", "<init>", "(I)V", (32,))
            .await?;

        Ok(())
    }

    async fn init_with_size(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, size: i32) -> Result<()> {
        tracing::debug!("java.io.ByteArrayOutputStream::<init>({this:?}, {size:?})");

        if size < 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "Negative initial size").await);
        }

        let _: () = jvm.invoke_special(&this, "java/io/OutputStream", "<init>", "()V", ()).await?;

        let array = jvm.instantiate_array("B", size as usize).await?;

        jvm.put_field(&mut this, "buf", "[B", array).await?;
        jvm.put_field(&mut this, "count", "I", 0).await?;

        Ok(())
    }

    async fn write_bytes(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        bytes: ClassInstanceRef<Array<i8>>,
        off: i32,
        len: i32,
    ) -> Result<()> {
        tracing::debug!("java.io.ByteArrayOutputStream::write({this:?}, {bytes:?}, {off}, {len})");

        let length = jvm.array_length(&bytes).await? as i32;
        if off < 0 || len < 0 || off > length - len {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "Invalid offset or length").await);
        }

        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        Self::ensure_capacity(jvm, &mut this, (count + len) as usize).await?;
        let mut buf = jvm.get_field(&this, "buf", "[B").await?;
        let values: Vec<i8> = jvm.load_array(&bytes, off as usize, len as usize).await?;
        jvm.store_array(&mut buf, count as usize, values).await?;
        jvm.put_field(&mut this, "count", "I", count + len).await
    }

    async fn write(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, b: i32) -> Result<()> {
        tracing::debug!("java.io.ByteArrayOutputStream::write({this:?}, {b:?})");

        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        Self::ensure_capacity(jvm, &mut this, (count + 1) as _).await?;

        let mut buf = jvm.get_field(&this, "buf", "[B").await?;
        jvm.store_array(&mut buf, count as _, vec![b as i8]).await?;

        jvm.put_field(&mut this, "count", "I", count + 1).await?;

        Ok(())
    }

    async fn write_to(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, out: ClassInstanceRef<OutputStream>) -> Result<()> {
        tracing::debug!("java.io.ByteArrayOutputStream::writeTo({this:?}, {out:?})");

        if out.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "output is null").await);
        }

        let buf: ClassInstanceRef<Array<i8>> = jvm.get_field(&this, "buf", "[B").await?;
        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        jvm.invoke_virtual(&out, "write", "([BII)V", (buf, 0, count)).await
    }

    async fn to_byte_array(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Array<i8>>> {
        tracing::debug!("java.io.ByteArrayOutputStream::to_byte_array({this:?})");

        let buf: ClassInstanceRef<Array<i8>> = jvm.get_field(&this, "buf", "[B").await?;
        let count: i32 = jvm.get_field(&this, "count", "I").await?;

        let dest = jvm.instantiate_array("B", count as _).await?;
        let _: () = jvm
            .invoke_static(
                "java/lang/System",
                "arraycopy",
                "(Ljava/lang/Object;ILjava/lang/Object;II)V",
                (buf.clone(), 0, dest.clone(), 0, count),
            )
            .await?;

        Ok(dest.into())
    }

    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.io.ByteArrayOutputStream::size({this:?})");

        let count: i32 = jvm.get_field(&this, "count", "I").await?;

        Ok(count)
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.io.ByteArrayOutputStream::toString({this:?})");
        let buf: ClassInstanceRef<Array<i8>> = jvm.get_field(&this, "buf", "[B").await?;
        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        let bytes = jvm.instantiate_array("B", count as usize).await?;
        let _: () = jvm
            .invoke_static(
                "java/lang/System",
                "arraycopy",
                "(Ljava/lang/Object;ILjava/lang/Object;II)V",
                (buf, 0, bytes.clone(), 0, count),
            )
            .await?;
        Ok(jvm.new_class("java/lang/String", "([B)V", (bytes,)).await?.into())
    }

    async fn to_string_with_encoding(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        encoding: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.io.ByteArrayOutputStream::toString({this:?}, {encoding:?})");

        if encoding.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "encoding is null").await);
        }

        let buf: ClassInstanceRef<Array<i8>> = jvm.get_field(&this, "buf", "[B").await?;
        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        Ok(jvm
            .new_class("java/lang/String", "([BIILjava/lang/String;)V", (buf, 0, count, encoding))
            .await?
            .into())
    }

    async fn reset(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.ByteArrayOutputStream::reset({this:?})");

        jvm.put_field(&mut this, "count", "I", 0).await?;

        Ok(())
    }

    async fn close(_jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.ByteArrayOutputStream::close({this:?})");

        Ok(())
    }

    async fn ensure_capacity(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, capacity: usize) -> Result<()> {
        let old_buf = jvm.get_field(this, "buf", "[B").await?;
        let current_capacity = jvm.array_length(&old_buf).await?;

        if current_capacity < capacity {
            let new_capacity = current_capacity.saturating_mul(2).max(capacity);
            let new_buf = jvm.instantiate_array("B", new_capacity).await?;
            let count: i32 = jvm.get_field(this, "count", "I").await?;

            let _: () = jvm
                .invoke_static(
                    "java/lang/System",
                    "arraycopy",
                    "(Ljava/lang/Object;ILjava/lang/Object;II)V",
                    (old_buf.clone(), 0, new_buf.clone(), 0, count),
                )
                .await?;

            jvm.put_field(this, "buf", "[B", new_buf.clone()).await?;
        }

        Ok(())
    }
}
