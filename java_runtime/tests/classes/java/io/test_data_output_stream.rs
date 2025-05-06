use alloc::vec;

use jvm::{Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_data_output_stream() -> Result<()> {
    let jvm = test_jvm().await?;

    let stream = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?;
    let data_output_stream = jvm
        .new_class("java/io/DataOutputStream", "(Ljava/io/OutputStream;)V", (stream.clone(),))
        .await?;

    let _: () = jvm.invoke_virtual(&data_output_stream, "write", "(I)V", (1,)).await?;
    let string = JavaLangString::from_rust_string(&jvm, "hello, world").await?;
    let _: () = jvm
        .invoke_virtual(&data_output_stream, "writeChars", "(Ljava/lang/String;)V", (string,))
        .await?;
    let _: () = jvm.invoke_virtual(&data_output_stream, "writeInt", "(I)V", (12341234,)).await?;
    let _: () = jvm.invoke_virtual(&data_output_stream, "writeLong", "(J)V", (123412341324i64,)).await?;

    let bytes = jvm.invoke_virtual(&stream, "toByteArray", "()[B", ()).await?;

    let length = jvm.array_length(&bytes).await?;
    let mut buf = vec![0; length];
    jvm.array_raw_buffer(&bytes).await?.read(0, &mut buf)?;

    assert_eq!(
        buf,
        vec![
            1, b'h', b'e', b'l', b'l', b'o', b',', b' ', b'w', b'o', b'r', b'l', b'd', 0, 0xbc, 0x4f, 0xf2, 0, 0, 0, 0x1c, 0xbb, 0xf2, 0xe2, 0x4c
        ]
    );

    Ok(())
}

#[tokio::test]
async fn test_data_output_stream_utf() -> Result<()> {
    let jvm = test_jvm().await?;

    let stream = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?;
    let data_output_stream = jvm
        .new_class("java/io/DataOutputStream", "(Ljava/io/OutputStream;)V", (stream.clone(),))
        .await?;

    let string = JavaLangString::from_rust_string(&jvm, "hello, world").await?;
    let _: () = jvm
        .invoke_virtual(&data_output_stream, "writeUTF", "(Ljava/lang/String;)V", (string,))
        .await?;

    let bytes = jvm.invoke_virtual(&stream, "toByteArray", "()[B", ()).await?;

    let length = jvm.array_length(&bytes).await?;
    let mut buf = vec![0; length];
    jvm.array_raw_buffer(&bytes).await?.read(0, &mut buf)?;

    assert_eq!(buf, vec![0, 12, b'h', b'e', b'l', b'l', b'o', b',', b' ', b'w', b'o', b'r', b'l', b'd']);

    Ok(())
}
