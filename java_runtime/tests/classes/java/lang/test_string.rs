use java_constants::{ClassAccessFlags, MethodAccessFlags};
use java_runtime::classes::java::lang::{Object, String as JavaString};
use jvm::{Array, ClassInstanceRef, JavaChar, JavaError, Result, runtime::JavaLangString};

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
        .invoke_static("java/lang/String", "valueOf", "(D)Ljava/lang/String;", (3.15f64,))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &result).await?, "3.15");

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

#[tokio::test]
async fn test_substring_invalid_range() -> Result<()> {
    let jvm = test_jvm().await?;

    let string = JavaLangString::from_rust_string(&jvm, "hello").await?;

    for (begin, end) in [(3i32, 1i32), (0, 10), (-1, 3)] {
        let result: Result<ClassInstanceRef<JavaString>> = jvm.invoke_virtual(&string, "substring", "(II)Ljava/lang/String;", (begin, end)).await;

        let Err(JavaError::JavaException(exception)) = result else {
            panic!("Expected JavaException for ({begin}, {end}), got {:?}", result);
        };
        assert!(jvm.is_instance(&*exception, "java/lang/StringIndexOutOfBoundsException"));
    }

    Ok(())
}

#[tokio::test]
async fn test_get_bytes_unmappable_charset() -> Result<()> {
    let jvm = test_jvm().await?;

    let string = JavaLangString::from_rust_string(&jvm, "a한b").await?;
    let charset = JavaLangString::from_rust_string(&jvm, "ISO-8859-1").await?;

    let bytes = jvm.invoke_virtual(&string, "getBytes", "(Ljava/lang/String;)[B", (charset,)).await?;
    let bytes = jvm.load_array::<i8>(&bytes, 0, 3).await?;

    assert_eq!(bytes, [0x61, 0x3f, 0x62]);

    Ok(())
}

#[tokio::test]
async fn test_substring_utf16_indices() -> Result<()> {
    let jvm = test_jvm().await?;

    // "a😀b" has Java length 4: the emoji is a surrogate pair
    let string = JavaLangString::from_rust_string(&jvm, "a😀b").await?;

    let full: ClassInstanceRef<JavaString> = jvm.invoke_virtual(&string, "substring", "(II)Ljava/lang/String;", (0, 4)).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &full).await?, "a😀b");

    let emoji: ClassInstanceRef<JavaString> = jvm.invoke_virtual(&string, "substring", "(II)Ljava/lang/String;", (1, 3)).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &emoji).await?, "😀");

    let tail: ClassInstanceRef<JavaString> = jvm.invoke_virtual(&string, "substring", "(I)Ljava/lang/String;", (3,)).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &tail).await?, "b");

    Ok(())
}

#[tokio::test]
async fn test_get_bytes_ascii_replaces_non_ascii() -> Result<()> {
    let jvm = test_jvm().await?;

    let string = JavaLangString::from_rust_string(&jvm, "aé한").await?;
    let charset = JavaLangString::from_rust_string(&jvm, "US-ASCII").await?;

    let bytes = jvm.invoke_virtual(&string, "getBytes", "(Ljava/lang/String;)[B", (charset,)).await?;
    let bytes = jvm.load_array::<i8>(&bytes, 0, 3).await?;

    assert_eq!(bytes, [0x61, 0x3f, 0x3f]);

    Ok(())
}

#[tokio::test]
async fn test_index_of_uses_utf16_indices_and_handles_empty_patterns() -> Result<()> {
    let jvm = test_jvm().await?;
    let string = JavaLangString::from_rust_string(&jvm, "a😀b").await?;
    let empty = JavaLangString::from_rust_string(&jvm, "").await?;
    let emoji = JavaLangString::from_rust_string(&jvm, "😀").await?;
    let tail = JavaLangString::from_rust_string(&jvm, "b").await?;

    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&string, "indexOf", "(Ljava/lang/String;)I", (emoji,))
            .await?,
        1
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&string, "indexOf", "(Ljava/lang/String;)I", (tail,)).await?,
        3
    );
    assert_eq!(jvm.invoke_virtual::<_, i32>(&string, "indexOf", "(II)I", (b'b' as i32, -10)).await?, 3);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&string, "lastIndexOf", "(I)I", (b'b' as i32,)).await?, 3);
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&string, "indexOf", "(Ljava/lang/String;I)I", (empty.clone(), -10))
            .await?,
        0
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&string, "indexOf", "(Ljava/lang/String;I)I", (empty, 99))
            .await?,
        4
    );

    Ok(())
}

#[tokio::test]
async fn test_unknown_string_charset_throws_unsupported_encoding() -> Result<()> {
    let jvm = test_jvm().await?;
    let string = JavaLangString::from_rust_string(&jvm, "value").await?;
    let charset = JavaLangString::from_rust_string(&jvm, "not-a-charset").await?;

    let result: Result<ClassInstanceRef<jvm::Array<i8>>> = jvm
        .invoke_virtual(&string, "getBytes", "(Ljava/lang/String;)[B", (charset.clone(),))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("unknown charset must throw UnsupportedEncodingException");
    };
    assert!(jvm.is_instance(&*exception, "java/io/UnsupportedEncodingException"));

    let mut bytes = jvm.instantiate_array("B", 1).await?;
    jvm.store_array(&mut bytes, 0, [b'a' as i8]).await?;
    let result = jvm.new_class("java/lang/String", "([BLjava/lang/String;)V", (bytes, charset)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("unknown constructor charset must throw UnsupportedEncodingException");
    };
    assert!(jvm.is_instance(&*exception, "java/io/UnsupportedEncodingException"));

    Ok(())
}

#[tokio::test]
async fn test_trim_uses_java_control_character_boundary() -> Result<()> {
    let jvm = test_jvm().await?;
    let string = JavaLangString::from_rust_string(&jvm, " \t\u{a0}value\u{a0}\n ").await?;
    let trimmed: ClassInstanceRef<JavaString> = jvm.invoke_virtual(&string, "trim", "()Ljava/lang/String;", ()).await?;

    assert_eq!(JavaLangString::to_rust_string(&jvm, &trimmed).await?, "\u{a0}value\u{a0}");

    let unchanged = JavaLangString::from_rust_string(&jvm, "value").await?;
    let same: ClassInstanceRef<JavaString> = jvm.invoke_virtual(&unchanged, "trim", "()Ljava/lang/String;", ()).await?;
    assert_eq!(unchanged.identity(), same.identity());

    Ok(())
}

