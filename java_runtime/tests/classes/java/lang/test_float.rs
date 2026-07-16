use java_runtime::classes::java::lang::{Float, String};
use jvm::{ClassInstanceRef, JavaError, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_float_constructors_and_number_conversions() -> Result<()> {
    let jvm = test_jvm().await?;

    let value = jvm.new_class("java/lang/Float", "(D)V", (130.75f64,)).await?;
    assert_eq!(jvm.invoke_virtual::<_, f32>(&value, "floatValue", "()F", ()).await?, 130.75);
    assert_eq!(jvm.invoke_virtual::<_, f64>(&value, "doubleValue", "()D", ()).await?, 130.75);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&value, "intValue", "()I", ()).await?, 130);
    assert_eq!(jvm.invoke_virtual::<_, i64>(&value, "longValue", "()J", ()).await?, 130);
    assert_eq!(jvm.invoke_virtual::<_, i8>(&value, "byteValue", "()B", ()).await?, -126);
    assert_eq!(jvm.invoke_virtual::<_, i16>(&value, "shortValue", "()S", ()).await?, 130);

    let text = JavaLangString::from_rust_string(&jvm, "-3.5").await?;
    let from_string = jvm.new_class("java/lang/Float", "(Ljava/lang/String;)V", (text,)).await?;
    assert_eq!(jvm.invoke_virtual::<_, f32>(&from_string, "floatValue", "()F", ()).await?, -3.5);

    Ok(())
}

#[tokio::test]
async fn test_float_parse_value_of_and_format() -> Result<()> {
    let jvm = test_jvm().await?;

    for (text, expected) in [(" \t-1.25e2F\n", -125.0), (".5d", 0.5), ("+42.", 42.0)] {
        let text = JavaLangString::from_rust_string(&jvm, text).await?;
        let parsed: f32 = jvm
            .invoke_static("java/lang/Float", "parseFloat", "(Ljava/lang/String;)F", (text,))
            .await?;
        assert_eq!(parsed, expected);
    }
    for suffix in ['f', 'F', 'd', 'D'] {
        let text = JavaLangString::from_rust_string(&jvm, &format!("1.5{suffix}")).await?;
        let parsed: f32 = jvm
            .invoke_static("java/lang/Float", "parseFloat", "(Ljava/lang/String;)F", (text,))
            .await?;
        assert_eq!(parsed, 1.5);
    }

    let infinity = JavaLangString::from_rust_string(&jvm, "+Infinity").await?;
    let infinity: f32 = jvm
        .invoke_static("java/lang/Float", "parseFloat", "(Ljava/lang/String;)F", (infinity,))
        .await?;
    assert_eq!(infinity, f32::INFINITY);

    let nan = JavaLangString::from_rust_string(&jvm, "-NaN").await?;
    let nan: ClassInstanceRef<Float> = jvm
        .invoke_static("java/lang/Float", "valueOf", "(Ljava/lang/String;)Ljava/lang/Float;", (nan,))
        .await?;
    assert!(jvm.invoke_virtual::<_, bool>(&nan, "isNaN", "()Z", ()).await?);

    for (value, expected) in [(12.0f32, "12.0"), (-0.0, "-0.0"), (10_000_000.0, "1.0E7"), (0.000_125, "1.25E-4")] {
        let text: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/Float", "toString", "(F)Ljava/lang/String;", (value,))
            .await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, expected);
    }

    for malformed in ["", "nan", "Infinityf", "1e", ".", "1_0", "0x1.0p0", "1.0 ff"] {
        let malformed = JavaLangString::from_rust_string(&jvm, malformed).await?;
        let result: Result<f32> = jvm
            .invoke_static("java/lang/Float", "parseFloat", "(Ljava/lang/String;)F", (malformed,))
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("Float.parseFloat must reject malformed input");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/NumberFormatException"));
    }

    let null_result: Result<f32> = jvm.invoke_static("java/lang/Float", "parseFloat", "(Ljava/lang/String;)F", (None,)).await;
    let Err(JavaError::JavaException(exception)) = null_result else {
        panic!("Float.parseFloat(null) must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    for (value, expected) in [(f32::from_bits(1), "1.4E-45"), (f32::from_bits(0x8000_0001), "-1.4E-45")] {
        let text: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/Float", "toString", "(F)Ljava/lang/String;", (value,))
            .await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, expected);
    }

    let min = JavaLangString::from_rust_string(&jvm, "1.4e-45").await?;
    assert_eq!(
        jvm.invoke_static::<_, f32>("java/lang/Float", "parseFloat", "(Ljava/lang/String;)F", (min,))
            .await?
            .to_bits(),
        1
    );
    let overflow = JavaLangString::from_rust_string(&jvm, "1e1000").await?;
    assert_eq!(
        jvm.invoke_static::<_, f32>("java/lang/Float", "parseFloat", "(Ljava/lang/String;)F", (overflow,))
            .await?,
        f32::INFINITY
    );
    let underflow = JavaLangString::from_rust_string(&jvm, "-1e-1000").await?;
    assert_eq!(
        jvm.invoke_static::<_, f32>("java/lang/Float", "parseFloat", "(Ljava/lang/String;)F", (underflow,))
            .await?
            .to_bits(),
        (-0.0f32).to_bits()
    );

    Ok(())
}

