use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use java_runtime::classes::java::{
    lang::String,
    util::{SimpleTimeZone, TimeZone},
};
use jvm::{Array, ClassInstanceRef, JavaError, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_timezone() -> Result<()> {
    let jvm = test_jvm().await?;

    let id = JavaLangString::from_rust_string(&jvm, "UTC").await?;
    let timezone: ClassInstanceRef<TimeZone> = jvm
        .invoke_static("java/util/TimeZone", "getTimeZone", "(Ljava/lang/String;)Ljava/util/TimeZone;", (id,))
        .await?;

    assert!(!timezone.is_null());

    let id: ClassInstanceRef<java_runtime::classes::java::lang::String> = jvm
        .invoke_virtual(&timezone, "java/util/TimeZone", "getID", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &id).await?, "UTC");
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&timezone, "java/util/TimeZone", "getRawOffset", "()I", ())
            .await?,
        0
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(&timezone, "java/util/TimeZone", "useDaylightTime", "()Z", ())
            .await?
    );

    let default: ClassInstanceRef<TimeZone> = jvm
        .invoke_static("java/util/TimeZone", "getDefault", "()Ljava/util/TimeZone;", ())
        .await?;
    let id: ClassInstanceRef<java_runtime::classes::java::lang::String> = jvm
        .invoke_virtual(&default, "java/util/TimeZone", "getID", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &id).await?, "GMT");

    let ids: ClassInstanceRef<Array<java_runtime::classes::java::lang::String>> = jvm
        .invoke_static("java/util/TimeZone", "getAvailableIDs", "()[Ljava/lang/String;", ())
        .await?;
    let ids = jvm
        .load_array::<ClassInstanceRef<java_runtime::classes::java::lang::String>>(&ids, 0, jvm.array_length(&ids).await?)
        .await?;
    let mut rust_ids = alloc::vec::Vec::new();
    for id in ids {
        rust_ids.push(JavaLangString::to_rust_string(&jvm, &id).await?);
    }
    assert!(rust_ids.iter().any(|id| id == "GMT"));

    let unknown = JavaLangString::from_rust_string(&jvm, "Unknown/Zone").await?;
    let fallback: ClassInstanceRef<TimeZone> = jvm
        .invoke_static(
            "java/util/TimeZone",
            "getTimeZone",
            "(Ljava/lang/String;)Ljava/util/TimeZone;",
            (unknown,),
        )
        .await?;
    let id: ClassInstanceRef<java_runtime::classes::java::lang::String> = jvm
        .invoke_virtual(&fallback, "java/util/TimeZone", "getID", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &id).await?, "GMT");

    Ok(())
}

