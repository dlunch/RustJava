use java_runtime::classes::java::{lang::Object, util::HashMapEntry};
use jvm::{Array, ClassInstanceRef, JavaError, Result, runtime::JavaLangString};

use test_utils::test_jvm;

async fn object_to_optional_string(jvm: &jvm::Jvm, value: &ClassInstanceRef<Object>) -> Result<Option<String>> {
    if value.is_null() {
        return Ok(None);
    }

    Ok(Some(JavaLangString::to_rust_string(jvm, value).await?))
}

async fn object_array_to_optional_strings(jvm: &jvm::Jvm, array: &ClassInstanceRef<Array<Object>>) -> Result<Vec<Option<String>>> {
    let len = jvm.array_length(array).await?;
    let values: Vec<ClassInstanceRef<Object>> = if len == 0 { Vec::new() } else { jvm.load_array(array, 0, len).await? };
    let mut strings = Vec::with_capacity(values.len());
    for value in values {
        strings.push(object_to_optional_string(jvm, &value).await?);
    }

    Ok(strings)
}

async fn iterator_to_optional_strings(jvm: &jvm::Jvm, iterator: &ClassInstanceRef<Object>) -> Result<Vec<Option<String>>> {
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

async fn assert_next_throws_no_such_element(jvm: &jvm::Jvm, iterator: &ClassInstanceRef<Object>) -> Result<()> {
    let result: Result<ClassInstanceRef<Object>> = jvm.invoke_virtual(iterator, "next", "()Ljava/lang/Object;", ()).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Expected JavaException, got {:?}", result);
    };
    assert!(jvm.is_instance(&*exception, "java/util/NoSuchElementException"));

    Ok(())
}

async fn assert_remove_throws_unsupported(jvm: &jvm::Jvm, iterator: &ClassInstanceRef<Object>) -> Result<()> {
    let result: Result<()> = jvm.invoke_virtual(iterator, "remove", "()V", ()).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Expected JavaException, got {:?}", result);
    };
    assert!(jvm.is_instance(&*exception, "java/lang/UnsupportedOperationException"));

    Ok(())
}

fn sorted_optional_strings(mut values: Vec<Option<String>>) -> Vec<Option<String>> {
    values.sort();
    values
}

#[tokio::test]
async fn test_hash_set_add_duplicate_contains_remove_null_and_clear() -> Result<()> {
    let jvm = test_jvm().await?;

    let hash_set = jvm.new_class("java/util/HashSet", "()V", ()).await?;
    assert!(jvm.is_instance(&*hash_set, "java/util/Set"));
    assert!(jvm.is_instance(&*hash_set, "java/util/Collection"));
    assert!(jvm.is_instance(&*hash_set, "java/util/AbstractSet"));
    assert!(!jvm.is_instance(&*hash_set, "java/util/List"));

    let present: ClassInstanceRef<Object> = jvm.get_field(&hash_set, "present", "Ljava/lang/Object;").await?;
    assert!(!present.is_null());

    let value = JavaLangString::from_rust_string(&jvm, "same").await?;
    let equal_value = JavaLangString::from_rust_string(&jvm, "same").await?;
    let missing = JavaLangString::from_rust_string(&jvm, "missing").await?;
    let null_ref: ClassInstanceRef<Object> = None.into();

    let added: bool = jvm.invoke_virtual(&hash_set, "add", "(Ljava/lang/Object;)Z", (value,)).await?;
    assert!(added);
    let added: bool = jvm
        .invoke_virtual(&hash_set, "add", "(Ljava/lang/Object;)Z", (equal_value.clone(),))
        .await?;
    assert!(!added);

    let contains: bool = jvm
        .invoke_virtual(&hash_set, "contains", "(Ljava/lang/Object;)Z", (equal_value.clone(),))
        .await?;
    assert!(contains);
    let contains: bool = jvm
        .invoke_virtual(&hash_set, "contains", "(Ljava/lang/Object;)Z", (missing.clone(),))
        .await?;
    assert!(!contains);

    let added_null: bool = jvm.invoke_virtual(&hash_set, "add", "(Ljava/lang/Object;)Z", (null_ref.clone(),)).await?;
    assert!(added_null);
    let added_null: bool = jvm.invoke_virtual(&hash_set, "add", "(Ljava/lang/Object;)Z", (null_ref.clone(),)).await?;
    assert!(!added_null);
    let contains_null: bool = jvm
        .invoke_virtual(&hash_set, "contains", "(Ljava/lang/Object;)Z", (null_ref.clone(),))
        .await?;
    assert!(contains_null);

    let size: i32 = jvm.invoke_virtual(&hash_set, "size", "()I", ()).await?;
    assert_eq!(size, 2);
    let is_empty: bool = jvm.invoke_virtual(&hash_set, "isEmpty", "()Z", ()).await?;
    assert!(!is_empty);

    let removed: bool = jvm
        .invoke_virtual(&hash_set, "remove", "(Ljava/lang/Object;)Z", (equal_value.clone(),))
        .await?;
    assert!(removed);
    let removed: bool = jvm.invoke_virtual(&hash_set, "remove", "(Ljava/lang/Object;)Z", (equal_value,)).await?;
    assert!(!removed);
    let removed_missing: bool = jvm.invoke_virtual(&hash_set, "remove", "(Ljava/lang/Object;)Z", (missing,)).await?;
    assert!(!removed_missing);

    let removed_null: bool = jvm
        .invoke_virtual(&hash_set, "remove", "(Ljava/lang/Object;)Z", (null_ref.clone(),))
        .await?;
    assert!(removed_null);
    let removed_null: bool = jvm
        .invoke_virtual(&hash_set, "remove", "(Ljava/lang/Object;)Z", (null_ref.clone(),))
        .await?;
    assert!(!removed_null);
    let contains_null: bool = jvm
        .invoke_virtual(&hash_set, "contains", "(Ljava/lang/Object;)Z", (null_ref.clone(),))
        .await?;
    assert!(!contains_null);

    let size: i32 = jvm.invoke_virtual(&hash_set, "size", "()I", ()).await?;
    assert_eq!(size, 0);

    let _: () = jvm.invoke_virtual(&hash_set, "clear", "()V", ()).await?;
    let size: i32 = jvm.invoke_virtual(&hash_set, "size", "()I", ()).await?;
    assert_eq!(size, 0);
    let is_empty: bool = jvm.invoke_virtual(&hash_set, "isEmpty", "()Z", ()).await?;
    assert!(is_empty);
    let contains_null: bool = jvm.invoke_virtual(&hash_set, "contains", "(Ljava/lang/Object;)Z", (null_ref,)).await?;
    assert!(!contains_null);

    Ok(())
}

