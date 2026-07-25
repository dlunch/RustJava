use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use java_runtime::classes::java::lang::Math;
use java_runtime::classes::java::util::Random;
use jvm::{ClassInstanceRef, Result};

use test_utils::test_jvm;

#[tokio::test]
async fn math_01_constants_descriptors_and_access_flags() -> Result<()> {
    let proto = Math::as_proto();
    assert!(proto.access_flags.contains(ClassAccessFlags::PUBLIC | ClassAccessFlags::FINAL));

    for (name, descriptor) in [
        ("abs", "(I)I"),
        ("abs", "(J)J"),
        ("abs", "(F)F"),
        ("abs", "(D)D"),
        ("ceil", "(D)D"),
        ("floor", "(D)D"),
        ("sqrt", "(D)D"),
        ("sin", "(D)D"),
        ("cos", "(D)D"),
        ("tan", "(D)D"),
        ("toDegrees", "(D)D"),
        ("toRadians", "(D)D"),
        ("min", "(II)I"),
        ("min", "(JJ)J"),
        ("min", "(FF)F"),
        ("min", "(DD)D"),
        ("max", "(II)I"),
        ("max", "(JJ)J"),
        ("max", "(FF)F"),
        ("max", "(DD)D"),
        ("acos", "(D)D"),
        ("asin", "(D)D"),
        ("atan", "(D)D"),
        ("atan2", "(DD)D"),
        ("exp", "(D)D"),
        ("log", "(D)D"),
        ("pow", "(DD)D"),
        ("rint", "(D)D"),
        ("IEEEremainder", "(DD)D"),
        ("round", "(F)I"),
        ("round", "(D)J"),
        ("random", "()D"),
    ] {
        let method = proto
            .methods
            .iter()
            .find(|method| method.name == name && method.descriptor == descriptor)
            .unwrap_or_else(|| panic!("missing java/lang/Math.{name}{descriptor}"));
        if name == "random" {
            assert_eq!(
                method.access_flags,
                MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC | MethodAccessFlags::SYNCHRONIZED,
                "wrong access flags for java/lang/Math.random()D"
            );
        } else {
            assert_eq!(
                method.access_flags,
                MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                "wrong access flags for java/lang/Math.{name}{descriptor}"
            );
        }
    }

    for name in ["E", "PI"] {
        let field = proto
            .fields
            .iter()
            .find(|field| field.name == name && field.descriptor == "D")
            .unwrap_or_else(|| panic!("missing java/lang/Math.{name}:D"));
        assert!(
            field
                .access_flags
                .contains(FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL)
        );
    }

    let jvm = test_jvm().await?;
    assert_eq!(
        jvm.get_static_field::<f64>("java/lang/Math", "E", "D").await?.to_bits(),
        core::f64::consts::E.to_bits()
    );
    assert_eq!(
        jvm.get_static_field::<f64>("java/lang/Math", "PI", "D").await?.to_bits(),
        core::f64::consts::PI.to_bits()
    );

    Ok(())
}