#[tokio::test]
async fn test_simple_timezone_constructors_and_offset_validation() -> Result<()> {
    let jvm = test_jvm().await?;

    let id = JavaLangString::from_rust_string(&jvm, "Fixed").await?;
    let timezone = jvm
        .new_class("java/util/SimpleTimeZone", "(ILjava/lang/String;)V", (3_600_000, id))
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&timezone, "java/util/TimeZone", "getRawOffset", "()I", ())
            .await?,
        3_600_000
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&timezone, "java/util/TimeZone", "getOffset", "(IIIIII)I", (1, 2026, 0, 1, 1, 0))
            .await?,
        3_600_000
    );
    for (year, month, day) in [(2024, 1, 29), (2000, 1, 29), (2026, 3, 30), (2026, 11, 31)] {
        assert_eq!(
            jvm.invoke_virtual::<_, i32>(&timezone, "java/util/TimeZone", "getOffset", "(IIIIII)I", (1, year, month, day, 1, 0))
                .await?,
            3_600_000
        );
    }

    for (era, year, month, day) in [
        (2, 2026, 0, 1),
        (1, 2023, 1, 29),
        (1, 1900, 1, 29),
        (1, 2024, 1, 30),
        (1, 2026, 3, 31),
        (1, 2026, 5, 31),
        (1, 2026, 8, 31),
        (1, 2026, 10, 31),
    ] {
        let invalid: Result<i32> = jvm
            .invoke_virtual(&timezone, "java/util/TimeZone", "getOffset", "(IIIIII)I", (era, year, month, day, 1, 0))
            .await;
        let Err(JavaError::JavaException(exception)) = invalid else {
            panic!("invalid calendar fields must throw IllegalArgumentException");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/IllegalArgumentException"));
        assert_eq!(
            jvm.invoke_virtual::<_, i32>(&timezone, "java/util/TimeZone", "getRawOffset", "()I", ())
                .await?,
            3_600_000
        );
    }

    let id = JavaLangString::from_rust_string(&jvm, "Legacy").await?;
    let legacy = jvm.new_class("java/util/SimpleTimeZone", "(Ljava/lang/String;)V", (id,)).await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&legacy, &legacy.class_definition().name(), "getRawOffset", "()I", ())
            .await?,
        0
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(&legacy, &legacy.class_definition().name(), "useDaylightTime", "()Z", ())
            .await?
    );

    let null_id: ClassInstanceRef<String> = None.into();
    let result = jvm.new_class("java/util/SimpleTimeZone", "(ILjava/lang/String;)V", (0, null_id)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("null ID must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    Ok(())
}

#[tokio::test]
async fn tz_01_default_is_cloned_on_set_and_get_and_null_resets_gmt() -> Result<()> {
    let proto = TimeZone::as_proto();
    assert!(proto.access_flags.contains(ClassAccessFlags::PUBLIC | ClassAccessFlags::ABSTRACT));
    let default = proto
        .fields
        .iter()
        .find(|field| field.name == "defaultTimeZone")
        .expect("defaultTimeZone field");
    assert_eq!(default.descriptor, "Ljava/util/TimeZone;");
    assert!(default.access_flags.contains(FieldAccessFlags::PRIVATE | FieldAccessFlags::STATIC));
    let set_default = proto
        .methods
        .iter()
        .find(|method| method.name == "setDefault" && method.descriptor == "(Ljava/util/TimeZone;)V")
        .expect("setDefault");
    assert!(
        set_default
            .access_flags
            .contains(MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC | MethodAccessFlags::SYNCHRONIZED)
    );

    let jvm = test_jvm().await?;
    let id = JavaLangString::from_rust_string(&jvm, "Custom").await?;
    let configured: ClassInstanceRef<TimeZone> = jvm
        .new_class("java/util/SimpleTimeZone", "(ILjava/lang/String;)V", (3_600_000, id))
        .await?
        .into();
    let _: () = jvm
        .invoke_static("java/util/TimeZone", "setDefault", "(Ljava/util/TimeZone;)V", (configured.clone(),))
        .await?;

    let _: () = jvm
        .invoke_virtual(&configured, "java/util/TimeZone", "setRawOffset", "(I)V", (7_200_000,))
        .await?;
    let changed = JavaLangString::from_rust_string(&jvm, "Changed").await?;
    let _: () = jvm
        .invoke_virtual(&configured, "java/util/TimeZone", "setID", "(Ljava/lang/String;)V", (changed,))
        .await?;

    let first: ClassInstanceRef<TimeZone> = jvm
        .invoke_static("java/util/TimeZone", "getDefault", "()Ljava/util/TimeZone;", ())
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&first, "java/util/TimeZone", "getRawOffset", "()I", ())
            .await?,
        3_600_000
    );
    let first_id: ClassInstanceRef<String> = jvm
        .invoke_virtual(&first, "java/util/TimeZone", "getID", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &first_id).await?, "Custom");

    let _: () = jvm
        .invoke_virtual(&first, "java/util/TimeZone", "setRawOffset", "(I)V", (10_800_000,))
        .await?;
    let second: ClassInstanceRef<TimeZone> = jvm
        .invoke_static("java/util/TimeZone", "getDefault", "()Ljava/util/TimeZone;", ())
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&second, "java/util/TimeZone", "getRawOffset", "()I", ())
            .await?,
        3_600_000
    );

    let null: ClassInstanceRef<TimeZone> = None.into();
    let _: () = jvm
        .invoke_static("java/util/TimeZone", "setDefault", "(Ljava/util/TimeZone;)V", (null,))
        .await?;
    let reset: ClassInstanceRef<TimeZone> = jvm
        .invoke_static("java/util/TimeZone", "getDefault", "()Ljava/util/TimeZone;", ())
        .await?;
    let id: ClassInstanceRef<String> = jvm
        .invoke_virtual(&reset, "java/util/TimeZone", "getID", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &id).await?, "GMT");
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&reset, "java/util/TimeZone", "getRawOffset", "()I", ())
            .await?,
        0
    );

    Ok(())
}

