use jvm::{ClassInstanceRef, Result, runtime::JavaLangString};
use rustjava_runtime::classes::java::lang::{Object, String};

use test_utils::test_jvm;

#[tokio::test]
async fn test_attribute_get_set() -> Result<()> {
    let jvm = test_jvm().await?;

    let attributes = jvm.new_class("java/util/jar/Attributes", "()V", ()).await?;
    assert!(jvm.is_instance(&*attributes, "java/util/Map"));
    let entries = jvm
        .invoke_virtual(&attributes, "java/util/Map", "entrySet", "()Ljava/util/Set;", ())
        .await?;
    let size: i32 = jvm.invoke_virtual(&entries, "java/util/Set", "size", "()I", ()).await?;
    assert_eq!(size, 0);

    let name = JavaLangString::from_rust_string(&jvm, "Name").await?;
    let value = JavaLangString::from_rust_string(&jvm, "Value").await?;

    let old: ClassInstanceRef<String> = jvm
        .invoke_virtual(
            &attributes,
            &attributes.class_definition().name(),
            "putValue",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
            (name.clone(), value),
        )
        .await?;
    assert!(old.is_null());

    let value = jvm
        .invoke_virtual(
            &attributes,
            &attributes.class_definition().name(),
            "getValue",
            "(Ljava/lang/String;)Ljava/lang/String;",
            (name,),
        )
        .await?;

    let value = JavaLangString::to_rust_string(&jvm, &value).await?;
    assert_eq!(value, "Value");

    let lowercase = JavaLangString::from_rust_string(&jvm, "name").await?;
    let replacement = JavaLangString::from_rust_string(&jvm, "Replacement").await?;
    let old = jvm
        .invoke_virtual(
            &attributes,
            "java/util/jar/Attributes",
            "putValue",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
            (lowercase.clone(), replacement),
        )
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &old).await?, "Value");
    let size: i32 = jvm.invoke_virtual(&entries, "java/util/Set", "size", "()I", ()).await?;
    assert_eq!(size, 1);

    let iterator = jvm
        .invoke_virtual(&entries, "java/util/Set", "iterator", "()Ljava/util/Iterator;", ())
        .await?;
    let entry = jvm
        .invoke_virtual(&iterator, "java/util/Iterator", "next", "()Ljava/lang/Object;", ())
        .await?;
    let key: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&entry, "java/util/Map$Entry", "getKey", "()Ljava/lang/Object;", ())
        .await?;
    assert!(jvm.is_instance(&**key, "java/util/jar/Attributes$Name"));
    let key_string = jvm
        .invoke_virtual(&key, "java/lang/Object", "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &key_string).await?, "Name");
    let value = jvm
        .invoke_virtual(
            &attributes,
            "java/util/jar/Attributes",
            "getValue",
            "(Ljava/util/jar/Attributes$Name;)Ljava/lang/String;",
            (key,),
        )
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, "Replacement");

    let updated = JavaLangString::from_rust_string(&jvm, "Updated").await?;
    let old = jvm
        .invoke_virtual(
            &entry,
            "java/util/Map$Entry",
            "setValue",
            "(Ljava/lang/Object;)Ljava/lang/Object;",
            (updated,),
        )
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &old).await?, "Replacement");
    let value = jvm
        .invoke_virtual(
            &attributes,
            "java/util/jar/Attributes",
            "getValue",
            "(Ljava/lang/String;)Ljava/lang/String;",
            (lowercase.clone(),),
        )
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, "Updated");

    let _: () = jvm.invoke_virtual(&iterator, "java/util/Iterator", "remove", "()V", ()).await?;
    let value: ClassInstanceRef<String> = jvm
        .invoke_virtual(
            &attributes,
            "java/util/jar/Attributes",
            "getValue",
            "(Ljava/lang/String;)Ljava/lang/String;",
            (lowercase,),
        )
        .await?;
    assert!(value.is_null());
    let size: i32 = jvm.invoke_virtual(&entries, "java/util/Set", "size", "()I", ()).await?;
    assert_eq!(size, 0);

    Ok(())
}

