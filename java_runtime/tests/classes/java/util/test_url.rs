use jvm::{Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_file_url() -> Result<()> {
    let jvm = test_jvm().await?;

    let url_spec = JavaLangString::from_rust_string(&jvm, "file:test.txt").await?;
    let url = jvm.new_class("java/net/URL", "(Ljava/lang/String;)V", (url_spec,)).await?;

    let protocol = jvm.invoke_virtual(&url, "getProtocol", "()Ljava/lang/String;", ()).await?;
    let host = jvm.invoke_virtual(&url, "getHost", "()Ljava/lang/String;", ()).await?;
    let port: i32 = jvm.invoke_virtual(&url, "getPort", "()I", ()).await?;
    let file = jvm.invoke_virtual(&url, "getFile", "()Ljava/lang/String;", ()).await?;

    assert_eq!(JavaLangString::to_rust_string(&jvm, &protocol).await?, "file");
    assert_eq!(port, -1);
    assert_eq!(JavaLangString::to_rust_string(&jvm, &host).await?, "");
    assert_eq!(JavaLangString::to_rust_string(&jvm, &file).await?, "test.txt");

    Ok(())
}
