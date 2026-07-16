use java_runtime::classes::java::lang::Object;
use jvm::{Array, ClassInstanceRef, JavaError, JavaValue, Jvm, Result, runtime::JavaLangString};

use test_utils::test_jvm;

async fn assert_index_out_of_bounds_message<T>(jvm: &Jvm, result: Result<T>, expected_message: &str) -> Result<()> {
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Expected JavaException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IndexOutOfBoundsException"));

    let exception_string: ClassInstanceRef<Object> = jvm.invoke_virtual(&exception, "toString", "()Ljava/lang/String;", ()).await?;
    assert_eq!(
        JavaLangString::to_rust_string(jvm, &exception_string).await?,
        format!("java.lang.IndexOutOfBoundsException: {expected_message}")
    );

    Ok(())
}

#[tokio::test]
async fn test_vector_cldc_legacy_api() -> Result<()> {
    let jvm = test_jvm().await?;
    let vector = jvm.new_class("java/util/Vector", "(II)V", (2, 3)).await?;
    let first = JavaLangString::from_rust_string(&jvm, "first").await?;
    let second = JavaLangString::from_rust_string(&jvm, "second").await?;
    let _: () = jvm.invoke_virtual(&vector, "addElement", "(Ljava/lang/Object;)V", (first,)).await?;
    let _: () = jvm
        .invoke_virtual(&vector, "addElement", "(Ljava/lang/Object;)V", (second.clone(),))
        .await?;

    assert_eq!(jvm.invoke_virtual::<_, i32>(&vector, "capacity", "()I", ()).await?, 2);
    let _: () = jvm.invoke_virtual(&vector, "ensureCapacity", "(I)V", (5,)).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&vector, "capacity", "()I", ()).await?, 5);
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&vector, "indexOf", "(Ljava/lang/Object;I)I", (second, 1))
            .await?,
        1
    );

    let destination: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 2).await?.into();
    let _: () = jvm
        .invoke_virtual(&vector, "copyInto", "([Ljava/lang/Object;)V", (destination.clone(),))
        .await?;
    let copied: Vec<ClassInstanceRef<Object>> = jvm.load_array(&destination, 0, 2).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &copied[0]).await?, "first");
    assert_eq!(JavaLangString::to_rust_string(&jvm, &copied[1]).await?, "second");

    let elements: ClassInstanceRef<Object> = jvm.invoke_virtual(&vector, "elements", "()Ljava/util/Enumeration;", ()).await?;
    let value: ClassInstanceRef<Object> = jvm.invoke_virtual(&elements, "nextElement", "()Ljava/lang/Object;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, "first");

    let replacement = JavaLangString::from_rust_string(&jvm, "replacement").await?;
    let _: () = jvm
        .invoke_virtual(&vector, "setElementAt", "(Ljava/lang/Object;I)V", (replacement, 1))
        .await?;
    let last: ClassInstanceRef<Object> = jvm.invoke_virtual(&vector, "lastElement", "()Ljava/lang/Object;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &last).await?, "replacement");

    let _: () = jvm.invoke_virtual(&vector, "setSize", "(I)V", (4,)).await?;
    let text: ClassInstanceRef<Object> = jvm.invoke_virtual(&vector, "toString", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "[first, replacement, null, null]");

    let result = jvm.new_class("java/util/Vector", "(I)V", (-1,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("negative capacity must throw IllegalArgumentException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalArgumentException"));

    Ok(())
}

#[tokio::test]
async fn test_vector() -> Result<()> {
    let jvm = test_jvm().await?;

    let vector = jvm.new_class("java/util/Vector", "()V", ()).await?;

    let is_empty: bool = jvm.invoke_virtual(&vector, "isEmpty", "()Z", ()).await?;
    assert!(is_empty);

    let element1 = JavaLangString::from_rust_string(&jvm, "testValue1").await?;
    let element2 = JavaLangString::from_rust_string(&jvm, "testValue2").await?;

    let _: bool = jvm.invoke_virtual(&vector, "add", "(Ljava/lang/Object;)Z", (element1.clone(),)).await?;
    let _: bool = jvm.invoke_virtual(&vector, "add", "(Ljava/lang/Object;)Z", (element2.clone(),)).await?;

    let size: i32 = jvm.invoke_virtual(&vector, "size", "()I", ()).await?;
    assert_eq!(size, 2);

    let element_at1: ClassInstanceRef<Object> = jvm.invoke_virtual(&vector, "elementAt", "(I)Ljava/lang/Object;", (0,)).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &element_at1).await?, "testValue1");

    let is_empty: bool = jvm.invoke_virtual(&vector, "isEmpty", "()Z", ()).await?;
    assert!(!is_empty);

    let removed: ClassInstanceRef<Object> = jvm.invoke_virtual(&vector, "remove", "(I)Ljava/lang/Object;", (0,)).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &removed).await?, "testValue1");

    let size: i32 = jvm.invoke_virtual(&vector, "size", "()I", ()).await?;
    assert_eq!(size, 1);

    let _: () = jvm.invoke_virtual(&vector, "removeElementAt", "(I)V", (0,)).await?;

    let size: i32 = jvm.invoke_virtual(&vector, "size", "()I", ()).await?;
    assert_eq!(size, 0);

    Ok(())
}

