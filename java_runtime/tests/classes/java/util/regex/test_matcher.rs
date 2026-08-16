use alloc::{boxed::Box, collections::BTreeMap, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use java_runtime::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        lang::{CharSequence, String, StringBuffer},
        util::regex::{Matcher, Pattern},
    },
    get_runtime_class_proto,
};
use jvm::{ClassInstanceRef, JavaChar, JavaError, Result, runtime::JavaLangString};
use jvm_rust::ClassDefinitionImpl;

use test_utils::{TestRuntime, create_test_jvm, test_jvm};

struct SnapshotFailingCharSequence;

impl SnapshotFailingCharSequence {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "SnapshotFailingCharSequence",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/lang/CharSequence"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("length", "()I", Self::length, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("charAt", "(I)C", Self::char_at, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "subSequence",
                    "(II)Ljava/lang/CharSequence;",
                    Self::sub_sequence,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("lengthCalls", "I", FieldAccessFlags::PUBLIC),
                JavaFieldProto::new("snapshotCalls", "I", FieldAccessFlags::PUBLIC),
                JavaFieldProto::new("failSnapshot", "Z", FieldAccessFlags::PUBLIC),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &jvm::Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "lengthCalls", "I", 0).await?;
        jvm.put_field(&mut this, "snapshotCalls", "I", 0).await?;
        jvm.put_field(&mut this, "failSnapshot", "Z", true).await
    }

    async fn length(jvm: &jvm::Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<i32> {
        let calls: i32 = jvm.get_field(&this, "lengthCalls", "I").await?;
        jvm.put_field(&mut this, "lengthCalls", "I", calls + 1).await?;
        Ok(1)
    }

    async fn char_at(_: &jvm::Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: i32) -> Result<JavaChar> {
        Ok('a' as JavaChar)
    }

    async fn sub_sequence(
        _: &jvm::Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        _: i32,
        _: i32,
    ) -> Result<ClassInstanceRef<CharSequence>> {
        Ok(ClassInstanceRef::new(this.instance))
    }

    async fn to_string(jvm: &jvm::Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let calls: i32 = jvm.get_field(&this, "snapshotCalls", "I").await?;
        jvm.put_field(&mut this, "snapshotCalls", "I", calls + 1).await?;
        if jvm.get_field::<bool>(&this, "failSnapshot", "Z").await? {
            Err(jvm.exception("java/lang/IllegalStateException", "snapshot requested").await)
        } else {
            Ok(JavaLangString::from_rust_string(jvm, "a").await?.into())
        }
    }
}

#[tokio::test]
async fn matcher_exposes_java_14_search_state() -> Result<()> {
    let proto = get_runtime_class_proto("java/util/regex/Matcher").expect("Matcher must be registered");
    assert_eq!(proto.parent_class, Some("java/lang/Object"));
    assert!(proto.interfaces.is_empty());
    assert_eq!(proto.access_flags, ClassAccessFlags::PUBLIC | ClassAccessFlags::FINAL);
    assert_eq!(proto.methods.len(), 19);
    assert_eq!(proto.fields.len(), 6);

    for (name, descriptor, flags) in [
        (
            "<init>",
            "(Ljava/util/regex/Pattern;Ljava/lang/CharSequence;)V",
            MethodAccessFlags::empty(),
        ),
        ("pattern", "()Ljava/util/regex/Pattern;", MethodAccessFlags::PUBLIC),
        ("reset", "()Ljava/util/regex/Matcher;", MethodAccessFlags::PUBLIC),
        ("reset", "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;", MethodAccessFlags::PUBLIC),
        ("matches", "()Z", MethodAccessFlags::PUBLIC),
        ("lookingAt", "()Z", MethodAccessFlags::PUBLIC),
        ("find", "()Z", MethodAccessFlags::PUBLIC),
        ("find", "(I)Z", MethodAccessFlags::PUBLIC),
        ("start", "()I", MethodAccessFlags::PUBLIC),
        ("start", "(I)I", MethodAccessFlags::PUBLIC),
        ("end", "()I", MethodAccessFlags::PUBLIC),
        ("end", "(I)I", MethodAccessFlags::PUBLIC),
        ("group", "()Ljava/lang/String;", MethodAccessFlags::PUBLIC),
        ("group", "(I)Ljava/lang/String;", MethodAccessFlags::PUBLIC),
        ("groupCount", "()I", MethodAccessFlags::PUBLIC),
    ] {
        let method = proto
            .methods
            .iter()
            .find(|method| method.name == name && method.descriptor == descriptor)
            .unwrap_or_else(|| panic!("missing Matcher.{name}{descriptor}"));
        assert_eq!(method.access_flags, flags);
    }
    assert!(!proto.methods.iter().any(|method| {
        matches!(
            method.name.as_str(),
            "quoteReplacement"
                | "region"
                | "regionStart"
                | "regionEnd"
                | "hasAnchoringBounds"
                | "hasTransparentBounds"
                | "useAnchoringBounds"
                | "usePattern"
                | "useTransparentBounds"
                | "toMatchResult"
        )
    }));
    assert!(
        !proto
            .methods
            .iter()
            .any(|method| method.descriptor.contains("Ljava/util/regex/MatchResult;"))
    );

    for (name, descriptor, flags) in [
        (
            "parentPattern",
            "Ljava/util/regex/Pattern;",
            FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL,
        ),
        ("text", "Ljava/lang/CharSequence;", FieldAccessFlags::PRIVATE),
        ("groups", "[I", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL),
        ("searchPosition", "I", FieldAccessFlags::PRIVATE),
        ("appendPosition", "I", FieldAccessFlags::PRIVATE),
        ("hasMatch", "Z", FieldAccessFlags::PRIVATE),
    ] {
        let field = proto
            .fields
            .iter()
            .find(|field| field.name == name && field.descriptor == descriptor)
            .unwrap_or_else(|| panic!("missing Matcher.{name}:{descriptor}"));
        assert_eq!(field.access_flags, flags);
    }

    Ok(())
}

