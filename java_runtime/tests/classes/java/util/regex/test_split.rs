use alloc::{string::String as RustString, vec::Vec};

use java_constants::MethodAccessFlags;
use java_runtime::{
    classes::java::{
        lang::{CharSequence, String},
        util::regex::Pattern,
    },
    get_runtime_class_proto,
};
use jvm::{Array, ClassInstanceRef, JavaError, Jvm, Result, runtime::JavaLangString};

use test_utils::test_jvm;

async fn split(jvm: &Jvm, input: &str, source: &str, limit: i32) -> Result<Vec<RustString>> {
    let source = JavaLangString::from_rust_string(jvm, source).await?;
    let pattern: ClassInstanceRef<Pattern> = jvm
        .invoke_static(
            "java/util/regex/Pattern",
            "compile",
            "(Ljava/lang/String;)Ljava/util/regex/Pattern;",
            (source,),
        )
        .await?;
    let input: ClassInstanceRef<CharSequence> = JavaLangString::from_rust_string(jvm, input).await?.into();
    let values: ClassInstanceRef<Array<String>> = jvm
        .invoke_virtual(&pattern, "split", "(Ljava/lang/CharSequence;I)[Ljava/lang/String;", (input, limit))
        .await?;
    let mut result = Vec::new();
    for value in jvm
        .load_array::<ClassInstanceRef<String>>(&values, 0, jvm.array_length(&values).await?)
        .await?
    {
        result.push(JavaLangString::to_rust_string(jvm, &value).await?);
    }
    Ok(result)
}

#[tokio::test]
async fn pattern_exposes_java_14_split_methods() -> Result<()> {
    let proto = get_runtime_class_proto("java/util/regex/Pattern").expect("Pattern must be registered");
    for descriptor in [
        "(Ljava/lang/CharSequence;)[Ljava/lang/String;",
        "(Ljava/lang/CharSequence;I)[Ljava/lang/String;",
    ] {
        let method = proto
            .methods
            .iter()
            .find(|method| method.name == "split" && method.descriptor == descriptor)
            .unwrap_or_else(|| panic!("missing Pattern.split{descriptor}"));
        assert_eq!(method.access_flags, MethodAccessFlags::PUBLIC);
    }

    Ok(())
}

#[tokio::test]
async fn split_applies_positive_negative_and_zero_limits() -> Result<()> {
    let jvm = test_jvm().await?;
    for (input, source, limit, expected) in [
        ("boo:and:foo", ":", 2, vec!["boo", "and:foo"]),
        ("boo:and:foo", ":", 5, vec!["boo", "and", "foo"]),
        ("boo:and:foo", ":", -2, vec!["boo", "and", "foo"]),
        ("boo:and:foo", "o", 5, vec!["b", "", ":and:f", "", ""]),
        ("boo:and:foo", "o", -2, vec!["b", "", ":and:f", "", ""]),
        ("boo:and:foo", "o", 0, vec!["b", "", ":and:f"]),
        ("a:", ":", i32::MIN, vec!["a", ""]),
        ("a:", ":", i32::MAX, vec!["a", ""]),
    ] {
        assert_eq!(split(&jvm, input, source, limit).await?, expected);
    }

    Ok(())
}

#[tokio::test]
async fn split_handles_no_match_whole_input_and_empty_input() -> Result<()> {
    let jvm = test_jvm().await?;
    for limit in [0, 2, -1] {
        assert_eq!(split(&jvm, "abc", ":", limit).await?, vec!["abc"]);
    }
    for (limit, expected) in [(0, Vec::<&str>::new()), (-1, vec!["", ""]), (1, vec!["abc"])] {
        assert_eq!(split(&jvm, "abc", "abc", limit).await?, expected);
    }
    for limit in [0, 1, -1] {
        assert_eq!(split(&jvm, "", ":", limit).await?, vec![""]);
        assert_eq!(split(&jvm, "", "", limit).await?, vec![""]);
    }

    Ok(())
}

#[tokio::test]
async fn split_preserves_java_14_zero_width_leading_behavior() -> Result<()> {
    let jvm = test_jvm().await?;
    assert_eq!(split(&jvm, "abc", "^", 0).await?, vec!["abc"]);
    assert_eq!(split(&jvm, "ab", "", 0).await?, vec!["", "a", "b"]);
    assert_eq!(split(&jvm, "ab", "", -1).await?, vec!["", "a", "b", ""]);
    assert_eq!(split(&jvm, "ab", "", 1).await?, vec!["ab"]);
    assert_eq!(split(&jvm, "ab", "^|b", -1).await?, vec!["", "a", ""]);

    Ok(())
}