#[tokio::test]
async fn test_vector_null() -> Result<()> {
    let jvm = test_jvm().await?;

    let vector = jvm.new_class("java/util/Vector", "()V", ()).await?;

    let _: bool = jvm.invoke_virtual(&vector, "add", "(Ljava/lang/Object;)Z", (None,)).await?;

    let size: i32 = jvm.invoke_virtual(&vector, "size", "()I", ()).await?;
    assert_eq!(size, 1);

    let element_at: ClassInstanceRef<Object> = jvm.invoke_virtual(&vector, "elementAt", "(I)Ljava/lang/Object;", (0,)).await?;
    assert!(element_at.is_null());

    Ok(())
}

#[tokio::test]
async fn test_vector_index_of() -> Result<()> {
    let jvm = test_jvm().await?;

    let vector = jvm.new_class("java/util/Vector", "()V", ()).await?;

    let element1 = JavaLangString::from_rust_string(&jvm, "testValue1").await?;
    let element2 = JavaLangString::from_rust_string(&jvm, "testValue2").await?;
    let element3 = JavaLangString::from_rust_string(&jvm, "testValue3").await?;

    let _: bool = jvm.invoke_virtual(&vector, "add", "(Ljava/lang/Object;)Z", (element1.clone(),)).await?;
    let _: bool = jvm.invoke_virtual(&vector, "add", "(Ljava/lang/Object;)Z", (element2.clone(),)).await?;
    let _: bool = jvm.invoke_virtual(&vector, "add", "(Ljava/lang/Object;)Z", (element3.clone(),)).await?;
    let _: bool = jvm.invoke_virtual(&vector, "add", "(Ljava/lang/Object;)Z", (element2.clone(),)).await?;

    let index: i32 = jvm
        .invoke_virtual(&vector, "lastIndexOf", "(Ljava/lang/Object;)I", (element2.clone(),))
        .await?;
    assert_eq!(index, 3);

    let index: i32 = jvm
        .invoke_virtual(&vector, "lastIndexOf", "(Ljava/lang/Object;)I", (element1.clone(),))
        .await?;
    assert_eq!(index, 0);

    let non_existing_element = JavaLangString::from_rust_string(&jvm, "nonExisting").await?;
    let index: i32 = jvm
        .invoke_virtual(&vector, "lastIndexOf", "(Ljava/lang/Object;)I", (non_existing_element,))
        .await?;
    assert_eq!(index, -1);

    let index: i32 = jvm
        .invoke_virtual(&vector, "lastIndexOf", "(Ljava/lang/Object;I)I", (element2.clone(), 2))
        .await?;
    assert_eq!(index, 1);

    let _: bool = jvm.invoke_virtual(&vector, "add", "(Ljava/lang/Object;)Z", (None,)).await?;
    let index: i32 = jvm.invoke_virtual(&vector, "lastIndexOf", "(Ljava/lang/Object;)I", (None,)).await?;
    assert_eq!(index, 4);

    let index: i32 = jvm.invoke_virtual(&vector, "indexOf", "(Ljava/lang/Object;)I", (element2,)).await?;
    assert_eq!(index, 1);

    let element2_same_value = JavaLangString::from_rust_string(&jvm, "testValue2").await?;
    let index: i32 = jvm
        .invoke_virtual(&vector, "indexOf", "(Ljava/lang/Object;)I", (element2_same_value.clone(),))
        .await?;
    assert_eq!(index, 1);

    let index: i32 = jvm
        .invoke_virtual(&vector, "lastIndexOf", "(Ljava/lang/Object;)I", (element2_same_value,))
        .await?;
    assert_eq!(index, 3);

    let index: i32 = jvm.invoke_virtual(&vector, "indexOf", "(Ljava/lang/Object;)I", (None,)).await?;
    assert_eq!(index, 4);

    let non_existing_element = JavaLangString::from_rust_string(&jvm, "nonExisting").await?;
    let index: i32 = jvm
        .invoke_virtual(&vector, "indexOf", "(Ljava/lang/Object;)I", (non_existing_element.clone(),))
        .await?;
    assert_eq!(index, -1);

    let index: i32 = jvm
        .invoke_virtual(&vector, "lastIndexOf", "(Ljava/lang/Object;)I", (non_existing_element,))
        .await?;
    assert_eq!(index, -1);

    Ok(())
}

