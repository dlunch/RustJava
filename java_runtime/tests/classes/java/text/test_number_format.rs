use java_runtime::classes::java::{
    lang::{Double, Long, Number, String, StringBuffer},
    text::{DecimalFormat, FieldPosition, NumberFormat, ParsePosition},
    util::Locale,
};
use jvm::{ClassInstanceRef, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_number_format_factories_format_values() -> Result<()> {
    let jvm = test_jvm().await?;

    let number: ClassInstanceRef<NumberFormat> = jvm
        .invoke_static("java/text/NumberFormat", "getInstance", "()Ljava/text/NumberFormat;", ())
        .await?;
    let integer: ClassInstanceRef<String> = jvm.invoke_virtual(&number, "format", "(J)Ljava/lang/String;", (1_234_567i64,)).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &integer).await?, "1,234,567");
    let decimal: ClassInstanceRef<String> = jvm.invoke_virtual(&number, "format", "(D)Ljava/lang/String;", (1234.5f64,)).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &decimal).await?, "1,234.5");

    let percent: ClassInstanceRef<NumberFormat> = jvm
        .invoke_static("java/text/NumberFormat", "getPercentInstance", "()Ljava/text/NumberFormat;", ())
        .await?;
    let text: ClassInstanceRef<String> = jvm.invoke_virtual(&percent, "format", "(D)Ljava/lang/String;", (0.12f64,)).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "12%");

    let currency: ClassInstanceRef<NumberFormat> = jvm
        .invoke_static("java/text/NumberFormat", "getCurrencyInstance", "()Ljava/text/NumberFormat;", ())
        .await?;
    let text: ClassInstanceRef<String> = jvm.invoke_virtual(&currency, "format", "(D)Ljava/lang/String;", (1234.5f64,)).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "$1,234.50");

    Ok(())
}

#[tokio::test]
async fn test_number_format_integer_factories() -> Result<()> {
    let jvm = test_jvm().await?;

    let default: ClassInstanceRef<NumberFormat> = jvm
        .invoke_static("java/text/NumberFormat", "getIntegerInstance", "()Ljava/text/NumberFormat;", ())
        .await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&default, "getMaximumFractionDigits", "()I", ()).await?, 0);
    assert!(jvm.invoke_virtual::<_, bool>(&default, "isParseIntegerOnly", "()Z", ()).await?);

    let locale: ClassInstanceRef<Locale> = jvm.invoke_static("java/util/Locale", "getDefault", "()Ljava/util/Locale;", ()).await?;
    let integer: ClassInstanceRef<NumberFormat> = jvm
        .invoke_static(
            "java/text/NumberFormat",
            "getIntegerInstance",
            "(Ljava/util/Locale;)Ljava/text/NumberFormat;",
            (locale,),
        )
        .await?;
    let text: ClassInstanceRef<String> = jvm.invoke_virtual(&integer, "format", "(D)Ljava/lang/String;", (1234.6f64,)).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "1,235");

    let source = JavaLangString::from_rust_string(&jvm, "1,234.5").await?;
    let parsed: ClassInstanceRef<Long> = jvm
        .invoke_virtual(&integer, "parse", "(Ljava/lang/String;)Ljava/lang/Number;", (source,))
        .await?;
    assert_eq!(jvm.invoke_virtual::<_, i64>(&parsed, "longValue", "()J", ()).await?, 1234);

    Ok(())
}

#[tokio::test]
async fn test_number_format_digit_and_grouping_settings() -> Result<()> {
    let jvm = test_jvm().await?;
    let number: ClassInstanceRef<NumberFormat> = jvm
        .invoke_static("java/text/NumberFormat", "getNumberInstance", "()Ljava/text/NumberFormat;", ())
        .await?;

    let _: () = jvm.invoke_virtual(&number, "setGroupingUsed", "(Z)V", (false,)).await?;
    let _: () = jvm.invoke_virtual(&number, "setMinimumFractionDigits", "(I)V", (2,)).await?;
    let text: ClassInstanceRef<String> = jvm.invoke_virtual(&number, "format", "(D)Ljava/lang/String;", (1234.5f64,)).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "1234.50");

    let _: () = jvm.invoke_virtual(&number, "setMaximumFractionDigits", "(I)V", (1,)).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&number, "getMinimumFractionDigits", "()I", ()).await?, 1);
    assert!(!jvm.invoke_virtual::<_, bool>(&number, "isGroupingUsed", "()Z", ()).await?);

    Ok(())
}

#[tokio::test]
async fn test_decimal_format_pattern_and_field_position() -> Result<()> {
    let jvm = test_jvm().await?;
    let pattern = JavaLangString::from_rust_string(&jvm, "0000.00").await?;
    let format: ClassInstanceRef<DecimalFormat> = jvm
        .new_class("java/text/DecimalFormat", "(Ljava/lang/String;)V", (pattern,))
        .await?
        .into();

    let text: ClassInstanceRef<String> = jvm.invoke_virtual(&format, "format", "(D)Ljava/lang/String;", (12.3f64,)).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "0012.30");

    let grouping = JavaLangString::from_rust_string(&jvm, "#,##0.###").await?;
    let _: () = jvm.invoke_virtual(&format, "applyPattern", "(Ljava/lang/String;)V", (grouping,)).await?;
    let prefix = JavaLangString::from_rust_string(&jvm, "pre ").await?;
    let buffer: ClassInstanceRef<StringBuffer> = jvm.new_class("java/lang/StringBuffer", "(Ljava/lang/String;)V", (prefix,)).await?.into();
    let position: ClassInstanceRef<FieldPosition> = jvm.new_class("java/text/FieldPosition", "(I)V", (0,)).await?.into();
    let _: ClassInstanceRef<StringBuffer> = jvm
        .invoke_virtual(
            &format,
            "format",
            "(DLjava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;",
            (1234.5f64, buffer.clone(), position.clone()),
        )
        .await?;
    let text: ClassInstanceRef<String> = jvm.invoke_virtual(&buffer, "toString", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "pre 1,234.5");
    assert_eq!(jvm.invoke_virtual::<_, i32>(&position, "getBeginIndex", "()I", ()).await?, 4);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&position, "getEndIndex", "()I", ()).await?, 9);

    Ok(())
}

