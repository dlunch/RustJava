use java_constants::MethodAccessFlags;
use java_runtime::{
    classes::java::{
        lang::{CharSequence, String, StringBuffer},
        util::regex::{Matcher, Pattern},
    },
    get_runtime_class_proto,
};
use jvm::{ClassInstanceRef, JavaError, Jvm, Result, runtime::JavaLangString};

use test_utils::test_jvm;

async fn new_matcher(jvm: &Jvm, source: &str, input: &str) -> Result<ClassInstanceRef<Matcher>> {
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
    jvm.invoke_virtual(&pattern, "matcher", "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;", (input,))
        .await
}

async fn buffer_text(jvm: &Jvm, buffer: &ClassInstanceRef<StringBuffer>) -> Result<alloc::string::String> {
    let value: ClassInstanceRef<String> = jvm.invoke_virtual(buffer, "toString", "()Ljava/lang/String;", ()).await?;
    JavaLangString::to_rust_string(jvm, &value).await
}

#[tokio::test]
async fn matcher_exposes_java_14_replacement_methods() -> Result<()> {
    let proto = get_runtime_class_proto("java/util/regex/Matcher").expect("Matcher must be registered");
    for (name, descriptor) in [
        (
            "appendReplacement",
            "(Ljava/lang/StringBuffer;Ljava/lang/String;)Ljava/util/regex/Matcher;",
        ),
        ("appendTail", "(Ljava/lang/StringBuffer;)Ljava/lang/StringBuffer;"),
        ("replaceAll", "(Ljava/lang/String;)Ljava/lang/String;"),
        ("replaceFirst", "(Ljava/lang/String;)Ljava/lang/String;"),
    ] {
        let method = proto
            .methods
            .iter()
            .find(|method| method.name == name && method.descriptor == descriptor)
            .unwrap_or_else(|| panic!("missing Matcher.{name}{descriptor}"));
        assert_eq!(method.access_flags, MethodAccessFlags::PUBLIC);
    }

    Ok(())
}

#[tokio::test]
async fn append_replacement_expands_groups_and_preserves_unmatched_input() -> Result<()> {
    let jvm = test_jvm().await?;
    let matcher = new_matcher(&jvm, "(cat)", "one cat two cats").await?;
    let buffer: ClassInstanceRef<StringBuffer> = jvm.new_class("java/lang/StringBuffer", "()V", ()).await?.into();
    let replacement = JavaLangString::from_rust_string(&jvm, "<$1>").await?;

    while jvm.invoke_virtual::<_, bool>(&matcher, "find", "()Z", ()).await? {
        let returned: ClassInstanceRef<Matcher> = jvm
            .invoke_virtual(
                &matcher,
                "appendReplacement",
                "(Ljava/lang/StringBuffer;Ljava/lang/String;)Ljava/util/regex/Matcher;",
                (buffer.clone(), replacement.clone()),
            )
            .await?;
        assert_eq!(returned.identity(), matcher.identity());
    }
    let returned: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(
            &matcher,
            "appendTail",
            "(Ljava/lang/StringBuffer;)Ljava/lang/StringBuffer;",
            (buffer.clone(),),
        )
        .await?;
    assert_eq!(returned.identity(), buffer.identity());
    assert_eq!(buffer_text(&jvm, &buffer).await?, "one <cat> two <cat>s");

    Ok(())
}

#[tokio::test]
async fn replacement_parser_handles_group_numbers_unmatched_groups_and_escapes() -> Result<()> {
    let jvm = test_jvm().await?;
    for (source, input, replacement, expected) in [
        ("cat", "cat", "$0!", "cat!"),
        ("(a)?b", "b", "x$1y", "xy"),
        ("(a)(b)", "ab", "$12", "a2"),
        ("(a)(b)(c)(d)(e)(f)(g)(h)(i)(j)(k)(l)", "abcdefghijkl", "$12", "l"),
        ("a", "a", "\\$", "$"),
        ("a", "a", "\\\\", "\\"),
        ("a", "a", "\\q", "q"),
    ] {
        let matcher = new_matcher(&jvm, source, input).await?;
        let replacement = JavaLangString::from_rust_string(&jvm, replacement).await?;
        let result: ClassInstanceRef<String> = jvm
            .invoke_virtual(&matcher, "replaceFirst", "(Ljava/lang/String;)Ljava/lang/String;", (replacement,))
            .await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &result).await?, expected);
    }

    let matcher = new_matcher(&jvm, "😀", "A😀B").await?;
    let replacement = JavaLangString::from_rust_string(&jvm, "한").await?;
    let result: ClassInstanceRef<String> = jvm
        .invoke_virtual(&matcher, "replaceFirst", "(Ljava/lang/String;)Ljava/lang/String;", (replacement,))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &result).await?, "A한B");

    Ok(())
}

