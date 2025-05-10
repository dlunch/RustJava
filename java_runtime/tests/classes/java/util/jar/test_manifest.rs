use jvm::{Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_manifest_parsing() -> Result<()> {
    let jvm = test_jvm().await?;

    let data = b"Main-Class: test";
    let mut bytes = jvm.instantiate_array("B", data.len() as _).await?;
    jvm.array_raw_buffer_mut(&mut bytes).await?.write(0, data).unwrap();

    let byte_array_stream = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (bytes,)).await?;
    let manifest = jvm
        .new_class("java/util/jar/Manifest", "(Ljava/io/InputStream;)V", (byte_array_stream,))
        .await?;

    let main_attributes = jvm
        .invoke_virtual(&manifest, "getMainAttributes", "()Ljava/util/jar/Attributes;", ())
        .await?;

    let key = JavaLangString::from_rust_string(&jvm, "Main-Class").await?;
    let value = jvm
        .invoke_virtual(&main_attributes, "getValue", "(Ljava/lang/String;)Ljava/lang/String;", (key,))
        .await?;

    assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, "test");

    Ok(())
}
