use alloc::vec;

use jvm::{Result, runtime::JavaLangString};

use test_utils::test_jvm_filesystem;

#[tokio::test]
async fn test_file_input_stream_read_buf() -> Result<()> {
    let filesystem = [("test.txt".into(), b"hello world".to_vec())];
    let jvm = test_jvm_filesystem(filesystem.into_iter().collect()).await?;

    let file = JavaLangString::from_rust_string(&jvm, "test.txt").await?;

    let java_file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (file,)).await?;
    let fis = jvm.new_class("java/io/FileInputStream", "(Ljava/io/File;)V", (java_file,)).await?;

    let bytes = jvm.instantiate_array("B", 11).await?;
    let read: i32 = jvm.invoke_virtual(&fis, "read", "([B)I", (bytes.clone(),)).await?;

    let length = jvm.array_length(&bytes).await?;
    let mut buf = vec![0; length];
    jvm.array_raw_buffer(&bytes).await?.read(0, &mut buf)?;

    assert_eq!(read, 11);
    assert_eq!(buf, vec![104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100]);

    Ok(())
}

#[tokio::test]
async fn test_file_input_stream_read_int() -> Result<()> {
    let filesystem = [("test.txt".into(), b"hello world".to_vec())];
    let jvm = test_jvm_filesystem(filesystem.into_iter().collect()).await?;

    let file = JavaLangString::from_rust_string(&jvm, "test.txt").await?;

    let java_file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (file,)).await?;
    let fis = jvm.new_class("java/io/FileInputStream", "(Ljava/io/File;)V", (java_file,)).await?;

    let read: i32 = jvm.invoke_virtual(&fis, "read", "()I", ()).await?;

    assert_eq!(read, 104);

    Ok(())
}
