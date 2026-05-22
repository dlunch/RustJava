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

#[tokio::test]
async fn test_last_index_of() -> Result<()> {
    let jvm = test_jvm().await?;

    let string = JavaLangString::from_rust_string(&jvm, "456 가나다 456").await?;

    let index: i32 = jvm.invoke_virtual(&string, "lastIndexOf", "(I)I", (b'4' as i32,)).await?;
    assert_eq!(index, 8);
    let index: i32 = jvm.invoke_virtual(&string, "lastIndexOf", "(I)I", (b'5' as i32,)).await?;
    assert_eq!(index, 9);
    let index: i32 = jvm.invoke_virtual(&string, "lastIndexOf", "(I)I", (b'6' as i32,)).await?;
    assert_eq!(index, 10);
    let index: i32 = jvm.invoke_virtual(&string, "lastIndexOf", "(I)I", (b'7' as i32,)).await?;
    assert_eq!(index, -1);

    Ok(())
}

#[tokio::test]
async fn test_get_chars() -> Result<()> {
    let jvm = test_jvm().await?;

    let string = JavaLangString::from_rust_string(&jvm, "Hello, 테스트!").await?;

    let char_array = jvm.instantiate_array("[C", 11).await?;

    let _: () = jvm
        .invoke_virtual(&string, "getChars", "(II[CI)V", (0i32, 11i32, char_array.clone(), 0i32))
        .await?;
    let chars = jvm.load_array::<u16>(&char_array, 0, 11).await?;
    let rust_string = String::from_utf16(&chars).unwrap();
    assert_eq!(rust_string, "Hello, 테스트!");

    let partial_array = jvm.instantiate_array("[C", 4).await?;

    let _: () = jvm
        .invoke_virtual(&string, "getChars", "(II[CI)V", (7i32, 11i32, partial_array.clone(), 0i32))
        .await?;
    let chars = jvm.load_array::<u16>(&partial_array, 0, 4).await?;
    let rust_string = String::from_utf16(&chars).unwrap();
    assert_eq!(rust_string, "테스트!");

    Ok(())
}

#[tokio::test]
async fn test_ends_with() -> Result<()> {
    let jvm = test_jvm().await?;

    let string = JavaLangString::from_rust_string(&jvm, "Hello, 테스트!").await?;

    let suffix = JavaLangString::from_rust_string(&jvm, "테스트!").await?;
    let result: bool = jvm.invoke_virtual(&string, "endsWith", "(Ljava/lang/String;)Z", (suffix,)).await?;
    assert!(result);

    let suffix = JavaLangString::from_rust_string(&jvm, "Hello").await?;
    let result: bool = jvm.invoke_virtual(&string, "endsWith", "(Ljava/lang/String;)Z", (suffix,)).await?;
    assert!(!result);

    Ok(())
}

#[tokio::test]
async fn test_equals_ignore_case() -> Result<()> {
    let jvm = test_jvm().await?;

    let a = JavaLangString::from_rust_string(&jvm, "Hello").await?;
    let b = JavaLangString::from_rust_string(&jvm, "HELLO").await?;
    let result: bool = jvm.invoke_virtual(&a, "equalsIgnoreCase", "(Ljava/lang/String;)Z", (b,)).await?;
    assert!(result);

    let a = JavaLangString::from_rust_string(&jvm, "Hello").await?;
    let b = JavaLangString::from_rust_string(&jvm, "World").await?;
    let result: bool = jvm.invoke_virtual(&a, "equalsIgnoreCase", "(Ljava/lang/String;)Z", (b,)).await?;
    assert!(!result);

    Ok(())
}

#[tokio::test]
async fn test_to_lower_case() -> Result<()> {
    let jvm = test_jvm().await?;

    let string = JavaLangString::from_rust_string(&jvm, "HELLO 테스트").await?;
    let result = jvm.invoke_virtual(&string, "toLowerCase", "()Ljava/lang/String;", ()).await?;
    let result_string = JavaLangString::to_rust_string(&jvm, &result).await?;
    assert_eq!(result_string, "hello 테스트");

    Ok(())
}

#[tokio::test]
async fn test_replace() -> Result<()> {
    let jvm = test_jvm().await?;

    let string = JavaLangString::from_rust_string(&jvm, "a.b.c.d").await?;
    let result = jvm
        .invoke_virtual(&string, "replace", "(CC)Ljava/lang/String;", (b'.' as u16, b'/' as u16))
        .await?;
    let result_string = JavaLangString::to_rust_string(&jvm, &result).await?;
    assert_eq!(result_string, "a/b/c/d");

    Ok(())
}