#[tokio::test]
async fn tz_02_to_04_id_abstract_contract_and_simple_timezone_mutation() -> Result<()> {
    let timezone_proto = TimeZone::as_proto();
    for (name, descriptor) in [
        ("setID", "(Ljava/lang/String;)V"),
        ("inDaylightTime", "(Ljava/util/Date;)Z"),
        ("setRawOffset", "(I)V"),
    ] {
        let method = timezone_proto
            .methods
            .iter()
            .find(|method| method.name == name && method.descriptor == descriptor)
            .expect("TimeZone method");
        assert!(method.access_flags.contains(MethodAccessFlags::PUBLIC));
        if name != "setID" {
            assert!(method.access_flags.contains(MethodAccessFlags::ABSTRACT));
        }
    }
    let simple_proto = SimpleTimeZone::as_proto();
    for (name, descriptor) in [("inDaylightTime", "(Ljava/util/Date;)Z"), ("setRawOffset", "(I)V")] {
        let method = simple_proto
            .methods
            .iter()
            .find(|method| method.name == name && method.descriptor == descriptor)
            .expect("SimpleTimeZone method");
        assert!(method.access_flags.contains(MethodAccessFlags::PUBLIC));
    }

    let jvm = test_jvm().await?;
    let id = JavaLangString::from_rust_string(&jvm, "Initial").await?;
    let timezone = jvm.new_class("java/util/SimpleTimeZone", "(ILjava/lang/String;)V", (1_000, id)).await?;
    let changed = JavaLangString::from_rust_string(&jvm, "Changed").await?;
    let _: () = jvm
        .invoke_virtual(&timezone, "java/util/TimeZone", "setID", "(Ljava/lang/String;)V", (changed,))
        .await?;
    let id: ClassInstanceRef<String> = jvm
        .invoke_virtual(&timezone, "java/util/TimeZone", "getID", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &id).await?, "Changed");

    let _: () = jvm
        .invoke_virtual(&timezone, "java/util/TimeZone", "setRawOffset", "(I)V", (-2_000,))
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&timezone, "java/util/TimeZone", "getRawOffset", "()I", ())
            .await?,
        -2_000
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&timezone, "java/util/TimeZone", "getOffset", "(IIIIII)I", (1, 2026, 0, 1, 1, 0))
            .await?,
        -2_000
    );
    let date = jvm.new_class("java/util/Date", "(J)V", (0i64,)).await?;
    assert!(
        !jvm.invoke_virtual::<_, bool>(&timezone, "java/util/TimeZone", "inDaylightTime", "(Ljava/util/Date;)Z", (date,))
            .await?
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(&timezone, "java/util/TimeZone", "useDaylightTime", "()Z", ())
            .await?
    );

    let null_date: ClassInstanceRef<java_runtime::classes::java::util::Date> = None.into();
    let result: Result<bool> = jvm
        .invoke_virtual(&timezone, "java/util/TimeZone", "inDaylightTime", "(Ljava/util/Date;)Z", (null_date,))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("null Date must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&timezone, "java/util/TimeZone", "getRawOffset", "()I", ())
            .await?,
        -2_000
    );

    let null_id: ClassInstanceRef<String> = None.into();
    let result: Result<()> = jvm
        .invoke_virtual(&timezone, "java/util/TimeZone", "setID", "(Ljava/lang/String;)V", (null_id,))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("null ID must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    Ok(())
}
