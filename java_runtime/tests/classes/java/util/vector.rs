use java_runtime::classes::java::lang::Object;
use jvm::{ClassInstanceRef, Result, runtime::JavaLangString};

use test_utils::test_jvm;

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
async fn test_vector_last_index_of() -> Result<()> {
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

    Ok(())
}