#[tokio::test]
async fn test_region_matches() -> Result<()> {
    let jvm = test_jvm().await?;

    let a = JavaLangString::from_rust_string(&jvm, "Hello World").await?;
    let b = JavaLangString::from_rust_string(&jvm, "WORLD!!!").await?;

    let result: bool = jvm
        .invoke_virtual(&a, "regionMatches", "(ZILjava/lang/String;II)Z", (false, 6i32, b.clone(), 0i32, 5i32))
        .await?;
    assert!(!result);

    let result: bool = jvm
        .invoke_virtual(&a, "regionMatches", "(ZILjava/lang/String;II)Z", (true, 6i32, b.clone(), 0i32, 5i32))
        .await?;
    assert!(result);

    let result: bool = jvm
        .invoke_virtual(&a, "regionMatches", "(ZILjava/lang/String;II)Z", (false, 0i32, b, 0i32, 3i32))
        .await?;
    assert!(!result);

    Ok(())
}

#[tokio::test]
async fn test_last_index_of_from() -> Result<()> {
    let jvm = test_jvm().await?;

    let string = JavaLangString::from_rust_string(&jvm, "abcabc").await?;

    let index: i32 = jvm.invoke_virtual(&string, "lastIndexOf", "(II)I", (b'a' as i32, 5i32)).await?;
    assert_eq!(index, 3);

    let index: i32 = jvm.invoke_virtual(&string, "lastIndexOf", "(II)I", (b'a' as i32, 2i32)).await?;
    assert_eq!(index, 0);

    let index: i32 = jvm.invoke_virtual(&string, "lastIndexOf", "(II)I", (b'z' as i32, 5i32)).await?;
    assert_eq!(index, -1);

    Ok(())
}

#[tokio::test]
async fn test_value_of_overloads() -> Result<()> {
    let jvm = test_jvm().await?;

    let result = jvm.invoke_static("java/lang/String", "valueOf", "(Z)Ljava/lang/String;", (true,)).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &result).await?, "true");

    let result = jvm
        .invoke_static("java/lang/String", "valueOf", "(J)Ljava/lang/String;", (12345i64,))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &result).await?, "12345");

    let chars = jvm.instantiate_array("C", 3).await?;
    jvm.store_array(&mut chars.clone(), 0, vec![b'a' as u16, b'b' as u16, b'c' as u16])
        .await?;
    let result = jvm
        .invoke_static("java/lang/String", "valueOf", "([C)Ljava/lang/String;", (chars,))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &result).await?, "abc");

    let result = jvm
        .invoke_static("java/lang/String", "valueOf", "(F)Ljava/lang/String;", (1.5f32,))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &result).await?, "1.5");

    let result = jvm
        .invoke_static("java/lang/String", "valueOf", "(D)Ljava/lang/String;", (3.14f64,))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &result).await?, "3.14");

    let mut chars = jvm.instantiate_array("C", 5).await?;
    jvm.store_array(&mut chars, 0, vec![b'h' as u16, b'e' as u16, b'l' as u16, b'l' as u16, b'o' as u16])
        .await?;
    let result = jvm
        .invoke_static("java/lang/String", "valueOf", "([CII)Ljava/lang/String;", (chars, 1i32, 3i32))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &result).await?, "ell");

    Ok(())
}

#[tokio::test]
async fn test_init_empty() -> Result<()> {
    let jvm = test_jvm().await?;

    let string = jvm.new_class("java/lang/String", "()V", ()).await?;
    let result = JavaLangString::to_rust_string(&jvm, &string).await?;
    assert_eq!(result, "");

    Ok(())
}

#[tokio::test]
async fn test_init_byte_array_charset() -> Result<()> {
    let jvm = test_jvm().await?;

    let bytes = vec![b'H' as i8, b'i' as i8, b'!' as i8];
    let mut array = jvm.instantiate_array("B", 3).await?;
    jvm.store_array(&mut array, 0, bytes).await?;

    let charset = JavaLangString::from_rust_string(&jvm, "UTF-8").await?;
    let string = jvm.new_class("java/lang/String", "([BLjava/lang/String;)V", (array, charset)).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &string).await?, "Hi!");

    Ok(())
}
