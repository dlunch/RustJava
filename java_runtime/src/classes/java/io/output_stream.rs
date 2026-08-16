use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::{ClassAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

// abstract class java.io.OutputStream
pub struct OutputStream;

impl OutputStream {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/OutputStream",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/io/Closeable", "java/io/Flushable"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("write", "([B)V", Self::write_bytes, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("write", "([BII)V", Self::write_bytes_offset, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new_abstract("write", "(I)V", MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new("flush", "()V", Self::flush, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("close", "()V", Self::close, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::ABSTRACT,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.OutputStream::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn write_bytes(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, buffer: ClassInstanceRef<Array<i8>>) -> Result<()> {
        tracing::debug!("java.io.OutputStream::write({this:?}, {buffer:?})");

        if buffer.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "buffer is null").await);
        }
        let length = jvm.array_length(&buffer).await?;

        let _: () = jvm
            .invoke_virtual(&this, "java/io/OutputStream", "write", "([BII)V", (buffer, 0, length as i32))
            .await?;

        Ok(())
    }

    async fn write_bytes_offset(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        buffer: ClassInstanceRef<Array<i8>>,
        offset: i32,
        length: i32,
    ) -> Result<()> {
        tracing::debug!("java.io.OutputStream::write({this:?}, {buffer:?}, {offset:?}, {length:?})");

        if buffer.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "buffer is null").await);
        }
        let mut bytes = vec![0; length as usize];
        jvm.array_raw_buffer(&buffer).await?.read(offset as _, &mut bytes)?;
        for byte in bytes {
            let _: () = jvm.invoke_virtual(&this, "java/io/OutputStream", "write", "(I)V", (byte as i32,)).await?;
        }

        Ok(())
    }

    async fn flush(_jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.OutputStream::flush({this:?})");

        Ok(())
    }

    async fn close(_jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.OutputStream::close({this:?})");

        Ok(())
    }
}
