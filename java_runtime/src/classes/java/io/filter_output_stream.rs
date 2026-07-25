use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, JavaError, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::io::OutputStream};

// class java.io.FilterOutputStream
pub struct FilterOutputStream;

impl FilterOutputStream {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/FilterOutputStream",
            parent_class: Some("java/io/OutputStream"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/OutputStream;)V", Self::init, Default::default()),
                JavaMethodProto::new("write", "([BII)V", Self::write_bytes_offset, Default::default()),
                JavaMethodProto::new("write", "(I)V", Self::write, Default::default()),
                JavaMethodProto::new("flush", "()V", Self::flush, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![JavaFieldProto::new("out", "Ljava/io/OutputStream;", FieldAccessFlags::PROTECTED)],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, out: ClassInstanceRef<OutputStream>) -> Result<()> {
        tracing::debug!("java.io.FilterOutputStream::<init>({this:?}, {out:?})");

        let _: () = jvm.invoke_special(&this, "java/io/OutputStream", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "out", "Ljava/io/OutputStream;", out).await?;

        Ok(())
    }

    async fn write_bytes_offset(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        bytes: ClassInstanceRef<Array<i8>>,
        offset: i32,
        length: i32,
    ) -> Result<()> {
        tracing::debug!(" java.io.FilterOutputStream::write({this:?}, {bytes:?}, {offset:?}, {length:?})");

        let out = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
        let _: () = jvm.invoke_virtual(&out, "write", "([BII)V", (bytes, offset, length)).await?;

        Ok(())
    }

    async fn write(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, byte: i32) -> Result<()> {
        tracing::debug!("java.io.FilterOutputStream::write({this:?}, {byte:?})");

        let out = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
        let _: () = jvm.invoke_virtual(&out, "write", "(I)V", (byte,)).await?;

        Ok(())
    }

    async fn flush(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.FilterOutputStream::flush({this:?})");
        let out = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
        jvm.invoke_virtual(&out, "flush", "()V", ()).await
    }

    async fn close(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.FilterOutputStream::close({this:?})");

        let out: ClassInstanceRef<OutputStream> = jvm.get_field(&this, "out", "Ljava/io/OutputStream;").await?;
        if out.is_null() {
            return Ok(());
        }

        match jvm.invoke_virtual::<_, ()>(&this, "flush", "()V", ()).await {
            Ok(()) => {}
            Err(JavaError::JavaException(exception)) if jvm.is_instance(&*exception, "java/io/IOException") => {}
            Err(error) => {
                let null_output: ClassInstanceRef<OutputStream> = None.into();
                jvm.put_field(&mut this, "out", "Ljava/io/OutputStream;", null_output).await?;
                return Err(error);
            }
        }

        let close_result: Result<()> = jvm.invoke_virtual(&out, "close", "()V", ()).await;
        let null_output: ClassInstanceRef<OutputStream> = None.into();
        jvm.put_field(&mut this, "out", "Ljava/io/OutputStream;", null_output).await?;
        close_result
    }
}
