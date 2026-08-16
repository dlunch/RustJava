use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use java_runtime::{
    classes::java::{
        lang::{Object, String},
        util::regex::PatternSyntaxException,
    },
    get_runtime_class_proto,
};
use jvm::{ClassInstanceRef, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn pattern_syntax_exception_exposes_the_java_14_contract() -> Result<()> {
    let proto = get_runtime_class_proto("java/util/regex/PatternSyntaxException").expect("PatternSyntaxException must be registered");
    assert_eq!(proto.parent_class, Some("java/lang/IllegalArgumentException"));
    assert_eq!(proto.access_flags, ClassAccessFlags::PUBLIC);
    assert!(proto.interfaces.is_empty());
    assert_eq!(proto.methods.len(), 6);
    assert_eq!(proto.fields.len(), 4);

    for (name, descriptor, flags) in [
        ("<init>", "(Ljava/lang/String;Ljava/lang/String;I)V", MethodAccessFlags::PUBLIC),
        ("getDescription", "()Ljava/lang/String;", MethodAccessFlags::PUBLIC),
        ("getPattern", "()Ljava/lang/String;", MethodAccessFlags::PUBLIC),
        ("getIndex", "()I", MethodAccessFlags::PUBLIC),
        ("getMessage", "()Ljava/lang/String;", MethodAccessFlags::PUBLIC),
    ] {
        let method = proto
            .methods
            .iter()
            .find(|method| method.name == name && method.descriptor == descriptor)
            .unwrap_or_else(|| panic!("missing PatternSyntaxException.{name}{descriptor}"));
        assert_eq!(method.access_flags, flags);
    }

    for (name, descriptor, flags) in [
        ("desc", "Ljava/lang/String;", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL),
        ("pattern", "Ljava/lang/String;", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL),
        ("index", "I", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL),
        (
            "nl",
            "Ljava/lang/String;",
            FieldAccessFlags::PRIVATE | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
        ),
    ] {
        let field = proto
            .fields
            .iter()
            .find(|field| field.name == name && field.descriptor == descriptor)
            .unwrap_or_else(|| panic!("missing PatternSyntaxException.{name}:{descriptor}"));
        assert_eq!(field.access_flags, flags);
    }

    Ok(())
}

#[tokio::test]
async fn pattern_syntax_exception_formats_index_and_caret_with_the_initial_line_separator() -> Result<()> {
    let jvm = test_jvm().await?;
    let key = JavaLangString::from_rust_string(&jvm, "line.separator").await?;
    let separator = JavaLangString::from_rust_string(&jvm, "|").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/lang/System",
            "setProperty",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
            (key.clone(), separator),
        )
        .await?;

    let description = JavaLangString::from_rust_string(&jvm, "Unclosed group").await?;
    let pattern = JavaLangString::from_rust_string(&jvm, "a(").await?;
    let exception: ClassInstanceRef<PatternSyntaxException> = jvm
        .new_class(
            "java/util/regex/PatternSyntaxException",
            "(Ljava/lang/String;Ljava/lang/String;I)V",
            (description.clone(), pattern.clone(), 1),
        )
        .await?
        .into();

    let actual_description: ClassInstanceRef<String> = jvm
        .invoke_virtual(
            &exception,
            "java/util/regex/PatternSyntaxException",
            "getDescription",
            "()Ljava/lang/String;",
            (),
        )
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &actual_description).await?, "Unclosed group");
    let actual_pattern: ClassInstanceRef<String> = jvm
        .invoke_virtual(
            &exception,
            "java/util/regex/PatternSyntaxException",
            "getPattern",
            "()Ljava/lang/String;",
            (),
        )
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &actual_pattern).await?, "a(");
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&exception, "java/util/regex/PatternSyntaxException", "getIndex", "()I", ())
            .await?,
        1
    );

    let message: ClassInstanceRef<String> = jvm
        .invoke_virtual(
            &exception,
            "java/util/regex/PatternSyntaxException",
            "getMessage",
            "()Ljava/lang/String;",
            (),
        )
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &message).await?, "Unclosed group near index 1|a(| ^");

    let changed_separator = JavaLangString::from_rust_string(&jvm, "~").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/lang/System",
            "setProperty",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
            (key, changed_separator),
        )
        .await?;
    let message: ClassInstanceRef<String> = jvm
        .invoke_virtual(
            &exception,
            "java/util/regex/PatternSyntaxException",
            "getMessage",
            "()Ljava/lang/String;",
            (),
        )
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &message).await?, "Unclosed group near index 1|a(| ^");

    Ok(())
}

