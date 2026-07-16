use alloc::{vec, vec::Vec};

use java_runtime::classes::java::lang::Object;
use jvm::{ClassInstanceRef, JavaError, Jvm, Result, runtime::JavaLangString};

use test_utils::test_jvm;

async fn object_to_optional_string(jvm: &Jvm, value: &ClassInstanceRef<Object>) -> Result<Option<String>> {
    if value.is_null() {
        return Ok(None);
    }

    Ok(Some(JavaLangString::to_rust_string(jvm, value).await?))
}

#[tokio::test]
async fn test_hashtable_cldc_legacy_api() -> Result<()> {
    let jvm = test_jvm().await?;
    let hashtable = jvm.new_class("java/util/Hashtable", "(I)V", (1,)).await?;
    let key = JavaLangString::from_rust_string(&jvm, "key").await?;
    let value = JavaLangString::from_rust_string(&jvm, "value").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &hashtable,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (key.clone(), value.clone()),
        )
        .await?;

    assert!(
        jvm.invoke_virtual::<_, bool>(&hashtable, "contains", "(Ljava/lang/Object;)Z", (value,))
            .await?
    );
    let keys: ClassInstanceRef<Object> = jvm.invoke_virtual(&hashtable, "keys", "()Ljava/util/Enumeration;", ()).await?;
    let enumerated_key: ClassInstanceRef<Object> = jvm.invoke_virtual(&keys, "nextElement", "()Ljava/lang/Object;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &enumerated_key).await?, "key");
    let elements: ClassInstanceRef<Object> = jvm.invoke_virtual(&hashtable, "elements", "()Ljava/util/Enumeration;", ()).await?;
    let enumerated_value: ClassInstanceRef<Object> = jvm.invoke_virtual(&elements, "nextElement", "()Ljava/lang/Object;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &enumerated_value).await?, "value");

    let _: () = jvm.invoke_virtual(&hashtable, "rehash", "()V", ()).await?;
    let found: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&hashtable, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (key,))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &found).await?, "value");
    let text: ClassInstanceRef<Object> = jvm.invoke_virtual(&hashtable, "toString", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "{key=value}");

    let result = jvm.new_class("java/util/Hashtable", "(I)V", (-1,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("negative capacity must throw IllegalArgumentException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalArgumentException"));

    Ok(())
}

async fn iterator_to_optional_strings(jvm: &Jvm, iterator: &ClassInstanceRef<Object>) -> Result<Vec<Option<String>>> {
    let mut values = Vec::new();
    loop {
        let has_next: bool = jvm.invoke_virtual(iterator, "hasNext", "()Z", ()).await?;
        if !has_next {
            break;
        }

        let value: ClassInstanceRef<Object> = jvm.invoke_virtual(iterator, "next", "()Ljava/lang/Object;", ()).await?;
        values.push(object_to_optional_string(jvm, &value).await?);
    }

    Ok(values)
}

async fn assert_next_throws_no_such_element(jvm: &Jvm, iterator: &ClassInstanceRef<Object>) -> Result<()> {
    let result: Result<ClassInstanceRef<Object>> = jvm.invoke_virtual(iterator, "next", "()Ljava/lang/Object;", ()).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Expected JavaException, got {:?}", result);
    };
    assert!(jvm.is_instance(&*exception, "java/util/NoSuchElementException"));

    Ok(())
}

async fn assert_remove_throws_unsupported(jvm: &Jvm, iterator: &ClassInstanceRef<Object>) -> Result<()> {
    let result: Result<()> = jvm.invoke_virtual(iterator, "remove", "()V", ()).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Expected JavaException, got {:?}", result);
    };
    assert!(jvm.is_instance(&*exception, "java/lang/UnsupportedOperationException"));

    Ok(())
}

async fn assert_null_pointer_exception<T>(jvm: &Jvm, result: Result<T>) -> Result<()> {
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Expected JavaException, got non-exception result");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    Ok(())
}

fn sorted_optional_strings(mut values: Vec<Option<String>>) -> Vec<Option<String>> {
    values.sort();
    values
}

