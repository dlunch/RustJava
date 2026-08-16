use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use java_runtime::{
    classes::java::{lang::String, util::logging::Level},
    get_runtime_class_proto,
};
use jvm::{ClassInstanceRef, JavaError, Result, runtime::JavaLangString};
use test_utils::test_jvm;

#[test]
fn logging_core_public_types_are_registered() {
    for (name, parent, flags) in [
        (
            "java/util/logging/Filter",
            None,
            ClassAccessFlags::PUBLIC | ClassAccessFlags::INTERFACE | ClassAccessFlags::ABSTRACT,
        ),
        (
            "java/util/logging/Formatter",
            Some("java/lang/Object"),
            ClassAccessFlags::PUBLIC | ClassAccessFlags::ABSTRACT,
        ),
        (
            "java/util/logging/Handler",
            Some("java/lang/Object"),
            ClassAccessFlags::PUBLIC | ClassAccessFlags::ABSTRACT,
        ),
        (
            "java/util/logging/StreamHandler",
            Some("java/util/logging/Handler"),
            ClassAccessFlags::PUBLIC,
        ),
        (
            "java/util/logging/ConsoleHandler",
            Some("java/util/logging/StreamHandler"),
            ClassAccessFlags::PUBLIC,
        ),
        (
            "java/util/logging/SimpleFormatter",
            Some("java/util/logging/Formatter"),
            ClassAccessFlags::PUBLIC,
        ),
        ("java/util/logging/Level", Some("java/lang/Object"), ClassAccessFlags::PUBLIC),
        ("java/util/logging/LogRecord", Some("java/lang/Object"), ClassAccessFlags::PUBLIC),
        ("java/util/logging/Logger", Some("java/lang/Object"), ClassAccessFlags::PUBLIC),
        ("java/util/logging/LogManager", Some("java/lang/Object"), ClassAccessFlags::PUBLIC),
    ] {
        let proto = get_runtime_class_proto(name).unwrap_or_else(|| panic!("{name} must be registered"));
        assert_eq!(proto.parent_class, parent);
        assert_eq!(proto.access_flags, flags);
    }

    let logger = get_runtime_class_proto("java/util/logging/Logger").unwrap();
    for (name, descriptor) in [
        ("getLogger", "(Ljava/lang/String;)Ljava/util/logging/Logger;"),
        ("log", "(Ljava/util/logging/Level;Ljava/lang/String;)V"),
        (
            "logp",
            "(Ljava/util/logging/Level;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
        ),
        ("info", "(Ljava/lang/String;)V"),
        ("warning", "(Ljava/lang/String;)V"),
        ("severe", "(Ljava/lang/String;)V"),
    ] {
        let method = logger
            .methods
            .iter()
            .find(|method| method.name == name && method.descriptor == descriptor)
            .unwrap_or_else(|| panic!("missing Logger.{name}{descriptor}"));
        assert!(method.access_flags.contains(MethodAccessFlags::PUBLIC));
    }

    let global = logger.fields.iter().find(|field| field.name == "global").unwrap();
    assert_eq!(
        global.access_flags,
        FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL
    );
}

#[tokio::test]
async fn standard_levels_have_java_values_and_parse_names_or_numbers() -> Result<()> {
    let jvm = test_jvm().await?;
    for (name, value) in [
        ("OFF", i32::MAX),
        ("SEVERE", 1000),
        ("WARNING", 900),
        ("INFO", 800),
        ("CONFIG", 700),
        ("FINE", 500),
        ("FINER", 400),
        ("FINEST", 300),
        ("ALL", i32::MIN),
    ] {
        let level: ClassInstanceRef<Level> = jvm.get_static_field("java/util/logging/Level", name, "Ljava/util/logging/Level;").await?;
        assert_eq!(
            jvm.invoke_virtual::<_, i32>(&level, "java/util/logging/Level", "intValue", "()I", ())
                .await?,
            value
        );
        let parsed: ClassInstanceRef<Level> = jvm
            .invoke_static(
                "java/util/logging/Level",
                "parse",
                "(Ljava/lang/String;)Ljava/util/logging/Level;",
                (JavaLangString::from_rust_string(&jvm, name).await?,),
            )
            .await?;
        assert_eq!(parsed.identity(), level.identity());
    }

    let custom_name = JavaLangString::from_rust_string(&jvm, "42").await?;
    let custom: ClassInstanceRef<Level> = jvm
        .invoke_static(
            "java/util/logging/Level",
            "parse",
            "(Ljava/lang/String;)Ljava/util/logging/Level;",
            (custom_name,),
        )
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&custom, "java/util/logging/Level", "intValue", "()I", ())
            .await?,
        42
    );
    let name: ClassInstanceRef<String> = jvm
        .invoke_virtual(&custom, "java/util/logging/Level", "getName", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &name).await?, "42");

    Ok(())
}

#[tokio::test]
async fn level_rejects_unknown_names() -> Result<()> {
    let jvm = test_jvm().await?;
    let name = JavaLangString::from_rust_string(&jvm, "TRACE").await?;
    let result: Result<ClassInstanceRef<Level>> = jvm
        .invoke_static(
            "java/util/logging/Level",
            "parse",
            "(Ljava/lang/String;)Ljava/util/logging/Level;",
            (name,),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("unknown level must throw IllegalArgumentException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalArgumentException"));
    Ok(())
}
