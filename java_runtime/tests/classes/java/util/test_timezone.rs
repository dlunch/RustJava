use java_runtime::classes::java::{lang::String, util::TimeZone};
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

    let id: ClassInstanceRef<java_runtime::classes::java::lang::String> = jvm.invoke_virtual(&timezone, "getID", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &id).await?, "UTC");
    assert_eq!(jvm.invoke_virtual::<_, i32>(&timezone, "getRawOffset", "()I", ()).await?, 0);
    assert!(!jvm.invoke_virtual::<_, bool>(&timezone, "useDaylightTime", "()Z", ()).await?);

    let default: ClassInstanceRef<TimeZone> = jvm
        .invoke_static("java/util/TimeZone", "getDefault", "()Ljava/util/TimeZone;", ())
        .await?;
    let id: ClassInstanceRef<java_runtime::classes::java::lang::String> = jvm.invoke_virtual(&default, "getID", "()Ljava/lang/String;", ()).await?;
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
    let id: ClassInstanceRef<java_runtime::classes::java::lang::String> = jvm.invoke_virtual(&fallback, "getID", "()Ljava/lang/String;", ()).await?;
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
    assert_eq!(jvm.invoke_virtual::<_, i32>(&timezone, "getRawOffset", "()I", ()).await?, 3_600_000);
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&timezone, "getOffset", "(IIIIII)I", (1, 2026, 0, 1, 1, 0))
            .await?,
        3_600_000
    );

    let invalid: Result<i32> = jvm.invoke_virtual(&timezone, "getOffset", "(IIIIII)I", (2, 2026, 0, 1, 1, 0)).await;
    let Err(JavaError::JavaException(exception)) = invalid else {
        panic!("invalid era must throw IllegalArgumentException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalArgumentException"));

    let id = JavaLangString::from_rust_string(&jvm, "Legacy").await?;
    let legacy = jvm.new_class("java/util/SimpleTimeZone", "(Ljava/lang/String;)V", (id,)).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&legacy, "getRawOffset", "()I", ()).await?, 0);
    assert!(!jvm.invoke_virtual::<_, bool>(&legacy, "useDaylightTime", "()Z", ()).await?);

    let null_id: ClassInstanceRef<String> = None.into();
    let result = jvm.new_class("java/util/SimpleTimeZone", "(ILjava/lang/String;)V", (0, null_id)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("null ID must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    Ok(())
}
