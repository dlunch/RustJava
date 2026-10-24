use alloc::vec;

use jvm::{Result, runtime::JavaLangString};

use test_utils::test_jvm_filesystem;

#[tokio::test]
async fn test_null_url_path() -> Result<()> {
    let jvm = test_jvm_filesystem(Default::default()).await?;
    for method in ["encode", "decode"] {
        let result: Result<jvm::ClassInstanceRef<rustjava_runtime::classes::java::lang::String>> = jvm
            .invoke_static("org/rustjava/net/URLPath", method, "(Ljava/lang/String;)Ljava/lang/String;", (None,))
            .await;
        let Err(jvm::JavaError::JavaException(exception)) = result else {
            panic!("expected NullPointerException for {method}(null)");
        };
        assert!(jvm.is_instance(exception.as_ref(), "java/lang/NullPointerException"));
    }
    Ok(())
}

#[tokio::test]
async fn test_file_url() -> Result<()> {
    let filesystem = [("test.txt".into(), b"test file content".to_vec())].into_iter().collect();
    let jvm = test_jvm_filesystem(filesystem).await?;

    let url_spec = JavaLangString::from_rust_string(&jvm, "file:test.txt").await?;
    let url = jvm.new_class("java/net/URL", "(Ljava/lang/String;)V", (url_spec,)).await?;

    let stream = jvm
        .invoke_virtual(&url, &url.class_definition().name(), "openStream", "()Ljava/io/InputStream;", ())
        .await?;

    let buf = jvm.instantiate_array("B", 17).await?;
    let len: i32 = jvm
        .invoke_virtual(&stream, &stream.class_definition().name(), "read", "([B)I", (buf.clone(),))
        .await?;

    let mut data = vec![0; len as usize];
    jvm.array_raw_buffer(&buf).await?.read(0, &mut data).unwrap();

    assert_eq!(data, b"test file content");

    Ok(())
}
