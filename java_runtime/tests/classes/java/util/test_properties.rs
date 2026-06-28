use java_runtime::classes::java::lang::{Object, String};
use jvm::{ClassInstanceRef, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_properties_inherits_hashtable_map_contract() -> Result<()> {
    let jvm = test_jvm().await?;

    let properties = jvm.new_class("java/util/Properties", "()V", ()).await?;
    assert!(jvm.is_instance(&*properties, "java/util/Properties"));
    assert!(jvm.is_instance(&*properties, "java/util/Hashtable"));
    assert!(jvm.is_instance(&*properties, "java/util/Dictionary"));
    assert!(jvm.is_instance(&*properties, "java/util/Map"));

    let key = JavaLangString::from_rust_string(&jvm, "name").await?;
    let equal_key = JavaLangString::from_rust_string(&jvm, "name").await?;
    let value = JavaLangString::from_rust_string(&jvm, "value").await?;

    let old: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &properties,
            "setProperty",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
            (key.clone(), value.clone()),
        )
        .await?;
    assert!(old.is_null());

    let found: ClassInstanceRef<String> = jvm
        .invoke_virtual(&properties, "getProperty", "(Ljava/lang/String;)Ljava/lang/String;", (equal_key,))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &found).await?, "value");

    let inherited_key = JavaLangString::from_rust_string(&jvm, "inherited").await?;
    let inherited_equal_key = JavaLangString::from_rust_string(&jvm, "inherited").await?;
    let inherited_value = JavaLangString::from_rust_string(&jvm, "map-value").await?;
    let old: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &properties,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (inherited_key, inherited_value),
        )
        .await?;
    assert!(old.is_null());

    let found: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&properties, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (inherited_equal_key,))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &found).await?, "map-value");

    let size: i32 = jvm.invoke_virtual(&properties, "size", "()I", ()).await?;
    assert_eq!(size, 2);
    let key_set: ClassInstanceRef<Object> = jvm.invoke_virtual(&properties, "keySet", "()Ljava/util/Set;", ()).await?;
    assert!(jvm.is_instance(&**key_set, "java/util/Set"));

    Ok(())
}
