use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use java_runtime::classes::java::util::Random;
use jvm::{Array, ClassInstanceRef, JavaError, Result};

use test_utils::test_jvm;

#[tokio::test]
async fn test_random() -> Result<()> {
    let jvm = test_jvm().await?;

    let seed = 42i64;
    let random = jvm.new_class("java/util/Random", "(J)V", (seed,)).await?;

    let next: i32 = jvm.invoke_virtual(&random, "nextInt", "()I", ()).await?;
    assert_eq!(next, -1170105035);

    let next: i32 = jvm.invoke_virtual(&random, "nextInt", "()I", ()).await?;
    assert_eq!(next, 234785527);

    Ok(())
}

#[tokio::test]
async fn test_random_cldc11_algorithms() -> Result<()> {
    let jvm = test_jvm().await?;

    let random = jvm.new_class("java/util/Random", "(J)V", (42i64,)).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&random, "nextInt", "(I)I", (100,)).await?, 30);

    let _: () = jvm.invoke_virtual(&random, "setSeed", "(J)V", (42i64,)).await?;
    assert_eq!(jvm.invoke_virtual::<_, i64>(&random, "nextLong", "()J", ()).await?, -5025562857975149833);

    let _: () = jvm.invoke_virtual(&random, "setSeed", "(J)V", (42i64,)).await?;
    let value: f32 = jvm.invoke_virtual(&random, "nextFloat", "()F", ()).await?;
    assert!((value - 0.7275637).abs() < f32::EPSILON);

    let _: () = jvm.invoke_virtual(&random, "setSeed", "(J)V", (42i64,)).await?;
    let value: f64 = jvm.invoke_virtual(&random, "nextDouble", "()D", ()).await?;
    assert!((value - 0.7275636800328681).abs() < f64::EPSILON);

    let result: Result<i32> = jvm.invoke_virtual(&random, "nextInt", "(I)I", (0,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("non-positive bound must throw IllegalArgumentException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalArgumentException"));

    Ok(())
}

#[test]
fn rng_01_descriptors_and_access_flags_match_jdk_12() {
    let proto = Random::as_proto();
    assert_eq!(proto.parent_class, Some("java/lang/Object"));
    assert_eq!(proto.interfaces, vec!["java/io/Serializable"]);
    assert_eq!(proto.access_flags, ClassAccessFlags::PUBLIC);

    let expected_methods = [
        ("<init>", "()V", MethodAccessFlags::PUBLIC),
        ("<init>", "(J)V", MethodAccessFlags::PUBLIC),
        ("next", "(I)I", MethodAccessFlags::PROTECTED | MethodAccessFlags::SYNCHRONIZED),
        ("nextBoolean", "()Z", MethodAccessFlags::PUBLIC),
        ("nextBytes", "([B)V", MethodAccessFlags::PUBLIC),
        ("nextInt", "()I", MethodAccessFlags::PUBLIC),
        ("nextInt", "(I)I", MethodAccessFlags::PUBLIC),
        ("nextLong", "()J", MethodAccessFlags::PUBLIC),
        ("nextFloat", "()F", MethodAccessFlags::PUBLIC),
        ("nextDouble", "()D", MethodAccessFlags::PUBLIC),
        ("nextGaussian", "()D", MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED),
        ("setSeed", "(J)V", MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED),
    ];
    assert_eq!(proto.methods.len(), expected_methods.len());
    for (name, descriptor, access_flags) in expected_methods {
        let method = proto
            .methods
            .iter()
            .find(|method| method.name == name && method.descriptor == descriptor)
            .unwrap_or_else(|| panic!("missing java/util/Random.{name}{descriptor}"));
        assert_eq!(method.access_flags, access_flags, "wrong access flags for {name}{descriptor}");
    }

    let expected_fields = [("seed", "J"), ("nextNextGaussian", "D"), ("haveNextNextGaussian", "Z")];
    assert_eq!(proto.fields.len(), expected_fields.len());
    for (name, descriptor) in expected_fields {
        let field = proto
            .fields
            .iter()
            .find(|field| field.name == name && field.descriptor == descriptor)
            .unwrap_or_else(|| panic!("missing java/util/Random.{name}:{descriptor}"));
        assert_eq!(field.access_flags, FieldAccessFlags::PRIVATE);
    }
}

