use java_runtime::classes::java::lang::{Double, String};
use jvm::{ClassInstanceRef, JavaError, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_double_constructors_and_number_conversions() -> Result<()> {
    let jvm = test_jvm().await?;

    let value = jvm.new_class("java/lang/Double", "(D)V", (65_537.75f64,)).await?;
    assert_eq!(jvm.invoke_virtual::<_, f64>(&value, "doubleValue", "()D", ()).await?, 65_537.75);
    assert_eq!(jvm.invoke_virtual::<_, f32>(&value, "floatValue", "()F", ()).await?, 65_537.75f32);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&value, "intValue", "()I", ()).await?, 65_537);
    assert_eq!(jvm.invoke_virtual::<_, i64>(&value, "longValue", "()J", ()).await?, 65_537);
    assert_eq!(jvm.invoke_virtual::<_, i8>(&value, "byteValue", "()B", ()).await?, 1);
    assert_eq!(jvm.invoke_virtual::<_, i16>(&value, "shortValue", "()S", ()).await?, 1);

    let text = JavaLangString::from_rust_string(&jvm, "-3.5").await?;
    let from_string = jvm.new_class("java/lang/Double", "(Ljava/lang/String;)V", (text,)).await?;
    assert_eq!(jvm.invoke_virtual::<_, f64>(&from_string, "doubleValue", "()D", ()).await?, -3.5);

    Ok(())
}

#[tokio::test]
async fn test_double_parse_value_of_and_format() -> Result<()> {
    let jvm = test_jvm().await?;

    for (text, expected) in [(" \t-1.25e2D\n", -125.0), (".5f", 0.5), ("+42.", 42.0)] {
        let text = JavaLangString::from_rust_string(&jvm, text).await?;
        let parsed: f64 = jvm
            .invoke_static("java/lang/Double", "parseDouble", "(Ljava/lang/String;)D", (text,))
            .await?;
        assert_eq!(parsed, expected);
    }
    for suffix in ['f', 'F', 'd', 'D'] {
        let text = JavaLangString::from_rust_string(&jvm, &format!("1.5{suffix}")).await?;
        let parsed: f64 = jvm
            .invoke_static("java/lang/Double", "parseDouble", "(Ljava/lang/String;)D", (text,))
            .await?;
        assert_eq!(parsed, 1.5);
    }

    let infinity = JavaLangString::from_rust_string(&jvm, "-Infinity").await?;
    let infinity: f64 = jvm
        .invoke_static("java/lang/Double", "parseDouble", "(Ljava/lang/String;)D", (infinity,))
        .await?;
    assert_eq!(infinity, f64::NEG_INFINITY);

    let nan = JavaLangString::from_rust_string(&jvm, "+NaN").await?;
    let nan: ClassInstanceRef<Double> = jvm
        .invoke_static("java/lang/Double", "valueOf", "(Ljava/lang/String;)Ljava/lang/Double;", (nan,))
        .await?;
    assert!(jvm.invoke_virtual::<_, bool>(&nan, "isNaN", "()Z", ()).await?);

    for (value, expected) in [(12.0f64, "12.0"), (-0.0, "-0.0"), (10_000_000.0, "1.0E7"), (0.000_125, "1.25E-4")] {
        let text: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/Double", "toString", "(D)Ljava/lang/String;", (value,))
            .await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, expected);
    }

    for malformed in ["", "nan", "Infinityd", "1e", ".", "1_0", "0x1.0p0", "1.0 dd"] {
        let malformed = JavaLangString::from_rust_string(&jvm, malformed).await?;
        let result: Result<f64> = jvm
            .invoke_static("java/lang/Double", "parseDouble", "(Ljava/lang/String;)D", (malformed,))
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("Double.parseDouble must reject malformed input");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/NumberFormatException"));
    }

    let null_result: Result<f64> = jvm
        .invoke_static("java/lang/Double", "parseDouble", "(Ljava/lang/String;)D", (None,))
        .await;
    let Err(JavaError::JavaException(exception)) = null_result else {
        panic!("Double.parseDouble(null) must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    for (value, expected) in [(f64::from_bits(1), "4.9E-324"), (f64::from_bits(0x8000_0000_0000_0001), "-4.9E-324")] {
        let text: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/Double", "toString", "(D)Ljava/lang/String;", (value,))
            .await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, expected);
    }

    let min = JavaLangString::from_rust_string(&jvm, "4.9e-324").await?;
    assert_eq!(
        jvm.invoke_static::<_, f64>("java/lang/Double", "parseDouble", "(Ljava/lang/String;)D", (min,))
            .await?
            .to_bits(),
        1
    );
    let overflow = JavaLangString::from_rust_string(&jvm, "1e10000").await?;
    assert_eq!(
        jvm.invoke_static::<_, f64>("java/lang/Double", "parseDouble", "(Ljava/lang/String;)D", (overflow,))
            .await?,
        f64::INFINITY
    );
    let underflow = JavaLangString::from_rust_string(&jvm, "-1e-10000").await?;
    assert_eq!(
        jvm.invoke_static::<_, f64>("java/lang/Double", "parseDouble", "(Ljava/lang/String;)D", (underflow,))
            .await?
            .to_bits(),
        (-0.0f64).to_bits()
    );

    Ok(())
}