#[tokio::test]
async fn test_vector_remove_element() -> Result<()> {
    let jvm = test_jvm().await?;

    let vector = jvm.new_class("java/util/Vector", "()V", ()).await?;

    let element1 = JavaLangString::from_rust_string(&jvm, "testValue1").await?;
    let element2 = JavaLangString::from_rust_string(&jvm, "testValue2").await?;
    let element3 = JavaLangString::from_rust_string(&jvm, "testValue3").await?;

    let _: bool = jvm.invoke_virtual(&vector, "add", "(Ljava/lang/Object;)Z", (element1.clone(),)).await?;
    let _: bool = jvm.invoke_virtual(&vector, "add", "(Ljava/lang/Object;)Z", (element2.clone(),)).await?;
    let _: bool = jvm.invoke_virtual(&vector, "add", "(Ljava/lang/Object;)Z", (element3.clone(),)).await?;

    let size: i32 = jvm.invoke_virtual(&vector, "size", "()I", ()).await?;
    assert_eq!(size, 3);

    let removed: bool = jvm
        .invoke_virtual(&vector, "removeElement", "(Ljava/lang/Object;)Z", (element2.clone(),))
        .await?;
    assert!(removed);

    let size: i32 = jvm.invoke_virtual(&vector, "size", "()I", ()).await?;
    assert_eq!(size, 2);

    let index: i32 = jvm
        .invoke_virtual(&vector, "indexOf", "(Ljava/lang/Object;)I", (element2.clone(),))
        .await?;
    assert_eq!(index, -1);

    let non_existing = JavaLangString::from_rust_string(&jvm, "nonExisting").await?;
    let removed: bool = jvm
        .invoke_virtual(&vector, "removeElement", "(Ljava/lang/Object;)Z", (non_existing,))
        .await?;
    assert!(!removed);

    let size: i32 = jvm.invoke_virtual(&vector, "size", "()I", ()).await?;
    assert_eq!(size, 2);

    Ok(())
}

#[tokio::test]
async fn test_vector_remove_element_uses_java_equals() -> Result<()> {
    let jvm = test_jvm().await?;

    let vector = jvm.new_class("java/util/Vector", "()V", ()).await?;

    let element = JavaLangString::from_rust_string(&jvm, "sameValue").await?;
    let equal_element = JavaLangString::from_rust_string(&jvm, "sameValue").await?;

    let _: bool = jvm.invoke_virtual(&vector, "add", "(Ljava/lang/Object;)Z", (element,)).await?;

    let contains: bool = jvm
        .invoke_virtual(&vector, "contains", "(Ljava/lang/Object;)Z", (equal_element.clone(),))
        .await?;
    assert!(contains);

    let removed: bool = jvm.invoke_virtual(&vector, "remove", "(Ljava/lang/Object;)Z", (equal_element,)).await?;
    assert!(removed);

    let size: i32 = jvm.invoke_virtual(&vector, "size", "()I", ()).await?;
    assert_eq!(size, 0);

    Ok(())
}

#[tokio::test]
async fn test_vector_trim_to_size() -> Result<()> {
    let jvm = test_jvm().await?;

    let vector = jvm.new_class("java/util/Vector", "(I)V", (100,)).await?;

    let element1 = JavaLangString::from_rust_string(&jvm, "testValue1").await?;
    let element2 = JavaLangString::from_rust_string(&jvm, "testValue2").await?;

    let _: bool = jvm.invoke_virtual(&vector, "add", "(Ljava/lang/Object;)Z", (element1,)).await?;
    let _: bool = jvm.invoke_virtual(&vector, "add", "(Ljava/lang/Object;)Z", (element2,)).await?;

    let _: () = jvm.invoke_virtual(&vector, "trimToSize", "()V", ()).await?;

    let size: i32 = jvm.invoke_virtual(&vector, "size", "()I", ()).await?;
    assert_eq!(size, 2);

    let element_at: ClassInstanceRef<Object> = jvm.invoke_virtual(&vector, "elementAt", "(I)Ljava/lang/Object;", (0,)).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &element_at).await?, "testValue1");

    Ok(())
}

