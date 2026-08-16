use java_constants::{ClassAccessFlags, MethodAccessFlags};
use java_runtime::{classes::java::lang::Object, get_runtime_class_proto};
use jvm::{ClassInstanceRef, JavaChar, Result, runtime::JavaLangString};
use test_utils::test_jvm;

#[test]
fn java_5_builder_hierarchy_and_appendable_descriptors_are_registered() {
    let appendable = get_runtime_class_proto("java/lang/Appendable").expect("Appendable must be registered");
    assert_eq!(appendable.parent_class, None);
    assert_eq!(
        appendable.access_flags,
        ClassAccessFlags::PUBLIC | ClassAccessFlags::INTERFACE | ClassAccessFlags::ABSTRACT
    );
    for descriptor in [
        "(Ljava/lang/CharSequence;)Ljava/lang/Appendable;",
        "(Ljava/lang/CharSequence;II)Ljava/lang/Appendable;",
        "(C)Ljava/lang/Appendable;",
    ] {
        let method = appendable
            .methods
            .iter()
            .find(|method| method.name == "append" && method.descriptor == descriptor)
            .unwrap_or_else(|| panic!("missing Appendable.append{descriptor}"));
        assert_eq!(method.access_flags, MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT);
    }

    let base = get_runtime_class_proto("java/lang/AbstractStringBuilder").expect("AbstractStringBuilder must be registered");
    assert_eq!(base.parent_class, Some("java/lang/Object"));
    assert_eq!(base.interfaces, vec!["java/lang/Appendable", "java/lang/CharSequence"]);
    assert_eq!(base.access_flags, ClassAccessFlags::ABSTRACT);

    let builder = get_runtime_class_proto("java/lang/StringBuilder").expect("StringBuilder must be registered");
    assert_eq!(builder.parent_class, Some("java/lang/AbstractStringBuilder"));
    assert_eq!(builder.interfaces, vec!["java/io/Serializable", "java/lang/CharSequence"]);
    assert_eq!(builder.access_flags, ClassAccessFlags::PUBLIC | ClassAccessFlags::FINAL);
    assert_eq!(builder.methods.len(), 72);

    for (name, descriptor) in [
        ("<init>", "()V"),
        ("<init>", "(I)V"),
        ("<init>", "(Ljava/lang/String;)V"),
        ("<init>", "(Ljava/lang/CharSequence;)V"),
        ("append", "(Ljava/lang/Object;)Ljava/lang/StringBuilder;"),
        ("append", "(Ljava/lang/String;)Ljava/lang/StringBuilder;"),
        ("append", "(Ljava/lang/StringBuffer;)Ljava/lang/StringBuilder;"),
        ("append", "(Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;"),
        ("append", "(Ljava/lang/CharSequence;II)Ljava/lang/StringBuilder;"),
        ("append", "([C)Ljava/lang/StringBuilder;"),
        ("append", "([CII)Ljava/lang/StringBuilder;"),
        ("append", "(Z)Ljava/lang/StringBuilder;"),
        ("append", "(C)Ljava/lang/StringBuilder;"),
        ("append", "(I)Ljava/lang/StringBuilder;"),
        ("append", "(J)Ljava/lang/StringBuilder;"),
        ("append", "(F)Ljava/lang/StringBuilder;"),
        ("append", "(D)Ljava/lang/StringBuilder;"),
        ("appendCodePoint", "(I)Ljava/lang/StringBuilder;"),
        ("delete", "(II)Ljava/lang/StringBuilder;"),
        ("deleteCharAt", "(I)Ljava/lang/StringBuilder;"),
        ("replace", "(IILjava/lang/String;)Ljava/lang/StringBuilder;"),
        ("insert", "(I[CII)Ljava/lang/StringBuilder;"),
        ("insert", "(ILjava/lang/Object;)Ljava/lang/StringBuilder;"),
        ("insert", "(ILjava/lang/String;)Ljava/lang/StringBuilder;"),
        ("insert", "(I[C)Ljava/lang/StringBuilder;"),
        ("insert", "(ILjava/lang/CharSequence;)Ljava/lang/StringBuilder;"),
        ("insert", "(ILjava/lang/CharSequence;II)Ljava/lang/StringBuilder;"),
        ("insert", "(IZ)Ljava/lang/StringBuilder;"),
        ("insert", "(IC)Ljava/lang/StringBuilder;"),
        ("insert", "(II)Ljava/lang/StringBuilder;"),
        ("insert", "(IJ)Ljava/lang/StringBuilder;"),
        ("insert", "(IF)Ljava/lang/StringBuilder;"),
        ("insert", "(ID)Ljava/lang/StringBuilder;"),
        ("indexOf", "(Ljava/lang/String;)I"),
        ("indexOf", "(Ljava/lang/String;I)I"),
        ("lastIndexOf", "(Ljava/lang/String;)I"),
        ("lastIndexOf", "(Ljava/lang/String;I)I"),
        ("reverse", "()Ljava/lang/StringBuilder;"),
        ("toString", "()Ljava/lang/String;"),
    ] {
        let method = builder
            .methods
            .iter()
            .find(|method| method.name == name && method.descriptor == descriptor)
            .unwrap_or_else(|| panic!("missing StringBuilder.{name}{descriptor}"));
        assert!(method.access_flags.contains(MethodAccessFlags::PUBLIC));
    }

    let abstract_string_builder_bridges = builder
        .methods
        .iter()
        .filter(|method| method.descriptor.ends_with("Ljava/lang/AbstractStringBuilder;"))
        .collect::<Vec<_>>();
    assert_eq!(abstract_string_builder_bridges.len(), 30);
    for method in abstract_string_builder_bridges {
        assert_eq!(
            method.access_flags,
            MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC
        );
    }
    let appendable_bridges = builder
        .methods
        .iter()
        .filter(|method| method.name == "append" && method.descriptor.ends_with("Ljava/lang/Appendable;"))
        .collect::<Vec<_>>();
    assert_eq!(appendable_bridges.len(), 3);
    for method in appendable_bridges {
        assert_eq!(
            method.access_flags,
            MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC
        );
    }

    let buffer = get_runtime_class_proto("java/lang/StringBuffer").expect("StringBuffer must be registered");
    assert_eq!(buffer.parent_class, Some("java/lang/AbstractStringBuilder"));
    assert_eq!(buffer.interfaces, vec!["java/io/Serializable", "java/lang/CharSequence"]);
    assert_eq!(buffer.access_flags, ClassAccessFlags::PUBLIC | ClassAccessFlags::FINAL);
    assert_eq!(buffer.methods.len(), 87);
    assert_eq!(
        buffer
            .methods
            .iter()
            .filter(|method| method.descriptor.ends_with("Ljava/lang/AbstractStringBuilder;"))
            .count(),
        30
    );
    assert_eq!(
        buffer
            .methods
            .iter()
            .filter(|method| method.name == "append" && method.descriptor.ends_with("Ljava/lang/Appendable;"))
            .count(),
        3
    );
}

