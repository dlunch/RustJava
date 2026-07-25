use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use java_runtime::{
    classes::java::{
        lang::{CharSequence, String as JavaString},
        util::regex::{Matcher, Pattern, PatternSyntaxException},
    },
    get_runtime_class_proto,
};
use jvm::{ClassInstanceRef, JavaError, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn pattern_exposes_the_java_14_compile_and_match_surface() -> Result<()> {
    let proto = get_runtime_class_proto("java/util/regex/Pattern").expect("Pattern must be registered");
    assert_eq!(proto.parent_class, Some("java/lang/Object"));
    assert_eq!(proto.interfaces, vec!["java/io/Serializable"]);
    assert_eq!(proto.access_flags, ClassAccessFlags::PUBLIC | ClassAccessFlags::FINAL);
    assert_eq!(proto.methods.len(), 10);
    assert_eq!(proto.fields.len(), 9);

    for (name, descriptor, flags) in [
        ("<init>", "(Ljava/lang/String;I)V", MethodAccessFlags::PRIVATE),
        (
            "compile",
            "(Ljava/lang/String;)Ljava/util/regex/Pattern;",
            MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
        ),
        (
            "compile",
            "(Ljava/lang/String;I)Ljava/util/regex/Pattern;",
            MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
        ),
        ("pattern", "()Ljava/lang/String;", MethodAccessFlags::PUBLIC),
        ("flags", "()I", MethodAccessFlags::PUBLIC),
        (
            "matcher",
            "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;",
            MethodAccessFlags::PUBLIC,
        ),
        (
            "matches",
            "(Ljava/lang/String;Ljava/lang/CharSequence;)Z",
            MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
        ),
    ] {
        let method = proto
            .methods
            .iter()
            .find(|method| method.name == name && method.descriptor == descriptor)
            .unwrap_or_else(|| panic!("missing Pattern.{name}{descriptor}"));
        assert_eq!(method.access_flags, flags);
    }

    assert!(!proto.fields.iter().any(|field| field.name == "LITERAL"));
    assert!(!proto.methods.iter().any(|method| matches!(method.name.as_str(), "quote" | "toString")));
    for (name, descriptor, flags) in [
        (
            "UNIX_LINES",
            "I",
            FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
        ),
        (
            "CASE_INSENSITIVE",
            "I",
            FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
        ),
        (
            "COMMENTS",
            "I",
            FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
        ),
        (
            "MULTILINE",
            "I",
            FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
        ),
        (
            "DOTALL",
            "I",
            FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
        ),
        (
            "UNICODE_CASE",
            "I",
            FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
        ),
        (
            "CANON_EQ",
            "I",
            FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
        ),
        ("pattern", "Ljava/lang/String;", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL),
        ("flags", "I", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL),
    ] {
        let field = proto
            .fields
            .iter()
            .find(|field| field.name == name && field.descriptor == descriptor)
            .unwrap_or_else(|| panic!("missing Pattern.{name}:{descriptor}"));
        assert_eq!(field.access_flags, flags);
    }

    let jvm = test_jvm().await?;
    for (name, value) in [
        ("UNIX_LINES", 1),
        ("CASE_INSENSITIVE", 2),
        ("COMMENTS", 4),
        ("MULTILINE", 8),
        ("DOTALL", 32),
        ("UNICODE_CASE", 64),
        ("CANON_EQ", 128),
    ] {
        assert_eq!(jvm.get_static_field::<i32>("java/util/regex/Pattern", name, "I").await?, value);
    }

    Ok(())
}

#[tokio::test]
async fn compile_preserves_the_source_and_valid_flags() -> Result<()> {
    let jvm = test_jvm().await?;
    let source = JavaLangString::from_rust_string(&jvm, "a+b").await?;
    let pattern: ClassInstanceRef<Pattern> = jvm
        .invoke_static(
            "java/util/regex/Pattern",
            "compile",
            "(Ljava/lang/String;)Ljava/util/regex/Pattern;",
            (source.clone(),),
        )
        .await?;
    let actual: ClassInstanceRef<JavaString> = jvm.invoke_virtual(&pattern, "pattern", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &actual).await?, "a+b");
    assert_eq!(jvm.invoke_virtual::<_, i32>(&pattern, "flags", "()I", ()).await?, 0);

    let source = JavaLangString::from_rust_string(&jvm, "a").await?;
    let pattern: ClassInstanceRef<Pattern> = jvm
        .invoke_static(
            "java/util/regex/Pattern",
            "compile",
            "(Ljava/lang/String;I)Ljava/util/regex/Pattern;",
            (source, 0xef),
        )
        .await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&pattern, "flags", "()I", ()).await?, 0xef);

    Ok(())
}

