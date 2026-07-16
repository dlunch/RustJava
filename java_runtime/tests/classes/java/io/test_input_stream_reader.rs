use alloc::{vec, vec::Vec};

use jvm::{JavaChar, JavaError, Result, runtime::JavaLangString};

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

#[tokio::test]
async fn test_input_stream_reader_preserves_split_multibyte_and_buffered_eof() -> Result<()> {
    let jvm = test_jvm().await?;
    let value = "123456789한";
    let mut bytes = jvm.instantiate_array("B", value.len()).await?;
    jvm.store_array(&mut bytes, 0, value.as_bytes().iter().map(|byte| *byte as i8)).await?;
    let input = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (bytes,)).await?;
    let reader = jvm.new_class("java/io/InputStreamReader", "(Ljava/io/InputStream;)V", (input,)).await?;
    let chars = jvm.instantiate_array("C", 16).await?;

    assert_eq!(jvm.invoke_virtual::<_, i32>(&reader, "read", "([CII)I", (chars.clone(), 0, 1)).await?, 1);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&reader, "read", "([CII)I", (chars.clone(), 1, 15)).await?, 9);
    let decoded: Vec<JavaChar> = jvm.load_array(&chars, 0, 10).await?;
    assert_eq!(alloc::string::String::from_utf16(&decoded).unwrap(), value);

    let invalid: Result<i32> = jvm.invoke_virtual(&reader, "read", "([CII)I", (chars, -1, 1)).await;
    let Err(JavaError::JavaException(exception)) = invalid else {
        panic!("invalid range must throw IndexOutOfBoundsException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IndexOutOfBoundsException"));

    Ok(())
}

#[tokio::test]
async fn test_input_stream_reader_rejects_unknown_encoding() -> Result<()> {
    let jvm = test_jvm().await?;

    let bytes = jvm.instantiate_array("B", 0).await?;
    let input = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (bytes,)).await?;
    let encoding = JavaLangString::from_rust_string(&jvm, "not-an-encoding").await?;
    let result = jvm
        .new_class(
            "java/io/InputStreamReader",
            "(Ljava/io/InputStream;Ljava/lang/String;)V",
            (input, encoding),
        )
        .await;

    let Err(JavaError::JavaException(exception)) = result else {
        panic!("unknown encoding must throw UnsupportedEncodingException");
    };
    assert!(jvm.is_instance(&*exception, "java/io/UnsupportedEncodingException"));

    Ok(())
}