#[tokio::test]
async fn rng_01_boolean_and_bytes_match_jdk_seed_oracle() -> Result<()> {
    let jvm = test_jvm().await?;

    let boolean_random = jvm.new_class("java/util/Random", "(J)V", (42i64,)).await?;
    assert!(jvm.invoke_virtual::<_, bool>(&boolean_random, "nextBoolean", "()Z", ()).await?);

    let bytes_random = jvm.new_class("java/util/Random", "(J)V", (42i64,)).await?;
    let mut bytes: ClassInstanceRef<Array<i8>> = jvm.instantiate_array("B", 10).await?.into();
    let _: () = jvm.invoke_virtual(&bytes_random, "nextBytes", "([B)V", (bytes.clone(),)).await?;
    assert_eq!(
        jvm.load_array::<i8>(&bytes, 0, 10).await?,
        vec![53, -99, 65, -70, -9, -118, -2, 13, -31, -69]
    );
    assert_eq!(jvm.invoke_virtual::<_, i32>(&bytes_random, "nextInt", "()I", ()).await?, 205897768);

    jvm.store_array(&mut bytes, 0, [0i8; 10]).await?;
    let _: () = jvm.invoke_virtual(&bytes_random, "setSeed", "(J)V", (42i64,)).await?;
    let _: () = jvm.invoke_virtual(&bytes_random, "nextBytes", "([B)V", (bytes.clone(),)).await?;
    assert_eq!(
        jvm.load_array::<i8>(&bytes, 0, 10).await?,
        vec![53, -99, 65, -70, -9, -118, -2, 13, -31, -69]
    );

    Ok(())
}

#[tokio::test]
async fn rng_01_next_bytes_null_and_empty_arrays_do_not_advance_seed() -> Result<()> {
    let jvm = test_jvm().await?;

    let random = jvm.new_class("java/util/Random", "(J)V", (42i64,)).await?;
    let null_bytes: ClassInstanceRef<Array<i8>> = None.into();
    let result: Result<()> = jvm.invoke_virtual(&random, "nextBytes", "([B)V", (null_bytes,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("nextBytes(null) must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));
    assert_eq!(jvm.invoke_virtual::<_, i32>(&random, "nextInt", "()I", ()).await?, -1170105035);

    let _: () = jvm.invoke_virtual(&random, "setSeed", "(J)V", (42i64,)).await?;
    let empty: ClassInstanceRef<Array<i8>> = jvm.instantiate_array("B", 0).await?.into();
    let _: () = jvm.invoke_virtual(&random, "nextBytes", "([B)V", (empty,)).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&random, "nextInt", "()I", ()).await?, -1170105035);

    let _: () = jvm.invoke_virtual(&random, "setSeed", "(J)V", (42i64,)).await?;
    let five: ClassInstanceRef<Array<i8>> = jvm.instantiate_array("B", 5).await?.into();
    let _: () = jvm.invoke_virtual(&random, "nextBytes", "([B)V", (five,)).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&random, "nextInt", "()I", ()).await?, -1360544799);

    Ok(())
}

#[tokio::test]
async fn rng_01_gaussian_matches_jdk_seed_oracle_and_consumes_cache() -> Result<()> {
    let jvm = test_jvm().await?;

    let random = jvm.new_class("java/util/Random", "(J)V", (42i64,)).await?;
    let first: f64 = jvm.invoke_virtual(&random, "nextGaussian", "()D", ()).await?;
    assert!((first - 1.1419053154730547).abs() < 1e-15);
    assert!(jvm.get_field::<bool>(&random, "haveNextNextGaussian", "Z").await?);
    let cached: f64 = jvm.get_field(&random, "nextNextGaussian", "D").await?;

    let second: f64 = jvm.invoke_virtual(&random, "nextGaussian", "()D", ()).await?;
    assert_eq!(second.to_bits(), cached.to_bits());
    assert!((second - 0.9194079489827879).abs() < 1e-15);
    assert!(!jvm.get_field::<bool>(&random, "haveNextNextGaussian", "Z").await?);

    let after_cached: i32 = jvm.invoke_virtual(&random, "nextInt", "()I", ()).await?;
    let control = jvm.new_class("java/util/Random", "(J)V", (42i64,)).await?;
    let _: f64 = jvm.invoke_virtual(&control, "nextDouble", "()D", ()).await?;
    let _: f64 = jvm.invoke_virtual(&control, "nextDouble", "()D", ()).await?;
    assert_eq!(after_cached, jvm.invoke_virtual::<_, i32>(&control, "nextInt", "()I", ()).await?);

    Ok(())
}

#[tokio::test]
async fn rng_02_set_seed_clears_gaussian_cache() -> Result<()> {
    let jvm = test_jvm().await?;

    let random = jvm.new_class("java/util/Random", "(J)V", (42i64,)).await?;
    let first: f64 = jvm.invoke_virtual(&random, "nextGaussian", "()D", ()).await?;
    assert!(jvm.get_field::<bool>(&random, "haveNextNextGaussian", "Z").await?);

    let _: () = jvm.invoke_virtual(&random, "setSeed", "(J)V", (42i64,)).await?;
    assert!(!jvm.get_field::<bool>(&random, "haveNextNextGaussian", "Z").await?);
    let reset_first: f64 = jvm.invoke_virtual(&random, "nextGaussian", "()D", ()).await?;
    assert_eq!(reset_first.to_bits(), first.to_bits());

    Ok(())
}
