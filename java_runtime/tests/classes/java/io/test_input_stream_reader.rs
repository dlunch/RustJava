use alloc::{vec, vec::Vec};

use java_runtime::classes::java::lang::Object;
use jvm::{ClassInstanceRef, JavaChar, JavaError, Result, runtime::JavaLangString};

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

#[tokio::test]
async fn test_reader_default_contract_and_lifecycle() -> Result<()> {
    let jvm = test_jvm().await?;

    let mut bytes = jvm.instantiate_array("B", 3).await?;
    jvm.store_array(&mut bytes, 0, [b'a' as i8, b'b' as i8, b'c' as i8]).await?;
    let input = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (bytes,)).await?;
    let reader = jvm.new_class("java/io/InputStreamReader", "(Ljava/io/InputStream;)V", (input,)).await?;

    assert!(jvm.invoke_virtual::<_, bool>(&reader, "ready", "()Z", ()).await?);
    assert!(!jvm.invoke_special::<_, bool>(&reader, "java/io/Reader", "ready", "()Z", ()).await?);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&reader, "read", "()I", ()).await?, 'a' as i32);

    let chars = jvm.instantiate_array("C", 2).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&reader, "read", "([C)I", (chars.clone(),)).await?, 2);
    assert_eq!(jvm.load_array::<JavaChar>(&chars, 0, 2).await?, ['b' as JavaChar, 'c' as JavaChar]);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&reader, "read", "()I", ()).await?, -1);

    let empty = jvm.instantiate_array("C", 0).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&reader, "read", "([CII)I", (empty, 0, 0)).await?, 0);
    assert!(!jvm.invoke_virtual::<_, bool>(&reader, "markSupported", "()Z", ()).await?);

    let mark: Result<()> = jvm.invoke_virtual(&reader, "mark", "(I)V", (1,)).await;
    let Err(JavaError::JavaException(exception)) = mark else {
        panic!("default mark must throw IOException");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));

    let reset: Result<()> = jvm.invoke_virtual(&reader, "reset", "()V", ()).await;
    let Err(JavaError::JavaException(exception)) = reset else {
        panic!("default reset must throw IOException");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));

    let negative_skip: Result<i64> = jvm.invoke_virtual(&reader, "skip", "(J)J", (-1i64,)).await;
    let Err(JavaError::JavaException(exception)) = negative_skip else {
        panic!("negative skip must throw IllegalArgumentException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalArgumentException"));

    let mut bytes = jvm.instantiate_array("B", 3).await?;
    jvm.store_array(&mut bytes, 0, [b'x' as i8, b'y' as i8, b'z' as i8]).await?;
    let input = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (bytes,)).await?;
    let reader = jvm.new_class("java/io/InputStreamReader", "(Ljava/io/InputStream;)V", (input,)).await?;
    assert_eq!(jvm.invoke_virtual::<_, i64>(&reader, "skip", "(J)J", (2i64,)).await?, 2);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&reader, "read", "()I", ()).await?, 'z' as i32);
    assert_eq!(jvm.invoke_virtual::<_, i64>(&reader, "skip", "(J)J", (2i64,)).await?, 0);
    let _: () = jvm.invoke_virtual(&reader, "close", "()V", ()).await?;

    let null_lock: ClassInstanceRef<Object> = None.into();
    let result: Result<()> = jvm
        .invoke_special(&reader, "java/io/Reader", "<init>", "(Ljava/lang/Object;)V", (null_lock,))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("null lock must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    Ok(())
}