#[tokio::test]
async fn test_hashtable_map_views_and_legacy_operations() -> Result<()> {
    let jvm = test_jvm().await?;

    let hashtable = jvm.new_class("java/util/Hashtable", "()V", ()).await?;
    assert!(jvm.is_instance(&*hashtable, "java/util/Map"));
    assert!(jvm.is_instance(&*hashtable, "java/util/Dictionary"));

    let size: i32 = jvm.invoke_virtual(&hashtable, "size", "()I", ()).await?;
    assert_eq!(size, 0);
    let is_empty: bool = jvm.invoke_virtual(&hashtable, "isEmpty", "()Z", ()).await?;
    assert!(is_empty);

    let test_key = JavaLangString::from_rust_string(&jvm, "testKey").await?;
    let test_value = JavaLangString::from_rust_string(&jvm, "testValue").await?;
    let second_key = JavaLangString::from_rust_string(&jvm, "secondKey").await?;
    let second_value = JavaLangString::from_rust_string(&jvm, "secondValue").await?;

    let old: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &hashtable,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (test_key.clone(), test_value.clone()),
        )
        .await?;
    assert!(old.is_null());
    let old: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &hashtable,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (second_key.clone(), second_value.clone()),
        )
        .await?;
    assert!(old.is_null());

    let size: i32 = jvm.invoke_virtual(&hashtable, "size", "()I", ()).await?;
    assert_eq!(size, 2);
    let is_empty: bool = jvm.invoke_virtual(&hashtable, "isEmpty", "()Z", ()).await?;
    assert!(!is_empty);

    let value = jvm
        .invoke_virtual(&hashtable, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (test_key.clone(),))
        .await?;

    let value_string = JavaLangString::to_rust_string(&jvm, &value).await?;
    assert_eq!(value_string, "testValue");

    let test_key_second = JavaLangString::from_rust_string(&jvm, "testKey").await?;

    let value = jvm
        .invoke_virtual(&hashtable, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (test_key_second.clone(),))
        .await?;

    let value_string = JavaLangString::to_rust_string(&jvm, &value).await?;
    assert_eq!(value_string, "testValue");

    let contains_key: bool = jvm
        .invoke_virtual(&hashtable, "containsKey", "(Ljava/lang/Object;)Z", (test_key_second.clone(),))
        .await?;
    assert!(contains_key);
    let contains_value: bool = jvm
        .invoke_virtual(
            &hashtable,
            "containsValue",
            "(Ljava/lang/Object;)Z",
            (JavaLangString::from_rust_string(&jvm, "testValue").await?,),
        )
        .await?;
    assert!(contains_value);

    let key_set: ClassInstanceRef<Object> = jvm.invoke_virtual(&hashtable, "keySet", "()Ljava/util/Set;", ()).await?;
    assert!(jvm.is_instance(&**key_set, "java/util/Set"));
    assert!(jvm.is_instance(&**key_set, "java/util/Collection"));
    assert!(jvm.is_instance(&**key_set, "java/util/AbstractSet"));
    let key_set_size: i32 = jvm.invoke_virtual(&key_set, "size", "()I", ()).await?;
    assert_eq!(key_set_size, 2);
    let contains_key: bool = jvm
        .invoke_virtual(&key_set, "contains", "(Ljava/lang/Object;)Z", (test_key_second.clone(),))
        .await?;
    assert!(contains_key);
    let missing_key = JavaLangString::from_rust_string(&jvm, "missing").await?;
    let contains_key: bool = jvm
        .invoke_virtual(&key_set, "contains", "(Ljava/lang/Object;)Z", (missing_key.clone(),))
        .await?;
    assert!(!contains_key);
    let key_iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&key_set, "iterator", "()Ljava/util/Iterator;", ()).await?;
    assert_eq!(key_iterator.class_definition().name(), "java/util/Hashtable$Enumerator");
    assert!(jvm.is_instance(&**key_iterator, "java/util/Iterator"));
    let key_snapshot = sorted_optional_strings(iterator_to_optional_strings(&jvm, &key_iterator).await?);
    assert_eq!(key_snapshot, vec![Some("secondKey".to_string()), Some("testKey".to_string())]);
    assert_next_throws_no_such_element(&jvm, &key_iterator).await?;
    assert_remove_throws_unsupported(&jvm, &key_iterator).await?;

    let values: ClassInstanceRef<Object> = jvm.invoke_virtual(&hashtable, "values", "()Ljava/util/Collection;", ()).await?;
    assert!(jvm.is_instance(&**values, "java/util/Collection"));
    assert!(jvm.is_instance(&**values, "java/util/AbstractCollection"));
    let values_size: i32 = jvm.invoke_virtual(&values, "size", "()I", ()).await?;
    assert_eq!(values_size, 2);
    let contains_value: bool = jvm
        .invoke_virtual(
            &values,
            "contains",
            "(Ljava/lang/Object;)Z",
            (JavaLangString::from_rust_string(&jvm, "secondValue").await?,),
        )
        .await?;
    assert!(contains_value);
    let values_iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&values, "iterator", "()Ljava/util/Iterator;", ()).await?;
    assert_eq!(values_iterator.class_definition().name(), "java/util/Hashtable$Enumerator");
    let values_snapshot = sorted_optional_strings(iterator_to_optional_strings(&jvm, &values_iterator).await?);
    assert_eq!(values_snapshot, vec![Some("secondValue".to_string()), Some("testValue".to_string())]);

    let entry_set: ClassInstanceRef<Object> = jvm.invoke_virtual(&hashtable, "entrySet", "()Ljava/util/Set;", ()).await?;
    assert!(jvm.is_instance(&**entry_set, "java/util/Set"));
    assert!(jvm.is_instance(&**entry_set, "java/util/Collection"));
    assert!(jvm.is_instance(&**entry_set, "java/util/AbstractSet"));
    let entry_set_size: i32 = jvm.invoke_virtual(&entry_set, "size", "()I", ()).await?;
    assert_eq!(entry_set_size, 2);
    let entry_iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&entry_set, "iterator", "()Ljava/util/Iterator;", ()).await?;
    assert_eq!(entry_iterator.class_definition().name(), "java/util/Hashtable$Enumerator");
    assert!(jvm.is_instance(&**entry_iterator, "java/util/Iterator"));
    let entry: ClassInstanceRef<Object> = jvm.invoke_virtual(&entry_iterator, "next", "()Ljava/lang/Object;", ()).await?;
    assert!(jvm.is_instance(&**entry, "java/util/Map$Entry"));
    let contains_entry: bool = jvm
        .invoke_virtual(&entry_set, "contains", "(Ljava/lang/Object;)Z", (entry.clone(),))
        .await?;
    assert!(contains_entry);
    let entry_key: ClassInstanceRef<Object> = jvm.invoke_virtual(&entry, "getKey", "()Ljava/lang/Object;", ()).await?;
    let entry_value: ClassInstanceRef<Object> = jvm.invoke_virtual(&entry, "getValue", "()Ljava/lang/Object;", ()).await?;
    assert!(!entry_key.is_null());
    assert!(!entry_value.is_null());

    let replacement = JavaLangString::from_rust_string(&jvm, "replacementValue").await?;
    let old_value: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&entry, "setValue", "(Ljava/lang/Object;)Ljava/lang/Object;", (replacement.clone(),))
        .await?;
    assert!(!old_value.is_null());
    let backing_value: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&hashtable, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (entry_key.clone(),))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &backing_value).await?, "replacementValue");

    let value: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&hashtable, "remove", "(Ljava/lang/Object;)Ljava/lang/Object;", (test_key.clone(),))
        .await?;

    assert!(!value.is_null());

    let value: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&hashtable, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (test_key.clone(),))
        .await?;

    assert!(value.is_null());
    let size: i32 = jvm.invoke_virtual(&hashtable, "size", "()I", ()).await?;
    assert_eq!(size, 1);

    let _: () = jvm.invoke_virtual(&hashtable, "clear", "()V", ()).await?;
    let size: i32 = jvm.invoke_virtual(&hashtable, "size", "()I", ()).await?;
    assert_eq!(size, 0);
    let is_empty: bool = jvm.invoke_virtual(&hashtable, "isEmpty", "()Z", ()).await?;
    assert!(is_empty);
    let value: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&hashtable, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (second_key.clone(),))
        .await?;
    assert!(value.is_null());
    let contains_key: bool = jvm
        .invoke_virtual(&hashtable, "containsKey", "(Ljava/lang/Object;)Z", (second_key.clone(),))
        .await?;
    assert!(!contains_key);
    let contains_value: bool = jvm
        .invoke_virtual(&hashtable, "containsValue", "(Ljava/lang/Object;)Z", (second_value.clone(),))
        .await?;
    assert!(!contains_value);

    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &hashtable,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (test_key.clone(), test_value.clone()),
        )
        .await?;
    let _: () = jvm.invoke_virtual(&key_set, "clear", "()V", ()).await?;
    let size: i32 = jvm.invoke_virtual(&hashtable, "size", "()I", ()).await?;
    assert_eq!(size, 0);

    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &hashtable,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (test_key.clone(), test_value.clone()),
        )
        .await?;
    let _: () = jvm.invoke_virtual(&values, "clear", "()V", ()).await?;
    let size: i32 = jvm.invoke_virtual(&hashtable, "size", "()I", ()).await?;
    assert_eq!(size, 0);

    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &hashtable,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (test_key.clone(), test_value),
        )
        .await?;
    let _: () = jvm.invoke_virtual(&entry_set, "clear", "()V", ()).await?;
    let size: i32 = jvm.invoke_virtual(&hashtable, "size", "()I", ()).await?;
    assert_eq!(size, 0);

    Ok(())
}