#[tokio::test]
async fn string_builder_mutation_queries_and_appendable_bridge_use_utf16() -> Result<()> {
    let jvm = test_jvm().await?;
    let builder = jvm.new_class("java/lang/StringBuilder", "()V", ()).await?;

    let text = JavaLangString::from_utf16(&jvm, vec!['A' as JavaChar, 0xd83d, 0xde00]).await?;
    let text: ClassInstanceRef<java_runtime::classes::java::lang::String> = text.into();
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &builder,
            &builder.class_definition().name(),
            "append",
            "(Ljava/lang/CharSequence;)Ljava/lang/Appendable;",
            (text,),
        )
        .await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &builder,
            &builder.class_definition().name(),
            "append",
            "(I)Ljava/lang/StringBuilder;",
            (42,),
        )
        .await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &builder,
            &builder.class_definition().name(),
            "insert",
            "(IC)Ljava/lang/StringBuilder;",
            (1, '-' as JavaChar),
        )
        .await?;

    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&builder, &builder.class_definition().name(), "length", "()I", ())
            .await?,
        6
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&builder, &builder.class_definition().name(), "codePointAt", "(I)I", (2,))
            .await?,
        0x1f600
    );

    let result = jvm
        .invoke_virtual(&builder, &builder.class_definition().name(), "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(
        JavaLangString::to_utf16(&jvm, &result).await?,
        vec![0x41, 0x2d, 0xd83d, 0xde00, 0x34, 0x32]
    );

    Ok(())
}