#[tokio::test]
async fn test_double_bits_equality_hash_and_comparison() -> Result<()> {
    let jvm = test_jvm().await?;

    let payload_nan_a: f64 = jvm
        .invoke_static("java/lang/Double", "longBitsToDouble", "(J)D", (0x7ff8_0000_0000_0001i64,))
        .await?;
    let payload_nan_b: f64 = jvm
        .invoke_static("java/lang/Double", "longBitsToDouble", "(J)D", (0x7fff_ffff_ffff_ffffi64,))
        .await?;
    assert_eq!(
        jvm.invoke_static::<_, i64>("java/lang/Double", "doubleToLongBits", "(D)J", (payload_nan_a,))
            .await?,
        0x7ff8_0000_0000_0000
    );

    let raw = (-12.5f64).to_bits() as i64;
    let round_trip: f64 = jvm.invoke_static("java/lang/Double", "longBitsToDouble", "(J)D", (raw,)).await?;
    assert_eq!(
        jvm.invoke_static::<_, i64>("java/lang/Double", "doubleToLongBits", "(D)J", (round_trip,))
            .await?,
        raw
    );

    let nan_a = jvm.new_class("java/lang/Double", "(D)V", (payload_nan_a,)).await?;
    let nan_b = jvm.new_class("java/lang/Double", "(D)V", (payload_nan_b,)).await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&nan_a, "equals", "(Ljava/lang/Object;)Z", (nan_b.clone(),))
            .await?
    );
    assert_eq!(jvm.invoke_virtual::<_, i32>(&nan_a, "hashCode", "()I", ()).await?, 0x7ff8_0000);
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&nan_a, "compareTo", "(Ljava/lang/Double;)I", (nan_b,))
            .await?,
        0
    );

    let negative_zero = jvm.new_class("java/lang/Double", "(D)V", (-0.0f64,)).await?;
    let positive_zero = jvm.new_class("java/lang/Double", "(D)V", (0.0f64,)).await?;
    assert!(
        !jvm.invoke_virtual::<_, bool>(&negative_zero, "equals", "(Ljava/lang/Object;)Z", (positive_zero.clone(),))
            .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&negative_zero, "compareTo", "(Ljava/lang/Double;)I", (positive_zero.clone(),),)
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
        jvm.invoke_virtual::<_, i32>(&jvm.new_class("java/lang/Double", "(D)V", (0.0f64,)).await?, "hashCode", "()I", (),)
            .await?,
        0
    );

    let infinity = jvm.new_class("java/lang/Double", "(D)V", (f64::INFINITY,)).await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&nan_a, "compareTo", "(Ljava/lang/Double;)I", (infinity,))
            .await?,
        1
    );

    let typed_null: Result<i32> = jvm.invoke_virtual(&negative_zero, "compareTo", "(Ljava/lang/Double;)I", (None,)).await;
    let Err(JavaError::JavaException(exception)) = typed_null else {
        panic!("Double typed compare null must throw NPE");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let raw_null: Result<i32> = jvm.invoke_virtual(&negative_zero, "compareTo", "(Ljava/lang/Object;)I", (None,)).await;
    let Err(JavaError::JavaException(exception)) = raw_null else {
        panic!("Double raw compare null must throw NPE");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let object = jvm.new_class("java/lang/Object", "()V", ()).await?;
    let wrong_type: Result<i32> = jvm.invoke_virtual(&negative_zero, "compareTo", "(Ljava/lang/Object;)I", (object,)).await;
    let Err(JavaError::JavaException(exception)) = wrong_type else {
        panic!("Double raw compare wrong type must throw CCE");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/ClassCastException"));

    for (value, int_value, long_value) in [
        (f64::NAN, 0, 0i64),
        (f64::INFINITY, i32::MAX, i64::MAX),
        (f64::NEG_INFINITY, i32::MIN, i64::MIN),
    ] {
        let wrapper = jvm.new_class("java/lang/Double", "(D)V", (value,)).await?;
        assert_eq!(jvm.invoke_virtual::<_, i32>(&wrapper, "intValue", "()I", ()).await?, int_value);
        assert_eq!(jvm.invoke_virtual::<_, i64>(&wrapper, "longValue", "()J", ()).await?, long_value);
    }
    let maximum = jvm.new_class("java/lang/Double", "(D)V", (f64::MAX,)).await?;
    assert!(jvm.invoke_virtual::<_, f32>(&maximum, "floatValue", "()F", ()).await?.is_infinite());
    let minimum = jvm.new_class("java/lang/Double", "(D)V", (f64::from_bits(1),)).await?;
    assert_eq!(jvm.invoke_virtual::<_, f32>(&minimum, "floatValue", "()F", ()).await?.to_bits(), 0);

    Ok(())
}

#[tokio::test]
async fn test_double_constants_type_and_predicates() -> Result<()> {
    let jvm = test_jvm().await?;

    assert_eq!(
        jvm.get_static_field::<f64>("java/lang/Double", "POSITIVE_INFINITY", "D").await?,
        f64::INFINITY
    );
    assert_eq!(
        jvm.get_static_field::<f64>("java/lang/Double", "NEGATIVE_INFINITY", "D").await?,
        f64::NEG_INFINITY
    );
    assert!(jvm.get_static_field::<f64>("java/lang/Double", "NaN", "D").await?.is_nan());
    assert_eq!(jvm.get_static_field::<f64>("java/lang/Double", "MAX_VALUE", "D").await?, f64::MAX);
    assert_eq!(jvm.get_static_field::<f64>("java/lang/Double", "MIN_VALUE", "D").await?.to_bits(), 1);
    assert!(jvm.invoke_static::<_, bool>("java/lang/Double", "isNaN", "(D)Z", (f64::NAN,)).await?);
    assert!(
        jvm.invoke_static::<_, bool>("java/lang/Double", "isInfinite", "(D)Z", (f64::NEG_INFINITY,))
            .await?
    );

    let typ = jvm.get_static_field("java/lang/Double", "TYPE", "Ljava/lang/Class;").await?;
    let name: ClassInstanceRef<String> = jvm.invoke_virtual(&typ, "getName", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &name).await?, "double");
    assert!(jvm.invoke_virtual::<_, bool>(&typ, "isPrimitive", "()Z", ()).await?);

    Ok(())
}