#[tokio::test]
async fn full_match_reselects_alternatives_without_shifting_capture_groups() -> Result<()> {
    let jvm = test_jvm().await?;
    for (source, expected_group_count) in [("a|ab", 0), ("(a|ab)", 1)] {
        let source = JavaLangString::from_rust_string(&jvm, source).await?;
        let pattern: ClassInstanceRef<Pattern> = jvm
            .invoke_static(
                "java/util/regex/Pattern",
                "compile",
                "(Ljava/lang/String;)Ljava/util/regex/Pattern;",
                (source,),
            )
            .await?;
        let input: ClassInstanceRef<CharSequence> = JavaLangString::from_rust_string(&jvm, "ab").await?.into();
        let matcher: ClassInstanceRef<Matcher> = jvm
            .invoke_virtual(
                &pattern,
                "java/util/regex/Pattern",
                "matcher",
                "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;",
                (input,),
            )
            .await?;

        assert!(
            jvm.invoke_virtual::<_, bool>(&matcher, "java/util/regex/Matcher", "matches", "()Z", ())
                .await?
        );
        let group: ClassInstanceRef<String> = jvm
            .invoke_virtual(&matcher, "java/util/regex/Matcher", "group", "()Ljava/lang/String;", ())
            .await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &group).await?, "ab");
        assert_eq!(
            jvm.invoke_virtual::<_, i32>(&matcher, "java/util/regex/Matcher", "groupCount", "()I", ())
                .await?,
            expected_group_count
        );
        if expected_group_count == 1 {
            let group: ClassInstanceRef<String> = jvm
                .invoke_virtual(&matcher, "java/util/regex/Matcher", "group", "(I)Ljava/lang/String;", (1,))
                .await?;
            assert_eq!(JavaLangString::to_rust_string(&jvm, &group).await?, "ab");
        }
    }

    let source = JavaLangString::from_rust_string(&jvm, "a|ab").await?;
    let pattern: ClassInstanceRef<Pattern> = jvm
        .invoke_static(
            "java/util/regex/Pattern",
            "compile",
            "(Ljava/lang/String;)Ljava/util/regex/Pattern;",
            (source,),
        )
        .await?;
    let input: ClassInstanceRef<CharSequence> = JavaLangString::from_rust_string(&jvm, "ab").await?.into();
    let matcher: ClassInstanceRef<Matcher> = jvm
        .invoke_virtual(
            &pattern,
            "java/util/regex/Pattern",
            "matcher",
            "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;",
            (input,),
        )
        .await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&matcher, "java/util/regex/Matcher", "lookingAt", "()Z", ())
            .await?
    );
    let group: ClassInstanceRef<String> = jvm
        .invoke_virtual(&matcher, "java/util/regex/Matcher", "group", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &group).await?, "a");

    Ok(())
}

