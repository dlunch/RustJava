use jvm::Result;

use test_utils::test_jvm;

#[tokio::test]
async fn test_abs() -> Result<()> {
    let jvm = test_jvm().await?;

    assert_eq!(42i32, jvm.invoke_static("java/lang/Math", "abs", "(I)I", (42,)).await?);
    assert_eq!(42i32, jvm.invoke_static("java/lang/Math", "abs", "(I)I", (-42,)).await?);

    Ok(())
}
