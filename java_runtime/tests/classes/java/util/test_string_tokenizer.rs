use alloc::vec::Vec;

use java_constants::{ClassAccessFlags, MethodAccessFlags};
use java_runtime::classes::java::{
    lang::{Object, String},
    util::StringTokenizer,
};
use jvm::{Array, ClassInstanceRef, JavaChar, JavaError, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn tok_01_constructors_and_utf16_delimiters() -> Result<()> {
    let proto = StringTokenizer::as_proto();
    assert_eq!(proto.parent_class, Some("java/lang/Object"));
    assert_eq!(proto.interfaces, alloc::vec!["java/util/Enumeration"]);
    assert!(proto.access_flags.contains(ClassAccessFlags::PUBLIC));
    for descriptor in [
        "(Ljava/lang/String;)V",
        "(Ljava/lang/String;Ljava/lang/String;)V",
        "(Ljava/lang/String;Ljava/lang/String;Z)V",
    ] {
        let constructor = proto
            .methods
            .iter()
            .find(|method| method.name == "<init>" && method.descriptor == descriptor)
            .expect("StringTokenizer constructor");
        assert!(constructor.access_flags.contains(MethodAccessFlags::PUBLIC));
    }
    for (name, descriptor) in [
        ("hasMoreTokens", "()Z"),
        ("nextToken", "()Ljava/lang/String;"),
        ("nextToken", "(Ljava/lang/String;)Ljava/lang/String;"),
        ("countTokens", "()I"),
        ("hasMoreElements", "()Z"),
        ("nextElement", "()Ljava/lang/Object;"),
    ] {
        let method = proto
            .methods
            .iter()
            .find(|method| method.name == name && method.descriptor == descriptor)
            .expect("StringTokenizer token method");
        assert!(method.access_flags.contains(MethodAccessFlags::PUBLIC));
    }
    let fields = proto
        .fields
        .iter()
        .map(|field| (field.name.as_str(), field.descriptor.as_str()))
        .collect::<Vec<_>>();
    assert_eq!(
        fields,
        [
            ("str", "Ljava/lang/String;"),
            ("delimiters", "Ljava/lang/String;"),
            ("currentPosition", "I"),
            ("maxPosition", "I"),
            ("returnDelimiters", "Z"),
        ]
    );

    let jvm = test_jvm().await?;
    let input = JavaLangString::from_rust_string(&jvm, " one\t two\nthree ").await?;
    let tokenizer = jvm.new_class("java/util/StringTokenizer", "(Ljava/lang/String;)V", (input,)).await?;
    assert_eq!(jvm.get_field::<i32>(&tokenizer, "maxPosition", "I").await?, 16);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&tokenizer, "countTokens", "()I", ()).await?, 3);
    for expected in ["one", "two", "three"] {
        let token: ClassInstanceRef<String> = jvm.invoke_virtual(&tokenizer, "nextToken", "()Ljava/lang/String;", ()).await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &token).await?, expected);
    }

    let mut input_chars = jvm.instantiate_array("C", 5).await?;
    jvm.store_array(
        &mut input_chars,
        0,
        [b'a' as JavaChar, 0xD800, b'b' as JavaChar, 0xD800, b'c' as JavaChar],
    )
    .await?;
    let utf16_input = jvm.new_class("java/lang/String", "([C)V", (input_chars,)).await?;
    let mut delimiter_chars = jvm.instantiate_array("C", 1).await?;
    jvm.store_array(&mut delimiter_chars, 0, [0xD800 as JavaChar]).await?;
    let utf16_delimiter = jvm.new_class("java/lang/String", "([C)V", (delimiter_chars,)).await?;
    let tokenizer = jvm
        .new_class(
            "java/util/StringTokenizer",
            "(Ljava/lang/String;Ljava/lang/String;Z)V",
            (utf16_input, utf16_delimiter, true),
        )
        .await?;

    assert_eq!(jvm.invoke_virtual::<_, i32>(&tokenizer, "countTokens", "()I", ()).await?, 5);
    let first: ClassInstanceRef<String> = jvm.invoke_virtual(&tokenizer, "nextToken", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &first).await?, "a");
    let delimiter: ClassInstanceRef<String> = jvm.invoke_virtual(&tokenizer, "nextToken", "()Ljava/lang/String;", ()).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&delimiter, "length", "()I", ()).await?, 1);
    assert_eq!(jvm.invoke_virtual::<_, JavaChar>(&delimiter, "charAt", "(I)C", (0,)).await?, 0xD800);

    let null_input: ClassInstanceRef<String> = None.into();
    let result = jvm.new_class("java/util/StringTokenizer", "(Ljava/lang/String;)V", (null_input,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("null input must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    Ok(())
}

#[tokio::test]
async fn tok_02_token_api_enumeration_and_delimiter_change() -> Result<()> {
    let jvm = test_jvm().await?;
    let input = JavaLangString::from_rust_string(&jvm, "a,b;c").await?;
    let comma = JavaLangString::from_rust_string(&jvm, ",").await?;
    let tokenizer = jvm
        .new_class("java/util/StringTokenizer", "(Ljava/lang/String;Ljava/lang/String;)V", (input, comma))
        .await?;

    assert!(jvm.invoke_virtual::<_, bool>(&tokenizer, "hasMoreTokens", "()Z", ()).await?);
    assert!(jvm.invoke_virtual::<_, bool>(&tokenizer, "hasMoreElements", "()Z", ()).await?);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&tokenizer, "countTokens", "()I", ()).await?, 2);

    let first: ClassInstanceRef<String> = jvm.invoke_virtual(&tokenizer, "nextToken", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &first).await?, "a");

    let semicolon = JavaLangString::from_rust_string(&jvm, ";").await?;
    let second: ClassInstanceRef<String> = jvm
        .invoke_virtual(&tokenizer, "nextToken", "(Ljava/lang/String;)Ljava/lang/String;", (semicolon,))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &second).await?, ",b");
    assert_eq!(jvm.invoke_virtual::<_, i32>(&tokenizer, "countTokens", "()I", ()).await?, 1);

    let last: ClassInstanceRef<Object> = jvm.invoke_virtual(&tokenizer, "nextElement", "()Ljava/lang/Object;", ()).await?;
    assert!(jvm.is_instance(&**last, "java/lang/String"));
    assert_eq!(JavaLangString::to_rust_string(&jvm, &last).await?, "c");
    assert!(!jvm.invoke_virtual::<_, bool>(&tokenizer, "hasMoreTokens", "()Z", ()).await?);

    let exhausted: Result<ClassInstanceRef<String>> = jvm.invoke_virtual(&tokenizer, "nextToken", "()Ljava/lang/String;", ()).await;
    let Err(JavaError::JavaException(exception)) = exhausted else {
        panic!("exhausted tokenizer must throw NoSuchElementException");
    };
    assert!(jvm.is_instance(&*exception, "java/util/NoSuchElementException"));

    let input = JavaLangString::from_rust_string(&jvm, "a,,b").await?;
    let comma = JavaLangString::from_rust_string(&jvm, ",").await?;
    let tokenizer = jvm
        .new_class(
            "java/util/StringTokenizer",
            "(Ljava/lang/String;Ljava/lang/String;Z)V",
            (input, comma, true),
        )
        .await?;
    let mut tokens = Vec::new();
    while jvm.invoke_virtual::<_, bool>(&tokenizer, "hasMoreElements", "()Z", ()).await? {
        let token: ClassInstanceRef<String> = jvm.invoke_virtual(&tokenizer, "nextElement", "()Ljava/lang/Object;", ()).await?;
        tokens.push(JavaLangString::to_rust_string(&jvm, &token).await?);
    }
    assert_eq!(tokens, ["a", ",", ",", "b"]);

    Ok(())
}