#[tokio::test]
async fn comments_mode_allows_a_terminal_comment_in_full_and_prefix_matches() -> Result<()> {
    let jvm = test_jvm().await?;
    let source = JavaLangString::from_rust_string(&jvm, "a # trailing").await?;
    let pattern: ClassInstanceRef<Pattern> = jvm
        .invoke_static(
            "java/util/regex/Pattern",
            "compile",
            "(Ljava/lang/String;I)Ljava/util/regex/Pattern;",
            (source, 4),
        )
        .await?;

    for method in ["matches", "lookingAt"] {
        let input: ClassInstanceRef<CharSequence> = JavaLangString::from_rust_string(&jvm, "a").await?.into();
        let matcher: ClassInstanceRef<Matcher> = jvm
            .invoke_virtual(
                &pattern,
                "java/util/regex/Pattern",
                "matcher",
                "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;",
                (input,),
            )
            .await?;
        assert!(
            jvm.invoke_virtual::<_, bool>(&matcher, "java/util/regex/Matcher", method, "()Z", ())
                .await?
        );
    }

    Ok(())
}

#[tokio::test]
async fn full_match_preserves_inline_comment_modes() -> Result<()> {
    let jvm = test_jvm().await?;
    let source = JavaLangString::from_rust_string(&jvm, "(?x)a # trailing").await?;
    let pattern: ClassInstanceRef<Pattern> = jvm
        .invoke_static(
            "java/util/regex/Pattern",
            "compile",
            "(Ljava/lang/String;)Ljava/util/regex/Pattern;",
            (source,),
        )
        .await?;
    let input: ClassInstanceRef<CharSequence> = JavaLangString::from_rust_string(&jvm, "a").await?.into();
    let matcher: ClassInstanceRef<Matcher> = jvm
        .invoke_virtual(
            &pattern,
            "java/util/regex/Pattern",
            "matcher",
            "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;",
            (input,),
        )
        .await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&matcher, "java/util/regex/Matcher", "matches", "()Z", ())
            .await?
    );

    let source = JavaLangString::from_rust_string(&jvm, "(?-x)a # trailing").await?;
    let pattern: ClassInstanceRef<Pattern> = jvm
        .invoke_static(
            "java/util/regex/Pattern",
            "compile",
            "(Ljava/lang/String;I)Ljava/util/regex/Pattern;",
            (source, 4),
        )
        .await?;
    let input: ClassInstanceRef<CharSequence> = JavaLangString::from_rust_string(&jvm, "a # trailing").await?.into();
    let matcher: ClassInstanceRef<Matcher> = jvm
        .invoke_virtual(
            &pattern,
            "java/util/regex/Pattern",
            "matcher",
            "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;",
            (input,),
        )
        .await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&matcher, "java/util/regex/Matcher", "matches", "()Z", ())
            .await?
    );

    Ok(())
}

