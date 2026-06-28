use java_runtime::classes::java::lang::Object;
use jvm::{Array, ClassInstanceRef, JavaError, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_array_list_basic_operations_and_capacity_zero() -> Result<()> {
    let jvm = test_jvm().await?;

    let array_list = jvm.new_class("java/util/ArrayList", "(I)V", (0,)).await?;
    let element_data: ClassInstanceRef<Array<Object>> = jvm.get_field(&array_list, "elementData", "[Ljava/lang/Object;").await?;
    assert_eq!(jvm.array_length(&element_data).await?, 0);

    let first = JavaLangString::from_rust_string(&jvm, "first").await?;
    let second = JavaLangString::from_rust_string(&jvm, "second").await?;
    let middle = JavaLangString::from_rust_string(&jvm, "middle").await?;
    let replacement = JavaLangString::from_rust_string(&jvm, "replacement").await?;

    let added: bool = jvm.invoke_virtual(&array_list, "add", "(Ljava/lang/Object;)Z", (first.clone(),)).await?;
    assert!(added);

    let element_data: ClassInstanceRef<Array<Object>> = jvm.get_field(&array_list, "elementData", "[Ljava/lang/Object;").await?;
    assert!(jvm.array_length(&element_data).await? >= 1);

    let _: bool = jvm.invoke_virtual(&array_list, "add", "(Ljava/lang/Object;)Z", (second.clone(),)).await?;
    let _: () = jvm
        .invoke_virtual(&array_list, "add", "(ILjava/lang/Object;)V", (1, middle.clone()))
        .await?;

    let size: i32 = jvm.invoke_virtual(&array_list, "size", "()I", ()).await?;
    assert_eq!(size, 3);

    let value: ClassInstanceRef<Object> = jvm.invoke_virtual(&array_list, "get", "(I)Ljava/lang/Object;", (0,)).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, "first");

    let value: ClassInstanceRef<Object> = jvm.invoke_virtual(&array_list, "get", "(I)Ljava/lang/Object;", (1,)).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, "middle");

    let old: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&array_list, "set", "(ILjava/lang/Object;)Ljava/lang/Object;", (1, replacement.clone()))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &old).await?, "middle");

    let removed: ClassInstanceRef<Object> = jvm.invoke_virtual(&array_list, "remove", "(I)Ljava/lang/Object;", (0,)).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &removed).await?, "first");

    let size: i32 = jvm.invoke_virtual(&array_list, "size", "()I", ()).await?;
    assert_eq!(size, 2);

    let element_data: ClassInstanceRef<Array<Object>> = jvm.get_field(&array_list, "elementData", "[Ljava/lang/Object;").await?;
    let removed_tail_slot: ClassInstanceRef<Object> = jvm.load_array(&element_data, 2, 1).await?.into_iter().next().unwrap();
    assert!(removed_tail_slot.is_null());

    let array: ClassInstanceRef<Array<Object>> = jvm.invoke_virtual(&array_list, "toArray", "()[Ljava/lang/Object;", ()).await?;
    assert_eq!(jvm.array_length(&array).await?, 2);
    let values: Vec<ClassInstanceRef<Object>> = jvm.load_array(&array, 0, 2).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &values[0]).await?, "replacement");
    assert_eq!(JavaLangString::to_rust_string(&jvm, &values[1]).await?, "second");

    let _: () = jvm.invoke_virtual(&array_list, "clear", "()V", ()).await?;
    let size: i32 = jvm.invoke_virtual(&array_list, "size", "()I", ()).await?;
    assert_eq!(size, 0);
    let is_empty: bool = jvm.invoke_virtual(&array_list, "isEmpty", "()Z", ()).await?;
    assert!(is_empty);

    let array: ClassInstanceRef<Array<Object>> = jvm.invoke_virtual(&array_list, "toArray", "()[Ljava/lang/Object;", ()).await?;
    assert_eq!(jvm.array_length(&array).await?, 0);

    let element_data: ClassInstanceRef<Array<Object>> = jvm.get_field(&array_list, "elementData", "[Ljava/lang/Object;").await?;
    let cleared_slots: Vec<ClassInstanceRef<Object>> = jvm.load_array(&element_data, 0, 2).await?;
    assert!(cleared_slots[0].is_null());
    assert!(cleared_slots[1].is_null());

    Ok(())
}

