use jvm::{Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_string() -> Result<()> {
    let jvm = test_jvm().await?;

    let string = JavaLangString::from_rust_string(&jvm, "test").await?;

    let string = JavaLangString::to_rust_string(&jvm, &string).await?;

    assert_eq!(string, "test");

    Ok(())
}

#[tokio::test]
async fn test_string_concat() -> Result<()> {
    let jvm = test_jvm().await?;

    let string1 = JavaLangString::from_rust_string(&jvm, "test1").await?;
    let string2 = JavaLangString::from_rust_string(&jvm, "test2").await?;

    let result = jvm
        .invoke_virtual(&string1, "concat", "(Ljava/lang/String;)Ljava/lang/String;", (string2,))
        .await?;

    let string = JavaLangString::to_rust_string(&jvm, &result).await?;

    assert_eq!(string, "test1test2");

    Ok(())
}

#[tokio::test]
async fn test_hash_code() -> Result<()> {
    let jvm = test_jvm().await?;

    let string = JavaLangString::from_rust_string(&jvm, "Hi").await?;
    let hash_code: i32 = jvm.invoke_virtual(&string, "hashCode", "()I", ()).await?;
    assert_eq!(hash_code, 2337);

    let string1 = JavaLangString::from_rust_string(&jvm, "test").await?;
    let hash_code1: i32 = jvm.invoke_virtual(&string1, "hashCode", "()I", ()).await?;
    assert_eq!(hash_code1, 3556498);

    let string2 = JavaLangString::from_rust_string(&jvm, "Hi").await?;
    let hash_code: i32 = jvm.invoke_virtual(&string2, "hashCode", "()I", ()).await?;
    assert_eq!(hash_code, 2337);

    Ok(())
}

#[tokio::test]
async fn test_index_of() -> Result<()> {
    let jvm = test_jvm().await?;

    let string = JavaLangString::from_rust_string(&jvm, "123 테스트 456").await?;

    let pattern = JavaLangString::from_rust_string(&jvm, "테스트").await?;
    let index: i32 = jvm.invoke_virtual(&string, "indexOf", "(Ljava/lang/String;)I", (pattern,)).await?;
    assert_eq!(index, 4);

    let pattern = JavaLangString::from_rust_string(&jvm, "456").await?;
    let index: i32 = jvm
        .invoke_virtual(&string, "indexOf", "(Ljava/lang/String;)I", (pattern.clone(),))
        .await?;
    assert_eq!(index, 8);

    let index: i32 = jvm
        .invoke_virtual(&string, "indexOf", "(Ljava/lang/String;I)I", (pattern.clone(), 5))
        .await?;
    assert_eq!(index, 8);

    let pattern = JavaLangString::from_rust_string(&jvm, "123").await?;
    let index: i32 = jvm
        .invoke_virtual(&string, "indexOf", "(Ljava/lang/String;)I", (pattern.clone(),))
        .await?;
    assert_eq!(index, 0);

    let index: i32 = jvm.invoke_virtual(&string, "indexOf", "(Ljava/lang/String;I)I", (pattern, 2)).await?;
    assert_eq!(index, -1);

    let pattern = JavaLangString::from_rust_string(&jvm, "789").await?;
    let index: i32 = jvm.invoke_virtual(&string, "indexOf", "(Ljava/lang/String;)I", (pattern,)).await?;
    assert_eq!(index, -1);

    let index: i32 = jvm.invoke_virtual(&string, "indexOf", "(I)I", (52,)).await?;
    assert_eq!(index, 8);

    let index: i32 = jvm.invoke_virtual(&string, "indexOf", "(II)I", (52, 8)).await?;
    assert_eq!(index, 8);

    let index: i32 = jvm.invoke_virtual(&string, "indexOf", "(II)I", (52, 9)).await?;
    assert_eq!(index, -1);

    Ok(())
}

#[tokio::test]
async fn test_starts_with() -> Result<()> {
    let jvm = test_jvm().await?;

    let string = JavaLangString::from_rust_string(&jvm, "123 테스트 456").await?;

    let pattern = JavaLangString::from_rust_string(&jvm, "123").await?;
    let result: bool = jvm.invoke_virtual(&string, "startsWith", "(Ljava/lang/String;)Z", (pattern,)).await?;
    assert!(result);

    let pattern = JavaLangString::from_rust_string(&jvm, "456").await?;
    let result: bool = jvm.invoke_virtual(&string, "startsWith", "(Ljava/lang/String;)Z", (pattern,)).await?;
    assert!(!result);

    let pattern = JavaLangString::from_rust_string(&jvm, "123 테스트").await?;
    let result: bool = jvm.invoke_virtual(&string, "startsWith", "(Ljava/lang/String;)Z", (pattern,)).await?;
    assert!(result);

    let pattern = JavaLangString::from_rust_string(&jvm, "테스트").await?;
    let result: bool = jvm.invoke_virtual(&string, "startsWith", "(Ljava/lang/String;)Z", (pattern,)).await?;
    assert!(!result);

    let pattern = JavaLangString::from_rust_string(&jvm, "테스트").await?;
    let result: bool = jvm.invoke_virtual(&string, "startsWith", "(Ljava/lang/String;I)Z", (pattern, 4)).await?;
    assert!(result);

    let pattern = JavaLangString::from_rust_string(&jvm, "테스트").await?;
    let result: bool = jvm.invoke_virtual(&string, "startsWith", "(Ljava/lang/String;I)Z", (pattern, 5)).await?;
    assert!(!result);

    Ok(())
}