#[tokio::test]
async fn captures_report_unmatched_empty_and_utf16_ranges() -> Result<()> {
    let jvm = test_jvm().await?;
    let source = JavaLangString::from_rust_string(&jvm, "(a)?(b*)").await?;
    let pattern: ClassInstanceRef<Pattern> = jvm
        .invoke_static(
            "java/util/regex/Pattern",
            "compile",
            "(Ljava/lang/String;)Ljava/util/regex/Pattern;",
            (source,),
        )
        .await?;
    let input: ClassInstanceRef<CharSequence> = JavaLangString::from_rust_string(&jvm, "bbb").await?.into();
    let matcher: ClassInstanceRef<Matcher> = jvm
        .invoke_virtual(
            &pattern,
            "java/util/regex/Pattern",
            "matcher",
            "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;",
            (input,),
        )
        .await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&matcher, "java/util/regex/Matcher", "matches", "()Z", ())
            .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&matcher, "java/util/regex/Matcher", "groupCount", "()I", ())
            .await?,
        2
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&matcher, "java/util/regex/Matcher", "start", "(I)I", (1,))
            .await?,
        -1
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&matcher, "java/util/regex/Matcher", "end", "(I)I", (1,))
            .await?,
        -1
    );
    let unmatched: ClassInstanceRef<String> = jvm
        .invoke_virtual(&matcher, "java/util/regex/Matcher", "group", "(I)Ljava/lang/String;", (1,))
        .await?;
    assert!(unmatched.is_null());
    let group: ClassInstanceRef<String> = jvm
        .invoke_virtual(&matcher, "java/util/regex/Matcher", "group", "(I)Ljava/lang/String;", (2,))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &group).await?, "bbb");

    let source = JavaLangString::from_rust_string(&jvm, "(a*)").await?;
    let pattern: ClassInstanceRef<Pattern> = jvm
        .invoke_static(
            "java/util/regex/Pattern",
            "compile",
            "(Ljava/lang/String;)Ljava/util/regex/Pattern;",
            (source,),
        )
        .await?;
    let input: ClassInstanceRef<CharSequence> = JavaLangString::from_rust_string(&jvm, "b").await?.into();
    let matcher: ClassInstanceRef<Matcher> = jvm
        .invoke_virtual(
            &pattern,
            "java/util/regex/Pattern",
            "matcher",
            "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;",
            (input,),
        )
        .await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&matcher, "java/util/regex/Matcher", "find", "()Z", ())
            .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&matcher, "java/util/regex/Matcher", "start", "(I)I", (1,))
            .await?,
        0
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&matcher, "java/util/regex/Matcher", "end", "(I)I", (1,))
            .await?,
        0
    );
    let empty: ClassInstanceRef<String> = jvm
        .invoke_virtual(&matcher, "java/util/regex/Matcher", "group", "(I)Ljava/lang/String;", (1,))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &empty).await?, "");

    let source = JavaLangString::from_rust_string(&jvm, "(😀)(한)").await?;
    let pattern: ClassInstanceRef<Pattern> = jvm
        .invoke_static(
            "java/util/regex/Pattern",
            "compile",
            "(Ljava/lang/String;)Ljava/util/regex/Pattern;",
            (source,),
        )
        .await?;
    let input: ClassInstanceRef<CharSequence> = JavaLangString::from_rust_string(&jvm, "A😀한B").await?.into();
    let matcher: ClassInstanceRef<Matcher> = jvm
        .invoke_virtual(
            &pattern,
            "java/util/regex/Pattern",
            "matcher",
            "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;",
            (input,),
        )
        .await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&matcher, "java/util/regex/Matcher", "find", "()Z", ())
            .await?
    );
    assert_eq!(
        (
            jvm.invoke_virtual::<_, i32>(&matcher, "java/util/regex/Matcher", "start", "()I", ())
                .await?,
            jvm.invoke_virtual::<_, i32>(&matcher, "java/util/regex/Matcher", "end", "()I", ())
                .await?,
        ),
        (1, 4)
    );
    assert_eq!(
        (
            jvm.invoke_virtual::<_, i32>(&matcher, "java/util/regex/Matcher", "start", "(I)I", (1,))
                .await?,
            jvm.invoke_virtual::<_, i32>(&matcher, "java/util/regex/Matcher", "end", "(I)I", (1,))
                .await?,
        ),
        (1, 3)
    );
    assert_eq!(
        (
            jvm.invoke_virtual::<_, i32>(&matcher, "java/util/regex/Matcher", "start", "(I)I", (2,))
                .await?,
            jvm.invoke_virtual::<_, i32>(&matcher, "java/util/regex/Matcher", "end", "(I)I", (2,))
                .await?,
        ),
        (3, 4)
    );

    Ok(())
}

