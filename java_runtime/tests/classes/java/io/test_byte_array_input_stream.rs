use jvm::Result;

use test_utils::test_jvm;

#[tokio::test]
async fn test_mark_reset() -> Result<()> {
    let jvm = test_jvm().await?;

    let mut buffer = jvm.instantiate_array("B", 5).await?;
    jvm.array_raw_buffer_mut(&mut buffer).await?.write(0, &[10, 20, 30, 40, 50])?;

    let stream = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (buffer,)).await?;

    let first: i32 = jvm.invoke_virtual(&stream, "read", "()I", ()).await?;
    assert_eq!(first, 10);

    let _: () = jvm.invoke_virtual(&stream, "mark", "(I)V", (100,)).await?;

    let second: i32 = jvm.invoke_virtual(&stream, "read", "()I", ()).await?;
    assert_eq!(second, 20);

    let _: () = jvm.invoke_virtual(&stream, "reset", "()V", ()).await?;

    let again: i32 = jvm.invoke_virtual(&stream, "read", "()I", ()).await?;
    assert_eq!(again, 20);

    Ok(())
}
