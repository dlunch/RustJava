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

async fn set_file_encoding(jvm: &jvm::Jvm, encoding: &str) -> Result<()> {
    let key = JavaLangString::from_rust_string(jvm, "file.encoding").await?;
    let value = JavaLangString::from_rust_string(jvm, encoding).await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/lang/System",
            "setProperty",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
            (key, value),
        )
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_isr_iso_8859_1() -> Result<()> {
    let jvm = test_jvm().await?;

    set_file_encoding(&jvm, "ISO-8859-1").await?;

    let mut buffer = jvm.instantiate_array("B", 4).await?;
    jvm.array_raw_buffer_mut(&mut buffer).await?.write(0, &[0x61, 0xe9, 0xfc, 0x62])?;

    let is = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (buffer,)).await?;
    let isr = jvm.new_class("java/io/InputStreamReader", "(Ljava/io/InputStream;)V", (is,)).await?;

    let buf = jvm.instantiate_array("C", 10).await?;
    let read: i32 = jvm.invoke_virtual(&isr, "read", "([CII)I", (buf.clone(), 0, 4)).await?;

    assert_eq!(read, 4);
    let buf_data: Vec<JavaChar> = jvm.load_array(&buf, 0, 4).await?;
    assert_eq!(buf_data, vec![0x61, 0xe9, 0xfc, 0x62]);

    Ok(())
}

#[tokio::test]
async fn test_isr_unsupported_charset_throws() -> Result<()> {
    let jvm = test_jvm().await?;

    set_file_encoding(&jvm, "UTF-16").await?;

    let mut buffer = jvm.instantiate_array("B", 2).await?;
    jvm.array_raw_buffer_mut(&mut buffer).await?.write(0, b"hi")?;

    let is = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (buffer,)).await?;
    let isr = jvm.new_class("java/io/InputStreamReader", "(Ljava/io/InputStream;)V", (is,)).await?;

    let buf = jvm.instantiate_array("C", 10).await?;
    let result: Result<i32> = jvm.invoke_virtual(&isr, "read", "([CII)I", (buf, 0, 2)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Expected JavaException, got {:?}", result);
    };
    assert!(jvm.is_instance(&*exception, "java/io/UnsupportedEncodingException"));
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));

    Ok(())
}