#[tokio::test]
async fn failed_or_missing_matches_enforce_state_before_group_bounds() -> Result<()> {
    let jvm = test_jvm().await?;
    let source = JavaLangString::from_rust_string(&jvm, "a+").await?;
    let pattern: ClassInstanceRef<Pattern> = jvm
        .invoke_static(
            "java/util/regex/Pattern",
            "compile",
            "(Ljava/lang/String;)Ljava/util/regex/Pattern;",
            (source,),
        )
        .await?;
    let input: ClassInstanceRef<CharSequence> = JavaLangString::from_rust_string(&jvm, "baa").await?.into();
    let matcher: ClassInstanceRef<Matcher> = jvm
        .invoke_virtual(
            &pattern,
            "java/util/regex/Pattern",
            "matcher",
            "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;",
            (input,),
        )
        .await?;

    let result: Result<ClassInstanceRef<String>> = jvm
        .invoke_virtual(&matcher, "java/util/regex/Matcher", "group", "(I)Ljava/lang/String;", (-1,))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("group before a match must throw");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalStateException"));

    assert!(
        !jvm.invoke_virtual::<_, bool>(&matcher, "java/util/regex/Matcher", "matches", "()Z", ())
            .await?
    );
    for (name, descriptor) in [("start", "()I"), ("end", "()I")] {
        let result: Result<i32> = jvm.invoke_virtual(&matcher, "java/util/regex/Matcher", name, descriptor, ()).await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("{name} after a failed match must throw");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/IllegalStateException"));
    }

    assert!(
        jvm.invoke_virtual::<_, bool>(&matcher, "java/util/regex/Matcher", "find", "()Z", ())
            .await?
    );
    for group in [-1, 1] {
        let result: Result<i32> = jvm.invoke_virtual(&matcher, "java/util/regex/Matcher", "start", "(I)I", (group,)).await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("out-of-range group must throw");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/IndexOutOfBoundsException"));
    }

    assert!(
        !jvm.invoke_virtual::<_, bool>(&matcher, "java/util/regex/Matcher", "find", "()Z", ())
            .await?
    );
    let result: Result<ClassInstanceRef<String>> = jvm
        .invoke_virtual(&matcher, "java/util/regex/Matcher", "group", "()Ljava/lang/String;", ())
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("group after find failure must throw");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalStateException"));

    Ok(())
}

#[tokio::test]
async fn find_advances_after_zero_width_matches_and_stops_after_the_end() -> Result<()> {
    let jvm = test_jvm().await?;
    for (input, expected) in [("ab", vec![(0, 0), (1, 1), (2, 2)]), ("😀", vec![(0, 0), (2, 2)])] {
        let source = JavaLangString::from_rust_string(&jvm, "").await?;
        let pattern: ClassInstanceRef<Pattern> = jvm
            .invoke_static(
                "java/util/regex/Pattern",
                "compile",
                "(Ljava/lang/String;)Ljava/util/regex/Pattern;",
                (source,),
            )
            .await?;
        let input: ClassInstanceRef<CharSequence> = JavaLangString::from_rust_string(&jvm, input).await?.into();
        let matcher: ClassInstanceRef<Matcher> = jvm
            .invoke_virtual(
                &pattern,
                "java/util/regex/Pattern",
                "matcher",
                "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;",
                (input,),
            )
            .await?;
        for range in expected {
            assert!(
                jvm.invoke_virtual::<_, bool>(&matcher, "java/util/regex/Matcher", "find", "()Z", ())
                    .await?
            );
            assert_eq!(
                (
                    jvm.invoke_virtual::<_, i32>(&matcher, "java/util/regex/Matcher", "start", "()I", ())
                        .await?,
                    jvm.invoke_virtual::<_, i32>(&matcher, "java/util/regex/Matcher", "end", "()I", ())
                        .await?,
                ),
                range
            );
        }
        assert!(
            !jvm.invoke_virtual::<_, bool>(&matcher, "java/util/regex/Matcher", "find", "()Z", ())
                .await?
        );
        assert!(
            !jvm.invoke_virtual::<_, bool>(&matcher, "java/util/regex/Matcher", "find", "()Z", ())
                .await?
        );
    }

    let source = JavaLangString::from_rust_string(&jvm, "a*").await?;
    let pattern: ClassInstanceRef<Pattern> = jvm
        .invoke_static(
            "java/util/regex/Pattern",
            "compile",
            "(Ljava/lang/String;)Ljava/util/regex/Pattern;",
            (source,),
        )
        .await?;
    let input: ClassInstanceRef<CharSequence> = JavaLangString::from_rust_string(&jvm, "a").await?.into();
    let matcher: ClassInstanceRef<Matcher> = jvm
        .invoke_virtual(
            &pattern,
            "java/util/regex/Pattern",
            "matcher",
            "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;",
            (input,),
        )
        .await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&matcher, "java/util/regex/Matcher", "matches", "()Z", ())
            .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(&matcher, "java/util/regex/Matcher", "find", "()Z", ())
            .await?
    );
    assert_eq!(
        (
            jvm.invoke_virtual::<_, i32>(&matcher, "java/util/regex/Matcher", "start", "()I", ())
                .await?,
            jvm.invoke_virtual::<_, i32>(&matcher, "java/util/regex/Matcher", "end", "()I", ())
                .await?,
        ),
        (1, 1)
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(&matcher, "java/util/regex/Matcher", "find", "()Z", ())
            .await?
    );

    Ok(())
}

