use java_runtime::classes::java::lang::{Long, Object, String};
use jvm::{ClassInstanceRef, JavaError, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_long_api() -> Result<()> {
    let jvm = test_jvm().await?;
    let value = jvm.new_class("java/lang/Long", "(J)V", (i64::MIN,)).await?;
    assert_eq!(jvm.invoke_virtual::<_, i8>(&value, "byteValue", "()B", ()).await?, 0);
    assert_eq!(jvm.invoke_virtual::<_, i16>(&value, "shortValue", "()S", ()).await?, 0);
    assert_eq!(jvm.invoke_virtual::<_, i64>(&value, "longValue", "()J", ()).await?, i64::MIN);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&value, "intValue", "()I", ()).await?, 0);
    assert_eq!(jvm.invoke_virtual::<_, f32>(&value, "floatValue", "()F", ()).await?, i64::MIN as f32);
    assert_eq!(jvm.invoke_virtual::<_, f64>(&value, "doubleValue", "()D", ()).await?, i64::MIN as f64);

    for (input, radix, expected) in [("7fffffffffffffff", 16, i64::MAX), ("-8000000000000000", 16, i64::MIN)] {
        let string = JavaLangString::from_rust_string(&jvm, input).await?;
        let parsed: i64 = jvm
            .invoke_static("java/lang/Long", "parseLong", "(Ljava/lang/String;I)J", (string, radix))
            .await?;
        assert_eq!(parsed, expected);
    }

    let value = jvm
        .invoke_static::<_, jvm::ClassInstanceRef<Long>>(
            "java/lang/Long",
            "valueOf",
            "(Ljava/lang/String;)Ljava/lang/Long;",
            (JavaLangString::from_rust_string(&jvm, "-1").await?,),
        )
        .await?;
    let hex: jvm::ClassInstanceRef<java_runtime::classes::java::lang::String> = jvm
        .invoke_static("java/lang/Long", "toHexString", "(J)Ljava/lang/String;", (-1i64,))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &hex).await?, "ffffffffffffffff");
    assert_eq!(jvm.invoke_virtual::<_, i64>(&value, "longValue", "()J", ()).await?, -1);

    let overflow = JavaLangString::from_rust_string(&jvm, "9223372036854775808").await?;
    let result: Result<i64> = jvm
        .invoke_static("java/lang/Long", "parseLong", "(Ljava/lang/String;)J", (overflow,))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Long.parseLong must reject overflow");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NumberFormatException"));

    let typed_null_result: Result<i32> = jvm.invoke_virtual(&value, "compareTo", "(Ljava/lang/Long;)I", (None,)).await;
    let Err(JavaError::JavaException(exception)) = typed_null_result else {
        panic!("Long typed compare null must throw NPE");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let misplaced_sign = JavaLangString::from_rust_string(&jvm, "#-1").await?;
    let result: Result<ClassInstanceRef<Long>> = jvm
        .invoke_static("java/lang/Long", "decode", "(Ljava/lang/String;)Ljava/lang/Long;", (misplaced_sign,))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Long.decode must reject a sign after the radix prefix");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NumberFormatException"));

    let key = JavaLangString::from_rust_string(&jvm, "rustjava.test.long").await?;
    let property_value = JavaLangString::from_rust_string(&jvm, "077").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/lang/System",
            "setProperty",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
            (key.clone(), property_value),
        )
        .await?;
    let property: ClassInstanceRef<Long> = jvm
        .invoke_static("java/lang/Long", "getLong", "(Ljava/lang/String;)Ljava/lang/Long;", (key.clone(),))
        .await?;
    assert_eq!(jvm.invoke_virtual::<_, i64>(&property, "longValue", "()J", ()).await?, 63);

    let invalid = JavaLangString::from_rust_string(&jvm, "invalid").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/lang/System",
            "setProperty",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
            (key.clone(), invalid),
        )
        .await?;
    let property: ClassInstanceRef<Long> = jvm
        .invoke_static("java/lang/Long", "getLong", "(Ljava/lang/String;J)Ljava/lang/Long;", (key, -9i64))
        .await?;
    assert_eq!(jvm.invoke_virtual::<_, i64>(&property, "longValue", "()J", ()).await?, -9);

    for (text, radix, expected) in [
        ("-1000000000000000000000000000000000000000000000000000000000000000", 2, i64::MIN),
        ("111111111111111111111111111111111111111111111111111111111111111", 2, i64::MAX),
        ("-9223372036854775808", 10, i64::MIN),
        ("9223372036854775807", 10, i64::MAX),
        ("-8000000000000000", 16, i64::MIN),
        ("7fffffffffffffff", 16, i64::MAX),
        ("-1y2p0ij32e8e8", 36, i64::MIN),
        ("1y2p0ij32e8e7", 36, i64::MAX),
    ] {
        let text = JavaLangString::from_rust_string(&jvm, text).await?;
        let parsed: i64 = jvm
            .invoke_static("java/lang/Long", "parseLong", "(Ljava/lang/String;I)J", (text, radix))
            .await?;
        assert_eq!(parsed, expected);
    }
    for text in ["-9223372036854775809", "9223372036854775808", "", "+", "-"] {
        let text = JavaLangString::from_rust_string(&jvm, text).await?;
        let result: Result<i64> = jvm.invoke_static("java/lang/Long", "parseLong", "(Ljava/lang/String;)J", (text,)).await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("Long.parseLong must reject invalid input");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/NumberFormatException"));
    }

    for (text, expected) in [("#7f", 127i64), ("077", 63i64), ("-0x8000000000000000", i64::MIN)] {
        let text = JavaLangString::from_rust_string(&jvm, text).await?;
        let decoded: ClassInstanceRef<Long> = jvm
            .invoke_static("java/lang/Long", "decode", "(Ljava/lang/String;)Ljava/lang/Long;", (text,))
            .await?;
        assert_eq!(jvm.invoke_virtual::<_, i64>(&decoded, "longValue", "()J", ()).await?, expected);
    }
    for (method, expected) in [
        ("toBinaryString", "1111111111111111111111111111111111111111111111111111111111111111"),
        ("toOctalString", "1777777777777777777777"),
        ("toHexString", "ffffffffffffffff"),
    ] {
        let text: ClassInstanceRef<String> = jvm.invoke_static("java/lang/Long", method, "(J)Ljava/lang/String;", (-1i64,)).await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, expected);
    }

    let equal = jvm.new_class("java/lang/Long", "(J)V", (-1i64,)).await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&value, "equals", "(Ljava/lang/Object;)Z", (equal.clone(),))
            .await?
    );
    assert_eq!(jvm.invoke_virtual::<_, i32>(&value, "hashCode", "()I", ()).await?, 0);
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&value, "compareTo", "(Ljava/lang/Object;)I", (equal,))
            .await?,
        0
    );
    let object = jvm.new_class("java/lang/Object", "()V", ()).await?;
    let result: Result<i32> = jvm.invoke_virtual(&value, "compareTo", "(Ljava/lang/Object;)I", (object,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Long raw compare must reject another type");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/ClassCastException"));
    let result: Result<i32> = jvm.invoke_virtual(&value, "compareTo", "(Ljava/lang/Object;)I", (None,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Long raw compare null must throw NPE");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    assert_eq!(jvm.get_static_field::<i64>("java/lang/Long", "MIN_VALUE", "J").await?, i64::MIN);
    assert_eq!(jvm.get_static_field::<i64>("java/lang/Long", "MAX_VALUE", "J").await?, i64::MAX);
    let typ = jvm.get_static_field("java/lang/Long", "TYPE", "Ljava/lang/Class;").await?;
    let type_name: ClassInstanceRef<String> = jvm.invoke_virtual(&typ, "getName", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &type_name).await?, "long");
    assert!(jvm.invoke_virtual::<_, bool>(&typ, "isPrimitive", "()Z", ()).await?);
    Ok(())
}
