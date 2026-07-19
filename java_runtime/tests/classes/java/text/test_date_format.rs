use java_runtime::classes::java::{
    lang::{String, StringBuffer},
    text::{DateFormat, FieldPosition, ParsePosition, SimpleDateFormat},
    util::{Calendar, Date, SimpleTimeZone, TimeZone},
};
use jvm::{ClassInstanceRef, JavaError, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_date_format_factories_use_english_patterns() -> Result<()> {
    let jvm = test_jvm().await?;
    let date: ClassInstanceRef<Date> = jvm.new_class("java/util/Date", "(J)V", (0i64,)).await?.into();

    let short: ClassInstanceRef<DateFormat> = jvm
        .invoke_static("java/text/DateFormat", "getDateInstance", "(I)Ljava/text/DateFormat;", (3,))
        .await?;
    let text: ClassInstanceRef<String> = jvm
        .invoke_virtual(&short, "format", "(Ljava/util/Date;)Ljava/lang/String;", (date.clone(),))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "1/1/70");

    let medium: ClassInstanceRef<DateFormat> = jvm
        .invoke_static("java/text/DateFormat", "getDateInstance", "(I)Ljava/text/DateFormat;", (2,))
        .await?;
    let text: ClassInstanceRef<String> = jvm
        .invoke_virtual(&medium, "format", "(Ljava/util/Date;)Ljava/lang/String;", (date.clone(),))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "Jan 1, 1970");

    let time: ClassInstanceRef<DateFormat> = jvm
        .invoke_static("java/text/DateFormat", "getTimeInstance", "(I)Ljava/text/DateFormat;", (3,))
        .await?;
    let text: ClassInstanceRef<String> = jvm
        .invoke_virtual(&time, "format", "(Ljava/util/Date;)Ljava/lang/String;", (date,))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "12:00 AM");

    Ok(())
}

#[tokio::test]
async fn test_simple_date_format_patterns_quotes_and_fields() -> Result<()> {
    let jvm = test_jvm().await?;
    let pattern = JavaLangString::from_rust_string(&jvm, "yyyy-MM-dd 'at' HH:mm:ss.SSS EEEE MMMM a z").await?;
    let format: ClassInstanceRef<SimpleDateFormat> = jvm
        .new_class("java/text/SimpleDateFormat", "(Ljava/lang/String;)V", (pattern,))
        .await?
        .into();
    let date: ClassInstanceRef<Date> = jvm.new_class("java/util/Date", "(J)V", (0i64,)).await?.into();
    let text: ClassInstanceRef<String> = jvm
        .invoke_virtual(&format, "format", "(Ljava/util/Date;)Ljava/lang/String;", (date.clone(),))
        .await?;
    assert_eq!(
        JavaLangString::to_rust_string(&jvm, &text).await?,
        "1970-01-01 at 00:00:00.000 Thursday January AM GMT"
    );

    let field_pattern = JavaLangString::from_rust_string(&jvm, "yyyy-MM").await?;
    let _: () = jvm
        .invoke_virtual(&format, "applyPattern", "(Ljava/lang/String;)V", (field_pattern,))
        .await?;
    let prefix = JavaLangString::from_rust_string(&jvm, "on ").await?;
    let buffer: ClassInstanceRef<StringBuffer> = jvm.new_class("java/lang/StringBuffer", "(Ljava/lang/String;)V", (prefix,)).await?.into();
    let position: ClassInstanceRef<FieldPosition> = jvm.new_class("java/text/FieldPosition", "(I)V", (1,)).await?.into();
    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(
            &format,
            "format",
            "(Ljava/util/Date;Ljava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;",
            (date, buffer.clone(), position.clone()),
        )
        .await?;
    let text: ClassInstanceRef<String> = jvm.invoke_virtual(&buffer, "toString", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "on 1970-01");
    assert_eq!(jvm.invoke_virtual::<_, i32>(&position, "getBeginIndex", "()I", ()).await?, 3);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&position, "getEndIndex", "()I", ()).await?, 7);

    Ok(())
}