#[tokio::test]
async fn find_at_input_length_matches_the_end_anchor() -> Result<()> {
    let jvm = test_jvm().await?;
    let source = JavaLangString::from_rust_string(&jvm, "$").await?;
    let pattern: ClassInstanceRef<Pattern> = jvm
        .invoke_static(
            "java/util/regex/Pattern",
            "compile",
            "(Ljava/lang/String;)Ljava/util/regex/Pattern;",
            (source,),
        )
        .await?;
    let input: ClassInstanceRef<CharSequence> = JavaLangString::from_rust_string(&jvm, "ab").await?.into();
    let matcher: ClassInstanceRef<Matcher> = jvm
        .invoke_virtual(
            &pattern,
            "java/util/regex/Pattern",
            "matcher",
            "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;",
            (input,),
        )
        .await?;

    assert!(
        jvm.invoke_virtual::<_, bool>(&matcher, "java/util/regex/Matcher", "find", "(I)Z", (2,))
            .await?
    );
    assert_eq!(
        (
            jvm.invoke_virtual::<_, i32>(&matcher, "java/util/regex/Matcher", "start", "()I", ())
                .await?,
            jvm.invoke_virtual::<_, i32>(&matcher, "java/util/regex/Matcher", "end", "()I", ())
                .await?,
        ),
        (2, 2)
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(&matcher, "java/util/regex/Matcher", "find", "()Z", ())
            .await?
    );

    Ok(())
}

