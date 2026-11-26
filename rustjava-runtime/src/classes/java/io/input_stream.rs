use jvm::{Array, ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::RuntimeContext;

// abstract class java.io.InputStream
#[java_class(name = "java/io/InputStream", extends = "java/lang/Object", abstract)]
pub struct InputStream;

#[java_methods]
impl InputStream {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.InputStream::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    #[method(public)]
    async fn read(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, b: ClassInstanceRef<Array<i8>>) -> Result<i32> {
        tracing::debug!("java.io.InputStream::read({this:?}, {b:?})");

        if b.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "buffer is null").await);
        }
        let array_length = jvm.array_length(&b).await? as i32;

        jvm.invoke_virtual(&this, "java/io/InputStream", "read", "([BII)I", (b, 0, array_length))
            .await
    }

    #[method(name = "read", public)]
    async fn read_offset(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        mut b: ClassInstanceRef<Array<i8>>,
        off: i32,
        len: i32,
    ) -> Result<i32> {
        tracing::debug!("java.io.InputStream::read({this:?}, {b:?}, {off}, {len})");

        if b.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "buffer is null").await);
        }
        let array_length = jvm.array_length(&b).await? as i32;
        if off < 0 || len < 0 || off > array_length - len {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "Invalid offset or length").await);
        }
        if len == 0 {
            return Ok(0);
        }

        let first: i32 = jvm.invoke_virtual(&this, "java/io/InputStream", "read", "()I", ()).await?;
        if first == -1 {
            return Ok(-1);
        }
        jvm.store_array(&mut b, off as usize, [first as i8]).await?;

        let mut count = 1;
        while count < len {
            let value: i32 = jvm.invoke_virtual(&this, "java/io/InputStream", "read", "()I", ()).await?;
            if value == -1 {
                break;
            }
            jvm.store_array(&mut b, (off + count) as usize, [value as i8]).await?;
            count += 1;
        }

        Ok(count)
    }

    #[method(public)]
    async fn available(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.io.InputStream::available({this:?})");
        Ok(0)
    }

    #[method(name = "read", public)]
    async fn read_byte(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32>;

    #[method(public)]
    async fn close(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.InputStream::close({this:?})");
        Ok(())
    }

    #[method(public)]
    async fn skip(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, n: i64) -> Result<i64> {
        tracing::debug!("java.io.InputStream::skip({this:?}, {n:?})");

        if n <= 0 {
            return Ok(0);
        }

        let scratch_size = n.min(4096);
        let scratch = jvm.instantiate_array("B", scratch_size as _).await?;

        let mut remaining = n;
        while remaining > 0 {
            let len_to_read = remaining.min(scratch_size) as i32;
            let read: i32 = jvm
                .invoke_virtual(&this, "java/io/InputStream", "read", "([BII)I", (scratch.clone(), 0, len_to_read))
                .await?;
            if read <= 0 {
                break;
            }

            remaining -= read as i64;
        }

        Ok(n - remaining)
    }

    #[method(public)]
    async fn mark(_jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, readlimit: i32) -> Result<()> {
        tracing::debug!("java.io.InputStream::mark({this:?}, {readlimit:?})");

        Ok(())
    }

    #[method(public)]
    async fn reset(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.InputStream::reset({this:?})");

        Err(jvm.exception("java/io/IOException", "reset not supported").await)
    }

    #[method(public)]
    async fn mark_supported(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.io.InputStream::markSupported({this:?})");
        Ok(false)
    }
}
