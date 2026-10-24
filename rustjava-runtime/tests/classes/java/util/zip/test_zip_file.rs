use alloc::vec;

use jvm::{ClassInstanceRef, JavaError, Result, runtime::JavaLangString};
use rustjava_runtime::classes::java::lang::Object;

use test_utils::test_jvm_filesystem;

#[tokio::test]
async fn test_zip_entry() -> Result<()> {
    let jar = include_bytes!("../../../../../../test-data/test.jar");
    let filesystem = [("test.jar".into(), jar.to_vec())].into_iter().collect();
    let jvm = test_jvm_filesystem(filesystem).await?;

    let name = JavaLangString::from_rust_string(&jvm, "test.jar").await?;
    let file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (name,)).await?;
    let zip = jvm.new_class("java/util/zip/ZipFile", "(Ljava/io/File;)V", (file,)).await?;

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
    let entry = jvm
        .invoke_virtual(
            &zip,
            &zip.class_definition().name(),
            "getEntry",
            "(Ljava/lang/String;)Ljava/util/zip/ZipEntry;",
            (entry_name,),
        )
        .await?;

    let size: i64 = jvm.invoke_virtual(&entry, &entry.class_definition().name(), "getSize", "()J", ()).await?;

    let is = jvm
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