#[tokio::test]
async fn split_uses_utf16_boundaries_and_keeps_leading_empty_parts() -> Result<()> {
    let jvm = test_jvm().await?;
    assert_eq!(split(&jvm, ":a", ":", -1).await?, vec!["", "a"]);
    assert_eq!(split(&jvm, "A😀B😀", "😀", -1).await?, vec!["A", "B", ""]);

    Ok(())
}

#[tokio::test]
async fn split_without_limit_matches_an_explicit_zero_limit() -> Result<()> {
    let jvm = test_jvm().await?;
    let source = JavaLangString::from_rust_string(&jvm, "o").await?;
    let pattern: ClassInstanceRef<Pattern> = jvm
        .invoke_static(
            "java/util/regex/Pattern",
            "compile",
            "(Ljava/lang/String;)Ljava/util/regex/Pattern;",
            (source,),
        )
        .await?;
    let input: ClassInstanceRef<CharSequence> = JavaLangString::from_rust_string(&jvm, "boo:and:foo").await?.into();
    let implicit: ClassInstanceRef<Array<String>> = jvm
        .invoke_virtual(&pattern, "split", "(Ljava/lang/CharSequence;)[Ljava/lang/String;", (input.clone(),))
        .await?;
    let explicit: ClassInstanceRef<Array<String>> = jvm
        .invoke_virtual(&pattern, "split", "(Ljava/lang/CharSequence;I)[Ljava/lang/String;", (input, 0))
        .await?;

    let implicit = jvm
        .load_array::<ClassInstanceRef<String>>(&implicit, 0, jvm.array_length(&implicit).await?)
        .await?;
    let explicit = jvm
        .load_array::<ClassInstanceRef<String>>(&explicit, 0, jvm.array_length(&explicit).await?)
        .await?;
    assert_eq!(implicit.len(), explicit.len());
    for (implicit, explicit) in implicit.into_iter().zip(explicit) {
        assert_eq!(
            JavaLangString::to_rust_string(&jvm, &implicit).await?,
            JavaLangString::to_rust_string(&jvm, &explicit).await?
        );
    }

    Ok(())
}

#[tokio::test]
async fn split_accepts_string_buffer_and_returns_a_java_string_array() -> Result<()> {
    let jvm = test_jvm().await?;
    let source = JavaLangString::from_rust_string(&jvm, ":").await?;
    let pattern: ClassInstanceRef<Pattern> = jvm
        .invoke_static(
            "java/util/regex/Pattern",
            "compile",
            "(Ljava/lang/String;)Ljava/util/regex/Pattern;",
            (source,),
        )
        .await?;
    let value = JavaLangString::from_rust_string(&jvm, "a:b").await?;
    let buffer = jvm.new_class("java/lang/StringBuffer", "(Ljava/lang/String;)V", (value,)).await?;
    let input: ClassInstanceRef<CharSequence> = ClassInstanceRef::new(Some(buffer));
    let values: ClassInstanceRef<Array<String>> = jvm
        .invoke_virtual(&pattern, "split", "(Ljava/lang/CharSequence;I)[Ljava/lang/String;", (input, -1))
        .await?;
    assert_eq!(values.class_definition().name(), "[Ljava/lang/String;");

    let values = jvm
        .load_array::<ClassInstanceRef<String>>(&values, 0, jvm.array_length(&values).await?)
        .await?;
    let mut result = Vec::new();
    for value in values {
        result.push(JavaLangString::to_rust_string(&jvm, &value).await?);
    }
    assert_eq!(result, vec!["a", "b"]);

    Ok(())
}

#[tokio::test]
async fn split_rejects_a_null_input() -> Result<()> {
    let jvm = test_jvm().await?;
    let source = JavaLangString::from_rust_string(&jvm, ":").await?;
    let pattern: ClassInstanceRef<Pattern> = jvm
        .invoke_static(
            "java/util/regex/Pattern",
            "compile",
            "(Ljava/lang/String;)Ljava/util/regex/Pattern;",
            (source,),
        )
        .await?;
    let input: ClassInstanceRef<CharSequence> = None.into();
    let result: Result<ClassInstanceRef<Array<String>>> = jvm
        .invoke_virtual(&pattern, "split", "(Ljava/lang/CharSequence;I)[Ljava/lang/String;", (input, 0))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Pattern.split(null) must throw");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    Ok(())
}
