use alloc::{vec, vec::Vec};

use jvm::{JavaChar, Result};

use test_utils::test_jvm;

#[tokio::test]
async fn test_isr() -> Result<()> {
    let jvm = test_jvm().await?;

    let mut buffer = jvm.instantiate_array("B", 11).await?;
    jvm.array_raw_buffer_mut(&mut buffer).await?.write(0, b"Hello\nWorld")?;

    let is = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (buffer,)).await?;
    let isr = jvm.new_class("java/io/InputStreamReader", "(Ljava/io/InputStream;)V", (is,)).await?;

    let buf = jvm.instantiate_array("C", 10).await?;
    let read: i32 = jvm.invoke_virtual(&isr, "read", "([CII)I", (buf.clone(), 0, 5)).await?;

    assert_eq!(read, 5);
    let buf_data: Vec<JavaChar> = jvm.load_array(&buf, 0, 5).await?;
    assert_eq!(buf_data, vec![72, 101, 108, 108, 111]);

    let read: i32 = jvm.invoke_virtual(&isr, "read", "([CII)I", (buf.clone(), 0, 6)).await?;

    assert_eq!(read, 6);
    let buf_data: Vec<JavaChar> = jvm.load_array(&buf, 0, 6).await?;
    assert_eq!(buf_data, vec![10, 87, 111, 114, 108, 100]);

    Ok(())
}
