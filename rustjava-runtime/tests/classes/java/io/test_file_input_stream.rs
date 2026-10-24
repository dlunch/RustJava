use alloc::vec;

use jvm::{Result, runtime::JavaLangString};

use test_utils::test_jvm_filesystem;

#[tokio::test]
async fn file_streams_reject_null_arguments() -> Result<()> {
    use jvm::{JavaError, JavaValue};

    let jvm = test_jvm_filesystem([("test.txt".into(), b"hello".to_vec())].into_iter().collect()).await?;
    for class in ["java/io/FileInputStream", "java/io/FileOutputStream"] {
        for descriptor in ["(Ljava/io/File;)V", "(Ljava/io/FileDescriptor;)V"] {
            let result = jvm.new_class(class, descriptor, [JavaValue::Object(None)]).await;
            let Err(JavaError::JavaException(exception)) = result else {
                panic!("{class}{descriptor} must reject null");
            };
            assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));
        }
    }
    let result = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", [JavaValue::Object(None)]).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("File must reject a null path");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let result = jvm
        .new_class("java/io/FileOutputStream", "(Ljava/io/File;Z)V", [JavaValue::Object(None), true.into()])
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("append constructor must reject a null file");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let path = JavaLangString::from_rust_string(&jvm, "test.txt").await?;
    let file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (path,)).await?;
    let input = jvm.new_class("java/io/FileInputStream", "(Ljava/io/File;)V", (file.clone(),)).await?;
    let descriptor = jvm.new_class("java/io/FileDescriptor", "()V", ()).await?;
    let output = jvm
        .new_class("java/io/FileOutputStream", "(Ljava/io/FileDescriptor;)V", (descriptor,))
        .await?;
    for (stream, class, method, descriptor) in [
        (&input, "java/io/FileInputStream", "read", "([BII)I"),
        (&output, "java/io/FileOutputStream", "write", "([BII)V"),
    ] {
        for length in [0, 1] {
            let result: Result<JavaValue> = jvm
                .invoke_virtual(stream, class, method, descriptor, [JavaValue::Object(None), 0.into(), length.into()])
                .await;
            let Err(JavaError::JavaException(exception)) = result else {
                panic!("{class}.{method} must reject null");
            };
            assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));
        }
    }
    let first: i32 = jvm.invoke_virtual(&input, "java/io/FileInputStream", "read", "()I", ()).await?;
    assert_eq!(first, b'h' as i32);
    Ok(())
}

#[tokio::test]
async fn unavailable_standard_streams_have_nonnull_descriptors() -> Result<()> {
    use jvm::{ClassInstanceRef, JavaError};
    use rustjava_runtime::classes::java::io::FileDescriptor;

    let jvm = test_jvm_filesystem(Default::default()).await?;
    for (name, class, method, signature) in [
        ("in", "java/io/FileInputStream", "read", "()I"),
        ("out", "java/io/FileOutputStream", "write", "(I)V"),
        ("err", "java/io/FileOutputStream", "write", "(I)V"),
    ] {
        let descriptor: ClassInstanceRef<FileDescriptor> = jvm.get_static_field("java/io/FileDescriptor", name, "Ljava/io/FileDescriptor;").await?;
        assert!(!descriptor.is_null(), "{name}");
        let stream = jvm.new_class(class, "(Ljava/io/FileDescriptor;)V", (descriptor,)).await?;
        let args = if name == "in" { vec![] } else { vec![1.into()] };
        let result: Result<jvm::JavaValue> = jvm.invoke_virtual(&stream, class, method, signature, args).await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("unavailable {name} must report IOException");
        };
        assert!(jvm.is_instance(&*exception, "java/io/IOException"));
    }
    Ok(())
}

#[tokio::test]
async fn test_file_input_stream_read_buf() -> Result<()> {
    let filesystem = [("test.txt".into(), b"hello world".to_vec())];
    let jvm = test_jvm_filesystem(filesystem.into_iter().collect()).await?;

    let file = JavaLangString::from_rust_string(&jvm, "test.txt").await?;

    let java_file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (file,)).await?;
    let fis = jvm.new_class("java/io/FileInputStream", "(Ljava/io/File;)V", (java_file,)).await?;

    let bytes = jvm.instantiate_array("B", 11).await?;
    let read: i32 = jvm
        .invoke_virtual(&fis, &fis.class_definition().name(), "read", "([B)I", (bytes.clone(),))
        .await?;

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

    let read: i32 = jvm.invoke_virtual(&fis, &fis.class_definition().name(), "read", "()I", ()).await?;

    assert_eq!(read, 104);

    Ok(())
}

#[tokio::test]
async fn test_file_input_stream_available() -> Result<()> {
    let filesystem = [("test.txt".into(), b"hello world".to_vec())];
    let jvm = test_jvm_filesystem(filesystem.into_iter().collect()).await?;

    let file = JavaLangString::from_rust_string(&jvm, "test.txt").await?;

    let java_file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (file,)).await?;
    let fis = jvm.new_class("java/io/FileInputStream", "(Ljava/io/File;)V", (java_file,)).await?;

    let available: i32 = jvm.invoke_virtual(&fis, &fis.class_definition().name(), "available", "()I", ()).await?;

    assert!(available > 0);

    Ok(())
}

#[tokio::test]
async fn test_file_input_stream_sequential_read() -> Result<()> {
    let filesystem = [("test.txt".into(), b"hello".to_vec())];
    let jvm = test_jvm_filesystem(filesystem.into_iter().collect()).await?;

    let file = JavaLangString::from_rust_string(&jvm, "test.txt").await?;

    let java_file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (file,)).await?;
    let fis = jvm.new_class("java/io/FileInputStream", "(Ljava/io/File;)V", (java_file,)).await?;

    let mut reads = vec![];
    for _ in 0..6 {
        reads.push(
            jvm.invoke_virtual::<_, i32>(&fis, &fis.class_definition().name(), "read", "()I", ())
                .await?,
        );
    }

    assert_eq!(reads, vec![104, 101, 108, 108, 111, -1]);

    Ok(())
}

#[tokio::test]
async fn test_file_input_stream_skip_past_eof() -> Result<()> {
    let filesystem = [("test.txt".into(), b"hello".to_vec())];
    let jvm = test_jvm_filesystem(filesystem.into_iter().collect()).await?;

    let file = JavaLangString::from_rust_string(&jvm, "test.txt").await?;

    let java_file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (file,)).await?;
    let fis = jvm.new_class("java/io/FileInputStream", "(Ljava/io/File;)V", (java_file,)).await?;

    let skipped: i64 = jvm.invoke_virtual(&fis, &fis.class_definition().name(), "skip", "(J)J", (10i64,)).await?;
    assert_eq!(skipped, 5);

    let read: i32 = jvm.invoke_virtual(&fis, &fis.class_definition().name(), "read", "()I", ()).await?;
    assert_eq!(read, -1);

    Ok(())
}