#[tokio::test]
async fn test_substring_shares_parent_value() -> Result<()> {
    let jvm = test_jvm().await?;

    let parent = JavaLangString::from_rust_string(&jvm, "HelloWorld").await?;
    let child: ClassInstanceRef<JavaString> = jvm.invoke_virtual(&parent, "substring", "(II)Ljava/lang/String;", (2, 5)).await?;

    let parent_value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&parent, "value", "[C").await?;
    let child_value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&child, "value", "[C").await?;
    assert_eq!(parent_value.identity(), child_value.identity());
    assert_eq!(jvm.get_field::<i32>(&child, "offset", "I").await?, 2);
    assert_eq!(jvm.get_field::<i32>(&child, "count", "I").await?, 3);
    assert_eq!(JavaLangString::to_rust_string(&jvm, &child).await?, "llo");

    let tail: ClassInstanceRef<JavaString> = jvm.invoke_virtual(&parent, "substring", "(I)Ljava/lang/String;", (5,)).await?;
    let tail_value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&tail, "value", "[C").await?;
    assert_eq!(parent_value.identity(), tail_value.identity());
    assert_eq!(jvm.get_field::<i32>(&tail, "offset", "I").await?, 5);
    assert_eq!(jvm.get_field::<i32>(&tail, "count", "I").await?, 5);
    assert_eq!(JavaLangString::to_rust_string(&jvm, &tail).await?, "World");

    Ok(())
}

#[tokio::test]
async fn test_full_range_substring_returns_this() -> Result<()> {
    let jvm = test_jvm().await?;

    let string = JavaLangString::from_rust_string(&jvm, "Hello").await?;

    let same: ClassInstanceRef<JavaString> = jvm.invoke_virtual(&string, "substring", "(I)Ljava/lang/String;", (0,)).await?;
    assert_eq!(string.identity(), same.identity());

    let same: ClassInstanceRef<JavaString> = jvm.invoke_virtual(&string, "substring", "(II)Ljava/lang/String;", (0, 5)).await?;
    assert_eq!(string.identity(), same.identity());

    Ok(())
}

#[tokio::test]
async fn test_nested_substring_shares_root_value() -> Result<()> {
    let jvm = test_jvm().await?;

    let root = JavaLangString::from_rust_string(&jvm, "abcdefghij").await?;
    let outer: ClassInstanceRef<JavaString> = jvm.invoke_virtual(&root, "substring", "(II)Ljava/lang/String;", (2, 8)).await?;
    let inner: ClassInstanceRef<JavaString> = jvm.invoke_virtual(&outer, "substring", "(II)Ljava/lang/String;", (1, 3)).await?;

    let root_value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&root, "value", "[C").await?;
    let inner_value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&inner, "value", "[C").await?;
    assert_eq!(root_value.identity(), inner_value.identity());
    assert_eq!(jvm.get_field::<i32>(&inner, "offset", "I").await?, 3);
    assert_eq!(jvm.get_field::<i32>(&inner, "count", "I").await?, 2);
    assert_eq!(jvm.invoke_virtual::<_, JavaChar>(&inner, "charAt", "(I)C", (0,)).await?, b'd' as JavaChar);
    assert_eq!(jvm.invoke_virtual::<_, JavaChar>(&inner, "charAt", "(I)C", (1,)).await?, b'e' as JavaChar);

    Ok(())
}

#[tokio::test]
async fn test_init_partial_char_array_is_defensive_copy() -> Result<()> {
    let jvm = test_jvm().await?;

    let mut chars = jvm.instantiate_array("C", 5).await?;
    jvm.store_array(&mut chars, 0, "Hello".encode_utf16().collect::<Vec<_>>()).await?;

    let string = jvm.new_class("java/lang/String", "([CII)V", (chars.clone(), 1, 3)).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &string).await?, "ell");

    let value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&string, "value", "[C").await?;
    assert_ne!(value.identity(), chars.identity());

    jvm.store_array(&mut chars, 0, "zzzzz".encode_utf16().collect::<Vec<_>>()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &string).await?, "ell");

    Ok(())
}

#[tokio::test]
async fn test_to_char_array_returns_copy() -> Result<()> {
    let jvm = test_jvm().await?;

    let string = JavaLangString::from_rust_string(&jvm, "abc").await?;
    let mut chars: ClassInstanceRef<Array<JavaChar>> = jvm.invoke_virtual(&string, "toCharArray", "()[C", ()).await?;

    let value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&string, "value", "[C").await?;
    assert_ne!(chars.identity(), value.identity());
    assert_eq!(jvm.array_length(&chars).await?, 3);

    jvm.store_array(&mut chars, 0, [b'z' as JavaChar]).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &string).await?, "abc");

    Ok(())
}

#[tokio::test]
async fn test_to_char_array_on_substring_covers_logical_range_only() -> Result<()> {
    let jvm = test_jvm().await?;

    let parent = JavaLangString::from_rust_string(&jvm, "HelloWorld").await?;
    let sub: ClassInstanceRef<JavaString> = jvm.invoke_virtual(&parent, "substring", "(II)Ljava/lang/String;", (2, 5)).await?;
    let chars: ClassInstanceRef<Array<JavaChar>> = jvm.invoke_virtual(&sub, "toCharArray", "()[C", ()).await?;

    assert_eq!(jvm.array_length(&chars).await?, 3);
    assert_eq!(jvm.load_array::<JavaChar>(&chars, 0, 3).await?, "llo".encode_utf16().collect::<Vec<_>>());

    Ok(())
}

