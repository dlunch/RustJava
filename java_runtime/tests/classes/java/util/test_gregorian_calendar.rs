use java_runtime::classes::java::lang::Object;
use jvm::{ClassInstanceRef, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_gregorian_calendar() -> Result<()> {
    let jvm = test_jvm().await?;

    let timestamp = 0i64;
    let calendar = jvm
        .invoke_static("java/util/Calendar", "getInstance", "()Ljava/util/Calendar;", ())
        .await?;
    let date = jvm.new_class("java/util/Date", "(J)V", (timestamp,)).await?;

    let _: () = jvm.invoke_virtual(&calendar, "setTime", "(Ljava/util/Date;)V", (date,)).await?;
    let year: i32 = jvm.invoke_virtual(&calendar, "get", "(I)I", (1,)).await?;
    assert_eq!(1970, year);

    let month: i32 = jvm.invoke_virtual(&calendar, "get", "(I)I", (2,)).await?;
    assert_eq!(0, month);

    let day: i32 = jvm.invoke_virtual(&calendar, "get", "(I)I", (5,)).await?;
    assert_eq!(1, day);

    let timestamp = 737521516000i64;
    let date = jvm.new_class("java/util/Date", "(J)V", (timestamp,)).await?;

    let _: () = jvm.invoke_virtual(&calendar, "setTime", "(Ljava/util/Date;)V", (date,)).await?;

    let year: i32 = jvm.invoke_virtual(&calendar, "get", "(I)I", (1,)).await?;
    assert_eq!(1993, year);

    let month: i32 = jvm.invoke_virtual(&calendar, "get", "(I)I", (2,)).await?;
    assert_eq!(4, month);

    let day: i32 = jvm.invoke_virtual(&calendar, "get", "(I)I", (5,)).await?;
    assert_eq!(16, day);

    let hour: i32 = jvm.invoke_virtual(&calendar, "get", "(I)I", (11,)).await?;
    assert_eq!(3, hour);

    let minute: i32 = jvm.invoke_virtual(&calendar, "get", "(I)I", (12,)).await?;
    assert_eq!(5, minute);

    let second: i32 = jvm.invoke_virtual(&calendar, "get", "(I)I", (13,)).await?;
    assert_eq!(16, second);

    let _: () = jvm.invoke_virtual(&calendar, "set", "(II)V", (1, 1999)).await?;
    let date = jvm.invoke_virtual(&calendar, "getTime", "()Ljava/util/Date;", ()).await?;
    let timestamp: i64 = jvm.invoke_virtual(&date, "getTime", "()J", ()).await?;
    assert_eq!(926823916000, timestamp);

    Ok(())
}

#[tokio::test]
async fn test_calendar_cldc11_time_and_comparison_api() -> Result<()> {
    let jvm = test_jvm().await?;
    let first: ClassInstanceRef<Object> = jvm
        .invoke_static("java/util/Calendar", "getInstance", "()Ljava/util/Calendar;", ())
        .await?;
    let second: ClassInstanceRef<Object> = jvm
        .invoke_static("java/util/Calendar", "getInstance", "()Ljava/util/Calendar;", ())
        .await?;

    let _: () = jvm.invoke_virtual(&first, "setTimeInMillis", "(J)V", (1000i64,)).await?;
    let _: () = jvm.invoke_virtual(&second, "setTimeInMillis", "(J)V", (2000i64,)).await?;
    assert_eq!(jvm.invoke_virtual::<_, i64>(&first, "getTimeInMillis", "()J", ()).await?, 1000);
    assert!(
        jvm.invoke_virtual::<_, bool>(&first, "before", "(Ljava/lang/Object;)Z", (second.clone(),))
            .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(&second, "after", "(Ljava/lang/Object;)Z", (first.clone(),))
            .await?
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(&first, "equals", "(Ljava/lang/Object;)Z", (second.clone(),))
            .await?
    );

    let _: () = jvm.invoke_virtual(&second, "setTimeInMillis", "(J)V", (1000i64,)).await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&first, "equals", "(Ljava/lang/Object;)Z", (second.clone(),))
            .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&first, "hashCode", "()I", ()).await?,
        jvm.invoke_virtual::<_, i32>(&second, "hashCode", "()I", ()).await?
    );

    let id = JavaLangString::from_rust_string(&jvm, "JST").await?;
    let zone = jvm
        .new_class("java/util/SimpleTimeZone", "(ILjava/lang/String;)V", (9 * 60 * 60 * 1000, id))
        .await?;
    let calendar: ClassInstanceRef<Object> = jvm
        .invoke_static("java/util/Calendar", "getInstance", "(Ljava/util/TimeZone;)Ljava/util/Calendar;", (zone,))
        .await?;
    let _: () = jvm.invoke_virtual(&calendar, "setTimeInMillis", "(J)V", (0i64,)).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&calendar, "get", "(I)I", (11,)).await?, 9);
    let zone: ClassInstanceRef<Object> = jvm.invoke_virtual(&calendar, "getTimeZone", "()Ljava/util/TimeZone;", ()).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&zone, "getRawOffset", "()I", ()).await?, 9 * 60 * 60 * 1000);

    Ok(())
}