#[tokio::test]
async fn test_array_list_null_equality_and_remove_object() -> Result<()> {
    let jvm = test_jvm().await?;

    let array_list = jvm.new_class("java/util/ArrayList", "()V", ()).await?;
    let null_ref: ClassInstanceRef<Object> = None.into();
    let stored = JavaLangString::from_rust_string(&jvm, "sameValue").await?;
    let equal = JavaLangString::from_rust_string(&jvm, "sameValue").await?;
    let tail = JavaLangString::from_rust_string(&jvm, "tail").await?;
    let missing = JavaLangString::from_rust_string(&jvm, "missing").await?;

    let _: bool = jvm
        .invoke_virtual(&array_list, "add", "(Ljava/lang/Object;)Z", (null_ref.clone(),))
        .await?;
    let _: bool = jvm.invoke_virtual(&array_list, "add", "(Ljava/lang/Object;)Z", (stored,)).await?;
    let _: bool = jvm.invoke_virtual(&array_list, "add", "(Ljava/lang/Object;)Z", (tail.clone(),)).await?;

    let index: i32 = jvm
        .invoke_virtual(&array_list, "indexOf", "(Ljava/lang/Object;)I", (null_ref.clone(),))
        .await?;
    assert_eq!(index, 0);

    let contains: bool = jvm
        .invoke_virtual(&array_list, "contains", "(Ljava/lang/Object;)Z", (equal.clone(),))
        .await?;
    assert!(contains);

    let index: i32 = jvm
        .invoke_virtual(&array_list, "indexOf", "(Ljava/lang/Object;)I", (equal.clone(),))
        .await?;
    assert_eq!(index, 1);

    let removed: bool = jvm
        .invoke_virtual(&array_list, "remove", "(Ljava/lang/Object;)Z", (equal.clone(),))
        .await?;
    assert!(removed);

    let index: i32 = jvm.invoke_virtual(&array_list, "indexOf", "(Ljava/lang/Object;)I", (equal,)).await?;
    assert_eq!(index, -1);

    let removed: bool = jvm.invoke_virtual(&array_list, "remove", "(Ljava/lang/Object;)Z", (missing,)).await?;
    assert!(!removed);

    let size: i32 = jvm.invoke_virtual(&array_list, "size", "()I", ()).await?;
    assert_eq!(size, 2);

    let value: ClassInstanceRef<Object> = jvm.invoke_virtual(&array_list, "get", "(I)Ljava/lang/Object;", (0,)).await?;
    assert!(value.is_null());

    let value: ClassInstanceRef<Object> = jvm.invoke_virtual(&array_list, "get", "(I)Ljava/lang/Object;", (1,)).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, "tail");

    Ok(())
}

#[tokio::test]
async fn test_array_list_assignability_and_interface_style_calls() -> Result<()> {
    let jvm = test_jvm().await?;

    for class_name in [
        "java/util/ArrayList",
        "java/util/ArrayList$Itr",
        "java/util/Collection",
        "java/util/List",
        "java/util/Iterator",
    ] {
        let _ = jvm.resolve_class(class_name).await?;
    }

    let array_list = jvm.new_class("java/util/ArrayList", "()V", ()).await?;
    assert!(jvm.is_instance(&*array_list, "java/util/List"));
    assert!(jvm.is_instance(&*array_list, "java/util/Collection"));
    assert!(jvm.is_instance(&*array_list, "java/util/AbstractList"));
    assert!(jvm.is_instance(&*array_list, "java/util/AbstractCollection"));
    assert!(!jvm.is_instance(&*array_list, "java/util/Set"));

    let value = JavaLangString::from_rust_string(&jvm, "interface-style").await?;
    let equal_value = JavaLangString::from_rust_string(&jvm, "interface-style").await?;
    let _: bool = jvm.invoke_virtual(&array_list, "add", "(Ljava/lang/Object;)Z", (value,)).await?;

    let size: i32 = jvm.invoke_virtual(&array_list, "size", "()I", ()).await?;
    assert_eq!(size, 1);

    let contains: bool = jvm
        .invoke_virtual(&array_list, "contains", "(Ljava/lang/Object;)Z", (equal_value.clone(),))
        .await?;
    assert!(contains);

    let value: ClassInstanceRef<Object> = jvm.invoke_virtual(&array_list, "get", "(I)Ljava/lang/Object;", (0,)).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, "interface-style");

    let removed: bool = jvm.invoke_virtual(&array_list, "remove", "(Ljava/lang/Object;)Z", (equal_value,)).await?;
    assert!(removed);

    let is_empty: bool = jvm.invoke_virtual(&array_list, "isEmpty", "()Z", ()).await?;
    assert!(is_empty);

    Ok(())
}

