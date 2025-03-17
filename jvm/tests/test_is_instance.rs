use jvm::{Result as JvmResult, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_is_instance() -> JvmResult<()> {
    let jvm = test_jvm().await?;

    let string = JavaLangString::from_rust_string(&jvm, "test").await?;

    assert!(jvm.is_instance(&*string, "java/lang/String"));
    assert!(jvm.is_instance(&*string, "java/lang/Object"));
    assert!(!jvm.is_instance(&*string, "java/lang/Integer"));

    Ok(())
}
