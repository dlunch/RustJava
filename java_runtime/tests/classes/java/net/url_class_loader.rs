use alloc::vec;

use java_runtime::classes::java::net::URL;
use jvm::{runtime::JavaLangString, ClassInstanceRef, Result};

use test_utils::test_jvm_filesystem;

#[tokio::test]
async fn test_jar_loading() -> Result<()> {
    let jar = include_bytes!("../../../../../test_data/test.jar");
    let filesystem = [("test.jar".into(), jar.to_vec())].into_iter().collect();
    let jvm = test_jvm_filesystem(filesystem).await?;

    let url_str = JavaLangString::from_rust_string(&jvm, "file:test.jar").await?;
    let url = jvm.new_class("java/net/URL", "(Ljava/lang/String;)V", (url_str,)).await?;
    let mut urls = jvm.instantiate_array("Ljava/net/URL;", 1).await?;
    jvm.store_array(&mut urls, 0, vec![url]).await?;

    let class_loader = jvm
        .new_class("java/net/URLClassLoader", "([Ljava/net/URL;Ljava/lang/ClassLoader;)V", (urls, None))
        .await?;

    let resource_name = JavaLangString::from_rust_string(&jvm, "test.txt").await?;
    let resource = jvm
        .invoke_virtual(&class_loader, "findResource", "(Ljava/lang/String;)Ljava/net/URL;", (resource_name,))
        .await?;

    let stream = jvm.invoke_virtual(&resource, "openStream", "()Ljava/io/InputStream;", ()).await?;

    let buf = jvm.instantiate_array("B", 17).await?;
    let len: i32 = jvm.invoke_virtual(&stream, "read", "([B)I", (buf.clone(),)).await?;

    let mut data = vec![0; len as _];
    jvm.array_raw_buffer(&buf).await?.read(0, &mut data)?;

    assert_eq!(data, b"test content\n");

    Ok(())
}

#[tokio::test]
async fn test_jar_loading_with_slash() -> Result<()> {
    let jar = include_bytes!("../../../../../test_data/test.jar");
    let filesystem = [("test.jar".into(), jar.to_vec())].into_iter().collect();
    let jvm = test_jvm_filesystem(filesystem).await?;

    let url_str = JavaLangString::from_rust_string(&jvm, "file:test.jar").await?;
    let url = jvm.new_class("java/net/URL", "(Ljava/lang/String;)V", (url_str,)).await?;
    let mut urls = jvm.instantiate_array("Ljava/net/URL;", 1).await?;
    jvm.store_array(&mut urls, 0, vec![url]).await?;

    let class_loader = jvm
        .new_class("java/net/URLClassLoader", "([Ljava/net/URL;Ljava/lang/ClassLoader;)V", (urls, None))
        .await?;

    let resource_name = JavaLangString::from_rust_string(&jvm, "/test.txt").await?;
    let stream = jvm
        .invoke_virtual(
            &class_loader,
            "getResourceAsStream",
            "(Ljava/lang/String;)Ljava/io/InputStream;",
            (resource_name,),
        )
        .await?;

    let buf = jvm.instantiate_array("B", 17).await?;
    let len: i32 = jvm.invoke_virtual(&stream, "read", "([B)I", (buf.clone(),)).await?;

    let mut data = vec![0; len as _];
    jvm.array_raw_buffer(&buf).await?.read(0, &mut data)?;

    assert_eq!(data, b"test content\n");

    Ok(())
}

#[tokio::test]
async fn test_load_from_dir() -> Result<()> {
    let filesystem = [("test.txt".into(), b"test content\n".to_vec())].into_iter().collect();
    let jvm = test_jvm_filesystem(filesystem).await?;

    let url_str = JavaLangString::from_rust_string(&jvm, "file:.").await?;
    let url = jvm.new_class("java/net/URL", "(Ljava/lang/String;)V", (url_str,)).await?;
    let mut urls = jvm.instantiate_array("Ljava/net/URL;", 1).await?;
    jvm.store_array(&mut urls, 0, vec![url]).await?;

    let class_loader = jvm
        .new_class("java/net/URLClassLoader", "([Ljava/net/URL;Ljava/lang/ClassLoader;)V", (urls, None))
        .await?;

    let resource_name = JavaLangString::from_rust_string(&jvm, "test.txt").await?;
    let resource = jvm
        .invoke_virtual(&class_loader, "findResource", "(Ljava/lang/String;)Ljava/net/URL;", (resource_name,))
        .await?;

    let stream = jvm.invoke_virtual(&resource, "openStream", "()Ljava/io/InputStream;", ()).await?;

    let buf = jvm.instantiate_array("B", 17).await?;
    let len: i32 = jvm.invoke_virtual(&stream, "read", "([B)I", (buf.clone(),)).await?;

    let mut data = vec![0; len as _];
    jvm.array_raw_buffer(&buf).await?.read(0, &mut data)?;

    assert_eq!(data, b"test content\n");

    Ok(())
}

#[tokio::test]
async fn test_jar_loading_no_file() -> Result<()> {
    let jar = include_bytes!("../../../../../test_data/test.jar");
    let filesystem = [("test.jar".into(), jar.to_vec())].into_iter().collect();
    let jvm = test_jvm_filesystem(filesystem).await?;

    let url_str = JavaLangString::from_rust_string(&jvm, "file:test.jar").await?;
    let url = jvm.new_class("java/net/URL", "(Ljava/lang/String;)V", (url_str,)).await?;
    let mut urls = jvm.instantiate_array("Ljava/net/URL;", 1).await?;
    jvm.store_array(&mut urls, 0, vec![url]).await?;

    let class_loader = jvm
        .new_class("java/net/URLClassLoader", "([Ljava/net/URL;Ljava/lang/ClassLoader;)V", (urls, None))
        .await?;

    let resource_name = JavaLangString::from_rust_string(&jvm, "does_not_exists.txt").await?;
    let resource: ClassInstanceRef<URL> = jvm
        .invoke_virtual(&class_loader, "findResource", "(Ljava/lang/String;)Ljava/net/URL;", (resource_name,))
        .await?;

    assert!(resource.is_null());

    Ok(())
}

#[tokio::test]
async fn test_load_from_dir_no_file() -> Result<()> {
    let filesystem = [("test.txt".into(), b"test content\n".to_vec())].into_iter().collect();
    let jvm = test_jvm_filesystem(filesystem).await?;

    let url_str = JavaLangString::from_rust_string(&jvm, "file:.").await?;
    let url = jvm.new_class("java/net/URL", "(Ljava/lang/String;)V", (url_str,)).await?;
    let mut urls = jvm.instantiate_array("Ljava/net/URL;", 1).await?;
    jvm.store_array(&mut urls, 0, vec![url]).await?;

    let class_loader = jvm
        .new_class("java/net/URLClassLoader", "([Ljava/net/URL;Ljava/lang/ClassLoader;)V", (urls, None))
        .await?;

    let resource_name = JavaLangString::from_rust_string(&jvm, "does_not_exists.txt").await?;
    let resource: ClassInstanceRef<URL> = jvm
        .invoke_virtual(&class_loader, "findResource", "(Ljava/lang/String;)Ljava/net/URL;", (resource_name,))
        .await?;
    assert!(resource.is_null());

    Ok(())
}