#[tokio::test]
async fn test_array_list_itr_snapshot_exhaustion_and_remove() -> Result<()> {
    let jvm = test_jvm().await?;

    let array_list = jvm.new_class("java/util/ArrayList", "()V", ()).await?;
    let first = JavaLangString::from_rust_string(&jvm, "first").await?;
    let second = JavaLangString::from_rust_string(&jvm, "second").await?;

    let _: bool = jvm.invoke_virtual(&array_list, "add", "(Ljava/lang/Object;)Z", (first,)).await?;
    let _: bool = jvm.invoke_virtual(&array_list, "add", "(Ljava/lang/Object;)Z", (second,)).await?;

    let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&array_list, "iterator", "()Ljava/util/Iterator;", ()).await?;
    assert!(jvm.is_instance(&**iterator, "java/util/Iterator"));

    let third = JavaLangString::from_rust_string(&jvm, "third").await?;
    let _: bool = jvm.invoke_virtual(&array_list, "add", "(Ljava/lang/Object;)Z", (third,)).await?;

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

    let empty_array_list = jvm.new_class("java/util/ArrayList", "()V", ()).await?;
    let empty_iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&empty_array_list, "iterator", "()Ljava/util/Iterator;", ()).await?;
    let has_next: bool = jvm.invoke_virtual(&empty_iterator, "hasNext", "()Z", ()).await?;
    assert!(!has_next);

    let result: Result<ClassInstanceRef<Object>> = jvm.invoke_virtual(&empty_iterator, "next", "()Ljava/lang/Object;", ()).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Expected JavaException, got {:?}", result);
    };
    assert!(jvm.is_instance(&*exception, "java/util/NoSuchElementException"));

    Ok(())
}

#[tokio::test]
async fn test_array_list_bounds_and_negative_capacity() -> Result<()> {
    let jvm = test_jvm().await?;

    let result = jvm.new_class("java/util/ArrayList", "(I)V", (-1,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Expected JavaException, got {:?}", result);
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalArgumentException"));

    let array_list = jvm.new_class("java/util/ArrayList", "()V", ()).await?;

    let result: Result<ClassInstanceRef<Object>> = jvm.invoke_virtual(&array_list, "get", "(I)Ljava/lang/Object;", (0,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Expected JavaException, got {:?}", result);
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IndexOutOfBoundsException"));

    let element = JavaLangString::from_rust_string(&jvm, "element").await?;
    let result: Result<()> = jvm
        .invoke_virtual(&array_list, "add", "(ILjava/lang/Object;)V", (1, element.clone()))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Expected JavaException, got {:?}", result);
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IndexOutOfBoundsException"));

    let result: Result<()> = jvm
        .invoke_virtual(&array_list, "add", "(ILjava/lang/Object;)V", (-1, element.clone()))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Expected JavaException, got {:?}", result);
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IndexOutOfBoundsException"));

    let _: bool = jvm
        .invoke_virtual(&array_list, "add", "(Ljava/lang/Object;)Z", (element.clone(),))
        .await?;

    let result: Result<ClassInstanceRef<Object>> = jvm
        .invoke_virtual(&array_list, "set", "(ILjava/lang/Object;)Ljava/lang/Object;", (1, element.clone()))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Expected JavaException, got {:?}", result);
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IndexOutOfBoundsException"));

    let result: Result<ClassInstanceRef<Object>> = jvm.invoke_virtual(&array_list, "remove", "(I)Ljava/lang/Object;", (-1,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Expected JavaException, got {:?}", result);
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IndexOutOfBoundsException"));

    let result: Result<ClassInstanceRef<Object>> = jvm.invoke_virtual(&array_list, "remove", "(I)Ljava/lang/Object;", (1,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Expected JavaException, got {:?}", result);
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IndexOutOfBoundsException"));

    Ok(())
}