#[tokio::test]
async fn pattern_syntax_exception_preserves_nulls_and_keeps_java_14_caret_formatting() -> Result<()> {
    let jvm = test_jvm().await?;
    let key = JavaLangString::from_rust_string(&jvm, "line.separator").await?;
    let separator = JavaLangString::from_rust_string(&jvm, "\n").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/lang/System",
            "setProperty",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
            (key, separator),
        )
        .await?;

    let null: ClassInstanceRef<String> = None.into();
    let exception: ClassInstanceRef<PatternSyntaxException> = jvm
        .new_class(
            "java/util/regex/PatternSyntaxException",
            "(Ljava/lang/String;Ljava/lang/String;I)V",
            (null.clone(), null, -1),
        )
        .await?
        .into();
    let description: ClassInstanceRef<String> = jvm
        .invoke_virtual(
            &exception,
            "java/util/regex/PatternSyntaxException",
            "getDescription",
            "()Ljava/lang/String;",
            (),
        )
        .await?;
    assert!(description.is_null());
    let pattern: ClassInstanceRef<String> = jvm
        .invoke_virtual(
            &exception,
            "java/util/regex/PatternSyntaxException",
            "getPattern",
            "()Ljava/lang/String;",
            (),
        )
        .await?;
    assert!(pattern.is_null());
    let message: ClassInstanceRef<String> = jvm
        .invoke_virtual(
            &exception,
            "java/util/regex/PatternSyntaxException",
            "getMessage",
            "()Ljava/lang/String;",
            (),
        )
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &message).await?, "null\nnull");

    let description = JavaLangString::from_rust_string(&jvm, "Bad pattern").await?;
    let pattern = JavaLangString::from_rust_string(&jvm, "abc").await?;
    let exception: ClassInstanceRef<PatternSyntaxException> = jvm
        .new_class(
            "java/util/regex/PatternSyntaxException",
            "(Ljava/lang/String;Ljava/lang/String;I)V",
            (description, pattern, 3),
        )
        .await?
        .into();
    let message: ClassInstanceRef<String> = jvm
        .invoke_virtual(
            &exception,
            "java/util/regex/PatternSyntaxException",
            "getMessage",
            "()Ljava/lang/String;",
            (),
        )
        .await?;
    assert_eq!(
        JavaLangString::to_rust_string(&jvm, &message).await?,
        "Bad pattern near index 3\nabc\n   ^"
    );

    let description = JavaLangString::from_rust_string(&jvm, "Missing pattern").await?;
    let null: ClassInstanceRef<String> = None.into();
    let exception: ClassInstanceRef<PatternSyntaxException> = jvm
        .new_class(
            "java/util/regex/PatternSyntaxException",
            "(Ljava/lang/String;Ljava/lang/String;I)V",
            (description, null, 2),
        )
        .await?
        .into();
    let message: ClassInstanceRef<String> = jvm
        .invoke_virtual(
            &exception,
            "java/util/regex/PatternSyntaxException",
            "getMessage",
            "()Ljava/lang/String;",
            (),
        )
        .await?;
    assert_eq!(
        JavaLangString::to_rust_string(&jvm, &message).await?,
        "Missing pattern near index 2\nnull\n  ^"
    );

    Ok(())
}
