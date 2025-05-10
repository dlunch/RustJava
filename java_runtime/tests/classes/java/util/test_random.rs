use jvm::Result;

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
