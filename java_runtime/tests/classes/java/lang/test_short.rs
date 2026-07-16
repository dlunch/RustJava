use java_runtime::classes::java::lang::String;
use jvm::{ClassInstanceRef, JavaError, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_short_api() -> Result<()> {
    let jvm = test_jvm().await?;
    let value = jvm.new_class("java/lang/Short", "(S)V", (-129i16,)).await?;
    assert_eq!(jvm.invoke_virtual::<_, i8>(&value, "byteValue", "()B", ()).await?, 127);
    assert_eq!(jvm.invoke_virtual::<_, i16>(&value, "shortValue", "()S", ()).await?, -129);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&value, "intValue", "()I", ()).await?, -129);
    assert_eq!(jvm.invoke_virtual::<_, i64>(&value, "longValue", "()J", ()).await?, -129);
    assert_eq!(jvm.invoke_virtual::<_, f32>(&value, "floatValue", "()F", ()).await?, -129.0);
    assert_eq!(jvm.invoke_virtual::<_, f64>(&value, "doubleValue", "()D", ()).await?, -129.0);

    for (input, radix, expected) in [("7f", 16, 127i16), ("-100000", 2, -32i16)] {
        let string = JavaLangString::from_rust_string(&jvm, input).await?;
        let parsed: i16 = jvm
            .invoke_static("java/lang/Short", "parseShort", "(Ljava/lang/String;I)S", (string, radix))
            .await?;
        assert_eq!(parsed, expected);
    }

    let min = jvm.get_static_field::<i16>("java/lang/Short", "MIN_VALUE", "S").await?;
    let max = jvm.get_static_field::<i16>("java/lang/Short", "MAX_VALUE", "S").await?;
    assert_eq!((min, max), (i16::MIN, i16::MAX));
    let name = JavaLangString::from_rust_string(&jvm, "077").await?;
    let decoded = jvm
        .invoke_static("java/lang/Short", "decode", "(Ljava/lang/String;)Ljava/lang/Short;", (name,))
        .await?;
    assert_eq!(jvm.invoke_virtual::<_, i16>(&decoded, "shortValue", "()S", ()).await?, 63);

    let null_result: Result<i16> = jvm.invoke_static("java/lang/Short", "parseShort", "(Ljava/lang/String;)S", (None,)).await;
    let Err(JavaError::JavaException(exception)) = null_result else {
        panic!("Short.parseShort(null) must reject null");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NumberFormatException"));

    let result: Result<i32> = jvm.invoke_virtual(&value, "compareTo", "(Ljava/lang/Short;)I", (None,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Short typed compare null must throw NPE");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    for (text, radix, expected) in [
        ("-1000000000000000", 2, i16::MIN),
        ("111111111111111", 2, i16::MAX),
        ("-32768", 10, i16::MIN),
        ("32767", 10, i16::MAX),
        ("-8000", 16, i16::MIN),
        ("7fff", 16, i16::MAX),
        ("-pa8", 36, i16::MIN),
        ("pa7", 36, i16::MAX),
    ] {
        let text = JavaLangString::from_rust_string(&jvm, text).await?;
        let parsed: i16 = jvm
            .invoke_static("java/lang/Short", "parseShort", "(Ljava/lang/String;I)S", (text, radix))
            .await?;
        assert_eq!(parsed, expected);
    }
    for text in ["-32769", "32768"] {
        let text = JavaLangString::from_rust_string(&jvm, text).await?;
        let result: Result<i16> = jvm.invoke_static("java/lang/Short", "parseShort", "(Ljava/lang/String;)S", (text,)).await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("Short.parseShort must reject overflow");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/NumberFormatException"));
    }

    let equal = jvm.new_class("java/lang/Short", "(S)V", (-129i16,)).await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&value, "equals", "(Ljava/lang/Object;)Z", (equal.clone(),))
            .await?
    );
    assert_eq!(jvm.invoke_virtual::<_, i32>(&value, "hashCode", "()I", ()).await?, -129);
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&value, "compareTo", "(Ljava/lang/Object;)I", (equal,))
            .await?,
        0
    );
    let object = jvm.new_class("java/lang/Object", "()V", ()).await?;
    let result: Result<i32> = jvm.invoke_virtual(&value, "compareTo", "(Ljava/lang/Object;)I", (object,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Short raw compare must reject another type");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/ClassCastException"));
    let result: Result<i32> = jvm.invoke_virtual(&value, "compareTo", "(Ljava/lang/Object;)I", (None,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Short raw compare null must throw NPE");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    for (text, expected) in [("#7fff", i16::MAX), ("077777", i16::MAX)] {
        let text = JavaLangString::from_rust_string(&jvm, text).await?;
        let decoded = jvm
            .invoke_static("java/lang/Short", "decode", "(Ljava/lang/String;)Ljava/lang/Short;", (text,))
            .await?;
        assert_eq!(jvm.invoke_virtual::<_, i16>(&decoded, "shortValue", "()S", ()).await?, expected);
    }

    let typ = jvm.get_static_field("java/lang/Short", "TYPE", "Ljava/lang/Class;").await?;
    let type_name: ClassInstanceRef<String> = jvm.invoke_virtual(&typ, "getName", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &type_name).await?, "short");
    assert!(jvm.invoke_virtual::<_, bool>(&typ, "isPrimitive", "()Z", ()).await?);
    Ok(())
}