#[tokio::test]
async fn test_char_at_and_get_chars_on_substring_check_bounds() -> Result<()> {
    let jvm = test_jvm().await?;

    let parent = JavaLangString::from_rust_string(&jvm, "xxHelloyy").await?;
    let sub: ClassInstanceRef<JavaString> = jvm.invoke_virtual(&parent, "substring", "(II)Ljava/lang/String;", (2, 7)).await?;

    assert_eq!(jvm.invoke_virtual::<_, i32>(&sub, "length", "()I", ()).await?, 5);
    assert_eq!(jvm.invoke_virtual::<_, JavaChar>(&sub, "charAt", "(I)C", (0,)).await?, b'H' as JavaChar);
    assert_eq!(jvm.invoke_virtual::<_, JavaChar>(&sub, "charAt", "(I)C", (4,)).await?, b'o' as JavaChar);

    for index in [5, -1] {
        let result: Result<JavaChar> = jvm.invoke_virtual(&sub, "charAt", "(I)C", (index,)).await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("charAt({index}) must throw");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/StringIndexOutOfBoundsException"));
    }

    let dst = jvm.instantiate_array("C", 3).await?;
    let _: () = jvm.invoke_virtual(&sub, "getChars", "(II[CI)V", (1, 4, dst.clone(), 0)).await?;
    assert_eq!(jvm.load_array::<JavaChar>(&dst, 0, 3).await?, "ell".encode_utf16().collect::<Vec<_>>());

    let dst = jvm.instantiate_array("C", 8).await?;
    let result: Result<()> = jvm.invoke_virtual(&sub, "getChars", "(II[CI)V", (1, 6, dst, 0)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("getChars beyond count must throw");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/StringIndexOutOfBoundsException"));

    Ok(())
}

#[tokio::test]
async fn test_search_on_substring_does_not_see_parent_data() -> Result<()> {
    let jvm = test_jvm().await?;

    let parent = JavaLangString::from_rust_string(&jvm, "xxHelloyy").await?;
    let sub: ClassInstanceRef<JavaString> = jvm.invoke_virtual(&parent, "substring", "(II)Ljava/lang/String;", (2, 7)).await?;

    assert_eq!(jvm.invoke_virtual::<_, i32>(&sub, "indexOf", "(I)I", (b'l' as i32,)).await?, 2);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&sub, "indexOf", "(I)I", (b'x' as i32,)).await?, -1);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&sub, "lastIndexOf", "(I)I", (b'l' as i32,)).await?, 3);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&sub, "lastIndexOf", "(II)I", (b'l' as i32, 2)).await?, 2);

    let pattern = JavaLangString::from_rust_string(&jvm, "llo").await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&sub, "indexOf", "(Ljava/lang/String;)I", (pattern.clone(),))
            .await?,
        2
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&sub, "lastIndexOf", "(Ljava/lang/String;)I", (pattern,))
            .await?,
        2
    );

    let outside = JavaLangString::from_rust_string(&jvm, "y").await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&sub, "lastIndexOf", "(Ljava/lang/String;)I", (outside,))
            .await?,
        -1
    );

    Ok(())
}

#[tokio::test]
async fn test_equality_and_hash_on_substring() -> Result<()> {
    let jvm = test_jvm().await?;

    let parent = JavaLangString::from_rust_string(&jvm, "xxHelloyy").await?;
    let sub: ClassInstanceRef<JavaString> = jvm.invoke_virtual(&parent, "substring", "(II)Ljava/lang/String;", (2, 7)).await?;
    let hello = JavaLangString::from_rust_string(&jvm, "Hello").await?;

    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&sub, "hashCode", "()I", ()).await?,
        jvm.invoke_virtual::<_, i32>(&hello, "hashCode", "()I", ()).await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(&sub, "equals", "(Ljava/lang/Object;)Z", (hello.clone(),))
            .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(&sub, "equals", "(Ljava/lang/Object;)Z", (sub.clone(),))
            .await?
    );

    let prefix = JavaLangString::from_rust_string(&jvm, "xxHel").await?;
    assert!(!jvm.invoke_virtual::<_, bool>(&sub, "equals", "(Ljava/lang/Object;)Z", (prefix,)).await?);

    let object = jvm.new_class("java/lang/Object", "()V", ()).await?;
    assert!(!jvm.invoke_virtual::<_, bool>(&sub, "equals", "(Ljava/lang/Object;)Z", (object,)).await?);

    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&sub, "compareTo", "(Ljava/lang/String;)I", (hello.clone(),))
            .await?,
        0
    );
    let upper = JavaLangString::from_rust_string(&jvm, "HELLO").await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&sub, "compareToIgnoreCase", "(Ljava/lang/String;)I", (upper,))
            .await?,
        0
    );

    Ok(())
}

#[tokio::test]
async fn test_string_api_on_substring() -> Result<()> {
    let jvm = test_jvm().await?;

    let parent = JavaLangString::from_rust_string(&jvm, "xxHelloyy").await?;
    let sub: ClassInstanceRef<JavaString> = jvm.invoke_virtual(&parent, "substring", "(II)Ljava/lang/String;", (2, 7)).await?;

    let prefix = JavaLangString::from_rust_string(&jvm, "He").await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&sub, "startsWith", "(Ljava/lang/String;)Z", (prefix,))
            .await?
    );

    let suffix = JavaLangString::from_rust_string(&jvm, "lo").await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&sub, "endsWith", "(Ljava/lang/String;)Z", (suffix,))
            .await?
    );

    let hello = JavaLangString::from_rust_string(&jvm, "Hello").await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&sub, "regionMatches", "(ILjava/lang/String;II)Z", (0, hello, 0, 5))
            .await?
    );

    let replaced: ClassInstanceRef<JavaString> = jvm
        .invoke_virtual(&sub, "replace", "(CC)Ljava/lang/String;", (b'l' as JavaChar, b'L' as JavaChar))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &replaced).await?, "HeLLo");

    let other = JavaLangString::from_rust_string(&jvm, "!").await?;
    let concat: ClassInstanceRef<JavaString> = jvm
        .invoke_virtual(&sub, "concat", "(Ljava/lang/String;)Ljava/lang/String;", (other,))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &concat).await?, "Hello!");

    let bytes: ClassInstanceRef<Array<i8>> = jvm.invoke_virtual(&sub, "getBytes", "()[B", ()).await?;
    assert_eq!(
        jvm.load_array::<i8>(&bytes, 0, 5).await?,
        b"Hello".iter().map(|&b| b as i8).collect::<Vec<_>>()
    );

    Ok(())
}