#[tokio::test]
async fn test_float_bits_equality_hash_and_comparison() -> Result<()> {
    let jvm = test_jvm().await?;

    let payload_nan_a: f32 = jvm.invoke_static("java/lang/Float", "intBitsToFloat", "(I)F", (0x7fc0_0001i32,)).await?;
    let payload_nan_b: f32 = jvm.invoke_static("java/lang/Float", "intBitsToFloat", "(I)F", (0x7fff_ffffi32,)).await?;
    assert_eq!(
        jvm.invoke_static::<_, i32>("java/lang/Float", "floatToIntBits", "(F)I", (payload_nan_a,))
            .await?,
        0x7fc0_0000
    );

    let raw = (-12.5f32).to_bits() as i32;
    let round_trip: f32 = jvm.invoke_static("java/lang/Float", "intBitsToFloat", "(I)F", (raw,)).await?;
    assert_eq!(
        jvm.invoke_static::<_, i32>("java/lang/Float", "floatToIntBits", "(F)I", (round_trip,))
            .await?,
        raw
    );

    let nan_a = jvm.new_class("java/lang/Float", "(F)V", (payload_nan_a,)).await?;
    let nan_b = jvm.new_class("java/lang/Float", "(F)V", (payload_nan_b,)).await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&nan_a, "equals", "(Ljava/lang/Object;)Z", (nan_b.clone(),))
            .await?
    );
    assert_eq!(jvm.invoke_virtual::<_, i32>(&nan_a, "hashCode", "()I", ()).await?, 0x7fc0_0000);
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&nan_a, "compareTo", "(Ljava/lang/Float;)I", (nan_b,))
            .await?,
        0
    );

    let negative_zero = jvm.new_class("java/lang/Float", "(F)V", (-0.0f32,)).await?;
    let positive_zero = jvm.new_class("java/lang/Float", "(F)V", (0.0f32,)).await?;
    assert!(
        !jvm.invoke_virtual::<_, bool>(&negative_zero, "equals", "(Ljava/lang/Object;)Z", (positive_zero.clone(),))
            .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&negative_zero, "compareTo", "(Ljava/lang/Float;)I", (positive_zero.clone(),),)
            .await?,
        -1
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&negative_zero, "compareTo", "(Ljava/lang/Object;)I", (positive_zero,))
            .await?,
        -1
    );
    assert_eq!(jvm.invoke_virtual::<_, i32>(&negative_zero, "hashCode", "()I", ()).await?, i32::MIN);
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&jvm.new_class("java/lang/Float", "(F)V", (0.0f32,)).await?, "hashCode", "()I", (),)
            .await?,
        0
    );

    let infinity = jvm.new_class("java/lang/Float", "(F)V", (f32::INFINITY,)).await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&nan_a, "compareTo", "(Ljava/lang/Float;)I", (infinity,))
            .await?,
        1
    );

    let typed_null: Result<i32> = jvm.invoke_virtual(&negative_zero, "compareTo", "(Ljava/lang/Float;)I", (None,)).await;
    let Err(JavaError::JavaException(exception)) = typed_null else {
        panic!("Float typed compare null must throw NPE");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let raw_null: Result<i32> = jvm.invoke_virtual(&negative_zero, "compareTo", "(Ljava/lang/Object;)I", (None,)).await;
    let Err(JavaError::JavaException(exception)) = raw_null else {
        panic!("Float raw compare null must throw NPE");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let object = jvm.new_class("java/lang/Object", "()V", ()).await?;
    let wrong_type: Result<i32> = jvm.invoke_virtual(&negative_zero, "compareTo", "(Ljava/lang/Object;)I", (object,)).await;
    let Err(JavaError::JavaException(exception)) = wrong_type else {
        panic!("Float raw compare wrong type must throw CCE");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/ClassCastException"));

    for (value, int_value, long_value) in [
        (f32::NAN, 0, 0i64),
        (f32::INFINITY, i32::MAX, i64::MAX),
        (f32::NEG_INFINITY, i32::MIN, i64::MIN),
    ] {
        let wrapper = jvm.new_class("java/lang/Float", "(F)V", (value,)).await?;
        assert_eq!(jvm.invoke_virtual::<_, i32>(&wrapper, "intValue", "()I", ()).await?, int_value);
        assert_eq!(jvm.invoke_virtual::<_, i64>(&wrapper, "longValue", "()J", ()).await?, long_value);
    }

    Ok(())
}

#[tokio::test]
async fn test_float_constants_type_and_predicates() -> Result<()> {
    let jvm = test_jvm().await?;

    assert_eq!(
        jvm.get_static_field::<f32>("java/lang/Float", "POSITIVE_INFINITY", "F").await?,
        f32::INFINITY
    );
    assert_eq!(
        jvm.get_static_field::<f32>("java/lang/Float", "NEGATIVE_INFINITY", "F").await?,
        f32::NEG_INFINITY
    );
    assert!(jvm.get_static_field::<f32>("java/lang/Float", "NaN", "F").await?.is_nan());
    assert_eq!(jvm.get_static_field::<f32>("java/lang/Float", "MAX_VALUE", "F").await?, f32::MAX);
    assert_eq!(jvm.get_static_field::<f32>("java/lang/Float", "MIN_VALUE", "F").await?.to_bits(), 1);
    assert!(jvm.invoke_static::<_, bool>("java/lang/Float", "isNaN", "(F)Z", (f32::NAN,)).await?);
    assert!(
        jvm.invoke_static::<_, bool>("java/lang/Float", "isInfinite", "(F)Z", (f32::NEG_INFINITY,))
            .await?
    );

    let typ = jvm.get_static_field("java/lang/Float", "TYPE", "Ljava/lang/Class;").await?;
    let name: ClassInstanceRef<String> = jvm.invoke_virtual(&typ, "getName", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &name).await?, "float");
    assert!(jvm.invoke_virtual::<_, bool>(&typ, "isPrimitive", "()Z", ()).await?);

    Ok(())
}
