use jvm::{Result, runtime::JavaLangString};

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

    Ok(())
}