#[tokio::test]
async fn invalid_find_start_is_checked_before_creating_an_input_snapshot() -> Result<()> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;
    let class = Box::new(ClassDefinitionImpl::from_class_proto(
        SnapshotFailingCharSequence::as_proto(),
        Box::new(runtime) as Box<dyn java_runtime::Runtime>,
    ));
    jvm.register_class(class, None).await?;

    let sequence: ClassInstanceRef<SnapshotFailingCharSequence> = jvm.new_class("SnapshotFailingCharSequence", "()V", ()).await?.into();
    let input: ClassInstanceRef<CharSequence> = ClassInstanceRef::new(sequence.clone().instance);
    let source = JavaLangString::from_rust_string(&jvm, "a").await?;
    let pattern: ClassInstanceRef<Pattern> = jvm
        .invoke_static(
            "java/util/regex/Pattern",
            "compile",
            "(Ljava/lang/String;)Ljava/util/regex/Pattern;",
            (source,),
        )
        .await?;
    let matcher: ClassInstanceRef<Matcher> = jvm
        .invoke_virtual(
            &pattern,
            "java/util/regex/Pattern",
            "matcher",
            "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;",
            (input,),
        )
        .await?;

    let result: Result<bool> = jvm.invoke_virtual(&matcher, "java/util/regex/Matcher", "find", "(I)Z", (-1,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("negative find start must throw");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IndexOutOfBoundsException"));
    assert_eq!(jvm.get_field::<i32>(&sequence, "lengthCalls", "I").await?, 0);
    assert_eq!(jvm.get_field::<i32>(&sequence, "snapshotCalls", "I").await?, 0);

    let result: Result<bool> = jvm.invoke_virtual(&matcher, "java/util/regex/Matcher", "find", "(I)Z", (2,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("find start beyond the input length must throw");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IndexOutOfBoundsException"));
    assert_eq!(jvm.get_field::<i32>(&sequence, "lengthCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&sequence, "snapshotCalls", "I").await?, 0);

    Ok(())
}

#[tokio::test]
async fn valid_find_start_resets_state_before_creating_an_input_snapshot() -> Result<()> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;
    let class = Box::new(ClassDefinitionImpl::from_class_proto(
        SnapshotFailingCharSequence::as_proto(),
        Box::new(runtime) as Box<dyn java_runtime::Runtime>,
    ));
    jvm.register_class(class, None).await?;

    let mut sequence: ClassInstanceRef<SnapshotFailingCharSequence> = jvm.new_class("SnapshotFailingCharSequence", "()V", ()).await?.into();
    jvm.put_field(&mut sequence, "failSnapshot", "Z", false).await?;
    let input: ClassInstanceRef<CharSequence> = ClassInstanceRef::new(sequence.clone().instance);
    let source = JavaLangString::from_rust_string(&jvm, "a").await?;
    let pattern: ClassInstanceRef<Pattern> = jvm
        .invoke_static(
            "java/util/regex/Pattern",
            "compile",
            "(Ljava/lang/String;)Ljava/util/regex/Pattern;",
            (source,),
        )
        .await?;
    let matcher: ClassInstanceRef<Matcher> = jvm
        .invoke_virtual(
            &pattern,
            "java/util/regex/Pattern",
            "matcher",
            "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;",
            (input,),
        )
        .await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&matcher, "java/util/regex/Matcher", "find", "()Z", ())
            .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&matcher, "java/util/regex/Matcher", "start", "()I", ())
            .await?,
        0
    );

    jvm.put_field(&mut sequence, "failSnapshot", "Z", true).await?;
    let result: Result<bool> = jvm.invoke_virtual(&matcher, "java/util/regex/Matcher", "find", "(I)Z", (0,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("snapshot failure must be observable");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalStateException"));
    assert_eq!(jvm.get_field::<i32>(&sequence, "lengthCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&sequence, "snapshotCalls", "I").await?, 2);

    let result: Result<i32> = jvm.invoke_virtual(&matcher, "java/util/regex/Matcher", "start", "()I", ()).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("valid find(start) must invalidate the previous match before searching");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalStateException"));
    assert_eq!(jvm.get_field::<i32>(&matcher, "searchPosition", "I").await?, 0);
    assert_eq!(jvm.get_field::<i32>(&matcher, "appendPosition", "I").await?, 0);

    Ok(())
}

#[tokio::test]
async fn find_continues_from_the_prefix_end_after_successful_looking_at() -> Result<()> {
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
    let input: ClassInstanceRef<CharSequence> = JavaLangString::from_rust_string(&jvm, "abca").await?.into();
    let matcher: ClassInstanceRef<Matcher> = jvm
        .invoke_virtual(
            &pattern,
            "java/util/regex/Pattern",
            "matcher",
            "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;",
            (input,),
        )
        .await?;

    assert!(
        jvm.invoke_virtual::<_, bool>(&matcher, "java/util/regex/Matcher", "lookingAt", "()Z", ())
            .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&matcher, "java/util/regex/Matcher", "end", "()I", ())
            .await?,
        1
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(&matcher, "java/util/regex/Matcher", "find", "()Z", ())
            .await?
    );
    assert_eq!(
        (
            jvm.invoke_virtual::<_, i32>(&matcher, "java/util/regex/Matcher", "start", "()I", ())
                .await?,
            jvm.invoke_virtual::<_, i32>(&matcher, "java/util/regex/Matcher", "end", "()I", ())
                .await?,
        ),
        (3, 4)
    );

    Ok(())
}

#[tokio::test]
async fn find_start_and_reset_preserve_or_replace_state_as_specified() -> Result<()> {
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
    let input: ClassInstanceRef<CharSequence> = JavaLangString::from_rust_string(&jvm, "baac").await?.into();
    let matcher: ClassInstanceRef<Matcher> = jvm
        .invoke_virtual(
            &pattern,
            "java/util/regex/Pattern",
            "matcher",
            "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;",
            (input,),
        )
        .await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&matcher, "java/util/regex/Matcher", "find", "()Z", ())
            .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&matcher, "java/util/regex/Matcher", "start", "()I", ())
            .await?,
        1
    );

    for invalid in [-1, 5] {
        let result: Result<bool> = jvm.invoke_virtual(&matcher, "java/util/regex/Matcher", "find", "(I)Z", (invalid,)).await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("invalid find start must throw");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/IndexOutOfBoundsException"));
        assert_eq!(
            jvm.invoke_virtual::<_, i32>(&matcher, "java/util/regex/Matcher", "start", "()I", ())
                .await?,
            1
        );
    }

    assert!(
        jvm.invoke_virtual::<_, bool>(&matcher, "java/util/regex/Matcher", "find", "(I)Z", (2,))
            .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&matcher, "java/util/regex/Matcher", "start", "()I", ())
            .await?,
        2
    );

    let null: ClassInstanceRef<CharSequence> = None.into();
    let result: Result<ClassInstanceRef<Matcher>> = jvm
        .invoke_virtual(
            &matcher,
            "java/util/regex/Matcher",
            "reset",
            "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;",
            (null,),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("reset(null) must throw");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&matcher, "java/util/regex/Matcher", "start", "()I", ())
            .await?,
        2
    );

    let reset: ClassInstanceRef<Matcher> = jvm
        .invoke_virtual(&matcher, "java/util/regex/Matcher", "reset", "()Ljava/util/regex/Matcher;", ())
        .await?;
    assert_eq!(reset.identity(), matcher.identity());
    let result: Result<i32> = jvm.invoke_virtual(&matcher, "java/util/regex/Matcher", "start", "()I", ()).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("reset must invalidate the previous match");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalStateException"));
    assert!(
        jvm.invoke_virtual::<_, bool>(&matcher, "java/util/regex/Matcher", "find", "()Z", ())
            .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&matcher, "java/util/regex/Matcher", "start", "()I", ())
            .await?,
        1
    );

    Ok(())
}