#[tokio::test]
async fn test_date_format_timezone_and_calendar_state() -> Result<()> {
    let jvm = test_jvm().await?;
    let pattern = JavaLangString::from_rust_string(&jvm, "yyyy-MM-dd HH:mm:ss z").await?;
    let format: ClassInstanceRef<DateFormat> = jvm
        .new_class("java/text/SimpleDateFormat", "(Ljava/lang/String;)V", (pattern,))
        .await?
        .into();
    let id = JavaLangString::from_rust_string(&jvm, "KST").await?;
    let timezone: ClassInstanceRef<SimpleTimeZone> = jvm
        .new_class("java/util/SimpleTimeZone", "(ILjava/lang/String;)V", (9 * 60 * 60 * 1000, id))
        .await?
        .into();
    let _: () = jvm.invoke_virtual(&format, "setTimeZone", "(Ljava/util/TimeZone;)V", (timezone,)).await?;

    let date: ClassInstanceRef<Date> = jvm.new_class("java/util/Date", "(J)V", (0i64,)).await?.into();
    let text: ClassInstanceRef<String> = jvm
        .invoke_virtual(&format, "format", "(Ljava/util/Date;)Ljava/lang/String;", (date,))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "1970-01-01 09:00:00 GMT+09:00");

    let actual: ClassInstanceRef<TimeZone> = jvm.invoke_virtual(&format, "getTimeZone", "()Ljava/util/TimeZone;", ()).await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&actual, "getRawOffset", "()I", ()).await?,
        9 * 60 * 60 * 1000
    );
    let _: () = jvm.invoke_virtual(&format, "setLenient", "(Z)V", (false,)).await?;
    assert!(!jvm.invoke_virtual::<_, bool>(&format, "isLenient", "()Z", ()).await?);

    Ok(())
}

#[tokio::test]
async fn test_simple_date_format_parse_and_positions() -> Result<()> {
    let jvm = test_jvm().await?;
    let pattern = JavaLangString::from_rust_string(&jvm, "yyyy-MM-dd HH:mm:ss.SSS").await?;
    let format: ClassInstanceRef<SimpleDateFormat> = jvm
        .new_class("java/text/SimpleDateFormat", "(Ljava/lang/String;)V", (pattern,))
        .await?
        .into();
    let source = JavaLangString::from_rust_string(&jvm, "1970-01-02 03:04:05.006tail").await?;
    let position: ClassInstanceRef<ParsePosition> = jvm.new_class("java/text/ParsePosition", "(I)V", (0,)).await?.into();
    let date: ClassInstanceRef<Date> = jvm
        .invoke_virtual(
            &format,
            "parse",
            "(Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/util/Date;",
            (source, position.clone()),
        )
        .await?;
    assert_eq!(jvm.invoke_virtual::<_, i64>(&date, "getTime", "()J", ()).await?, 97_445_006);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&position, "getIndex", "()I", ()).await?, 23);

    let text_pattern = JavaLangString::from_rust_string(&jvm, "MMMM d, yyyy h:mm a z").await?;
    let _: () = jvm
        .invoke_virtual(&format, "applyPattern", "(Ljava/lang/String;)V", (text_pattern,))
        .await?;
    let source = JavaLangString::from_rust_string(&jvm, "January 2, 1970 3:04 PM GMT").await?;
    let date: ClassInstanceRef<Date> = jvm
        .invoke_virtual(&format, "parse", "(Ljava/lang/String;)Ljava/util/Date;", (source,))
        .await?;
    assert_eq!(jvm.invoke_virtual::<_, i64>(&date, "getTime", "()J", ()).await?, 140_640_000);

    let invalid = JavaLangString::from_rust_string(&jvm, "not a date").await?;
    let result = jvm
        .invoke_virtual::<_, ClassInstanceRef<Date>>(&format, "parse", "(Ljava/lang/String;)Ljava/util/Date;", (invalid,))
        .await;
    assert!(matches!(result, Err(JavaError::JavaException(_))));

    Ok(())
}