#[tokio::test]
async fn tok_03_next_token_shares_input_value() -> Result<()> {
    let jvm = test_jvm().await?;
    let input = JavaLangString::from_rust_string(&jvm, " one two").await?;
    let tokenizer = jvm
        .new_class("java/util/StringTokenizer", "(Ljava/lang/String;)V", (input.clone(),))
        .await?;

    let input_value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&input, "value", "[C").await?;
    for (expected, offset) in [("one", 1), ("two", 5)] {
        let token: ClassInstanceRef<String> = jvm.invoke_virtual(&tokenizer, "nextToken", "()Ljava/lang/String;", ()).await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &token).await?, expected);

        let token_value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&token, "value", "[C").await?;
        assert_eq!(input_value.identity(), token_value.identity());
        assert_eq!(jvm.get_field::<i32>(&token, "offset", "I").await?, offset);
        assert_eq!(jvm.get_field::<i32>(&token, "count", "I").await?, 3);
    }

    Ok(())
}

#[tokio::test]
async fn tok_04_substring_input_composes_offsets_and_shares_root_value() -> Result<()> {
    let jvm = test_jvm().await?;
    let root = JavaLangString::from_rust_string(&jvm, "xx one two yy").await?;
    let input: ClassInstanceRef<String> = jvm.invoke_virtual(&root, "substring", "(II)Ljava/lang/String;", (3, 10)).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &input).await?, "one two");

    let root_value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&root, "value", "[C").await?;

    let tokenizer = jvm
        .new_class("java/util/StringTokenizer", "(Ljava/lang/String;)V", (input.clone(),))
        .await?;
    assert_eq!(jvm.get_field::<i32>(&tokenizer, "maxPosition", "I").await?, 7);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&tokenizer, "countTokens", "()I", ()).await?, 2);
    for (expected, offset) in [("one", 3), ("two", 7)] {
        let token: ClassInstanceRef<String> = jvm.invoke_virtual(&tokenizer, "nextToken", "()Ljava/lang/String;", ()).await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &token).await?, expected);

        let token_value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&token, "value", "[C").await?;
        assert_eq!(root_value.identity(), token_value.identity());
        assert_eq!(jvm.get_field::<i32>(&token, "offset", "I").await?, offset);
        assert_eq!(jvm.get_field::<i32>(&token, "count", "I").await?, 3);
    }
    assert!(!jvm.invoke_virtual::<_, bool>(&tokenizer, "hasMoreTokens", "()Z", ()).await?);

    let space = JavaLangString::from_rust_string(&jvm, " ").await?;
    let tokenizer = jvm
        .new_class(
            "java/util/StringTokenizer",
            "(Ljava/lang/String;Ljava/lang/String;Z)V",
            (input, space, true),
        )
        .await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&tokenizer, "countTokens", "()I", ()).await?, 3);
    let mut tokens = Vec::new();
    while jvm.invoke_virtual::<_, bool>(&tokenizer, "hasMoreTokens", "()Z", ()).await? {
        let token: ClassInstanceRef<String> = jvm.invoke_virtual(&tokenizer, "nextToken", "()Ljava/lang/String;", ()).await?;
        tokens.push(JavaLangString::to_rust_string(&jvm, &token).await?);
    }
    assert_eq!(tokens, ["one", " ", "two"]);

    Ok(())
}