#[tokio::test]
async fn reset_accepts_string_buffer_snapshots_and_matchers_keep_independent_state() -> Result<()> {
    let jvm = test_jvm().await?;
    let source = JavaLangString::from_rust_string(&jvm, "x").await?;
    let pattern: ClassInstanceRef<Pattern> = jvm
        .invoke_static(
            "java/util/regex/Pattern",
            "compile",
            "(Ljava/lang/String;)Ljava/util/regex/Pattern;",
            (source,),
        )
        .await?;
    let first_input: ClassInstanceRef<CharSequence> = JavaLangString::from_rust_string(&jvm, "x!x").await?.into();
    let second_input: ClassInstanceRef<CharSequence> = JavaLangString::from_rust_string(&jvm, "!x").await?.into();
    let first: ClassInstanceRef<Matcher> = jvm
        .invoke_virtual(
            &pattern,
            "java/util/regex/Pattern",
            "matcher",
            "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;",
            (first_input,),
        )
        .await?;
    let second: ClassInstanceRef<Matcher> = jvm
        .invoke_virtual(
            &pattern,
            "java/util/regex/Pattern",
            "matcher",
            "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;",
            (second_input,),
        )
        .await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&first, "java/util/regex/Matcher", "find", "()Z", ())
            .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(&second, "java/util/regex/Matcher", "find", "()Z", ())
            .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&first, "java/util/regex/Matcher", "start", "()I", ())
            .await?,
        0
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&second, "java/util/regex/Matcher", "start", "()I", ())
            .await?,
        1
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(&first, "java/util/regex/Matcher", "find", "()Z", ())
            .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&first, "java/util/regex/Matcher", "start", "()I", ())
            .await?,
        2
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&second, "java/util/regex/Matcher", "start", "()I", ())
            .await?,
        1
    );

    let value = JavaLangString::from_rust_string(&jvm, "xy").await?;
    let buffer: ClassInstanceRef<StringBuffer> = jvm.new_class("java/lang/StringBuffer", "(Ljava/lang/String;)V", (value,)).await?.into();
    let buffer_as_sequence: ClassInstanceRef<CharSequence> = ClassInstanceRef::new(buffer.clone().instance);
    let reset: ClassInstanceRef<Matcher> = jvm
        .invoke_virtual(
            &second,
            "java/util/regex/Matcher",
            "reset",
            "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;",
            (buffer_as_sequence,),
        )
        .await?;
    assert_eq!(reset.identity(), second.identity());
    assert!(
        jvm.invoke_virtual::<_, bool>(&second, "java/util/regex/Matcher", "find", "()Z", ())
            .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&second, "java/util/regex/Matcher", "start", "()I", ())
            .await?,
        0
    );
    let _: () = jvm
        .invoke_virtual(&buffer, "java/lang/StringBuffer", "setCharAt", "(IC)V", (0, 'y' as JavaChar))
        .await?;
    let parent: ClassInstanceRef<Pattern> = jvm
        .invoke_virtual(&second, "java/util/regex/Matcher", "pattern", "()Ljava/util/regex/Pattern;", ())
        .await?;
    assert_eq!(parent.identity(), pattern.identity());

    Ok(())
}
