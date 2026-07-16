use jvm::{JavaError, Result};

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