#[tokio::test]
async fn replace_all_advances_through_zero_width_matches_without_losing_input() -> Result<()> {
    let jvm = test_jvm().await?;
    let matcher = new_matcher(&jvm, "", "ab").await?;
    let replacement = JavaLangString::from_rust_string(&jvm, "-").await?;
    let result: ClassInstanceRef<String> = jvm
        .invoke_virtual(&matcher, "replaceAll", "(Ljava/lang/String;)Ljava/lang/String;", (replacement,))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &result).await?, "-a-b-");

    Ok(())
}

#[tokio::test]
async fn replacement_accepts_string_buffer_as_a_char_sequence() -> Result<()> {
    let jvm = test_jvm().await?;
    let source = JavaLangString::from_rust_string(&jvm, "a").await?;
    let pattern: ClassInstanceRef<Pattern> = jvm
        .invoke_static(
            "java/util/regex/Pattern",
            "compile",
            "(Ljava/lang/String;)Ljava/util/regex/Pattern;",
            (source,),
        )
        .await?;
    let value = JavaLangString::from_rust_string(&jvm, "aba").await?;
    let buffer: ClassInstanceRef<StringBuffer> = jvm.new_class("java/lang/StringBuffer", "(Ljava/lang/String;)V", (value,)).await?.into();
    let input: ClassInstanceRef<CharSequence> = ClassInstanceRef::new(buffer.clone().instance);
    let matcher: ClassInstanceRef<Matcher> = jvm
        .invoke_virtual(&pattern, "matcher", "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;", (input,))
        .await?;
    let replacement = JavaLangString::from_rust_string(&jvm, "x").await?;
    let result: ClassInstanceRef<String> = jvm
        .invoke_virtual(&matcher, "replaceAll", "(Ljava/lang/String;)Ljava/lang/String;", (replacement,))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &result).await?, "xbx");

    Ok(())
}