#[tokio::test]
async fn math_02_cldc_transcendentals_and_conversions() -> Result<()> {
    let jvm = test_jvm().await?;

    assert_eq!(jvm.invoke_static::<_, f64>("java/lang/Math", "ceil", "(D)D", (1.25,)).await?, 2.0);
    assert_eq!(jvm.invoke_static::<_, f64>("java/lang/Math", "ceil", "(D)D", (-1.25,)).await?, -1.0);
    assert_eq!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "ceil", "(D)D", (-0.0,)).await?.to_bits(),
        (-0.0f64).to_bits()
    );
    assert!(jvm.invoke_static::<_, f64>("java/lang/Math", "ceil", "(D)D", (f64::NAN,)).await?.is_nan());
    assert_eq!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "ceil", "(D)D", (f64::INFINITY,)).await?,
        f64::INFINITY
    );

    assert_eq!(jvm.invoke_static::<_, f64>("java/lang/Math", "floor", "(D)D", (1.75,)).await?, 1.0);
    assert_eq!(jvm.invoke_static::<_, f64>("java/lang/Math", "floor", "(D)D", (-1.25,)).await?, -2.0);
    assert_eq!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "floor", "(D)D", (-0.0,)).await?.to_bits(),
        (-0.0f64).to_bits()
    );

    assert_eq!(jvm.invoke_static::<_, f64>("java/lang/Math", "sqrt", "(D)D", (4.0,)).await?, 2.0);
    assert!(jvm.invoke_static::<_, f64>("java/lang/Math", "sqrt", "(D)D", (-1.0,)).await?.is_nan());
    assert_eq!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "sqrt", "(D)D", (-0.0,)).await?.to_bits(),
        (-0.0f64).to_bits()
    );
    assert_eq!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "sqrt", "(D)D", (f64::INFINITY,)).await?,
        f64::INFINITY
    );

    assert_eq!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "sin", "(D)D", (-0.0,)).await?.to_bits(),
        (-0.0f64).to_bits()
    );
    assert_eq!(jvm.invoke_static::<_, f64>("java/lang/Math", "cos", "(D)D", (0.0,)).await?, 1.0);
    assert_eq!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "tan", "(D)D", (-0.0,)).await?.to_bits(),
        (-0.0f64).to_bits()
    );
    assert!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "sin", "(D)D", (f64::INFINITY,))
            .await?
            .is_nan()
    );
    assert!(jvm.invoke_static::<_, f64>("java/lang/Math", "cos", "(D)D", (f64::NAN,)).await?.is_nan());

    let degrees: f64 = jvm.invoke_static("java/lang/Math", "toDegrees", "(D)D", (core::f64::consts::PI,)).await?;
    assert!((degrees - 180.0).abs() <= f64::EPSILON);
    let radians: f64 = jvm.invoke_static("java/lang/Math", "toRadians", "(D)D", (180.0,)).await?;
    assert!((radians - core::f64::consts::PI).abs() <= f64::EPSILON);
    assert_eq!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "toDegrees", "(D)D", (-0.0,))
            .await?
            .to_bits(),
        (-0.0f64).to_bits()
    );
    assert_eq!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "toRadians", "(D)D", (f64::INFINITY,))
            .await?,
        f64::INFINITY
    );
    assert!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "toDegrees", "(D)D", (f64::NAN,))
            .await?
            .is_nan()
    );

    Ok(())
}

#[tokio::test]
async fn math_03_floating_min_max_propagate_nan_and_order_signed_zero() -> Result<()> {
    let jvm = test_jvm().await?;

    assert_eq!(
        jvm.invoke_static::<_, f32>("java/lang/Math", "min", "(FF)F", (3.0f32, 4.0f32)).await?,
        3.0
    );
    assert_eq!(
        jvm.invoke_static::<_, f32>("java/lang/Math", "max", "(FF)F", (3.0f32, 4.0f32)).await?,
        4.0
    );
    assert!(
        jvm.invoke_static::<_, f32>("java/lang/Math", "min", "(FF)F", (f32::NAN, 1.0f32))
            .await?
            .is_nan()
    );
    assert!(
        jvm.invoke_static::<_, f32>("java/lang/Math", "max", "(FF)F", (1.0f32, f32::NAN))
            .await?
            .is_nan()
    );
    assert_eq!(
        jvm.invoke_static::<_, f32>("java/lang/Math", "min", "(FF)F", (0.0f32, -0.0f32))
            .await?
            .to_bits(),
        (-0.0f32).to_bits()
    );
    assert_eq!(
        jvm.invoke_static::<_, f32>("java/lang/Math", "max", "(FF)F", (-0.0f32, 0.0f32))
            .await?
            .to_bits(),
        0.0f32.to_bits()
    );

    assert_eq!(jvm.invoke_static::<_, f64>("java/lang/Math", "min", "(DD)D", (3.0, 4.0)).await?, 3.0);
    assert_eq!(jvm.invoke_static::<_, f64>("java/lang/Math", "max", "(DD)D", (3.0, 4.0)).await?, 4.0);
    assert!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "min", "(DD)D", (f64::NAN, 1.0))
            .await?
            .is_nan()
    );
    assert!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "max", "(DD)D", (1.0, f64::NAN))
            .await?
            .is_nan()
    );
    assert_eq!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "min", "(DD)D", (0.0, -0.0))
            .await?
            .to_bits(),
        (-0.0f64).to_bits()
    );
    assert_eq!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "max", "(DD)D", (-0.0, 0.0))
            .await?
            .to_bits(),
        0.0f64.to_bits()
    );
    assert_eq!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "min", "(DD)D", (f64::NEG_INFINITY, f64::INFINITY))
            .await?,
        f64::NEG_INFINITY
    );
    assert_eq!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "max", "(DD)D", (f64::NEG_INFINITY, f64::INFINITY))
            .await?,
        f64::INFINITY
    );

    Ok(())
}

