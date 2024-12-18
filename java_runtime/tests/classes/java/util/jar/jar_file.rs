use java_runtime::classes::java::util::jar::JarEntry;
use jvm::{runtime::JavaLangString, ClassInstanceRef, Result};

use test_utils::test_jvm_filesystem;

#[tokio::test]
async fn test_jar_manifest() -> Result<()> {
    let jar = include_bytes!("../../../../../../test_data/test.jar");
    let filesystem = [("test.jar".into(), jar.to_vec())].into_iter().collect();
    let jvm = test_jvm_filesystem(filesystem).await?;

    let name = JavaLangString::from_rust_string(&jvm, "test.jar").await?;
    let file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (name,)).await?;
    let jar = jvm.new_class("java/util/jar/JarFile", "(Ljava/io/File;)V", (file,)).await?;

    let manifest = jvm.invoke_virtual(&jar, "getManifest", "()Ljava/util/jar/Manifest;", ()).await?;

    let main_attributes = jvm
        .invoke_virtual(&manifest, "getMainAttributes", "()Ljava/util/jar/Attributes;", ())
        .await?;

    let key = JavaLangString::from_rust_string(&jvm, "Main-Class").await?;
    let value = jvm
        .invoke_virtual(&main_attributes, "getValue", "(Ljava/lang/String;)Ljava/lang/String;", (key,))
        .await?;

    assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, "JarTest");

    Ok(())
}

#[tokio::test]
async fn test_entries() -> Result<()> {
    let jar = include_bytes!("../../../../../../test_data/test.jar");
    let filesystem = [("test.jar".into(), jar.to_vec())].into_iter().collect();
    let jvm = test_jvm_filesystem(filesystem).await?;

    let name = JavaLangString::from_rust_string(&jvm, "test.jar").await?;
    let jar = jvm.new_class("java/util/jar/JarFile", "(Ljava/lang/String;)V", (name,)).await?;

    let entries = jvm.invoke_virtual(&jar, "entries", "()Ljava/util/Enumeration;", ()).await?;

    assert!(jvm.invoke_virtual(&entries, "hasMoreElements", "()Z", ()).await?);
    let next_element: ClassInstanceRef<JarEntry> = jvm.invoke_virtual(&entries, "nextElement", "()Ljava/lang/Object;", ()).await?;
    let name = jvm.get_field(&next_element, "name", "Ljava/lang/String;").await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &name).await?, "META-INF/");

    assert!(jvm.invoke_virtual(&entries, "hasMoreElements", "()Z", ()).await?);
    let next_element: ClassInstanceRef<JarEntry> = jvm.invoke_virtual(&entries, "nextElement", "()Ljava/lang/Object;", ()).await?;
    let name = jvm.get_field(&next_element, "name", "Ljava/lang/String;").await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &name).await?, "META-INF/MANIFEST.MF");

    assert!(jvm.invoke_virtual(&entries, "hasMoreElements", "()Z", ()).await?);
    let next_element: ClassInstanceRef<JarEntry> = jvm.invoke_virtual(&entries, "nextElement", "()Ljava/lang/Object;", ()).await?;
    let name = jvm.get_field(&next_element, "name", "Ljava/lang/String;").await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &name).await?, "JarTest.class");

    assert!(jvm.invoke_virtual(&entries, "hasMoreElements", "()Z", ()).await?);
    let next_element: ClassInstanceRef<JarEntry> = jvm.invoke_virtual(&entries, "nextElement", "()Ljava/lang/Object;", ()).await?;
    let name = jvm.get_field(&next_element, "name", "Ljava/lang/String;").await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &name).await?, "test.txt");

    assert!(!jvm.invoke_virtual(&entries, "hasMoreElements", "()Z", ()).await?);

    Ok(())
}
