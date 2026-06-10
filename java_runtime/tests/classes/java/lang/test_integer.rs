use jvm::{JavaError, Result, runtime::JavaLangString};

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

#[tokio::test]
async fn test_parse_int_invalid() -> Result<()> {
    let jvm = test_jvm().await?;

    let string = JavaLangString::from_rust_string(&jvm, "abc").await?;
    let result: Result<i32> = jvm.invoke_static("java/lang/Integer", "parseInt", "(Ljava/lang/String;)I", (string,)).await;

    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Expected JavaException, got {:?}", result);
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NumberFormatException"));

    Ok(())
}