#[tokio::test]
async fn test_attribute_name_validation() -> Result<()> {
    let jvm = test_jvm().await?;
    for spelling in ["a", "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_-", &"A".repeat(70)] {
        let name = JavaLangString::from_rust_string(&jvm, spelling).await?;
        let name = jvm.new_class("java/util/jar/Attributes$Name", "(Ljava/lang/String;)V", (name,)).await?;
        let text = jvm
            .invoke_virtual(&name, "java/util/jar/Attributes$Name", "toString", "()Ljava/lang/String;", ())
            .await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, spelling);
    }
    for spelling in ["", "a b", "\u{e9}", &"A".repeat(71)] {
        let name = JavaLangString::from_rust_string(&jvm, spelling).await?;
        let result = jvm.new_class("java/util/jar/Attributes$Name", "(Ljava/lang/String;)V", (name,)).await;
        let Err(jvm::JavaError::JavaException(exception)) = result else {
            panic!("expected IllegalArgumentException for {spelling:?}");
        };
        assert!(jvm.is_instance(exception.as_ref(), "java/lang/IllegalArgumentException"));
    }
    let result = jvm
        .new_class(
            "java/util/jar/Attributes$Name",
            "(Ljava/lang/String;)V",
            (ClassInstanceRef::<String>::new(None),),
        )
        .await;
    let Err(jvm::JavaError::JavaException(exception)) = result else {
        panic!("expected NullPointerException");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/NullPointerException"));
    Ok(())
}

#[tokio::test]
async fn test_attribute_name_equality_and_hash() -> Result<()> {
    let jvm = test_jvm().await?;
    let spelling = JavaLangString::from_rust_string(&jvm, "MiXeD-Name_123").await?;
    let name = jvm
        .new_class("java/util/jar/Attributes$Name", "(Ljava/lang/String;)V", (spelling.clone(),))
        .await?;
    let hash: i32 = jvm.invoke_virtual(&name, "java/util/jar/Attributes$Name", "hashCode", "()I", ()).await?;
    for (text, expected) in [("mixed-name_123", true), ("Other", false)] {
        let text = JavaLangString::from_rust_string(&jvm, text).await?;
        let other = jvm.new_class("java/util/jar/Attributes$Name", "(Ljava/lang/String;)V", (text,)).await?;
        let equal: bool = jvm
            .invoke_virtual(
                &name,
                "java/util/jar/Attributes$Name",
                "equals",
                "(Ljava/lang/Object;)Z",
                (other.clone(),),
            )
            .await?;
        assert_eq!(equal, expected);
        if equal {
            let other_hash: i32 = jvm.invoke_virtual(&other, "java/util/jar/Attributes$Name", "hashCode", "()I", ()).await?;
            assert_eq!(hash, other_hash);
        }
    }
    for other in [ClassInstanceRef::<Object>::from(spelling), ClassInstanceRef::new(None)] {
        let equal: bool = jvm
            .invoke_virtual(&name, "java/util/jar/Attributes$Name", "equals", "(Ljava/lang/Object;)Z", (other,))
            .await?;
        assert!(!equal);
    }
    Ok(())
}

#[tokio::test]
async fn test_attribute_name_constants() -> Result<()> {
    let jvm = test_jvm().await?;
    for (field, expected) in [
        ("MANIFEST_VERSION", "Manifest-Version"),
        ("SIGNATURE_VERSION", "Signature-Version"),
        ("CONTENT_TYPE", "Content-Type"),
        ("CLASS_PATH", "Class-Path"),
        ("MAIN_CLASS", "Main-Class"),
        ("SEALED", "Sealed"),
        ("EXTENSION_LIST", "Extension-List"),
        ("EXTENSION_NAME", "Extension-Name"),
        ("EXTENSION_INSTALLATION", "Extension-Installation"),
        ("IMPLEMENTATION_TITLE", "Implementation-Title"),
        ("IMPLEMENTATION_VERSION", "Implementation-Version"),
        ("IMPLEMENTATION_VENDOR", "Implementation-Vendor"),
        ("IMPLEMENTATION_VENDOR_ID", "Implementation-Vendor-Id"),
        ("IMPLEMENTATION_URL", "Implementation-URL"),
        ("SPECIFICATION_TITLE", "Specification-Title"),
        ("SPECIFICATION_VERSION", "Specification-Version"),
        ("SPECIFICATION_VENDOR", "Specification-Vendor"),
    ] {
        let name: ClassInstanceRef<Object> = jvm
            .get_static_field("java/util/jar/Attributes$Name", field, "Ljava/util/jar/Attributes$Name;")
            .await?;
        assert!(!name.is_null(), "{field}");
        assert!(jvm.is_instance(&**name, "java/util/jar/Attributes$Name"));
        let text = jvm
            .invoke_virtual(&name, "java/util/jar/Attributes$Name", "toString", "()Ljava/lang/String;", ())
            .await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, expected);
    }
    Ok(())
}
