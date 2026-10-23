use jvm::{Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_jar_filename() -> Result<()> {
    let jvm = test_jvm().await?;

    let url = JavaLangString::from_rust_string(&jvm, "jar:file:path/to/%ED%95%9C%EA%B8%80.jar!/path/to/%ED%95%9C%EA%B8%80%20+%2520").await?;
    let url = jvm.new_class("java/net/URL", "(Ljava/lang/String;)V", (url,)).await?;

    let connection = jvm.new_class("org/rustjava/net/JarURLConnection", "(Ljava/net/URL;)V", (url,)).await?;

    let jar_file_url = jvm
        .invoke_virtual(
            &connection,
            &connection.class_definition().name(),
            "getJarFileURL",
            "()Ljava/net/URL;",
            (),
        )
        .await?;
    let file = jvm
        .invoke_virtual(
            &jar_file_url,
            &jar_file_url.class_definition().name(),
            "getFile",
            "()Ljava/lang/String;",
            (),
        )
        .await?;
    let protocol = jvm
        .invoke_virtual(
            &jar_file_url,
            &jar_file_url.class_definition().name(),
            "getProtocol",
            "()Ljava/lang/String;",
            (),
        )
        .await?;

    assert_eq!(JavaLangString::to_rust_string(&jvm, &file).await?, "path/to/%ED%95%9C%EA%B8%80.jar");
    assert_eq!(JavaLangString::to_rust_string(&jvm, &protocol).await?, "file");

    let entry_name = jvm
        .invoke_virtual(
            &connection,
            &connection.class_definition().name(),
            "getEntryName",
            "()Ljava/lang/String;",
            (),
        )
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &entry_name).await?, "path/to/\u{d55c}\u{ae00} +%20");

    let invalid = JavaLangString::from_rust_string(&jvm, "jar:file:test.jar!/%FF").await?;
    let invalid = jvm.new_class("java/net/URL", "(Ljava/lang/String;)V", (invalid,)).await?;
    let result = jvm.new_class("org/rustjava/net/JarURLConnection", "(Ljava/net/URL;)V", (invalid,)).await;
    let Err(jvm::JavaError::JavaException(exception)) = result else {
        panic!("expected IllegalArgumentException for invalid UTF-8");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/IllegalArgumentException"));

    Ok(())
}