#[tokio::test]
async fn test_number_format_parse_and_positions() -> Result<()> {
    let jvm = test_jvm().await?;
    let number: ClassInstanceRef<NumberFormat> = jvm
        .invoke_static("java/text/NumberFormat", "getInstance", "()Ljava/text/NumberFormat;", ())
        .await?;
    let source = JavaLangString::from_rust_string(&jvm, "1,234.5rest").await?;
    let position: ClassInstanceRef<ParsePosition> = jvm.new_class("java/text/ParsePosition", "(I)V", (0,)).await?.into();
    let parsed: ClassInstanceRef<Double> = jvm
        .invoke_virtual(
            &number,
            "parse",
            "(Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/lang/Number;",
            (source, position.clone()),
        )
        .await?;
    assert_eq!(jvm.invoke_virtual::<_, f64>(&parsed, "doubleValue", "()D", ()).await?, 1234.5);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&position, "getIndex", "()I", ()).await?, 7);

    let integer_source = JavaLangString::from_rust_string(&jvm, "42").await?;
    let parsed: ClassInstanceRef<Long> = jvm
        .invoke_virtual(&number, "parse", "(Ljava/lang/String;)Ljava/lang/Number;", (integer_source,))
        .await?;
    assert_eq!(jvm.invoke_virtual::<_, i64>(&parsed, "longValue", "()J", ()).await?, 42);

    let invalid = JavaLangString::from_rust_string(&jvm, "not-a-number").await?;
    let position: ClassInstanceRef<ParsePosition> = jvm.new_class("java/text/ParsePosition", "(I)V", (0,)).await?.into();
    let parsed: ClassInstanceRef<Long> = jvm
        .invoke_virtual(
            &number,
            "parse",
            "(Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/lang/Number;",
            (invalid, position.clone()),
        )
        .await?;
    assert!(parsed.is_null());
    assert_eq!(jvm.invoke_virtual::<_, i32>(&position, "getIndex", "()I", ()).await?, 0);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&position, "getErrorIndex", "()I", ()).await?, 0);

    Ok(())
}

#[tokio::test]
async fn test_decimal_format_quoted_affixes_and_integer_boundaries() -> Result<()> {
    let jvm = test_jvm().await?;
    let pattern = JavaLangString::from_rust_string(&jvm, "'%'0").await?;
    let format: ClassInstanceRef<DecimalFormat> = jvm
        .new_class("java/text/DecimalFormat", "(Ljava/lang/String;)V", (pattern,))
        .await?
        .into();
    let text: ClassInstanceRef<String> = jvm.invoke_virtual(&format, "format", "(J)Ljava/lang/String;", (12i64,)).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "%12");

    let source = JavaLangString::from_rust_string(&jvm, "%12").await?;
    let parsed: ClassInstanceRef<Long> = jvm
        .invoke_virtual(&format, "parse", "(Ljava/lang/String;)Ljava/lang/Number;", (source,))
        .await?;
    assert_eq!(jvm.invoke_virtual::<_, i64>(&parsed, "longValue", "()J", ()).await?, 12);

    let apostrophe_pattern = JavaLangString::from_rust_string(&jvm, "''0").await?;
    let _: () = jvm
        .invoke_virtual(&format, "applyPattern", "(Ljava/lang/String;)V", (apostrophe_pattern,))
        .await?;
    let text: ClassInstanceRef<String> = jvm.invoke_virtual(&format, "format", "(J)Ljava/lang/String;", (12i64,)).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "'12");

    let plain_pattern = JavaLangString::from_rust_string(&jvm, "0").await?;
    let _: () = jvm
        .invoke_virtual(&format, "applyPattern", "(Ljava/lang/String;)V", (plain_pattern,))
        .await?;
    let maximum = JavaLangString::from_rust_string(&jvm, "9223372036854775807").await?;
    let parsed: ClassInstanceRef<Long> = jvm
        .invoke_virtual(&format, "parse", "(Ljava/lang/String;)Ljava/lang/Number;", (maximum,))
        .await?;
    assert_eq!(jvm.invoke_virtual::<_, i64>(&parsed, "longValue", "()J", ()).await?, i64::MAX);

    let overflow = JavaLangString::from_rust_string(&jvm, "9223372036854775808").await?;
    let parsed: ClassInstanceRef<Number> = jvm
        .invoke_virtual(&format, "parse", "(Ljava/lang/String;)Ljava/lang/Number;", (overflow,))
        .await?;
    assert!(jvm.is_instance(&**parsed, "java/lang/Double"));

    Ok(())
}
