use jvm::{runtime::JavaLangString, Result};

use test_utils::test_jvm;

#[tokio::test]
async fn test_jar_filename() -> Result<()> {
    let jvm = test_jvm().await?;

    let url = JavaLangString::from_rust_string(&jvm, "jar:file:path/to/file.jar!/path/to/entry").await?;
    let url = jvm.new_class("java/net/URL", "(Ljava/lang/String;)V", (url,)).await?;

    let connection = jvm.new_class("org/rustjava/net/JarURLConnection", "(Ljava/net/URL;)V", (url,)).await?;

    let jar_file_url = jvm.invoke_virtual(&connection, "getJarFileURL", "()Ljava/net/URL;", ()).await?;
    let file = jvm.invoke_virtual(&jar_file_url, "getFile", "()Ljava/lang/String;", ()).await?;
    let protocol = jvm.invoke_virtual(&jar_file_url, "getProtocol", "()Ljava/lang/String;", ()).await?;

    assert_eq!(JavaLangString::to_rust_string(&jvm, &file).await?, "path/to/file.jar");
    assert_eq!(JavaLangString::to_rust_string(&jvm, &protocol).await?, "file");

    let entry_name = jvm.invoke_virtual(&connection, "getEntryName", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &entry_name).await?, "path/to/entry");

    Ok(())
}
