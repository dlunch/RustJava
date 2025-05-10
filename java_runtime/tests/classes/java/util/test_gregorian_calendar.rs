use jvm::Result;

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