#[tokio::test]
async fn test_first_element_empty() -> Result<()> {
    let jvm = test_jvm().await?;

    let vector = jvm.new_class("java/util/Vector", "()V", ()).await?;

    let result: Result<ClassInstanceRef<Object>> = jvm.invoke_virtual(&vector, "firstElement", "()Ljava/lang/Object;", ()).await;

    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Expected JavaException, got {:?}", result);
    };
    assert!(jvm.is_instance(&*exception, "java/util/NoSuchElementException"));

    Ok(())
}

#[tokio::test]
async fn test_index_of_null() -> Result<()> {
    let jvm = test_jvm().await?;

    let vector = jvm.new_class("java/util/Vector", "()V", ()).await?;

    let element = JavaLangString::from_rust_string(&jvm, "testValue").await?;
    let _: bool = jvm.invoke_virtual(&vector, "add", "(Ljava/lang/Object;)Z", (element,)).await?;
    let _: bool = jvm
        .invoke_virtual(&vector, "add", "(Ljava/lang/Object;)Z", (JavaValue::Object(None),))
        .await?;

    let index: i32 = jvm
        .invoke_virtual(&vector, "indexOf", "(Ljava/lang/Object;)I", (JavaValue::Object(None),))
        .await?;
    assert_eq!(index, 1);

    Ok(())
}

#[tokio::test]
async fn test_collections_type_resolve_and_vector_assignability() -> Result<()> {
    let jvm = test_jvm().await?;

    for class_name in [
        "java/util/Collection",
        "java/util/List",
        "java/util/Set",
        "java/util/Map",
        "java/util/Map$Entry",
        "java/util/Iterator",
        "java/util/AbstractCollection",
        "java/util/AbstractList",
        "java/util/AbstractSet",
        "java/util/AbstractMap",
        "java/util/Vector$Itr",
    ] {
        let _ = jvm.resolve_class(class_name).await?;
    }

    let vector = jvm.new_class("java/util/Vector", "()V", ()).await?;
    assert!(jvm.is_instance(&*vector, "java/util/List"));
    assert!(jvm.is_instance(&*vector, "java/util/Collection"));
    assert!(jvm.is_instance(&*vector, "java/util/AbstractList"));
    assert!(jvm.is_instance(&*vector, "java/util/AbstractCollection"));

    let stack = jvm.new_class("java/util/Stack", "()V", ()).await?;
    assert!(jvm.is_instance(&*stack, "java/util/List"));
    assert!(jvm.is_instance(&*stack, "java/util/Collection"));

    Ok(())
}