#[tokio::test]
async fn test_trim_on_substring_shares_buffer() -> Result<()> {
    let jvm = test_jvm().await?;

    let parent = JavaLangString::from_rust_string(&jvm, "xx  hi  yy").await?;
    let sub: ClassInstanceRef<JavaString> = jvm.invoke_virtual(&parent, "substring", "(II)Ljava/lang/String;", (2, 8)).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &sub).await?, "  hi  ");

    let trimmed: ClassInstanceRef<JavaString> = jvm.invoke_virtual(&sub, "trim", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &trimmed).await?, "hi");

    let parent_value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&parent, "value", "[C").await?;
    let trimmed_value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&trimmed, "value", "[C").await?;
    assert_eq!(parent_value.identity(), trimmed_value.identity());
    assert_eq!(jvm.get_field::<i32>(&trimmed, "offset", "I").await?, 4);
    assert_eq!(jvm.get_field::<i32>(&trimmed, "count", "I").await?, 2);

    let no_trim: ClassInstanceRef<JavaString> = jvm.invoke_virtual(&parent, "substring", "(II)Ljava/lang/String;", (4, 6)).await?;
    let same: ClassInstanceRef<JavaString> = jvm.invoke_virtual(&no_trim, "trim", "()Ljava/lang/String;", ()).await?;
    assert_eq!(no_trim.identity(), same.identity());

    Ok(())
}

#[tokio::test]
async fn test_substring_bounds_on_substring() -> Result<()> {
    let jvm = test_jvm().await?;

    let parent = JavaLangString::from_rust_string(&jvm, "xxHelloyy").await?;
    let sub: ClassInstanceRef<JavaString> = jvm.invoke_virtual(&parent, "substring", "(II)Ljava/lang/String;", (2, 7)).await?;

    let result: Result<ClassInstanceRef<JavaString>> = jvm.invoke_virtual(&sub, "substring", "(I)Ljava/lang/String;", (-1,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("substring(-1) must throw");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/StringIndexOutOfBoundsException"));

    let result: Result<ClassInstanceRef<JavaString>> = jvm.invoke_virtual(&sub, "substring", "(I)Ljava/lang/String;", (6,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("substring beyond count must throw");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/StringIndexOutOfBoundsException"));

    for (begin, end) in [(3, 999), (4, 2), (-1, 3)] {
        let result: Result<ClassInstanceRef<JavaString>> = jvm.invoke_virtual(&sub, "substring", "(II)Ljava/lang/String;", (begin, end)).await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("substring({begin}, {end}) must throw");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/StringIndexOutOfBoundsException"));
    }

    Ok(())
}

#[tokio::test]
async fn test_substring_preserves_unpaired_surrogate() -> Result<()> {
    let jvm = test_jvm().await?;

    let mut chars = jvm.instantiate_array("C", 3).await?;
    jvm.store_array(&mut chars, 0, [0x61 as JavaChar, 0xd800, 0x62]).await?;
    let string = jvm.new_class("java/lang/String", "([C)V", (chars,)).await?;

    let sub: ClassInstanceRef<JavaString> = jvm.invoke_virtual(&string, "substring", "(II)Ljava/lang/String;", (1, 2)).await?;

    assert_eq!(jvm.invoke_virtual::<_, JavaChar>(&sub, "charAt", "(I)C", (0,)).await?, 0xd800);

    let sub_chars: ClassInstanceRef<Array<JavaChar>> = jvm.invoke_virtual(&sub, "toCharArray", "()[C", ()).await?;
    assert_eq!(jvm.load_array::<JavaChar>(&sub_chars, 0, 1).await?, [0xd800]);

    let dst = jvm.instantiate_array("C", 1).await?;
    let _: () = jvm.invoke_virtual(&sub, "getChars", "(II[CI)V", (0, 1, dst.clone(), 0)).await?;
    assert_eq!(jvm.load_array::<JavaChar>(&dst, 0, 1).await?, [0xd800]);

    Ok(())
}

#[tokio::test]
async fn test_equals_uses_utf16_code_units() -> Result<()> {
    let jvm = test_jvm().await?;

    let mut first_chars = jvm.instantiate_array("C", 1).await?;
    jvm.store_array(&mut first_chars, 0, [0xd800 as JavaChar]).await?;
    let first = jvm.new_class("java/lang/String", "([C)V", (first_chars,)).await?;

    let mut second_chars = jvm.instantiate_array("C", 1).await?;
    jvm.store_array(&mut second_chars, 0, [0xd801 as JavaChar]).await?;
    let second = jvm.new_class("java/lang/String", "([C)V", (second_chars,)).await?;

    let mut third_chars = jvm.instantiate_array("C", 1).await?;
    jvm.store_array(&mut third_chars, 0, [0xd800 as JavaChar]).await?;
    let third = jvm.new_class("java/lang/String", "([C)V", (third_chars,)).await?;

    assert!(
        !jvm.invoke_virtual::<_, bool>(&first, "equals", "(Ljava/lang/Object;)Z", (second,))
            .await?
    );
    assert!(jvm.invoke_virtual::<_, bool>(&first, "equals", "(Ljava/lang/Object;)Z", (third,)).await?);

    Ok(())
}

#[tokio::test]
async fn test_from_rust_string_representation() -> Result<()> {
    let jvm = test_jvm().await?;

    let string = JavaLangString::from_rust_string(&jvm, "test").await?;

    let value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&string, "value", "[C").await?;
    assert_eq!(jvm.array_length(&value).await?, 4);
    assert_eq!(jvm.get_field::<i32>(&string, "offset", "I").await?, 0);
    assert_eq!(jvm.get_field::<i32>(&string, "count", "I").await?, 4);
    assert_eq!(JavaLangString::to_rust_string(&jvm, &string).await?, "test");

    Ok(())
}

#[tokio::test]
async fn test_init_with_string_shares_full_range_value() -> Result<()> {
    let jvm = test_jvm().await?;

    let original = JavaLangString::from_rust_string(&jvm, "Hello").await?;
    let copy = jvm.new_class("java/lang/String", "(Ljava/lang/String;)V", (original.clone(),)).await?;

    let original_value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&original, "value", "[C").await?;
    let copy_value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&copy, "value", "[C").await?;
    assert_eq!(original_value.identity(), copy_value.identity());
    assert_eq!(jvm.get_field::<i32>(&copy, "offset", "I").await?, 0);
    assert_eq!(jvm.get_field::<i32>(&copy, "count", "I").await?, 5);
    assert!(
        jvm.invoke_virtual::<_, bool>(&copy, "equals", "(Ljava/lang/Object;)Z", (original,))
            .await?
    );

    Ok(())
}

#[tokio::test]
async fn test_init_with_string_detaches_substring_with_exact_size_copy() -> Result<()> {
    let jvm = test_jvm().await?;

    let parent = JavaLangString::from_rust_string(&jvm, "HelloWorld").await?;
    let sub: ClassInstanceRef<JavaString> = jvm.invoke_virtual(&parent, "substring", "(II)Ljava/lang/String;", (2, 5)).await?;
    let detached = jvm.new_class("java/lang/String", "(Ljava/lang/String;)V", (sub.clone(),)).await?;

    let sub_value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&sub, "value", "[C").await?;
    let detached_value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&detached, "value", "[C").await?;
    assert_ne!(sub_value.identity(), detached_value.identity());
    assert_eq!(jvm.array_length(&detached_value).await?, 3);
    assert_eq!(jvm.get_field::<i32>(&detached, "offset", "I").await?, 0);
    assert_eq!(jvm.get_field::<i32>(&detached, "count", "I").await?, 3);
    assert!(
        jvm.invoke_virtual::<_, bool>(&detached, "equals", "(Ljava/lang/Object;)Z", (sub,))
            .await?
    );
    assert_eq!(JavaLangString::to_rust_string(&jvm, &detached).await?, "llo");

    Ok(())
}