#[tokio::test]
async fn malformed_replacements_leave_the_buffer_and_append_position_retryable() -> Result<()> {
    let jvm = test_jvm().await?;
    let matcher = new_matcher(&jvm, "a", "aba").await?;
    assert!(jvm.invoke_virtual::<_, bool>(&matcher, "find", "()Z", ()).await?);
    let seed = JavaLangString::from_rust_string(&jvm, "seed").await?;
    let buffer: ClassInstanceRef<StringBuffer> = jvm.new_class("java/lang/StringBuffer", "(Ljava/lang/String;)V", (seed,)).await?.into();

    for (replacement, expected_exception) in [
        ("$x", "java/lang/IllegalArgumentException"),
        ("$9", "java/lang/IndexOutOfBoundsException"),
        ("$", "java/lang/StringIndexOutOfBoundsException"),
        ("\\", "java/lang/StringIndexOutOfBoundsException"),
    ] {
        let replacement = JavaLangString::from_rust_string(&jvm, replacement).await?;
        let result: Result<ClassInstanceRef<Matcher>> = jvm
            .invoke_virtual(
                &matcher,
                "appendReplacement",
                "(Ljava/lang/StringBuffer;Ljava/lang/String;)Ljava/util/regex/Matcher;",
                (buffer.clone(), replacement),
            )
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("malformed replacement must throw");
        };
        assert!(jvm.is_instance(&*exception, expected_exception));
        assert_eq!(buffer_text(&jvm, &buffer).await?, "seed");
    }

    let null_replacement: ClassInstanceRef<String> = None.into();
    let result: Result<ClassInstanceRef<Matcher>> = jvm
        .invoke_virtual(
            &matcher,
            "appendReplacement",
            "(Ljava/lang/StringBuffer;Ljava/lang/String;)Ljava/util/regex/Matcher;",
            (buffer.clone(), null_replacement),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("null replacement must throw");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));
    assert_eq!(buffer_text(&jvm, &buffer).await?, "seed");

    let malformed = JavaLangString::from_rust_string(&jvm, "$x").await?;
    let null_buffer: ClassInstanceRef<StringBuffer> = None.into();
    let result: Result<ClassInstanceRef<Matcher>> = jvm
        .invoke_virtual(
            &matcher,
            "appendReplacement",
            "(Ljava/lang/StringBuffer;Ljava/lang/String;)Ljava/util/regex/Matcher;",
            (null_buffer.clone(), malformed),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("malformed replacement must be checked before a null buffer");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalArgumentException"));

    let valid = JavaLangString::from_rust_string(&jvm, "x").await?;
    let result: Result<ClassInstanceRef<Matcher>> = jvm
        .invoke_virtual(
            &matcher,
            "appendReplacement",
            "(Ljava/lang/StringBuffer;Ljava/lang/String;)Ljava/util/regex/Matcher;",
            (null_buffer, valid.clone()),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("a null buffer with a valid replacement must throw");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let _: ClassInstanceRef<Matcher> = jvm
        .invoke_virtual(
            &matcher,
            "appendReplacement",
            "(Ljava/lang/StringBuffer;Ljava/lang/String;)Ljava/util/regex/Matcher;",
            (buffer.clone(), valid),
        )
        .await?;
    assert!(jvm.invoke_virtual::<_, bool>(&matcher, "find", "()Z", ()).await?);
    let replacement = JavaLangString::from_rust_string(&jvm, "y").await?;
    let _: ClassInstanceRef<Matcher> = jvm
        .invoke_virtual(
            &matcher,
            "appendReplacement",
            "(Ljava/lang/StringBuffer;Ljava/lang/String;)Ljava/util/regex/Matcher;",
            (buffer.clone(), replacement),
        )
        .await?;
    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(
            &matcher,
            "appendTail",
            "(Ljava/lang/StringBuffer;)Ljava/lang/StringBuffer;",
            (buffer.clone(),),
        )
        .await?;
    assert_eq!(buffer_text(&jvm, &buffer).await?, "seedxby");

    Ok(())
}

#[tokio::test]
async fn append_methods_enforce_state_before_arguments_and_append_tail_needs_no_match() -> Result<()> {
    let jvm = test_jvm().await?;
    let matcher = new_matcher(&jvm, "a", "ba").await?;
    let null_buffer: ClassInstanceRef<StringBuffer> = None.into();
    let malformed = JavaLangString::from_rust_string(&jvm, "$x").await?;
    let result: Result<ClassInstanceRef<Matcher>> = jvm
        .invoke_virtual(
            &matcher,
            "appendReplacement",
            "(Ljava/lang/StringBuffer;Ljava/lang/String;)Ljava/util/regex/Matcher;",
            (null_buffer.clone(), malformed),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("appendReplacement before a match must throw");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalStateException"));

    let before: ClassInstanceRef<StringBuffer> = jvm.new_class("java/lang/StringBuffer", "()V", ()).await?.into();
    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(
            &matcher,
            "appendTail",
            "(Ljava/lang/StringBuffer;)Ljava/lang/StringBuffer;",
            (before.clone(),),
        )
        .await?;
    assert_eq!(buffer_text(&jvm, &before).await?, "ba");

    let result: Result<ClassInstanceRef<StringBuffer>> = jvm
        .invoke_virtual(
            &matcher,
            "appendTail",
            "(Ljava/lang/StringBuffer;)Ljava/lang/StringBuffer;",
            (null_buffer.clone(),),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("appendTail(null) before a match must throw");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    assert!(jvm.invoke_virtual::<_, bool>(&matcher, "find", "()Z", ()).await?);
    assert!(!jvm.invoke_virtual::<_, bool>(&matcher, "find", "()Z", ()).await?);
    let valid = JavaLangString::from_rust_string(&jvm, "x").await?;
    let result: Result<ClassInstanceRef<Matcher>> = jvm
        .invoke_virtual(
            &matcher,
            "appendReplacement",
            "(Ljava/lang/StringBuffer;Ljava/lang/String;)Ljava/util/regex/Matcher;",
            (null_buffer.clone(), valid),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("appendReplacement after a failed find must throw");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalStateException"));

    let result: Result<ClassInstanceRef<StringBuffer>> = jvm
        .invoke_virtual(
            &matcher,
            "appendTail",
            "(Ljava/lang/StringBuffer;)Ljava/lang/StringBuffer;",
            (null_buffer,),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("appendTail(null) after a match must throw");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    Ok(())
}

#[tokio::test]
async fn replace_all_and_first_preserve_their_documented_final_match_state() -> Result<()> {
    let jvm = test_jvm().await?;
    let replacement = JavaLangString::from_rust_string(&jvm, "x").await?;

    let all = new_matcher(&jvm, "a", "aba").await?;
    let result: ClassInstanceRef<String> = jvm
        .invoke_virtual(&all, "replaceAll", "(Ljava/lang/String;)Ljava/lang/String;", (replacement.clone(),))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &result).await?, "xbx");
    let state: Result<i32> = jvm.invoke_virtual(&all, "start", "()I", ()).await;
    let Err(JavaError::JavaException(exception)) = state else {
        panic!("replaceAll must end with an invalid match state");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalStateException"));

    let first = new_matcher(&jvm, "a", "aba").await?;
    let result: ClassInstanceRef<String> = jvm
        .invoke_virtual(&first, "replaceFirst", "(Ljava/lang/String;)Ljava/lang/String;", (replacement,))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &result).await?, "xba");
    assert_eq!(jvm.invoke_virtual::<_, i32>(&first, "start", "()I", ()).await?, 0);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&first, "end", "()I", ()).await?, 1);

    Ok(())
}

