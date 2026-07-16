use java_runtime::classes::java::lang::{Integer, Object, String};
use jvm::{ClassInstanceRef, JavaError, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_parse_int() -> Result<()> {
    let jvm = test_jvm().await?;

    let string = JavaLangString::from_rust_string(&jvm, "42").await?;
    assert_eq!(
        42i32,
        jvm.invoke_static("java/lang/Integer", "parseInt", "(Ljava/lang/String;)I", (string,))
            .await?
    );

    let boxed: ClassInstanceRef<Integer> = jvm.invoke_static("java/lang/Integer", "valueOf", "(I)Ljava/lang/Integer;", (42,)).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&boxed, "intValue", "()I", ()).await?, 42);

    Ok(())
}

#[tokio::test]
async fn test_parse_int_invalid() -> Result<()> {
    let jvm = test_jvm().await?;

    let string = JavaLangString::from_rust_string(&jvm, "abc").await?;
    let result: Result<i32> = jvm
        .invoke_static("java/lang/Integer", "parseInt", "(Ljava/lang/String;)I", (string,))
        .await;

    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Expected JavaException, got {:?}", result);
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NumberFormatException"));

    Ok(())
}

#[tokio::test]
async fn test_integer_strict_api() -> Result<()> {
    let jvm = test_jvm().await?;

    for (text, radix, expected) in [("7fffffff", 16, i32::MAX), ("-80000000", 16, i32::MIN), ("z", 36, 35)] {
        let string = JavaLangString::from_rust_string(&jvm, text).await?;
        let value: i32 = jvm
            .invoke_static("java/lang/Integer", "parseInt", "(Ljava/lang/String;I)I", (string, radix))
            .await?;
        assert_eq!(value, expected);
    }
    for (text, radix, expected) in [
        ("-10000000000000000000000000000000", 2, i32::MIN),
        ("1111111111111111111111111111111", 2, i32::MAX),
        ("-2147483648", 10, i32::MIN),
        ("2147483647", 10, i32::MAX),
        ("-zik0zk", 36, i32::MIN),
        ("zik0zj", 36, i32::MAX),
    ] {
        let text = JavaLangString::from_rust_string(&jvm, text).await?;
        let parsed: i32 = jvm
            .invoke_static("java/lang/Integer", "parseInt", "(Ljava/lang/String;I)I", (text, radix))
            .await?;
        assert_eq!(parsed, expected);
    }

    let decoded_name = JavaLangString::from_rust_string(&jvm, "-0x80000000").await?;
    let decoded: ClassInstanceRef<Integer> = jvm
        .invoke_static("java/lang/Integer", "decode", "(Ljava/lang/String;)Ljava/lang/Integer;", (decoded_name,))
        .await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&decoded, "intValue", "()I", ()).await?, i32::MIN);

    for (method, expected) in [
        ("toBinaryString", "11111111111111111111111111111111"),
        ("toOctalString", "37777777777"),
        ("toHexString", "ffffffff"),
    ] {
        let formatted: ClassInstanceRef<String> = jvm.invoke_static("java/lang/Integer", method, "(I)Ljava/lang/String;", (-1,)).await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &formatted).await?, expected);
    }

    let left = jvm.new_class("java/lang/Integer", "(I)V", (1,)).await?;
    let right = jvm.new_class("java/lang/Integer", "(I)V", (2,)).await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&left, "compareTo", "(Ljava/lang/Integer;)I", (right.clone(),))
            .await?,
        -1
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&left, "compareTo", "(Ljava/lang/Object;)I", (right.clone(),))
            .await?,
        -1
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(&left, "equals", "(Ljava/lang/Object;)Z", (right.clone(),))
            .await?
    );
    assert_eq!(jvm.invoke_virtual::<_, i32>(&left, "hashCode", "()I", ()).await?, 1);

    let typed_null_result: Result<i32> = jvm.invoke_virtual(&left, "compareTo", "(Ljava/lang/Integer;)I", (None,)).await;
    let Err(JavaError::JavaException(exception)) = typed_null_result else {
        panic!("Integer typed compare null must throw NPE");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let null_result: Result<i32> = jvm.invoke_virtual(&left, "compareTo", "(Ljava/lang/Object;)I", (None,)).await;
    let Err(JavaError::JavaException(exception)) = null_result else {
        panic!("Integer raw compare null must throw NPE");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let object = jvm.new_class("java/lang/Object", "()V", ()).await?;
    let wrong_result: Result<i32> = jvm.invoke_virtual(&left, "compareTo", "(Ljava/lang/Object;)I", (object,)).await;
    let Err(JavaError::JavaException(exception)) = wrong_result else {
        panic!("Integer raw compare wrong type must throw CCE");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/ClassCastException"));

    let min = jvm.get_static_field::<i32>("java/lang/Integer", "MIN_VALUE", "I").await?;
    let max = jvm.get_static_field::<i32>("java/lang/Integer", "MAX_VALUE", "I").await?;
    assert_eq!((min, max), (i32::MIN, i32::MAX));
    let typ = jvm.get_static_field("java/lang/Integer", "TYPE", "Ljava/lang/Class;").await?;
    let name: ClassInstanceRef<String> = jvm.invoke_virtual(&typ, "getName", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &name).await?, "int");
    assert!(jvm.invoke_virtual::<_, bool>(&typ, "isPrimitive", "()Z", ()).await?);
    let _: () = jvm.invoke_static("java/lang/System", "gc", "()V", ()).await?;
    let typ = jvm.get_static_field("java/lang/Integer", "TYPE", "Ljava/lang/Class;").await?;
    assert!(jvm.invoke_virtual::<_, bool>(&typ, "isPrimitive", "()Z", ()).await?);
    Ok(())
}

#[tokio::test]
async fn test_integer_rejects_invalid_forms_and_uses_property_defaults() -> Result<()> {
    let jvm = test_jvm().await?;

    for (text, radix) in [
        ("", 10),
        ("+", 10),
        ("-", 10),
        (" 1", 10),
        ("2147483648", 10),
        ("-2147483649", 10),
        ("1", 1),
        ("1", 37),
    ] {
        let text = JavaLangString::from_rust_string(&jvm, text).await?;
        let result: Result<i32> = jvm
            .invoke_static("java/lang/Integer", "parseInt", "(Ljava/lang/String;I)I", (text, radix))
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("Integer.parseInt must reject invalid input");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/NumberFormatException"));
    }

    let misplaced_sign = JavaLangString::from_rust_string(&jvm, "0x-1").await?;
    let result: Result<ClassInstanceRef<Integer>> = jvm
        .invoke_static(
            "java/lang/Integer",
            "decode",
            "(Ljava/lang/String;)Ljava/lang/Integer;",
            (misplaced_sign,),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Integer.decode must reject a sign after the radix prefix");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NumberFormatException"));

    for (text, expected) in [("#7f", 127i32), ("0177", 127i32)] {
        let text = JavaLangString::from_rust_string(&jvm, text).await?;
        let decoded: ClassInstanceRef<Integer> = jvm
            .invoke_static("java/lang/Integer", "decode", "(Ljava/lang/String;)Ljava/lang/Integer;", (text,))
            .await?;
        assert_eq!(jvm.invoke_virtual::<_, i32>(&decoded, "intValue", "()I", ()).await?, expected);
    }

    let key = JavaLangString::from_rust_string(&jvm, "rustjava.test.integer").await?;
    let value = JavaLangString::from_rust_string(&jvm, "0x2a").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/lang/System",
            "setProperty",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
            (key.clone(), value),
        )
        .await?;
    let property: ClassInstanceRef<Integer> = jvm
        .invoke_static(
            "java/lang/Integer",
            "getInteger",
            "(Ljava/lang/String;)Ljava/lang/Integer;",
            (key.clone(),),
        )
        .await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&property, "intValue", "()I", ()).await?, 42);

    let invalid = JavaLangString::from_rust_string(&jvm, "not-an-integer").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/lang/System",
            "setProperty",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
            (key.clone(), invalid),
        )
        .await?;
    let property: ClassInstanceRef<Integer> = jvm
        .invoke_static(
            "java/lang/Integer",
            "getInteger",
            "(Ljava/lang/String;I)Ljava/lang/Integer;",
            (key.clone(), 17),
        )
        .await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&property, "intValue", "()I", ()).await?, 17);

    let default = jvm.new_class("java/lang/Integer", "(I)V", (23,)).await?;
    let property: ClassInstanceRef<Integer> = jvm
        .invoke_static(
            "java/lang/Integer",
            "getInteger",
            "(Ljava/lang/String;Ljava/lang/Integer;)Ljava/lang/Integer;",
            (key, default),
        )
        .await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&property, "intValue", "()I", ()).await?, 23);

    Ok(())
}
