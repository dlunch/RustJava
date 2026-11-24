use alloc::vec;

use jvm::{ClassInstance, ClassInstanceRef, JavaError, Result, runtime::JavaLangString};
use rustjava_runtime::classes::java::lang::Object;

use test_utils::{TestRuntime, create_test_jvm, test_jvm_filesystem};

#[tokio::test]
async fn test_zip_entry() -> Result<()> {
    let jar = include_bytes!("../../../../../../test-data/test.jar");
    let filesystem = [("test.jar".into(), jar.to_vec())].into_iter().collect();
    let runtime = TestRuntime::new(filesystem);
    let jvm = create_test_jvm(runtime.clone()).await?;

    let name = JavaLangString::from_rust_string(&jvm, "test.jar").await?;
    let file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (name,)).await?;
    let zip = jvm.new_class("java/util/zip/ZipFile", "(Ljava/io/File;)V", (file,)).await?;
    assert_eq!(runtime.open_file_count(), 0);

    for (method, descriptor) in [
        ("getEntry", "(Ljava/lang/String;)Ljava/util/zip/ZipEntry;"),
        ("getInputStream", "(Ljava/util/zip/ZipEntry;)Ljava/io/InputStream;"),
    ] {
        let result: Result<ClassInstanceRef<Object>> = jvm
            .invoke_virtual(
                &zip,
                "java/util/zip/ZipFile",
                method,
                descriptor,
                (ClassInstanceRef::<Object>::new(None),),
            )
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("{method}(null) must throw NullPointerException");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));
    }

    let entry_name = JavaLangString::from_rust_string(&jvm, "test.txt").await?;
    let entry: Box<dyn ClassInstance> = jvm
        .invoke_virtual(
            &zip,
            &zip.class_definition().name(),
            "getEntry",
            "(Ljava/lang/String;)Ljava/util/zip/ZipEntry;",
            (entry_name,),
        )
        .await?;

    let size: i64 = jvm.invoke_virtual(&entry, &entry.class_definition().name(), "getSize", "()J", ()).await?;

    let is: Box<dyn ClassInstance> = jvm
        .invoke_virtual(
            &zip,
            &zip.class_definition().name(),
            "getInputStream",
            "(Ljava/util/zip/ZipEntry;)Ljava/io/InputStream;",
            (entry,),
        )
        .await?;

    let buf = jvm.instantiate_array("B", size as _).await?;
    let _: i32 = jvm
        .invoke_virtual(&is, &is.class_definition().name(), "read", "([B)I", (buf.clone(),))
        .await?;

    let mut data = vec![0; size as _];
    jvm.array_raw_buffer(&buf).await?.read(0, &mut data).unwrap();
    assert_eq!(data, b"test content\n".to_vec());

    Ok(())
}

#[tokio::test]
async fn test_zip_null_constructor_arguments() -> Result<()> {
    let jvm = test_utils::test_jvm().await?;
    for (class, descriptor) in [
        ("java/util/zip/ZipEntry", "(Ljava/lang/String;)V"),
        ("java/util/zip/ZipEntry", "(Ljava/util/zip/ZipEntry;)V"),
        ("java/util/zip/ZipFile", "(Ljava/io/File;)V"),
    ] {
        let result = jvm.new_class(class, descriptor, (ClassInstanceRef::<Object>::new(None),)).await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("{class}{descriptor} must reject null");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));
    }
    Ok(())
}

#[tokio::test]
async fn test_zip_entry_offset_past_end() -> Result<()> {
    let mut jar = include_bytes!("../../../../../../test-data/test.jar").to_vec();
    // Point the central directory entry of test.txt at a local header beyond the archive end.
    let name_offset = jar.windows(8).rposition(|x| x == b"test.txt").unwrap();
    let header_offset = name_offset - 46;
    assert_eq!(&jar[header_offset..header_offset + 4], b"PK\x01\x02");
    jar[header_offset + 42..header_offset + 46].copy_from_slice(&0x7fff_0000u32.to_le_bytes());

    let filesystem = [("test.jar".into(), jar)].into_iter().collect();
    let jvm = test_jvm_filesystem(filesystem).await?;

    let name = JavaLangString::from_rust_string(&jvm, "test.jar").await?;
    let file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (name,)).await?;
    let zip = jvm.new_class("java/util/zip/ZipFile", "(Ljava/io/File;)V", (file,)).await?;

    let entry_name = JavaLangString::from_rust_string(&jvm, "test.txt").await?;
    let entry: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &zip,
            "java/util/zip/ZipFile",
            "getEntry",
            "(Ljava/lang/String;)Ljava/util/zip/ZipEntry;",
            (entry_name,),
        )
        .await?;
    assert!(entry.is_null());

    Ok(())
}