#[tokio::test]
async fn test_init_with_string_buffer_is_independent_of_buffer() -> Result<()> {
    let jvm = test_jvm().await?;

    let string_buffer = jvm.new_class("java/lang/StringBuffer", "()V", ()).await?;
    let hello = JavaLangString::from_rust_string(&jvm, "Hello").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&string_buffer, "append", "(Ljava/lang/String;)Ljava/lang/StringBuffer;", (hello,))
        .await?;

    let string = jvm
        .new_class("java/lang/String", "(Ljava/lang/StringBuffer;)V", (string_buffer.clone(),))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &string).await?, "Hello");

    let world = JavaLangString::from_rust_string(&jvm, "World").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&string_buffer, "append", "(Ljava/lang/String;)Ljava/lang/StringBuffer;", (world,))
        .await?;
    let _: () = jvm.invoke_virtual(&string_buffer, "setCharAt", "(IC)V", (0, b'X' as JavaChar)).await?;

    assert_eq!(JavaLangString::to_rust_string(&jvm, &string).await?, "Hello");

    Ok(())
}

#[tokio::test]
async fn test_str_01_string_declares_jdk12_interfaces_and_access() -> Result<()> {
    let jvm = test_jvm().await?;
    let class = jvm.get_class("java/lang/String").expect("String must be loaded");
    let interfaces = class.definition.interface_names();

    assert!(interfaces.iter().any(|name| name == "java/lang/Comparable"));
    assert!(interfaces.iter().any(|name| name == "java/io/Serializable"));
    assert!(
        class
            .definition
            .access_flags()
            .contains(ClassAccessFlags::PUBLIC | ClassAccessFlags::FINAL)
    );
    assert!(
        class
            .definition
            .method("compareTo", "(Ljava/lang/String;)I", false)
            .expect("typed compareTo")
            .access_flags()
            .contains(MethodAccessFlags::PUBLIC)
    );
    assert!(
        class
            .definition
            .method("compareTo", "(Ljava/lang/Object;)I", false)
            .expect("raw compareTo")
            .access_flags()
            .contains(MethodAccessFlags::PUBLIC)
    );

    Ok(())
}