#[tokio::test]
async fn test_hashtable_legacy_null_and_snapshot_edges() -> Result<()> {
    let jvm = test_jvm().await?;

    let hashtable = jvm.new_class("java/util/Hashtable", "()V", ()).await?;
    let null_ref: ClassInstanceRef<Object> = None.into();
    let value = JavaLangString::from_rust_string(&jvm, "value").await?;

    let result: Result<ClassInstanceRef<Object>> = jvm
        .invoke_virtual(
            &hashtable,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (null_ref.clone(), value.clone()),
        )
        .await;
    assert_null_pointer_exception(&jvm, result).await?;
    let result: Result<ClassInstanceRef<Object>> = jvm
        .invoke_virtual(&hashtable, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (null_ref.clone(),))
        .await;
    assert_null_pointer_exception(&jvm, result).await?;
    let result: Result<bool> = jvm
        .invoke_virtual(&hashtable, "containsKey", "(Ljava/lang/Object;)Z", (null_ref.clone(),))
        .await;
    assert_null_pointer_exception(&jvm, result).await?;

    let null_value_key = JavaLangString::from_rust_string(&jvm, "null-value-key").await?;
    let result: Result<ClassInstanceRef<Object>> = jvm
        .invoke_virtual(
            &hashtable,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (null_value_key.clone(), null_ref.clone()),
        )
        .await;
    assert_null_pointer_exception(&jvm, result).await?;

    let old: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &hashtable,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (null_value_key.clone(), value.clone()),
        )
        .await?;
    assert!(old.is_null());

    let contains_key: bool = jvm
        .invoke_virtual(&hashtable, "containsKey", "(Ljava/lang/Object;)Z", (null_value_key.clone(),))
        .await?;
    assert!(contains_key);
    let result: Result<bool> = jvm
        .invoke_virtual(&hashtable, "containsValue", "(Ljava/lang/Object;)Z", (null_ref.clone(),))
        .await;
    assert_null_pointer_exception(&jvm, result).await?;

    let values: ClassInstanceRef<Object> = jvm.invoke_virtual(&hashtable, "values", "()Ljava/util/Collection;", ()).await?;
    let result: Result<bool> = jvm
        .invoke_virtual(&values, "contains", "(Ljava/lang/Object;)Z", (null_ref.clone(),))
        .await;
    assert_null_pointer_exception(&jvm, result).await?;

    let entry_set: ClassInstanceRef<Object> = jvm.invoke_virtual(&hashtable, "entrySet", "()Ljava/util/Set;", ()).await?;
    let entry_iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&entry_set, "iterator", "()Ljava/util/Iterator;", ()).await?;
    let entry: ClassInstanceRef<Object> = jvm.invoke_virtual(&entry_iterator, "next", "()Ljava/lang/Object;", ()).await?;
    assert!(jvm.is_instance(&**entry, "java/util/Map$Entry"));
    let result: Result<ClassInstanceRef<Object>> = jvm
        .invoke_virtual(&entry, "setValue", "(Ljava/lang/Object;)Ljava/lang/Object;", (null_ref.clone(),))
        .await;
    assert_null_pointer_exception(&jvm, result).await?;
    let replacement = JavaLangString::from_rust_string(&jvm, "replacement").await?;
    let old_value: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&entry, "setValue", "(Ljava/lang/Object;)Ljava/lang/Object;", (replacement.clone(),))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &old_value).await?, "value");
    let found: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&hashtable, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (null_value_key.clone(),))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &found).await?, "replacement");

    let key_set: ClassInstanceRef<Object> = jvm.invoke_virtual(&hashtable, "keySet", "()Ljava/util/Set;", ()).await?;
    let key_iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&key_set, "iterator", "()Ljava/util/Iterator;", ()).await?;
    let second_key = JavaLangString::from_rust_string(&jvm, "second").await?;
    let second_value = JavaLangString::from_rust_string(&jvm, "second-value").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &hashtable,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (second_key.clone(), second_value),
        )
        .await?;

    let snapshot = sorted_optional_strings(iterator_to_optional_strings(&jvm, &key_iterator).await?);
    assert_eq!(snapshot, vec![Some("null-value-key".to_string())]);
    let size: i32 = jvm.invoke_virtual(&key_set, "size", "()I", ()).await?;
    assert_eq!(size, 2);

    Ok(())
}