#[tokio::test]
async fn test_simple_date_format_uses_utf16_positions() -> Result<()> {
    let jvm = test_jvm().await?;
    let pattern = JavaLangString::from_rust_string(&jvm, "'\u{1f600}'yyyy").await?;
    let format: ClassInstanceRef<SimpleDateFormat> = jvm
        .new_class("java/text/SimpleDateFormat", "(Ljava/lang/String;)V", (pattern,))
        .await?
        .into();
    let date: ClassInstanceRef<Date> = jvm.new_class("java/util/Date", "(J)V", (0i64,)).await?.into();
    let buffer: ClassInstanceRef<StringBuffer> = jvm.new_class("java/lang/StringBuffer", "()V", ()).await?.into();
    let position: ClassInstanceRef<FieldPosition> = jvm.new_class("java/text/FieldPosition", "(I)V", (1,)).await?.into();
    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(
            &format,
            "format",
            "(Ljava/util/Date;Ljava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;",
            (date, buffer.clone(), position.clone()),
        )
        .await?;
    let text: ClassInstanceRef<String> = jvm.invoke_virtual(&buffer, "toString", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "\u{1f600}1970");
    assert_eq!(jvm.invoke_virtual::<_, i32>(&position, "getBeginIndex", "()I", ()).await?, 2);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&position, "getEndIndex", "()I", ()).await?, 6);

    let pattern = JavaLangString::from_rust_string(&jvm, "yyyy-MM-dd").await?;
    let _: () = jvm.invoke_virtual(&format, "applyPattern", "(Ljava/lang/String;)V", (pattern,)).await?;
    let source = JavaLangString::from_rust_string(&jvm, "\u{1f600}1970-01-02").await?;
    let position: ClassInstanceRef<ParsePosition> = jvm.new_class("java/text/ParsePosition", "(I)V", (2,)).await?.into();
    let date: ClassInstanceRef<Date> = jvm
        .invoke_virtual(
            &format,
            "parse",
            "(Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/util/Date;",
            (source, position.clone()),
        )
        .await?;
    assert_eq!(jvm.invoke_virtual::<_, i64>(&date, "getTime", "()J", ()).await?, 86_400_000);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&position, "getIndex", "()I", ()).await?, 12);

    let source = JavaLangString::from_rust_string(&jvm, "\u{1f600}197x").await?;
    let position: ClassInstanceRef<ParsePosition> = jvm.new_class("java/text/ParsePosition", "(I)V", (2,)).await?.into();
    let date: ClassInstanceRef<Date> = jvm
        .invoke_virtual(
            &format,
            "parse",
            "(Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/util/Date;",
            (source, position.clone()),
        )
        .await?;
    assert!(date.is_null());
    assert_eq!(jvm.invoke_virtual::<_, i32>(&position, "getErrorIndex", "()I", ()).await?, 5);

    Ok(())
}

#[tokio::test]
async fn test_date_format_clone_and_calendar_leniency_are_isolated() -> Result<()> {
    let jvm = test_jvm().await?;
    let pattern = JavaLangString::from_rust_string(&jvm, "yyyy-MM-dd").await?;
    let format: ClassInstanceRef<DateFormat> = jvm
        .new_class("java/text/SimpleDateFormat", "(Ljava/lang/String;)V", (pattern,))
        .await?
        .into();
    let calendar: ClassInstanceRef<Calendar> = jvm.invoke_virtual(&format, "getCalendar", "()Ljava/util/Calendar;", ()).await?;
    let _: () = jvm.invoke_virtual(&calendar, "setTimeInMillis", "(J)V", (0i64,)).await?;

    let cloned: ClassInstanceRef<DateFormat> = jvm.invoke_virtual(&format, "clone", "()Ljava/lang/Object;", ()).await?;
    let cloned_calendar: ClassInstanceRef<Calendar> = jvm.invoke_virtual(&cloned, "getCalendar", "()Ljava/util/Calendar;", ()).await?;
    let _: () = jvm.invoke_virtual(&cloned_calendar, "setTimeInMillis", "(J)V", (86_400_000i64,)).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&calendar, "get", "(I)I", (5,)).await?, 1);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&cloned_calendar, "get", "(I)I", (5,)).await?, 2);

    let other: ClassInstanceRef<Calendar> = jvm
        .invoke_static("java/util/Calendar", "getInstance", "()Ljava/util/Calendar;", ())
        .await?;
    let _: () = jvm.invoke_virtual(&other, "setTimeInMillis", "(J)V", (0i64,)).await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&calendar, "equals", "(Ljava/lang/Object;)Z", (other.clone(),))
            .await?
    );
    let _: () = jvm.invoke_virtual(&other, "setLenient", "(Z)V", (false,)).await?;
    assert!(
        !jvm.invoke_virtual::<_, bool>(&calendar, "equals", "(Ljava/lang/Object;)Z", (other,))
            .await?
    );

    Ok(())
}
