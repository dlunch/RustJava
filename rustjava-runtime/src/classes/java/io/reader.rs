use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::Object};

// abstract class java.io.Reader
#[java_class(name = "java/io/Reader", extends = "java/lang/Object", abstract)]
pub struct Reader {
    #[field(protected)]
    lock: ClassInstanceRef<Object>,
}

#[java_methods]
impl Reader {
    #[method(constructor, protected)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.Reader::<init>({this:?})");

        let _: () = jvm
            .invoke_special(&this, "java/io/Reader", "<init>", "(Ljava/lang/Object;)V", (this.clone(),))
            .await?;

        Ok(())
    }

    #[method(constructor, protected)]
    async fn init_with_lock(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, lock: ClassInstanceRef<Object>) -> Result<()> {
        tracing::debug!("java.io.Reader::<init>({this:?}, {lock:?})");

        if lock.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "lock is null").await);
        }

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/io/Reader", "lock", "Ljava/lang/Object;", lock).await?;

        Ok(())
    }

    #[method(name = "read", public)]
    async fn read_char(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.io.Reader::read({this:?})");

        let chars = jvm.instantiate_array("C", 1).await?;
        let read: i32 = jvm
            .invoke_virtual(&this, "java/io/Reader", "read", "([CII)I", (chars.clone(), 0, 1))
            .await?;
        if read == -1 {
            return Ok(-1);
        }

        let value: JavaChar = jvm.load_array(&chars, 0, 1).await?[0];
        Ok(value as i32)
    }

    #[method(public)]
    async fn read(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, buf: ClassInstanceRef<Array<JavaChar>>) -> Result<i32> {
        tracing::debug!("java.io.Reader::read({this:?}, {buf:?})");
        if buf.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "argument is null").await);
        }

        let len = jvm.array_length(&buf).await? as i32;
        let result = jvm.invoke_virtual(&this, "java/io/Reader", "read", "([CII)I", (buf, 0, len)).await?;

        Ok(result)
    }

    #[method(name = "read", public)]
    async fn read_offset(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        target: ClassInstanceRef<Array<JavaChar>>,
        offset: i32,
        length: i32,
    ) -> Result<i32>;

    #[method(public)]
    async fn skip(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, n: i64) -> Result<i64> {
        tracing::debug!("java.io.Reader::skip({this:?}, {n})");

        if n < 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "skip value is negative").await);
        }

        let buffer_size = n.min(8192) as usize;
        let buffer = jvm.instantiate_array("C", buffer_size).await?;
        let mut remaining = n;
        while remaining > 0 {
            let read: i32 = jvm
                .invoke_virtual(
                    &this,
                    "java/io/Reader",
                    "read",
                    "([CII)I",
                    (buffer.clone(), 0, remaining.min(buffer_size as i64) as i32),
                )
                .await?;
            if read == -1 {
                break;
            }
            remaining -= read as i64;
        }

        Ok(n - remaining)
    }

    #[method(public)]
    async fn ready(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.io.Reader::ready({this:?})");
        Ok(false)
    }

    #[method(public)]
    async fn mark_supported(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.io.Reader::markSupported({this:?})");
        Ok(false)
    }

    #[method(public)]
    async fn mark(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, read_ahead_limit: i32) -> Result<()> {
        tracing::debug!("java.io.Reader::mark({this:?}, {read_ahead_limit})");
        Err(jvm.exception("java/io/IOException", "mark not supported").await)
    }

    #[method(public)]
    async fn reset(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.Reader::reset({this:?})");
        Err(jvm.exception("java/io/IOException", "reset not supported").await)
    }

    #[method(public)]
    async fn close(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()>;
}