#[tokio::test]
async fn test_str_02_compare_to_uses_utf16_code_units_and_bridge_exceptions() -> Result<()> {
    let jvm = test_jvm().await?;

    let mut supplementary_chars = jvm.instantiate_array("C", 2).await?;
    jvm.store_array(&mut supplementary_chars, 0, [0xd83d as JavaChar, 0xde00 as JavaChar])
        .await?;
    let supplementary = jvm.new_class("java/lang/String", "([C)V", (supplementary_chars,)).await?;
    let mut private_use_chars = jvm.instantiate_array("C", 1).await?;
    jvm.store_array(&mut private_use_chars, 0, [0xe000 as JavaChar]).await?;
    let private_use = jvm.new_class("java/lang/String", "([C)V", (private_use_chars,)).await?;

    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&supplementary, "compareTo", "(Ljava/lang/String;)I", (private_use.clone(),))
            .await?,
        0xd83d - 0xe000
    );

    let mut first_unpaired_chars = jvm.instantiate_array("C", 1).await?;
    jvm.store_array(&mut first_unpaired_chars, 0, [0xd800 as JavaChar]).await?;
    let first_unpaired = jvm.new_class("java/lang/String", "([C)V", (first_unpaired_chars,)).await?;
    let mut second_unpaired_chars = jvm.instantiate_array("C", 1).await?;
    jvm.store_array(&mut second_unpaired_chars, 0, [0xd801 as JavaChar]).await?;
    let second_unpaired = jvm.new_class("java/lang/String", "([C)V", (second_unpaired_chars,)).await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&first_unpaired, "compareTo", "(Ljava/lang/String;)I", (second_unpaired.clone(),),)
            .await?,
        -1
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&first_unpaired, "compareTo", "(Ljava/lang/Object;)I", (second_unpaired,))
            .await?,
        -1
    );

    let a = JavaLangString::from_rust_string(&jvm, "a").await?;
    let ac = JavaLangString::from_rust_string(&jvm, "ac").await?;
    let az = JavaLangString::from_rust_string(&jvm, "az").await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&ac, "compareTo", "(Ljava/lang/String;)I", (az,)).await?, -23);
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&a, "compareTo", "(Ljava/lang/String;)I", (ac.clone(),))
            .await?,
        -1
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&ac, "compareTo", "(Ljava/lang/Object;)I", (ac.clone(),))
            .await?,
        0
    );

    let null: ClassInstanceRef<Object> = None.into();
    let result: Result<i32> = jvm.invoke_virtual(&ac, "compareTo", "(Ljava/lang/String;)I", (null.clone(),)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("String.compareTo(String) must reject null");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let result: Result<i32> = jvm.invoke_virtual(&ac, "compareTo", "(Ljava/lang/Object;)I", (null,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("String.compareTo(Object) must reject null");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let object = jvm.new_class("java/lang/Object", "()V", ()).await?;
    let result: Result<i32> = jvm.invoke_virtual(&ac, "compareTo", "(Ljava/lang/Object;)I", (object,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("String.compareTo(Object) must reject non-String values");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/ClassCastException"));

    Ok(())
}

#[tokio::test]
async fn test_str_03_compare_to_ignore_case() -> Result<()> {
    let jvm = test_jvm().await?;
    let mixed = JavaLangString::from_rust_string(&jvm, "AbC").await?;
    let lower = JavaLangString::from_rust_string(&jvm, "aBc").await?;
    let later = JavaLangString::from_rust_string(&jvm, "abd").await?;

    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&mixed, "compareToIgnoreCase", "(Ljava/lang/String;)I", (lower,))
            .await?,
        0
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&mixed, "compareToIgnoreCase", "(Ljava/lang/String;)I", (later,))
            .await?,
        -1
    );

    let null: ClassInstanceRef<JavaString> = None.into();
    let result: Result<i32> = jvm.invoke_virtual(&mixed, "compareToIgnoreCase", "(Ljava/lang/String;)I", (null,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("compareToIgnoreCase must reject null");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    Ok(())
}

#[tokio::test]
async fn test_str_04_last_index_of_string_uses_utf16_indices() -> Result<()> {
    let jvm = test_jvm().await?;
    let string = JavaLangString::from_rust_string(&jvm, "a😀ba😀b").await?;
    let emoji = JavaLangString::from_rust_string(&jvm, "😀").await?;
    let empty = JavaLangString::from_rust_string(&jvm, "").await?;

    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&string, "lastIndexOf", "(Ljava/lang/String;)I", (emoji.clone(),))
            .await?,
        5
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&string, "lastIndexOf", "(Ljava/lang/String;I)I", (emoji.clone(), 4))
            .await?,
        1
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&string, "lastIndexOf", "(Ljava/lang/String;I)I", (emoji, -1))
            .await?,
        -1
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&string, "lastIndexOf", "(Ljava/lang/String;I)I", (empty.clone(), -1))
            .await?,
        -1
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&string, "lastIndexOf", "(Ljava/lang/String;I)I", (empty.clone(), 8))
            .await?,
        8
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&string, "lastIndexOf", "(Ljava/lang/String;I)I", (empty, 9))
            .await?,
        8
    );

    let null: ClassInstanceRef<JavaString> = None.into();
    let result: Result<i32> = jvm.invoke_virtual(&string, "lastIndexOf", "(Ljava/lang/String;)I", (null,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("lastIndexOf(String) must reject null");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    Ok(())
}

#[tokio::test]
async fn test_str_05_copy_value_of_copies_and_checks_ranges() -> Result<()> {
    let jvm = test_jvm().await?;
    let mut chars = jvm.instantiate_array("C", 4).await?;
    jvm.store_array(&mut chars, 0, ['a' as JavaChar, 'b' as JavaChar, 'c' as JavaChar, 'd' as JavaChar])
        .await?;

    let full: ClassInstanceRef<JavaString> = jvm
        .invoke_static("java/lang/String", "copyValueOf", "([C)Ljava/lang/String;", (chars.clone(),))
        .await?;
    let partial: ClassInstanceRef<JavaString> = jvm
        .invoke_static("java/lang/String", "copyValueOf", "([CII)Ljava/lang/String;", (chars.clone(), 1, 2))
        .await?;
    jvm.store_array(&mut chars, 0, ['z' as JavaChar]).await?;

    assert_eq!(JavaLangString::to_rust_string(&jvm, &full).await?, "abcd");
    assert_eq!(JavaLangString::to_rust_string(&jvm, &partial).await?, "bc");

    let null: ClassInstanceRef<Array<JavaChar>> = None.into();
    let result: Result<ClassInstanceRef<JavaString>> = jvm
        .invoke_static("java/lang/String", "copyValueOf", "([C)Ljava/lang/String;", (null,))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("copyValueOf must reject null");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    for (offset, count) in [(-1, 1), (0, -1), (3, 2)] {
        let result: Result<ClassInstanceRef<JavaString>> = jvm
            .invoke_static(
                "java/lang/String",
                "copyValueOf",
                "([CII)Ljava/lang/String;",
                (chars.clone(), offset, count),
            )
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("copyValueOf must reject range ({offset}, {count})");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/IndexOutOfBoundsException"));
    }

    Ok(())
}

#[tokio::test]
async fn test_str_06_region_matches_without_ignore_case() -> Result<()> {
    let jvm = test_jvm().await?;
    let source = JavaLangString::from_rust_string(&jvm, "a😀bc").await?;
    let same = JavaLangString::from_rust_string(&jvm, "x😀by").await?;

    assert!(
        jvm.invoke_virtual::<_, bool>(&source, "regionMatches", "(ILjava/lang/String;II)Z", (1, same.clone(), 1, 3))
            .await?
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(&source, "regionMatches", "(ILjava/lang/String;II)Z", (-1, same.clone(), 1, 1))
            .await?
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(&source, "regionMatches", "(ILjava/lang/String;II)Z", (1, same.clone(), 1, 99))
            .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(&source, "regionMatches", "(ILjava/lang/String;II)Z", (1, same, 1, -1))
            .await?
    );

    let mut first_surrogate = jvm.instantiate_array("C", 1).await?;
    jvm.store_array(&mut first_surrogate, 0, [0xd800 as JavaChar]).await?;
    let first_surrogate = jvm.new_class("java/lang/String", "([C)V", (first_surrogate,)).await?;
    let mut second_surrogate = jvm.instantiate_array("C", 1).await?;
    jvm.store_array(&mut second_surrogate, 0, [0xd801 as JavaChar]).await?;
    let second_surrogate = jvm.new_class("java/lang/String", "([C)V", (second_surrogate,)).await?;
    assert!(
        !jvm.invoke_virtual::<_, bool>(&first_surrogate, "regionMatches", "(ILjava/lang/String;II)Z", (0, second_surrogate, 0, 1),)
            .await?
    );

    let null: ClassInstanceRef<JavaString> = None.into();
    let result: Result<bool> = jvm
        .invoke_virtual(&source, "regionMatches", "(ILjava/lang/String;II)Z", (0, null, 0, 0))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("regionMatches must reject null");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    Ok(())
}

#[tokio::test]
async fn test_region_matches_non_positive_len() -> Result<()> {
    let jvm = test_jvm().await?;
    let source = JavaLangString::from_rust_string(&jvm, "Hello").await?;
    let other = JavaLangString::from_rust_string(&jvm, "World").await?;

    for len in [0, -1, i32::MIN] {
        assert!(
            jvm.invoke_virtual::<_, bool>(&source, "regionMatches", "(ILjava/lang/String;II)Z", (1, other.clone(), 2, len))
                .await?
        );
        assert!(
            jvm.invoke_virtual::<_, bool>(&source, "regionMatches", "(ZILjava/lang/String;II)Z", (true, 1, other.clone(), 2, len))
                .await?
        );
    }

    assert!(
        jvm.invoke_virtual::<_, bool>(&source, "regionMatches", "(ILjava/lang/String;II)Z", (5, other.clone(), 5, 0))
            .await?
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(&source, "regionMatches", "(ILjava/lang/String;II)Z", (6, other.clone(), 0, 0))
            .await?
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(&source, "regionMatches", "(ILjava/lang/String;II)Z", (0, other.clone(), 0, i32::MAX))
            .await?
    );

    let sub: ClassInstanceRef<JavaString> = jvm.invoke_virtual(&source, "substring", "(II)Ljava/lang/String;", (1, 3)).await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&sub, "regionMatches", "(ILjava/lang/String;II)Z", (2, other, 0, -1))
            .await?
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(&sub, "regionMatches", "(ILjava/lang/String;II)Z", (3, source, 0, 0))
            .await?
    );

    Ok(())
}