#[tokio::test]
async fn test_hash_set_to_array_and_iterator_snapshot_exhaustion_remove() -> Result<()> {
    let jvm = test_jvm().await?;

    let hash_set = jvm.new_class("java/util/HashSet", "()V", ()).await?;
    let first = JavaLangString::from_rust_string(&jvm, "first").await?;
    let second = JavaLangString::from_rust_string(&jvm, "second").await?;
    let third = JavaLangString::from_rust_string(&jvm, "third").await?;

    let _: bool = jvm.invoke_virtual(&hash_set, "add", "(Ljava/lang/Object;)Z", (first,)).await?;
    let _: bool = jvm.invoke_virtual(&hash_set, "add", "(Ljava/lang/Object;)Z", (second,)).await?;

    let array: ClassInstanceRef<Array<Object>> = jvm.invoke_virtual(&hash_set, "toArray", "()[Ljava/lang/Object;", ()).await?;
    let values = sorted_optional_strings(object_array_to_optional_strings(&jvm, &array).await?);
    assert_eq!(values, vec![Some("first".to_string()), Some("second".to_string())]);

    let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&hash_set, "iterator", "()Ljava/util/Iterator;", ()).await?;
    assert!(jvm.is_instance(&**iterator, "java/util/Iterator"));

    let _: bool = jvm.invoke_virtual(&hash_set, "add", "(Ljava/lang/Object;)Z", (third,)).await?;

    let snapshot = sorted_optional_strings(iterator_to_optional_strings(&jvm, &iterator).await?);
    assert_eq!(snapshot, vec![Some("first".to_string()), Some("second".to_string())]);
    assert_next_throws_no_such_element(&jvm, &iterator).await?;
    assert_remove_throws_unsupported(&jvm, &iterator).await?;

    let size: i32 = jvm.invoke_virtual(&hash_set, "size", "()I", ()).await?;
    assert_eq!(size, 3);

    let empty_hash_set = jvm.new_class("java/util/HashSet", "()V", ()).await?;
    let empty_iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&empty_hash_set, "iterator", "()Ljava/util/Iterator;", ()).await?;
    let has_next: bool = jvm.invoke_virtual(&empty_iterator, "hasNext", "()Z", ()).await?;
    assert!(!has_next);
    assert_next_throws_no_such_element(&jvm, &empty_iterator).await?;

    Ok(())
}

#[tokio::test]
async fn test_hash_set_zero_capacity_add_path() -> Result<()> {
    let jvm = test_jvm().await?;

    let hash_set = jvm.new_class("java/util/HashSet", "(I)V", (0,)).await?;
    let map: ClassInstanceRef<Object> = jvm.get_field(&hash_set, "map", "Ljava/util/HashMap;").await?;
    let table: ClassInstanceRef<Array<HashMapEntry>> = jvm.get_field(&map, "table", "[Ljava/util/HashMap$Entry;").await?;
    assert_eq!(jvm.array_length(&table).await?, 0);

    let value = JavaLangString::from_rust_string(&jvm, "zero-capacity").await?;
    let equal_value = JavaLangString::from_rust_string(&jvm, "zero-capacity").await?;

    let added: bool = jvm.invoke_virtual(&hash_set, "add", "(Ljava/lang/Object;)Z", (value,)).await?;
    assert!(added);
    let contains: bool = jvm
        .invoke_virtual(&hash_set, "contains", "(Ljava/lang/Object;)Z", (equal_value.clone(),))
        .await?;
    assert!(contains);
    let added: bool = jvm
        .invoke_virtual(&hash_set, "add", "(Ljava/lang/Object;)Z", (equal_value.clone(),))
        .await?;
    assert!(!added);

    let table: ClassInstanceRef<Array<HashMapEntry>> = jvm.get_field(&map, "table", "[Ljava/util/HashMap$Entry;").await?;
    assert!(jvm.array_length(&table).await? >= 1);

    let size: i32 = jvm.invoke_virtual(&hash_set, "size", "()I", ()).await?;
    assert_eq!(size, 1);

    let result = jvm.new_class("java/util/HashSet", "(I)V", (-1,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Expected JavaException, got {:?}", result);
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalArgumentException"));

    Ok(())
}