#[tokio::test]
async fn compile_passes_modern_regex_syntax_through_without_translation() -> Result<()> {
    let jvm = test_jvm().await?;
    let source = JavaLangString::from_rust_string(&jvm, r"(?P<word>a+)").await?;
    let pattern: ClassInstanceRef<Pattern> = jvm
        .invoke_static(
            "java/util/regex/Pattern",
            "compile",
            "(Ljava/lang/String;)Ljava/util/regex/Pattern;",
            (source.clone(),),
        )
        .await?;
    let preserved: ClassInstanceRef<JavaString> = jvm.invoke_virtual(&pattern, "pattern", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &preserved).await?, r"(?P<word>a+)");

    let input: ClassInstanceRef<CharSequence> = JavaLangString::from_rust_string(&jvm, "aaa").await?.into();
    let matcher: ClassInstanceRef<Matcher> = jvm
        .invoke_virtual(&pattern, "matcher", "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;", (input,))
        .await?;
    assert!(jvm.invoke_virtual::<_, bool>(&matcher, "matches", "()Z", ()).await?);
    let group: ClassInstanceRef<JavaString> = jvm.invoke_virtual(&matcher, "group", "(I)Ljava/lang/String;", (1,)).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &group).await?, "aaa");

    Ok(())
}

#[tokio::test]
async fn compile_validates_flags_before_reading_the_pattern() -> Result<()> {
    let jvm = test_jvm().await?;
    for flags in [0x10, 0x100, -1] {
        let source = JavaLangString::from_rust_string(&jvm, "a").await?;
        let result: Result<ClassInstanceRef<Pattern>> = jvm
            .invoke_static(
                "java/util/regex/Pattern",
                "compile",
                "(Ljava/lang/String;I)Ljava/util/regex/Pattern;",
                (source, flags),
            )
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("undefined Pattern flags must throw IllegalArgumentException");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/IllegalArgumentException"));
    }

    let null: ClassInstanceRef<JavaString> = None.into();
    let result: Result<ClassInstanceRef<Pattern>> = jvm
        .invoke_static(
            "java/util/regex/Pattern",
            "compile",
            "(Ljava/lang/String;I)Ljava/util/regex/Pattern;",
            (null.clone(), 0x10),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("invalid flags must win over a null pattern");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalArgumentException"));

    let result: Result<ClassInstanceRef<Pattern>> = jvm
        .invoke_static(
            "java/util/regex/Pattern",
            "compile",
            "(Ljava/lang/String;)Ljava/util/regex/Pattern;",
            (null,),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("a null pattern must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    Ok(())
}

#[tokio::test]
async fn compile_reports_rust_regex_errors_as_pattern_syntax_exception() -> Result<()> {
    let jvm = test_jvm().await?;
    for source in ["(", r"(a)\1"] {
        let regex = JavaLangString::from_rust_string(&jvm, source).await?;
        let result: Result<ClassInstanceRef<Pattern>> = jvm
            .invoke_static(
                "java/util/regex/Pattern",
                "compile",
                "(Ljava/lang/String;)Ljava/util/regex/Pattern;",
                (regex,),
            )
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("unsupported or malformed regex must throw PatternSyntaxException");
        };
        assert!(jvm.is_instance(&*exception, "java/util/regex/PatternSyntaxException"));
        let exception: ClassInstanceRef<PatternSyntaxException> = exception.into();
        let actual: ClassInstanceRef<JavaString> = jvm.invoke_virtual(&exception, "getPattern", "()Ljava/lang/String;", ()).await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &actual).await?, source);
        assert_eq!(jvm.invoke_virtual::<_, i32>(&exception, "getIndex", "()I", ()).await?, -1);
        let description: ClassInstanceRef<JavaString> = jvm.invoke_virtual(&exception, "getDescription", "()Ljava/lang/String;", ()).await?;
        assert!(!JavaLangString::to_rust_string(&jvm, &description).await?.is_empty());
    }

    Ok(())
}

