use alloc::vec;

use jvm::{Result, runtime::JavaLangString};

use test_utils::test_jvm_filesystem;

#[tokio::test]
async fn test_random_access_file() -> Result<()> {
    let filesystem = [("test.txt".into(), b"hello world".to_vec())];
    let jvm = test_jvm_filesystem(filesystem.into_iter().collect()).await?;

    let file = JavaLangString::from_rust_string(&jvm, "test.txt").await?;
    let mode = JavaLangString::from_rust_string(&jvm, "r").await?;

    let raf = jvm
        .new_class("java/io/RandomAccessFile", "(Ljava/lang/String;Ljava/lang/String;)V", (file, mode))
        .await?;

    let buf = jvm.instantiate_array("B", 11).await?;
    let read: i32 = jvm.invoke_virtual(&raf, "read", "([B)I", (buf.clone(),)).await?;
    assert_eq!(read, 11);

    let mut rust_buf = vec![0; 11];
    jvm.array_raw_buffer(&buf).await?.read(0, &mut rust_buf).unwrap();
    assert_eq!(rust_buf, vec![104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100]);

    Ok(())
}
