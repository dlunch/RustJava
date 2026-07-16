use java_runtime::classes::java::lang::{Byte, String};
use jvm::{ClassInstanceRef, JavaError, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_byte_api() -> Result<()> {
    let jvm = test_jvm().await?;
    let value = jvm.new_class("java/lang/Byte", "(B)V", (-2i8,)).await?;
    assert_eq!(jvm.invoke_virtual::<_, i8>(&value, "byteValue", "()B", ()).await?, -2);
    assert_eq!(jvm.invoke_virtual::<_, i16>(&value, "shortValue", "()S", ()).await?, -2);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&value, "intValue", "()I", ()).await?, -2);
    assert_eq!(jvm.invoke_virtual::<_, i64>(&value, "longValue", "()J", ()).await?, -2);
    assert_eq!(jvm.invoke_virtual::<_, f32>(&value, "floatValue", "()F", ()).await?, -2.0);
    assert_eq!(jvm.invoke_virtual::<_, f64>(&value, "doubleValue", "()D", ()).await?, -2.0);
    let text: jvm::ClassInstanceRef<java_runtime::classes::java::lang::String> =
        jvm.invoke_virtual(&value, "toString", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "-2");

    let min = jvm.get_static_field::<i8>("java/lang/Byte", "MIN_VALUE", "B").await?;
    let max = jvm.get_static_field::<i8>("java/lang/Byte", "MAX_VALUE", "B").await?;
    assert_eq!((min, max), (i8::MIN, i8::MAX));
    let typ = jvm.get_static_field("java/lang/Byte", "TYPE", "Ljava/lang/Class;").await?;
    let name: jvm::ClassInstanceRef<java_runtime::classes::java::lang::String> =
        jvm.invoke_virtual(&typ, "getName", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &name).await?, "byte");
    assert!(jvm.invoke_virtual::<_, bool>(&typ, "isPrimitive", "()Z", ()).await?);

    let string = JavaLangString::from_rust_string(&jvm, "0x7f").await?;
    let decoded = jvm
        .invoke_static("java/lang/Byte", "decode", "(Ljava/lang/String;)Ljava/lang/Byte;", (string,))
        .await?;
    assert_eq!(jvm.invoke_virtual::<_, i8>(&decoded, "byteValue", "()B", ()).await?, 127);

    let invalid = JavaLangString::from_rust_string(&jvm, "128").await?;
    let result: Result<jvm::ClassInstanceRef<Byte>> = jvm
        .invoke_static("java/lang/Byte", "valueOf", "(Ljava/lang/String;)Ljava/lang/Byte;", (invalid,))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Byte.valueOf must reject overflow");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NumberFormatException"));

    let result: Result<i32> = jvm.invoke_virtual(&value, "compareTo", "(Ljava/lang/Byte;)I", (None,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Byte typed compare null must throw NPE");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    for (text, radix, expected) in [
        ("-10000000", 2, i8::MIN),
        ("1111111", 2, i8::MAX),
        ("-128", 10, i8::MIN),
        ("127", 10, i8::MAX),
        ("-80", 16, i8::MIN),
        ("7f", 16, i8::MAX),
        ("-3k", 36, i8::MIN),
        ("3j", 36, i8::MAX),
    ] {
        let text = JavaLangString::from_rust_string(&jvm, text).await?;
        let parsed: i8 = jvm
            .invoke_static("java/lang/Byte", "parseByte", "(Ljava/lang/String;I)B", (text, radix))
            .await?;
        assert_eq!(parsed, expected);
    }
    for text in ["-129", "128"] {
        let text = JavaLangString::from_rust_string(&jvm, text).await?;
        let result: Result<i8> = jvm.invoke_static("java/lang/Byte", "parseByte", "(Ljava/lang/String;)B", (text,)).await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("Byte.parseByte must reject overflow");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/NumberFormatException"));
    }

    let equal = jvm.new_class("java/lang/Byte", "(B)V", (-2i8,)).await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&value, "equals", "(Ljava/lang/Object;)Z", (equal.clone(),))
            .await?
    );
    assert_eq!(jvm.invoke_virtual::<_, i32>(&value, "hashCode", "()I", ()).await?, -2);
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&value, "compareTo", "(Ljava/lang/Object;)I", (equal,))
            .await?,
        0
    );
    let object = jvm.new_class("java/lang/Object", "()V", ()).await?;
    let result: Result<i32> = jvm.invoke_virtual(&value, "compareTo", "(Ljava/lang/Object;)I", (object,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Byte raw compare must reject another type");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/ClassCastException"));
    let result: Result<i32> = jvm.invoke_virtual(&value, "compareTo", "(Ljava/lang/Object;)I", (None,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Byte raw compare null must throw NPE");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    for (text, expected) in [("#7f", 127i8), ("0177", 127i8)] {
        let text = JavaLangString::from_rust_string(&jvm, text).await?;
        let decoded: ClassInstanceRef<Byte> = jvm
            .invoke_static("java/lang/Byte", "decode", "(Ljava/lang/String;)Ljava/lang/Byte;", (text,))
            .await?;
        assert_eq!(jvm.invoke_virtual::<_, i8>(&decoded, "byteValue", "()B", ()).await?, expected);
    }

    let typ = jvm.get_static_field("java/lang/Byte", "TYPE", "Ljava/lang/Class;").await?;
    let name: ClassInstanceRef<String> = jvm.invoke_virtual(&typ, "getName", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &name).await?, "byte");
    assert!(jvm.invoke_virtual::<_, bool>(&typ, "isPrimitive", "()Z", ()).await?);
    Ok(())
}