#[tokio::test]
async fn math_04_j2se_unary_functions_and_rint() -> Result<()> {
    let jvm = test_jvm().await?;

    assert_eq!(jvm.invoke_static::<_, f64>("java/lang/Math", "acos", "(D)D", (1.0,)).await?, 0.0);
    assert!(jvm.invoke_static::<_, f64>("java/lang/Math", "acos", "(D)D", (2.0,)).await?.is_nan());
    assert_eq!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "asin", "(D)D", (-0.0,)).await?.to_bits(),
        (-0.0f64).to_bits()
    );
    assert_eq!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "atan", "(D)D", (-0.0,)).await?.to_bits(),
        (-0.0f64).to_bits()
    );
    assert_eq!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "exp", "(D)D", (f64::NEG_INFINITY,)).await?,
        0.0
    );
    assert_eq!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "exp", "(D)D", (f64::INFINITY,)).await?,
        f64::INFINITY
    );
    assert_eq!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "log", "(D)D", (0.0,)).await?,
        f64::NEG_INFINITY
    );
    assert!(jvm.invoke_static::<_, f64>("java/lang/Math", "log", "(D)D", (-1.0,)).await?.is_nan());

    assert_eq!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "rint", "(D)D", (0.5,)).await?.to_bits(),
        0.0f64.to_bits()
    );
    assert_eq!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "rint", "(D)D", (-0.5,)).await?.to_bits(),
        (-0.0f64).to_bits()
    );
    assert_eq!(jvm.invoke_static::<_, f64>("java/lang/Math", "rint", "(D)D", (1.5,)).await?, 2.0);
    assert_eq!(jvm.invoke_static::<_, f64>("java/lang/Math", "rint", "(D)D", (2.5,)).await?, 2.0);
    assert_eq!(jvm.invoke_static::<_, f64>("java/lang/Math", "rint", "(D)D", (-1.5,)).await?, -2.0);
    assert_eq!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "rint", "(D)D", (f64::INFINITY,)).await?,
        f64::INFINITY
    );
    assert!(jvm.invoke_static::<_, f64>("java/lang/Math", "rint", "(D)D", (f64::NAN,)).await?.is_nan());

    Ok(())
}

#[tokio::test]
async fn math_05_binary_functions_and_ieee_remainder() -> Result<()> {
    let jvm = test_jvm().await?;

    assert_eq!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "atan2", "(DD)D", (0.0, -1.0))
            .await?
            .to_bits(),
        core::f64::consts::PI.to_bits()
    );
    assert_eq!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "atan2", "(DD)D", (-0.0, -1.0))
            .await?
            .to_bits(),
        (-core::f64::consts::PI).to_bits()
    );
    assert_eq!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "atan2", "(DD)D", (-0.0, 1.0))
            .await?
            .to_bits(),
        (-0.0f64).to_bits()
    );

    assert!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "pow", "(DD)D", (1.0, f64::NAN))
            .await?
            .is_nan()
    );
    assert!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "pow", "(DD)D", (-1.0, f64::NAN))
            .await?
            .is_nan()
    );
    assert_eq!(jvm.invoke_static::<_, f64>("java/lang/Math", "pow", "(DD)D", (f64::NAN, 0.0)).await?, 1.0);
    assert_eq!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "pow", "(DD)D", (f64::NAN, -0.0)).await?,
        1.0
    );
    assert!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "pow", "(DD)D", (-1.0, f64::INFINITY))
            .await?
            .is_nan()
    );
    assert!(jvm.invoke_static::<_, f64>("java/lang/Math", "pow", "(DD)D", (-2.0, 0.5)).await?.is_nan());
    assert_eq!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "pow", "(DD)D", (-0.0, -3.0)).await?,
        f64::NEG_INFINITY
    );
    assert_eq!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "pow", "(DD)D", (-0.0, 3.0))
            .await?
            .to_bits(),
        (-0.0f64).to_bits()
    );

    assert_eq!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "IEEEremainder", "(DD)D", (5.0, 2.0))
            .await?,
        1.0
    );
    assert_eq!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "IEEEremainder", "(DD)D", (7.0, 2.0))
            .await?,
        -1.0
    );
    assert_eq!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "IEEEremainder", "(DD)D", (-0.0, 3.0))
            .await?
            .to_bits(),
        (-0.0f64).to_bits()
    );
    assert!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "IEEEremainder", "(DD)D", (f64::INFINITY, 2.0))
            .await?
            .is_nan()
    );
    assert!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "IEEEremainder", "(DD)D", (2.0, 0.0))
            .await?
            .is_nan()
    );
    assert_eq!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "IEEEremainder", "(DD)D", (2.0, f64::INFINITY))
            .await?,
        2.0
    );

    Ok(())
}