#[tokio::test]
async fn test_vector_collection_and_list_wrappers() -> Result<()> {
    let jvm = test_jvm().await?;

    let vector = jvm.new_class("java/util/Vector", "()V", ()).await?;

    let first = JavaLangString::from_rust_string(&jvm, "first").await?;
    let second = JavaLangString::from_rust_string(&jvm, "second").await?;
    let middle = JavaLangString::from_rust_string(&jvm, "middle").await?;
    let missing = JavaLangString::from_rust_string(&jvm, "missing").await?;

    let _: bool = jvm.invoke_virtual(&vector, "add", "(Ljava/lang/Object;)Z", (first.clone(),)).await?;
    let _: bool = jvm.invoke_virtual(&vector, "add", "(Ljava/lang/Object;)Z", (second.clone(),)).await?;
    let _: () = jvm.invoke_virtual(&vector, "add", "(ILjava/lang/Object;)V", (1, middle.clone())).await?;

    let result: Result<()> = jvm.invoke_virtual(&vector, "add", "(ILjava/lang/Object;)V", (-1, middle.clone())).await;
    assert_index_out_of_bounds_message(&jvm, result, "Index: -1, Size: 3").await?;
    let result: Result<()> = jvm.invoke_virtual(&vector, "add", "(ILjava/lang/Object;)V", (4, middle.clone())).await;
    assert_index_out_of_bounds_message(&jvm, result, "Index: 4, Size: 3").await?;

    let value: ClassInstanceRef<Object> = jvm.invoke_virtual(&vector, "get", "(I)Ljava/lang/Object;", (1,)).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, "middle");

    let value: ClassInstanceRef<Object> = jvm.invoke_virtual(&vector, "get", "(I)Ljava/lang/Object;", (2,)).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, "second");

    let contains: bool = jvm
        .invoke_virtual(&vector, "contains", "(Ljava/lang/Object;)Z", (middle.clone(),))
        .await?;
    assert!(contains);

    let contains: bool = jvm
        .invoke_virtual(&vector, "contains", "(Ljava/lang/Object;)Z", (missing.clone(),))
        .await?;
    assert!(!contains);

    let removed: bool = jvm.invoke_virtual(&vector, "remove", "(Ljava/lang/Object;)Z", (middle.clone(),)).await?;
    assert!(removed);

    let removed: bool = jvm.invoke_virtual(&vector, "remove", "(Ljava/lang/Object;)Z", (missing.clone(),)).await?;
    assert!(!removed);

    let array: ClassInstanceRef<Array<Object>> = jvm.invoke_virtual(&vector, "toArray", "()[Ljava/lang/Object;", ()).await?;
    assert_eq!(jvm.array_length(&array).await?, 2);
    let values: Vec<ClassInstanceRef<Object>> = jvm.load_array(&array, 0, 2).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &values[0]).await?, "first");
    assert_eq!(JavaLangString::to_rust_string(&jvm, &values[1]).await?, "second");

    let _: () = jvm.invoke_virtual(&vector, "clear", "()V", ()).await?;
    let size: i32 = jvm.invoke_virtual(&vector, "size", "()I", ()).await?;
    assert_eq!(size, 0);

    let array: ClassInstanceRef<Array<Object>> = jvm.invoke_virtual(&vector, "toArray", "()[Ljava/lang/Object;", ()).await?;
    assert_eq!(jvm.array_length(&array).await?, 0);

    Ok(())
}

#[tokio::test]
async fn test_vector_itr_wrapper() -> Result<()> {
    let jvm = test_jvm().await?;

    let vector = jvm.new_class("java/util/Vector", "()V", ()).await?;
    let first = JavaLangString::from_rust_string(&jvm, "first").await?;
    let second = JavaLangString::from_rust_string(&jvm, "second").await?;

    let _: bool = jvm.invoke_virtual(&vector, "add", "(Ljava/lang/Object;)Z", (first,)).await?;
    let _: bool = jvm.invoke_virtual(&vector, "add", "(Ljava/lang/Object;)Z", (second,)).await?;

    let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&vector, "iterator", "()Ljava/util/Iterator;", ()).await?;
    assert!(jvm.is_instance(&**iterator, "java/util/Iterator"));

    let third = JavaLangString::from_rust_string(&jvm, "third").await?;
    let _: bool = jvm.invoke_virtual(&vector, "add", "(Ljava/lang/Object;)Z", (third,)).await?;

    let has_next: bool = jvm.invoke_virtual(&iterator, "hasNext", "()Z", ()).await?;
    assert!(has_next);

    let value: ClassInstanceRef<Object> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, "first");

    let value: ClassInstanceRef<Object> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, "second");

    let has_next: bool = jvm.invoke_virtual(&iterator, "hasNext", "()Z", ()).await?;
    assert!(!has_next);

    let result: Result<ClassInstanceRef<Object>> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Expected JavaException, got {:?}", result);
    };
    assert!(jvm.is_instance(&*exception, "java/util/NoSuchElementException"));

    let result: Result<()> = jvm.invoke_virtual(&iterator, "remove", "()V", ()).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Expected JavaException, got {:?}", result);
    };
    assert!(jvm.is_instance(&*exception, "java/lang/UnsupportedOperationException"));

    let empty_vector = jvm.new_class("java/util/Vector", "()V", ()).await?;
    let empty_iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&empty_vector, "iterator", "()Ljava/util/Iterator;", ()).await?;
    let has_next: bool = jvm.invoke_virtual(&empty_iterator, "hasNext", "()Z", ()).await?;
    assert!(!has_next);

    let result: Result<ClassInstanceRef<Object>> = jvm.invoke_virtual(&empty_iterator, "next", "()Ljava/lang/Object;", ()).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Expected JavaException, got {:?}", result);
    };
    assert!(jvm.is_instance(&*exception, "java/util/NoSuchElementException"));

    Ok(())
}