#[tokio::test]
async fn replace_without_a_match_does_not_read_a_null_replacement() -> Result<()> {
    let jvm = test_jvm().await?;
    for method in ["replaceAll", "replaceFirst"] {
        let matcher = new_matcher(&jvm, "z", "abc").await?;
        let replacement: ClassInstanceRef<String> = None.into();
        let result: ClassInstanceRef<String> = jvm
            .invoke_virtual(&matcher, method, "(Ljava/lang/String;)Ljava/lang/String;", (replacement,))
            .await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &result).await?, "abc");
    }

    for method in ["replaceAll", "replaceFirst"] {
        let matcher = new_matcher(&jvm, "a", "abc").await?;
        let replacement: ClassInstanceRef<String> = None.into();
        let result: Result<ClassInstanceRef<String>> = jvm
            .invoke_virtual(&matcher, method, "(Ljava/lang/String;)Ljava/lang/String;", (replacement,))
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("{method} with a matching pattern and null replacement must throw");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));
    }

    Ok(())
}

#[tokio::test]
async fn invalid_find_and_reset_leave_search_and_append_positions_usable() -> Result<()> {
    let jvm = test_jvm().await?;
    let matcher = new_matcher(&jvm, "a", "aXa").await?;
    let buffer: ClassInstanceRef<StringBuffer> = jvm.new_class("java/lang/StringBuffer", "()V", ()).await?.into();
    assert!(jvm.invoke_virtual::<_, bool>(&matcher, "find", "()Z", ()).await?);
    let first = JavaLangString::from_rust_string(&jvm, "x").await?;
    let _: ClassInstanceRef<Matcher> = jvm
        .invoke_virtual(
            &matcher,
            "appendReplacement",
            "(Ljava/lang/StringBuffer;Ljava/lang/String;)Ljava/util/regex/Matcher;",
            (buffer.clone(), first),
        )
        .await?;

    let result: Result<bool> = jvm.invoke_virtual(&matcher, "find", "(I)Z", (-1,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("find(-1) must throw");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IndexOutOfBoundsException"));

    let null: ClassInstanceRef<CharSequence> = None.into();
    let result: Result<ClassInstanceRef<Matcher>> = jvm
        .invoke_virtual(&matcher, "reset", "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;", (null,))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("reset(null) must throw");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    assert!(jvm.invoke_virtual::<_, bool>(&matcher, "find", "()Z", ()).await?);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&matcher, "start", "()I", ()).await?, 2);
    let second = JavaLangString::from_rust_string(&jvm, "y").await?;
    let _: ClassInstanceRef<Matcher> = jvm
        .invoke_virtual(
            &matcher,
            "appendReplacement",
            "(Ljava/lang/StringBuffer;Ljava/lang/String;)Ljava/util/regex/Matcher;",
            (buffer.clone(), second),
        )
        .await?;
    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(
            &matcher,
            "appendTail",
            "(Ljava/lang/StringBuffer;)Ljava/lang/StringBuffer;",
            (buffer.clone(),),
        )
        .await?;
    assert_eq!(buffer_text(&jvm, &buffer).await?, "xXy");

    Ok(())
}
