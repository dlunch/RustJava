use java_runtime::classes::java::{
    lang::{Long, Object, String},
    text::{FieldPosition, Format, NumberFormat, ParseException, ParsePosition},
};
use jvm::{ClassInstanceRef, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_text_common_class_hierarchy() -> Result<()> {
    let jvm = test_jvm().await?;

    let format = jvm.resolve_class("java/text/Format").await?;
    assert!(jvm.is_inherited_from(&*format.definition, "java/lang/Object"));

    let parse_exception = jvm.resolve_class("java/text/ParseException").await?;
    assert!(jvm.is_inherited_from(&*parse_exception.definition, "java/lang/Exception"));

    Ok(())
}

#[tokio::test]
async fn test_field_position_accessors_and_equality() -> Result<()> {
    let jvm = test_jvm().await?;
    let position: ClassInstanceRef<FieldPosition> = jvm.new_class("java/text/FieldPosition", "(I)V", (1,)).await?.into();

    assert_eq!(jvm.invoke_virtual::<_, i32>(&position, "getField", "()I", ()).await?, 1);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&position, "getBeginIndex", "()I", ()).await?, 0);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&position, "getEndIndex", "()I", ()).await?, 0);

    let _: () = jvm.invoke_virtual(&position, "setBeginIndex", "(I)V", (3,)).await?;
    let _: () = jvm.invoke_virtual(&position, "setEndIndex", "(I)V", (7,)).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&position, "getBeginIndex", "()I", ()).await?, 3);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&position, "getEndIndex", "()I", ()).await?, 7);

    let same: ClassInstanceRef<FieldPosition> = jvm.new_class("java/text/FieldPosition", "(I)V", (1,)).await?.into();
    let _: () = jvm.invoke_virtual(&same, "setBeginIndex", "(I)V", (3,)).await?;
    let _: () = jvm.invoke_virtual(&same, "setEndIndex", "(I)V", (7,)).await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&position, "equals", "(Ljava/lang/Object;)Z", (same.clone(),))
            .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&position, "hashCode", "()I", ()).await?,
        jvm.invoke_virtual::<_, i32>(&same, "hashCode", "()I", ()).await?
    );

    Ok(())
}

#[tokio::test]
async fn test_parse_position_accessors_and_equality() -> Result<()> {
    let jvm = test_jvm().await?;
    let position: ClassInstanceRef<ParsePosition> = jvm.new_class("java/text/ParsePosition", "(I)V", (2,)).await?.into();

    assert_eq!(jvm.invoke_virtual::<_, i32>(&position, "getIndex", "()I", ()).await?, 2);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&position, "getErrorIndex", "()I", ()).await?, -1);

    let _: () = jvm.invoke_virtual(&position, "setIndex", "(I)V", (5,)).await?;
    let _: () = jvm.invoke_virtual(&position, "setErrorIndex", "(I)V", (4,)).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&position, "getIndex", "()I", ()).await?, 5);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&position, "getErrorIndex", "()I", ()).await?, 4);

    let same: ClassInstanceRef<ParsePosition> = jvm.new_class("java/text/ParsePosition", "(I)V", (5,)).await?.into();
    let _: () = jvm.invoke_virtual(&same, "setErrorIndex", "(I)V", (4,)).await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&position, "equals", "(Ljava/lang/Object;)Z", (same,))
            .await?
    );

    Ok(())
}

#[tokio::test]
async fn test_parse_exception_error_offset_and_message() -> Result<()> {
    let jvm = test_jvm().await?;
    let message = JavaLangString::from_rust_string(&jvm, "bad date").await?;
    let exception: ClassInstanceRef<ParseException> = jvm
        .new_class("java/text/ParseException", "(Ljava/lang/String;I)V", (message, 6))
        .await?
        .into();

    assert_eq!(jvm.invoke_virtual::<_, i32>(&exception, "getErrorOffset", "()I", ()).await?, 6);
    let actual: ClassInstanceRef<String> = jvm.invoke_virtual(&exception, "getMessage", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &actual).await?, "bad date");

    Ok(())
}

#[tokio::test]
async fn test_format_polymorphic_format_and_parse_object() -> Result<()> {
    let jvm = test_jvm().await?;
    let number: ClassInstanceRef<NumberFormat> = jvm
        .invoke_static("java/text/NumberFormat", "getInstance", "()Ljava/text/NumberFormat;", ())
        .await?;
    let format: ClassInstanceRef<Format> = ClassInstanceRef::new(number.instance);
    let value: ClassInstanceRef<Long> = jvm.new_class("java/lang/Long", "(J)V", (1234i64,)).await?.into();
    let text: ClassInstanceRef<String> = jvm
        .invoke_virtual(&format, "format", "(Ljava/lang/Object;)Ljava/lang/String;", (value,))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "1,234");

    let source = JavaLangString::from_rust_string(&jvm, "1,234").await?;
    let parsed: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&format, "parseObject", "(Ljava/lang/String;)Ljava/lang/Object;", (source,))
        .await?;
    assert!(jvm.is_instance(&**parsed, "java/lang/Long"));

    Ok(())
}