#[tokio::test]
async fn math_06_round_uses_java_saturation_and_half_toward_positive_infinity() -> Result<()> {
    let jvm = test_jvm().await?;

    assert_eq!(jvm.invoke_static::<_, i32>("java/lang/Math", "round", "(F)I", (1.4f32,)).await?, 1);
    assert_eq!(jvm.invoke_static::<_, i32>("java/lang/Math", "round", "(F)I", (1.5f32,)).await?, 2);
    assert_eq!(jvm.invoke_static::<_, i32>("java/lang/Math", "round", "(F)I", (-1.5f32,)).await?, -1);
    assert_eq!(jvm.invoke_static::<_, i32>("java/lang/Math", "round", "(F)I", (-1.6f32,)).await?, -2);
    assert_eq!(jvm.invoke_static::<_, i32>("java/lang/Math", "round", "(F)I", (f32::NAN,)).await?, 0);
    assert_eq!(
        jvm.invoke_static::<_, i32>("java/lang/Math", "round", "(F)I", (f32::INFINITY,)).await?,
        i32::MAX
    );
    assert_eq!(
        jvm.invoke_static::<_, i32>("java/lang/Math", "round", "(F)I", (f32::NEG_INFINITY,))
            .await?,
        i32::MIN
    );
    assert_eq!(
        jvm.invoke_static::<_, i32>("java/lang/Math", "round", "(F)I", (f32::MAX,)).await?,
        i32::MAX
    );
    assert_eq!(
        jvm.invoke_static::<_, i32>("java/lang/Math", "round", "(F)I", (-f32::MAX,)).await?,
        i32::MIN
    );
    assert_eq!(
        jvm.invoke_static::<_, i32>("java/lang/Math", "round", "(F)I", (f32::from_bits((i32::MAX as f32).to_bits() - 1),),)
            .await?,
        2_147_483_520
    );
    assert_eq!(
        jvm.invoke_static::<_, i32>("java/lang/Math", "round", "(F)I", (f32::from_bits((i32::MIN as f32).to_bits() - 1),),)
            .await?,
        -2_147_483_520
    );

    assert_eq!(jvm.invoke_static::<_, i64>("java/lang/Math", "round", "(D)J", (1.4,)).await?, 1);
    assert_eq!(jvm.invoke_static::<_, i64>("java/lang/Math", "round", "(D)J", (1.5,)).await?, 2);
    assert_eq!(jvm.invoke_static::<_, i64>("java/lang/Math", "round", "(D)J", (-1.5,)).await?, -1);
    assert_eq!(jvm.invoke_static::<_, i64>("java/lang/Math", "round", "(D)J", (-1.6,)).await?, -2);
    assert_eq!(jvm.invoke_static::<_, i64>("java/lang/Math", "round", "(D)J", (f64::NAN,)).await?, 0);
    assert_eq!(
        jvm.invoke_static::<_, i64>("java/lang/Math", "round", "(D)J", (f64::INFINITY,)).await?,
        i64::MAX
    );
    assert_eq!(
        jvm.invoke_static::<_, i64>("java/lang/Math", "round", "(D)J", (f64::NEG_INFINITY,))
            .await?,
        i64::MIN
    );
    assert_eq!(
        jvm.invoke_static::<_, i64>("java/lang/Math", "round", "(D)J", (f64::MAX,)).await?,
        i64::MAX
    );
    assert_eq!(
        jvm.invoke_static::<_, i64>("java/lang/Math", "round", "(D)J", (-f64::MAX,)).await?,
        i64::MIN
    );
    assert_eq!(
        jvm.invoke_static::<_, i64>("java/lang/Math", "round", "(D)J", (f64::from_bits((i64::MAX as f64).to_bits() - 1),),)
            .await?,
        9_223_372_036_854_774_784
    );
    assert_eq!(
        jvm.invoke_static::<_, i64>("java/lang/Math", "round", "(D)J", (f64::from_bits((i64::MIN as f64).to_bits() - 1),),)
            .await?,
        -9_223_372_036_854_774_784
    );

    Ok(())
}