#[tokio::test]
async fn test_str_07_locale_case_overloads_and_float_formatting() -> Result<()> {
    let jvm = test_jvm().await?;
    let language = JavaLangString::from_rust_string(&jvm, "en").await?;
    let locale = jvm.new_class("java/util/Locale", "(Ljava/lang/String;)V", (language,)).await?;
    let mixed = JavaLangString::from_rust_string(&jvm, "AbC").await?;

    let lower: ClassInstanceRef<JavaString> = jvm
        .invoke_virtual(&mixed, "toLowerCase", "(Ljava/util/Locale;)Ljava/lang/String;", (locale.clone(),))
        .await?;
    let upper: ClassInstanceRef<JavaString> = jvm
        .invoke_virtual(&mixed, "toUpperCase", "(Ljava/util/Locale;)Ljava/lang/String;", (locale,))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &lower).await?, "abc");
    assert_eq!(JavaLangString::to_rust_string(&jvm, &upper).await?, "ABC");

    let null: ClassInstanceRef<Object> = None.into();
    let result: Result<ClassInstanceRef<JavaString>> = jvm
        .invoke_virtual(&mixed, "toLowerCase", "(Ljava/util/Locale;)Ljava/lang/String;", (null,))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("locale case conversion must reject null Locale");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let null: ClassInstanceRef<Object> = None.into();
    let result: Result<ClassInstanceRef<JavaString>> = jvm
        .invoke_virtual(&mixed, "toUpperCase", "(Ljava/util/Locale;)Ljava/lang/String;", (null,))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("uppercase locale conversion must reject null Locale");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    for (descriptor, value, expected) in [
        ("(F)Ljava/lang/String;", 0.0f64, "0.0"),
        ("(F)Ljava/lang/String;", -0.0f64, "-0.0"),
        ("(F)Ljava/lang/String;", f32::INFINITY as f64, "Infinity"),
        ("(D)Ljava/lang/String;", f64::NAN, "NaN"),
        ("(D)Ljava/lang/String;", 1.0e20, "1.0E20"),
    ] {
        let text: ClassInstanceRef<JavaString> = if descriptor.starts_with("(F)") {
            jvm.invoke_static("java/lang/String", "valueOf", descriptor, (value as f32,)).await?
        } else {
            jvm.invoke_static("java/lang/String", "valueOf", descriptor, (value,)).await?
        };
        assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, expected);
    }

    Ok(())
}

