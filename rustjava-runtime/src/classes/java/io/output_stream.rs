use alloc::vec;

use jvm::{Array, ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::RuntimeContext;

// abstract class java.io.OutputStream
#[java_class(
    name = "java/io/OutputStream",
    extends = "java/lang/Object",
    implements("java/io/Closeable", "java/io/Flushable"),
    public,
    abstract
)]
pub struct OutputStream;

#[java_methods]
impl OutputStream {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.OutputStream::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    #[method(name = "write", public)]
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

    #[method(name = "write", public)]
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

    #[method(public)]
    async fn write(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i32) -> Result<()>;

    #[method(public)]
    async fn flush(_jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.OutputStream::flush({this:?})");

        Ok(())
    }

    #[method(public)]
    async fn close(_jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.OutputStream::close({this:?})");

        Ok(())
    }
}
