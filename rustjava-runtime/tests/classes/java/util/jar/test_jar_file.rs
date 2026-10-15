use std::io::{Cursor, Write};

use jvm::{ClassInstanceRef, Result, runtime::JavaLangString};
use rustjava_runtime::classes::java::util::{
    jar::{JarEntry, Manifest},
    zip::ZipEntry,
};
use zip::{ZipWriter, write::SimpleFileOptions};

use test_utils::test_jvm_filesystem;

#[tokio::test]
async fn test_jar_manifest() -> Result<()> {
    for (names, expected) in [
        (&["META-INF/MANIFEST.MF"][..], Some("META-INF/MANIFEST.MF")),
        (&["meta-inf/manifest.mf"][..], Some("meta-inf/manifest.mf")),
        (&["other.txt", "MeTa-InF/MaNiFeSt.Mf"][..], Some("MeTa-InF/MaNiFeSt.Mf")),
        (&["other.txt"][..], None),
        (&["meta-inf/manifest.mf", "META-INF/MANIFEST.MF"][..], Some("META-INF/MANIFEST.MF")),
    ] {
        let mut archive = ZipWriter::new(Cursor::new(Vec::new()));
        for name in names {
            archive.start_file(*name, SimpleFileOptions::default()).unwrap();
            write!(archive, "Manifest-Version: 1.0\r\nMain-Class: {name}\r\n\r\n").unwrap();
        }
        let filesystem = [("test.jar".into(), archive.finish().unwrap().into_inner())].into_iter().collect();
        let jvm = test_jvm_filesystem(filesystem).await?;

        let name = JavaLangString::from_rust_string(&jvm, "test.jar").await?;
        let file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (name,)).await?;
        let jar = jvm.new_class("java/util/jar/JarFile", "(Ljava/io/File;)V", (file,)).await?;
        let manifest: ClassInstanceRef<Manifest> = jvm
            .invoke_virtual(&jar, "java/util/jar/JarFile", "getManifest", "()Ljava/util/jar/Manifest;", ())
            .await?;

        if let Some(expected) = expected {
            let attributes = jvm
                .invoke_virtual(
                    &manifest,
                    "java/util/jar/Manifest",
                    "getMainAttributes",
                    "()Ljava/util/jar/Attributes;",
                    (),
                )
                .await?;
            let key = JavaLangString::from_rust_string(&jvm, "Main-Class").await?;
            let value = jvm
                .invoke_virtual(
                    &attributes,
                    "java/util/jar/Attributes",
                    "getValue",
                    "(Ljava/lang/String;)Ljava/lang/String;",
                    (key,),
                )
                .await?;
            assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, expected);
        } else {
            assert!(manifest.is_null());
        }

        let canonical = JavaLangString::from_rust_string(&jvm, "META-INF/MANIFEST.MF").await?;
        let entry: ClassInstanceRef<JarEntry> = jvm
            .invoke_virtual(
                &jar,
                "java/util/jar/JarFile",
                "getJarEntry",
                "(Ljava/lang/String;)Ljava/util/jar/JarEntry;",
                (canonical.clone(),),
            )
            .await?;
        assert_eq!(entry.is_null(), !names.contains(&"META-INF/MANIFEST.MF"));
        let entry: ClassInstanceRef<ZipEntry> = jvm
            .invoke_virtual(
                &jar,
                "java/util/zip/ZipFile",
                "getEntry",
                "(Ljava/lang/String;)Ljava/util/zip/ZipEntry;",
                (canonical,),
            )
            .await?;
        assert_eq!(entry.is_null(), !names.contains(&"META-INF/MANIFEST.MF"));
    }

    Ok(())
}

#[tokio::test]
async fn test_entries() -> Result<()> {
    let jar = include_bytes!("../../../../../../test-data/test.jar");
    let filesystem = [("test.jar".into(), jar.to_vec())].into_iter().collect();
    let jvm = test_jvm_filesystem(filesystem).await?;

    let name = JavaLangString::from_rust_string(&jvm, "test.jar").await?;
    let jar = jvm.new_class("java/util/jar/JarFile", "(Ljava/lang/String;)V", (name,)).await?;

    let entries = jvm
        .invoke_virtual(&jar, &jar.class_definition().name(), "entries", "()Ljava/util/Enumeration;", ())
        .await?;

    let has_more_elements: bool = jvm
        .invoke_virtual(&entries, &entries.class_definition().name(), "hasMoreElements", "()Z", ())
        .await?;

    assert!(has_more_elements);
    let next_element: ClassInstanceRef<JarEntry> = jvm
        .invoke_virtual(&entries, &entries.class_definition().name(), "nextElement", "()Ljava/lang/Object;", ())
        .await?;
    let name = jvm
        .get_field(&next_element, "java/util/jar/JarEntry", "name", "Ljava/lang/String;")
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &name).await?, "META-INF/");

    let has_more_elements: bool = jvm
        .invoke_virtual(&entries, &entries.class_definition().name(), "hasMoreElements", "()Z", ())
        .await?;
    assert!(has_more_elements);
    let next_element: ClassInstanceRef<JarEntry> = jvm
        .invoke_virtual(&entries, &entries.class_definition().name(), "nextElement", "()Ljava/lang/Object;", ())
        .await?;
    let name = jvm
        .get_field(&next_element, "java/util/jar/JarEntry", "name", "Ljava/lang/String;")
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &name).await?, "META-INF/MANIFEST.MF");

    let has_more_elements: bool = jvm
        .invoke_virtual(&entries, &entries.class_definition().name(), "hasMoreElements", "()Z", ())
        .await?;
    assert!(has_more_elements);

    let next_element: ClassInstanceRef<JarEntry> = jvm
        .invoke_virtual(&entries, &entries.class_definition().name(), "nextElement", "()Ljava/lang/Object;", ())
        .await?;
    let name = jvm
        .get_field(&next_element, "java/util/jar/JarEntry", "name", "Ljava/lang/String;")
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &name).await?, "JarTest.class");

    let has_more_elements: bool = jvm
        .invoke_virtual(&entries, &entries.class_definition().name(), "hasMoreElements", "()Z", ())
        .await?;
    assert!(has_more_elements);

    let next_element: ClassInstanceRef<JarEntry> = jvm
        .invoke_virtual(&entries, &entries.class_definition().name(), "nextElement", "()Ljava/lang/Object;", ())
        .await?;
    let name = jvm
        .get_field(&next_element, "java/util/jar/JarEntry", "name", "Ljava/lang/String;")
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &name).await?, "test.txt");

    let has_more_elements: bool = jvm
        .invoke_virtual(&entries, &entries.class_definition().name(), "hasMoreElements", "()Z", ())
        .await?;
    assert!(!has_more_elements);

    Ok(())
}