#[tokio::test]
async fn math_07_random_stays_in_unit_interval_and_advances_one_generator() -> Result<()> {
    let jvm = test_jvm().await?;

    let uninitialized: ClassInstanceRef<Random> = jvm
        .get_static_field("java/lang/Math", "randomNumberGenerator", "Ljava/util/Random;")
        .await?;
    assert!(uninitialized.is_null());

    let first: f64 = jvm.invoke_static("java/lang/Math", "random", "()D", ()).await?;
    assert!((0.0..1.0).contains(&first));
    let generator: ClassInstanceRef<Random> = jvm
        .get_static_field("java/lang/Math", "randomNumberGenerator", "Ljava/util/Random;")
        .await?;
    assert!(!generator.is_null());
    let generator_identity = generator.identity();

    let mut changed = false;
    for _ in 0..256 {
        let value: f64 = jvm.invoke_static("java/lang/Math", "random", "()D", ()).await?;
        assert!((0.0..1.0).contains(&value));
        changed |= value != first;
    }
    assert!(changed);
    let same_generator: ClassInstanceRef<Random> = jvm
        .get_static_field("java/lang/Math", "randomNumberGenerator", "Ljava/util/Random;")
        .await?;
    assert_eq!(same_generator.identity(), generator_identity);

    Ok(())
}

#[tokio::test]
async fn math_08_existing_abs_and_integer_min_max_cover_java_edges() -> Result<()> {
    let jvm = test_jvm().await?;

    assert_eq!(jvm.invoke_static::<_, i32>("java/lang/Math", "abs", "(I)I", (i32::MIN,)).await?, i32::MIN);
    assert_eq!(jvm.invoke_static::<_, i64>("java/lang/Math", "abs", "(J)J", (i64::MIN,)).await?, i64::MIN);
    assert_eq!(
        jvm.invoke_static::<_, f32>("java/lang/Math", "abs", "(F)F", (-0.0f32,)).await?.to_bits(),
        0.0f32.to_bits()
    );
    assert_eq!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "abs", "(D)D", (-0.0,)).await?.to_bits(),
        0.0f64.to_bits()
    );
    assert!(jvm.invoke_static::<_, f32>("java/lang/Math", "abs", "(F)F", (f32::NAN,)).await?.is_nan());
    assert!(jvm.invoke_static::<_, f64>("java/lang/Math", "abs", "(D)D", (f64::NAN,)).await?.is_nan());
    assert_eq!(
        jvm.invoke_static::<_, f64>("java/lang/Math", "abs", "(D)D", (f64::NEG_INFINITY,)).await?,
        f64::INFINITY
    );

    assert_eq!(
        jvm.invoke_static::<_, i32>("java/lang/Math", "min", "(II)I", (i32::MIN, i32::MAX))
            .await?,
        i32::MIN
    );
    assert_eq!(
        jvm.invoke_static::<_, i32>("java/lang/Math", "max", "(II)I", (i32::MIN, i32::MAX))
            .await?,
        i32::MAX
    );
    assert_eq!(
        jvm.invoke_static::<_, i64>("java/lang/Math", "min", "(JJ)J", (i64::MIN, i64::MAX))
            .await?,
        i64::MIN
    );
    assert_eq!(
        jvm.invoke_static::<_, i64>("java/lang/Math", "max", "(JJ)J", (i64::MIN, i64::MAX))
            .await?,
        i64::MAX
    );

    Ok(())
}
