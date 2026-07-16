use alloc::{vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

// class java.io.ByteArrayOutputStream
pub struct ByteArrayOutputStream;

impl ByteArrayOutputStream {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/ByteArrayOutputStream",
            parent_class: Some("java/io/OutputStream"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(I)V", Self::init_with_size, Default::default()),
                JavaMethodProto::new("write", "(I)V", Self::write, Default::default()),
                JavaMethodProto::new("write", "([BII)V", Self::write_bytes, Default::default()),
                JavaMethodProto::new("toByteArray", "()[B", Self::to_byte_array, Default::default()),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, Default::default()),
                JavaMethodProto::new("size", "()I", Self::size, Default::default()),
                JavaMethodProto::new("reset", "()V", Self::reset, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("buf", "[B", Default::default()),
                JavaFieldProto::new("pos", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.ByteArrayOutputStream::<init>({this:?})");

        let _: () = jvm
            .invoke_special(&this, "java/io/ByteArrayOutputStream", "<init>", "(I)V", (1024,))
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
        jvm.put_field(&mut this, "pos", "I", 0).await?;

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

        let pos: i32 = jvm.get_field(&this, "pos", "I").await?;
        Self::ensure_capacity(jvm, &mut this, (pos + len) as usize).await?;
        let mut buf = jvm.get_field(&this, "buf", "[B").await?;
        let values: Vec<i8> = jvm.load_array(&bytes, off as usize, len as usize).await?;
        jvm.store_array(&mut buf, pos as usize, values).await?;
        jvm.put_field(&mut this, "pos", "I", pos + len).await
    }

    async fn write(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, b: i32) -> Result<()> {
        tracing::debug!("java.io.ByteArrayOutputStream::write({this:?}, {b:?})");

        let pos: i32 = jvm.get_field(&this, "pos", "I").await?;
        Self::ensure_capacity(jvm, &mut this, (pos + 1) as _).await?;

        let mut buf = jvm.get_field(&this, "buf", "[B").await?;
        jvm.store_array(&mut buf, pos as _, vec![b as i8]).await?;

        jvm.put_field(&mut this, "pos", "I", pos + 1).await?;

        Ok(())
    }

    async fn to_byte_array(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Array<i8>>> {
        tracing::debug!("java.io.ByteArrayOutputStream::to_byte_array({this:?})");

        let buf: ClassInstanceRef<Array<i8>> = jvm.get_field(&this, "buf", "[B").await?;
        let pos: i32 = jvm.get_field(&this, "pos", "I").await?;

        let dest = jvm.instantiate_array("B", pos as _).await?;
        let _: () = jvm
            .invoke_static(
                "java/lang/System",
                "arraycopy",
                "(Ljava/lang/Object;ILjava/lang/Object;II)V",
                (buf.clone(), 0, dest.clone(), 0, pos),
            )
            .await?;

        Ok(dest.into())
    }

    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.io.ByteArrayOutputStream::size({this:?})");

        let pos: i32 = jvm.get_field(&this, "pos", "I").await?;

        Ok(pos)
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.io.ByteArrayOutputStream::toString({this:?})");
        let buf: ClassInstanceRef<Array<i8>> = jvm.get_field(&this, "buf", "[B").await?;
        let pos: i32 = jvm.get_field(&this, "pos", "I").await?;
        let bytes = jvm.instantiate_array("B", pos as usize).await?;
        let _: () = jvm
            .invoke_static(
                "java/lang/System",
                "arraycopy",
                "(Ljava/lang/Object;ILjava/lang/Object;II)V",
                (buf, 0, bytes.clone(), 0, pos),
            )
            .await?;
        Ok(jvm.new_class("java/lang/String", "([B)V", (bytes,)).await?.into())
    }

    async fn reset(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.ByteArrayOutputStream::reset({this:?})");

        jvm.put_field(&mut this, "pos", "I", 0).await?;

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
            let new_capacity = capacity * 2;
            let new_buf = jvm.instantiate_array("B", new_capacity).await?;

            let _: () = jvm
                .invoke_static(
                    "java/lang/System",
                    "arraycopy",
                    "(Ljava/lang/Object;ILjava/lang/Object;II)V",
                    (old_buf.clone(), 0, new_buf.clone(), 0, current_capacity as i32),
                )
                .await?;

            jvm.put_field(this, "buf", "[B", new_buf.clone()).await?;
        }

        Ok(())
    }
}
