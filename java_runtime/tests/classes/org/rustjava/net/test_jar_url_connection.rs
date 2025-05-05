use alloc::vec;

use java_runtime::classes::java::util::jar::JarFile;
use jvm::{ClassInstanceRef, Result, runtime::JavaLangString};

use test_utils::test_jvm_filesystem;

#[tokio::test]
async fn test_jar_entry() -> Result<()> {
    let jar = include_bytes!("../../../../../../test_data/test.jar");
    let filesystem = [("test.jar".into(), jar.to_vec())].into_iter().collect();
    let jvm = test_jvm_filesystem(filesystem).await?;

    let url_spec = JavaLangString::from_rust_string(&jvm, "jar:file:test.jar!/test.txt").await?;
    let url = jvm.new_class("java/net/URL", "(Ljava/lang/String;)V", (url_spec,)).await?;

    let connection = jvm.invoke_virtual(&url, "openConnection", "()Ljava/net/URLConnection;", ()).await?;

    let stream = jvm.invoke_virtual(&connection, "getInputStream", "()Ljava/io/InputStream;", ()).await?;

    let buf = jvm.instantiate_array("B", 17).await?;
    let len: i32 = jvm.invoke_virtual(&stream, "read", "([B)I", (buf.clone(),)).await?;

    let mut data = vec![0; len as usize];
    jvm.array_raw_buffer(&buf).await?.read(0, &mut data).unwrap();

    assert_eq!(data, b"test content\n");

    Ok(())
}

#[tokio::test]
async fn test_jar_file() -> Result<()> {
    let jar = include_bytes!("../../../../../../test_data/test.jar");
    let filesystem = [("test.jar".into(), jar.to_vec())].into_iter().collect();
    let jvm = test_jvm_filesystem(filesystem).await?;

    let url_spec = JavaLangString::from_rust_string(&jvm, "jar:file:test.jar!/").await?;
    let url = jvm.new_class("java/net/URL", "(Ljava/lang/String;)V", (url_spec,)).await?;

    let connection = jvm.invoke_virtual(&url, "openConnection", "()Ljava/net/URLConnection;", ()).await?;

    let attributes = jvm
        .invoke_virtual(&connection, "getMainAttributes", "()Ljava/util/jar/Attributes;", ())
        .await?;

    let key = JavaLangString::from_rust_string(&jvm, "Main-Class").await?;
    let value = jvm
        .invoke_virtual(&attributes, "getValue", "(Ljava/lang/String;)Ljava/lang/String;", (key,))
        .await?;

    assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, "JarTest");

    Ok(())
}

#[tokio::test]
async fn test_jar_cache() -> Result<()> {
    let jar = include_bytes!("../../../../../../test_data/test.jar");
    let filesystem = [("test.jar".into(), jar.to_vec())].into_iter().collect();
    let jvm = test_jvm_filesystem(filesystem).await?;

    let url_spec = JavaLangString::from_rust_string(&jvm, "jar:file:test.jar!/").await?;
    let url = jvm.new_class("java/net/URL", "(Ljava/lang/String;)V", (url_spec,)).await?;

    let connection = jvm.invoke_virtual(&url, "openConnection", "()Ljava/net/URLConnection;", ()).await?;

    let jar_file = jvm.invoke_virtual(&connection, "getJarFile", "()Ljava/util/jar/JarFile;", ()).await?;
    let jar_file2: ClassInstanceRef<JarFile> = jvm.invoke_virtual(&connection, "getJarFile", "()Ljava/util/jar/JarFile;", ()).await?;

    let equals: bool = jvm.invoke_virtual(&jar_file, "equals", "(Ljava/lang/Object;)Z", (jar_file2,)).await?;

    assert!(equals);

    Ok(())
}