#[tokio::test]
async fn pattern_flags_control_rust_regex_matching() -> Result<()> {
    let jvm = test_jvm().await?;
    for (source, flags, input, expected) in [
        ("abc", 2, "AbC", true),
        ("a # note\n b", 4, "ab", true),
        ("a.b", 32, "a\nb", true),
        ("abc", 1 | 64 | 128, "abc", true),
    ] {
        let source = JavaLangString::from_rust_string(&jvm, source).await?;
        let pattern: ClassInstanceRef<Pattern> = jvm
            .invoke_static(
                "java/util/regex/Pattern",
                "compile",
                "(Ljava/lang/String;I)Ljava/util/regex/Pattern;",
                (source, flags),
            )
            .await?;
        let input: ClassInstanceRef<CharSequence> = JavaLangString::from_rust_string(&jvm, input).await?.into();
        let matcher: ClassInstanceRef<Matcher> = jvm
            .invoke_virtual(&pattern, "matcher", "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;", (input,))
            .await?;
        assert_eq!(jvm.invoke_virtual::<_, bool>(&matcher, "matches", "()Z", ()).await?, expected);
    }

    let source = JavaLangString::from_rust_string(&jvm, "^b$").await?;
    let pattern: ClassInstanceRef<Pattern> = jvm
        .invoke_static(
            "java/util/regex/Pattern",
            "compile",
            "(Ljava/lang/String;I)Ljava/util/regex/Pattern;",
            (source, 8),
        )
        .await?;
    let input: ClassInstanceRef<CharSequence> = JavaLangString::from_rust_string(&jvm, "a\nb\nc").await?.into();
    let matcher: ClassInstanceRef<Matcher> = jvm
        .invoke_virtual(&pattern, "matcher", "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;", (input,))
        .await?;
    assert!(jvm.invoke_virtual::<_, bool>(&matcher, "find", "()Z", ()).await?);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&matcher, "start", "()I", ()).await?, 2);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&matcher, "end", "()I", ()).await?, 3);

    Ok(())
}

#[tokio::test]
async fn static_matches_and_matcher_reject_null_inputs() -> Result<()> {
    let jvm = test_jvm().await?;
    let source = JavaLangString::from_rust_string(&jvm, "a*b").await?;
    let input: ClassInstanceRef<CharSequence> = JavaLangString::from_rust_string(&jvm, "aaaaab").await?.into();
    assert!(
        jvm.invoke_static::<_, bool>(
            "java/util/regex/Pattern",
            "matches",
            "(Ljava/lang/String;Ljava/lang/CharSequence;)Z",
            (source.clone(), input),
        )
        .await?
    );

    let pattern: ClassInstanceRef<Pattern> = jvm
        .invoke_static(
            "java/util/regex/Pattern",
            "compile",
            "(Ljava/lang/String;)Ljava/util/regex/Pattern;",
            (source.clone(),),
        )
        .await?;
    let null_input: ClassInstanceRef<CharSequence> = None.into();
    let result: Result<ClassInstanceRef<Matcher>> = jvm
        .invoke_virtual(
            &pattern,
            "matcher",
            "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;",
            (null_input.clone(),),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Pattern.matcher(null) must throw");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let result: Result<bool> = jvm
        .invoke_static(
            "java/util/regex/Pattern",
            "matches",
            "(Ljava/lang/String;Ljava/lang/CharSequence;)Z",
            (source, null_input),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Pattern.matches with a null input must throw");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    Ok(())
}
