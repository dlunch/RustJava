use alloc::vec;

use jvm::{Result, runtime::JavaLangString};

use test_utils::test_jvm_filesystem;

#[tokio::test]
async fn random_access_file_rejects_null_arguments() -> Result<()> {
    use jvm::{JavaError, JavaValue};

    let jvm = test_jvm_filesystem([("test.txt".into(), b"hello".to_vec())].into_iter().collect()).await?;
    let path = JavaLangString::from_rust_string(&jvm, "test.txt").await?;
    let mode = JavaLangString::from_rust_string(&jvm, "r").await?;
    let file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (path.clone(),)).await?;
    for (signature, args) in [
        ("(Ljava/lang/String;Ljava/lang/String;)V", [JavaValue::Object(None), mode.clone().into()]),
        ("(Ljava/lang/String;Ljava/lang/String;)V", [path.clone().into(), JavaValue::Object(None)]),
        ("(Ljava/io/File;Ljava/lang/String;)V", [JavaValue::Object(None), mode.clone().into()]),
        ("(Ljava/io/File;Ljava/lang/String;)V", [file.into(), JavaValue::Object(None)]),
    ] {
        let result = jvm.new_class("java/io/RandomAccessFile", signature, args).await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("{signature} must reject null");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));
    }
    let stream = jvm
        .new_class("java/io/RandomAccessFile", "(Ljava/lang/String;Ljava/lang/String;)V", (path, mode))
        .await?;
    for (method, signature, args) in [
        ("read", "([B)I", vec![JavaValue::Object(None)]),
        ("write", "([B)V", vec![JavaValue::Object(None)]),
        ("read", "([BII)I", vec![JavaValue::Object(None), 0.into(), 0.into()]),
        ("read", "([BII)I", vec![JavaValue::Object(None), 0.into(), 1.into()]),
        ("write", "([BII)V", vec![JavaValue::Object(None), 0.into(), 0.into()]),
    ] {
        let result: Result<JavaValue> = jvm.invoke_virtual(&stream, "java/io/RandomAccessFile", method, signature, args).await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("{method}{signature} must reject null");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));
    }
    let position: i64 = jvm
        .invoke_virtual(&stream, "java/io/RandomAccessFile", "getFilePointer", "()J", ())
        .await?;
    assert_eq!(position, 0);
    Ok(())
}

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
    let read: i32 = jvm
        .invoke_virtual(&raf, &raf.class_definition().name(), "read", "([B)I", (buf.clone(),))
        .await?;
    assert_eq!(read, 11);

    let mut rust_buf = vec![0; 11];
    jvm.array_raw_buffer(&buf).await?.read(0, &mut rust_buf).unwrap();
    assert_eq!(rust_buf, vec![104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100]);

    Ok(())
}
