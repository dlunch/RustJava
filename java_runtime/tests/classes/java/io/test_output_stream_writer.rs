use alloc::vec::Vec;

use jvm::{Array, ClassInstanceRef, JavaError, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_output_stream_writer_utf8() -> Result<()> {
    let jvm = test_jvm().await?;

    let output = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?;
    let encoding = JavaLangString::from_rust_string(&jvm, "UTF-8").await?;
    let writer = jvm
        .new_class(
            "java/io/OutputStreamWriter",
            "(Ljava/io/OutputStream;Ljava/lang/String;)V",
            (output.clone(), encoding),
        )
        .await?;

    let value = JavaLangString::from_rust_string(&jvm, "A한😀").await?;
    let _: () = jvm.invoke_virtual(&writer, "write", "(Ljava/lang/String;)V", (value,)).await?;
    let _: () = jvm.invoke_virtual(&writer, "flush", "()V", ()).await?;

    let bytes: ClassInstanceRef<Array<i8>> = jvm.invoke_virtual(&output, "toByteArray", "()[B", ()).await?;
    let length = jvm.array_length(&bytes).await?;
    let actual: Vec<i8> = jvm.load_array(&bytes, 0, length).await?;
    assert_eq!(actual, "A한😀".as_bytes().iter().map(|value| *value as i8).collect::<Vec<_>>());

    Ok(())
}

#[tokio::test]
async fn test_output_stream_writer_preserves_surrogate_across_writes() -> Result<()> {
    let jvm = test_jvm().await?;
    let output = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?;
    let writer = jvm
        .new_class("java/io/OutputStreamWriter", "(Ljava/io/OutputStream;)V", (output.clone(),))
        .await?;

    let _: () = jvm.invoke_virtual(&writer, "write", "(I)V", (0xd83d,)).await?;
    let _: () = jvm.invoke_virtual(&writer, "write", "(I)V", (0xde00,)).await?;
    let _: () = jvm.invoke_virtual(&writer, "close", "()V", ()).await?;

    let bytes: ClassInstanceRef<Array<i8>> = jvm.invoke_virtual(&output, "toByteArray", "()[B", ()).await?;
    let actual: Vec<i8> = jvm.load_array(&bytes, 0, jvm.array_length(&bytes).await?).await?;
    assert_eq!(actual, "😀".as_bytes().iter().map(|value| *value as i8).collect::<Vec<_>>());

    let output = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?;
    let writer = jvm
        .new_class("java/io/OutputStreamWriter", "(Ljava/io/OutputStream;)V", (output.clone(),))
        .await?;
    let _: () = jvm.invoke_virtual(&writer, "write", "(I)V", (0xd83d,)).await?;
    let _: () = jvm.invoke_virtual(&writer, "close", "()V", ()).await?;

    let bytes: ClassInstanceRef<Array<i8>> = jvm.invoke_virtual(&output, "toByteArray", "()[B", ()).await?;
    assert_eq!(jvm.load_array::<i8>(&bytes, 0, jvm.array_length(&bytes).await?).await?, [b'?' as i8]);

    Ok(())
}

#[tokio::test]
async fn test_output_stream_writer_rejects_unknown_encoding() -> Result<()> {
    let jvm = test_jvm().await?;

    let output = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?;
    let encoding = JavaLangString::from_rust_string(&jvm, "not-an-encoding").await?;
    let result = jvm
        .new_class(
            "java/io/OutputStreamWriter",
            "(Ljava/io/OutputStream;Ljava/lang/String;)V",
            (output, encoding),
        )
        .await;

    let Err(JavaError::JavaException(exception)) = result else {
        panic!("unknown encoding must throw UnsupportedEncodingException");
    };
    assert!(jvm.is_instance(&*exception, "java/io/UnsupportedEncodingException"));

    Ok(())
}
