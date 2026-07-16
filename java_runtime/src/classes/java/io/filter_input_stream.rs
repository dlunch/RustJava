use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::FieldAccessFlags;
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::io::InputStream};

// class java.io.FilterInputStream
pub struct FilterInputStream;

impl FilterInputStream {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/FilterInputStream",
            parent_class: Some("java/io/InputStream"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/InputStream;)V", Self::init, Default::default()),
                JavaMethodProto::new("available", "()I", Self::available, Default::default()),
                JavaMethodProto::new("close", "()V", Self::close, Default::default()),
                JavaMethodProto::new("read", "()I", Self::read_byte_int, Default::default()),
                JavaMethodProto::new("read", "([B)I", Self::read, Default::default()),
                JavaMethodProto::new("read", "([BII)I", Self::read_with_offset_length, Default::default()),
                JavaMethodProto::new("skip", "(J)J", Self::skip, Default::default()),
                JavaMethodProto::new("mark", "(I)V", Self::mark, Default::default()),
                JavaMethodProto::new("reset", "()V", Self::reset, Default::default()),
                JavaMethodProto::new("markSupported", "()Z", Self::mark_supported, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("in", "Ljava/io/InputStream;", FieldAccessFlags::PROTECTED)],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, r#in: ClassInstanceRef<InputStream>) -> Result<()> {
        tracing::debug!("java.io.FilterInputStream::<init>({this:?}, {:?})", &r#in);

        let _: () = jvm.invoke_special(&this, "java/io/InputStream", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "in", "Ljava/io/InputStream;", r#in).await?;

        Ok(())
    }

    async fn available(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.io.FilterInputStream::available({this:?})");

        let r#in = jvm.get_field(&this, "in", "Ljava/io/InputStream;").await?;
        let available: i32 = jvm.invoke_virtual(&r#in, "available", "()I", ()).await?;

        Ok(available)
    }

    async fn close(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.FilterInputStream::close({this:?})");

        let r#in = jvm.get_field(&this, "in", "Ljava/io/InputStream;").await?;
        let _: () = jvm.invoke_virtual(&r#in, "close", "()V", ()).await?;

        Ok(())
    }

    async fn reset(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.FilterInputStream::reset({this:?})");

        let r#in = jvm.get_field(&this, "in", "Ljava/io/InputStream;").await?;
        let _: () = jvm.invoke_virtual(&r#in, "reset", "()V", ()).await?;

        Ok(())
    }

    async fn skip(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, n: i64) -> Result<i64> {
        tracing::debug!("java.io.FilterInputStream::skip({this:?}, {n})");
        let r#in = jvm.get_field(&this, "in", "Ljava/io/InputStream;").await?;
        jvm.invoke_virtual(&r#in, "skip", "(J)J", (n,)).await
    }

    async fn mark(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, readlimit: i32) -> Result<()> {
        tracing::debug!("java.io.FilterInputStream::mark({this:?}, {readlimit})");
        let r#in = jvm.get_field(&this, "in", "Ljava/io/InputStream;").await?;
        jvm.invoke_virtual(&r#in, "mark", "(I)V", (readlimit,)).await
    }

    async fn mark_supported(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.io.FilterInputStream::markSupported({this:?})");
        let r#in = jvm.get_field(&this, "in", "Ljava/io/InputStream;").await?;
        jvm.invoke_virtual(&r#in, "markSupported", "()Z", ()).await
    }

    async fn read(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, b: ClassInstanceRef<Array<i8>>) -> Result<i32> {
        tracing::debug!("java.io.FilterInputStream::read({this:?}, {b:?})");

        let r#in = jvm.get_field(&this, "in", "Ljava/io/InputStream;").await?;
        let result: i32 = jvm.invoke_virtual(&r#in, "read", "([B)I", (b,)).await?;

        Ok(result)
    }

    async fn read_with_offset_length(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        b: ClassInstanceRef<Array<i8>>,
        off: i32,
        len: i32,
    ) -> Result<i32> {
        tracing::debug!("java.io.FilterInputStream::read({this:?}, {b:?}, {off}, {len})");

        let r#in = jvm.get_field(&this, "in", "Ljava/io/InputStream;").await?;
        let result: i32 = jvm.invoke_virtual(&r#in, "read", "([BII)I", (b, off, len)).await?;

        Ok(result)
    }

    async fn read_byte_int(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.io.FilterInputStream::read({this:?})");

        let r#in = jvm.get_field(&this, "in", "Ljava/io/InputStream;").await?;
        let result: i32 = jvm.invoke_virtual(&r#in, "read", "()I", ()).await?;

        Ok(result)
    }
}
