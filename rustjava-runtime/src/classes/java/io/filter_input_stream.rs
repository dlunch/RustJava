use jvm::{Array, ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::io::InputStream};

// class java.io.FilterInputStream
#[java_class(name = "java/io/FilterInputStream", extends = "java/io/InputStream", public)]
pub struct FilterInputStream {
    #[field(protected)]
    r#in: ClassInstanceRef<InputStream>,
}

#[java_methods]
impl FilterInputStream {
    #[method(constructor, protected)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, r#in: ClassInstanceRef<InputStream>) -> Result<()> {
        tracing::debug!("java.io.FilterInputStream::<init>({this:?}, {:?})", &r#in);

        let _: () = jvm.invoke_special(&this, "java/io/InputStream", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "java/io/FilterInputStream", "in", "Ljava/io/InputStream;", r#in)
            .await?;

        Ok(())
    }

    #[method(public)]
    async fn available(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.io.FilterInputStream::available({this:?})");

        let r#in: ClassInstanceRef<InputStream> = jvm.get_field(&this, "java/io/FilterInputStream", "in", "Ljava/io/InputStream;").await?;
        if r#in.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "underlying stream is null").await);
        }
        let available: i32 = jvm.invoke_virtual(&r#in, "java/io/InputStream", "available", "()I", ()).await?;

        Ok(available)
    }

    #[method(public)]
    async fn close(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.FilterInputStream::close({this:?})");

        let r#in: ClassInstanceRef<InputStream> = jvm.get_field(&this, "java/io/FilterInputStream", "in", "Ljava/io/InputStream;").await?;
        if r#in.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "underlying stream is null").await);
        }
        let _: () = jvm.invoke_virtual(&r#in, "java/io/InputStream", "close", "()V", ()).await?;

        Ok(())
    }

    #[method(public)]
    async fn reset(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.FilterInputStream::reset({this:?})");

        let r#in: ClassInstanceRef<InputStream> = jvm.get_field(&this, "java/io/FilterInputStream", "in", "Ljava/io/InputStream;").await?;
        if r#in.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "underlying stream is null").await);
        }
        let _: () = jvm.invoke_virtual(&r#in, "java/io/InputStream", "reset", "()V", ()).await?;

        Ok(())
    }

    #[method(public)]
    async fn skip(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, n: i64) -> Result<i64> {
        tracing::debug!("java.io.FilterInputStream::skip({this:?}, {n})");
        let r#in: ClassInstanceRef<InputStream> = jvm.get_field(&this, "java/io/FilterInputStream", "in", "Ljava/io/InputStream;").await?;
        if r#in.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "underlying stream is null").await);
        }
        jvm.invoke_virtual(&r#in, "java/io/InputStream", "skip", "(J)J", (n,)).await
    }

    #[method(public)]
    async fn mark(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, readlimit: i32) -> Result<()> {
        tracing::debug!("java.io.FilterInputStream::mark({this:?}, {readlimit})");
        let r#in: ClassInstanceRef<InputStream> = jvm.get_field(&this, "java/io/FilterInputStream", "in", "Ljava/io/InputStream;").await?;
        if r#in.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "underlying stream is null").await);
        }
        jvm.invoke_virtual(&r#in, "java/io/InputStream", "mark", "(I)V", (readlimit,)).await
    }

    #[method(public)]
    async fn mark_supported(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.io.FilterInputStream::markSupported({this:?})");
        let r#in: ClassInstanceRef<InputStream> = jvm.get_field(&this, "java/io/FilterInputStream", "in", "Ljava/io/InputStream;").await?;
        if r#in.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "underlying stream is null").await);
        }
        jvm.invoke_virtual(&r#in, "java/io/InputStream", "markSupported", "()Z", ()).await
    }

    #[method(public)]
    async fn read(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, b: ClassInstanceRef<Array<i8>>) -> Result<i32> {
        tracing::debug!("java.io.FilterInputStream::read({this:?}, {b:?})");

        let r#in: ClassInstanceRef<InputStream> = jvm.get_field(&this, "java/io/FilterInputStream", "in", "Ljava/io/InputStream;").await?;
        if r#in.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "underlying stream is null").await);
        }
        let result: i32 = jvm.invoke_virtual(&r#in, "java/io/InputStream", "read", "([B)I", (b,)).await?;

        Ok(result)
    }

    #[method(name = "read", public)]
    async fn read_with_offset_length(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        b: ClassInstanceRef<Array<i8>>,
        off: i32,
        len: i32,
    ) -> Result<i32> {
        tracing::debug!("java.io.FilterInputStream::read({this:?}, {b:?}, {off}, {len})");

        let r#in: ClassInstanceRef<InputStream> = jvm.get_field(&this, "java/io/FilterInputStream", "in", "Ljava/io/InputStream;").await?;
        if r#in.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "underlying stream is null").await);
        }
        let result: i32 = jvm.invoke_virtual(&r#in, "java/io/InputStream", "read", "([BII)I", (b, off, len)).await?;

        Ok(result)
    }

    #[method(name = "read", public)]
    async fn read_byte_int(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.io.FilterInputStream::read({this:?})");

        let r#in: ClassInstanceRef<InputStream> = jvm.get_field(&this, "java/io/FilterInputStream", "in", "Ljava/io/InputStream;").await?;
        if r#in.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "underlying stream is null").await);
        }
        let result: i32 = jvm.invoke_virtual(&r#in, "java/io/InputStream", "read", "()I", ()).await?;

        Ok(result)
    }
}
