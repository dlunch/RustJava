use jvm::Result;

use test_utils::test_jvm;

#[tokio::test]
async fn test_abs() -> Result<()> {
    let jvm = test_jvm().await?;

    assert_eq!(42i32, jvm.invoke_static("java/lang/Math", "abs", "(I)I", (42,)).await?);
    assert_eq!(42i32, jvm.invoke_static("java/lang/Math", "abs", "(I)I", (-42,)).await?);

    assert_eq!(42i64, jvm.invoke_static("java/lang/Math", "abs", "(J)J", (42i64,)).await?);
    assert_eq!(42i64, jvm.invoke_static("java/lang/Math", "abs", "(J)J", (-42i64,)).await?);

    assert_eq!(3.15f32, jvm.invoke_static("java/lang/Math", "abs", "(F)F", (3.15f32,)).await?);
    assert_eq!(3.15f32, jvm.invoke_static("java/lang/Math", "abs", "(F)F", (-3.15f32,)).await?);

    assert_eq!(2.818f64, jvm.invoke_static("java/lang/Math", "abs", "(D)D", (2.818f64,)).await?);
    assert_eq!(2.818f64, jvm.invoke_static("java/lang/Math", "abs", "(D)D", (-2.818f64,)).await?);

    Ok(())
}

#[tokio::test]
async fn test_min_max() -> Result<()> {
    let jvm = test_jvm().await?;

    assert_eq!(10i32, jvm.invoke_static("java/lang/Math", "max", "(II)I", (10, 5)).await?);
    assert_eq!(5i32, jvm.invoke_static("java/lang/Math", "min", "(II)I", (10, 5)).await?);

    assert_eq!(20i64, jvm.invoke_static("java/lang/Math", "max", "(JJ)J", (20i64, 15i64)).await?);
    assert_eq!(15i64, jvm.invoke_static("java/lang/Math", "min", "(JJ)J", (20i64, 15i64)).await?);

    Ok(())
}