#[tokio::test]
async fn test_empty_substring_behaves_like_empty_string() -> Result<()> {
    let jvm = test_jvm().await?;

    let parent = JavaLangString::from_rust_string(&jvm, "HelloWorld").await?;

    // offset ends exactly at value.length (offset 10, count 0)
    let tail_empty: ClassInstanceRef<JavaString> = jvm.invoke_virtual(&parent, "substring", "(I)Ljava/lang/String;", (10,)).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&tail_empty, "length", "()I", ()).await?, 0);
    assert_eq!(JavaLangString::to_rust_string(&jvm, &tail_empty).await?, "");
    assert_eq!(jvm.invoke_virtual::<_, i32>(&tail_empty, "hashCode", "()I", ()).await?, 0);

    let mid_empty: ClassInstanceRef<JavaString> = jvm.invoke_virtual(&parent, "substring", "(II)Ljava/lang/String;", (5, 5)).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&mid_empty, "length", "()I", ()).await?, 0);

    let empty = JavaLangString::from_rust_string(&jvm, "").await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&mid_empty, "equals", "(Ljava/lang/Object;)Z", (empty.clone(),))
            .await?
    );

    // searches on empty substring
    assert_eq!(jvm.invoke_virtual::<_, i32>(&mid_empty, "indexOf", "(I)I", (b'l' as i32,)).await?, -1);
    let empty_pattern = JavaLangString::from_rust_string(&jvm, "").await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&mid_empty, "indexOf", "(Ljava/lang/String;)I", (empty_pattern,))
            .await?,
        0
    );

    // trim/toCharArray/charAt on empty substring
    let trimmed: ClassInstanceRef<JavaString> = jvm.invoke_virtual(&mid_empty, "trim", "()Ljava/lang/String;", ()).await?;
    assert_eq!(mid_empty.identity(), trimmed.identity());
    let chars: ClassInstanceRef<Array<JavaChar>> = jvm.invoke_virtual(&mid_empty, "toCharArray", "()[C", ()).await?;
    assert_eq!(jvm.array_length(&chars).await?, 0);
    let result: Result<JavaChar> = jvm.invoke_virtual(&mid_empty, "charAt", "(I)C", (0,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("charAt(0) on empty substring must throw");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/StringIndexOutOfBoundsException"));

    // intern of empty substring meets the pooled empty string
    let interned: ClassInstanceRef<JavaString> = jvm.invoke_virtual(&mid_empty, "intern", "()Ljava/lang/String;", ()).await?;
    let pooled = jvm.intern_string("").await?;
    assert_eq!(interned.identity(), ClassInstanceRef::<JavaString>::from(pooled).identity());

    Ok(())
}

#[tokio::test]
async fn test_trim_to_empty_on_all_whitespace_substring() -> Result<()> {
    let jvm = test_jvm().await?;

    let parent = JavaLangString::from_rust_string(&jvm, "ab   cd").await?;
    let blank: ClassInstanceRef<JavaString> = jvm.invoke_virtual(&parent, "substring", "(II)Ljava/lang/String;", (2, 5)).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &blank).await?, "   ");

    let trimmed: ClassInstanceRef<JavaString> = jvm.invoke_virtual(&blank, "trim", "()Ljava/lang/String;", ()).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&trimmed, "length", "()I", ()).await?, 0);
    assert_eq!(JavaLangString::to_rust_string(&jvm, &trimmed).await?, "");

    Ok(())
}

#[tokio::test]
async fn test_last_index_of_from_beyond_count_on_substring() -> Result<()> {
    let jvm = test_jvm().await?;

    let parent = JavaLangString::from_rust_string(&jvm, "xxHelloyy").await?;
    let sub: ClassInstanceRef<JavaString> = jvm.invoke_virtual(&parent, "substring", "(II)Ljava/lang/String;", (2, 7)).await?;

    // fromIndex beyond count is clamped and must not see parent's 'y'
    assert_eq!(jvm.invoke_virtual::<_, i32>(&sub, "lastIndexOf", "(II)I", (b'o' as i32, 99)).await?, 4);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&sub, "lastIndexOf", "(II)I", (b'y' as i32, 99)).await?, -1);

    let pattern = JavaLangString::from_rust_string(&jvm, "He").await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&sub, "lastIndexOf", "(Ljava/lang/String;I)I", (pattern, 99))
            .await?,
        0
    );

    Ok(())
}

#[tokio::test]
async fn test_shared_constructor_rejects_invalid_range() -> Result<()> {
    let jvm = test_jvm().await?;

    let make_chars = async || -> Result<_> {
        let mut chars = jvm.instantiate_array("C", 5).await?;
        jvm.store_array(&mut chars, 0, "Hello".encode_utf16().collect::<Vec<_>>()).await?;
        Ok(chars)
    };

    for (offset, count) in [(-1, 3), (0, -1), (i32::MIN, 1), (0, 6), (3, 3), (i32::MAX, i32::MAX)] {
        let chars = make_chars().await?;
        let result = jvm.new_class("java/lang/String", "(II[C)V", (offset, count, chars)).await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("String({offset}, {count}, [C) must be rejected");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/StringIndexOutOfBoundsException"));
    }

    let null: ClassInstanceRef<Array<JavaChar>> = None.into();
    let result = jvm.new_class("java/lang/String", "(II[C)V", (0, 0, null)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("String(0, 0, null) must be rejected");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let chars = make_chars().await?;
    let whole = jvm.new_class("java/lang/String", "(II[C)V", (0, 5, chars)).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &whole).await?, "Hello");

    Ok(())
}

#[tokio::test]
async fn test_corrupted_length_fields_raise_java_exceptions() -> Result<()> {
    let jvm = test_jvm().await?;

    for field in ["offset", "count"] {
        let mut string = JavaLangString::from_rust_string(&jvm, "Hello").await?;
        jvm.put_field(&mut string, field, "I", -1i32).await?;

        let result: Result<i32> = jvm.invoke_virtual(&string, "hashCode", "()I", ()).await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("a negative {field} must not be read as a length");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/StringIndexOutOfBoundsException"));

        let result: Result<u16> = jvm.invoke_virtual(&string, "charAt", "(I)C", (0i32,)).await;
        assert!(matches!(result, Err(JavaError::JavaException(_))));

        let result = JavaLangString::to_rust_string(&jvm, &string).await;
        assert!(matches!(result, Err(JavaError::JavaException(_))));
    }

    let mut string = JavaLangString::from_rust_string(&jvm, "Hello").await?;
    jvm.put_field(&mut string, "count", "I", i32::MAX).await?;
    let result: Result<i32> = jvm.invoke_virtual(&string, "hashCode", "()I", ()).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("a count past the backing array must not be read");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/ArrayIndexOutOfBoundsException"));

    Ok(())
}
