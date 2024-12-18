use alloc::vec;

use bytemuck::cast_vec;

use jvm::Result;

use test_utils::test_jvm;

#[tokio::test]
async fn test_byte_array_output_stream() -> Result<()> {
    let jvm = test_jvm().await?;

    let stream = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?;
    let _: () = jvm.invoke_virtual(&stream, "write", "(I)V", (b'H' as i32,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "write", "(I)V", (b'e' as i32,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "write", "(I)V", (b'l' as i32,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "write", "(I)V", (b'l' as i32,)).await?;
    let _: () = jvm.invoke_virtual(&stream, "write", "(I)V", (b'o' as i32,)).await?;

    let buf = jvm.invoke_virtual(&stream, "toByteArray", "()[B", ()).await?;

    let mut bytes = vec![0; 5];
    jvm.array_raw_buffer(&buf).await?.read(0, &mut bytes)?;

    assert_eq!(bytes, cast_vec(b"Hello".to_vec()));

    Ok(())
}
