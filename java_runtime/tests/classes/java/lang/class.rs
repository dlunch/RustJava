use jvm::{Result, runtime::JavaLangClass};

use test_utils::test_jvm;

#[tokio::test]
async fn test_class() -> Result<()> {
    let jvm = test_jvm().await?;

    let java_class = jvm.resolve_class("java/lang/String").await?.java_class();

    let rust_class = JavaLangClass::to_rust_class(&jvm, &java_class).await?;
    assert_eq!(rust_class.name(), "java/lang/String");

    // try call to_rust_class twice to test if box is not dropped
    let rust_class = JavaLangClass::to_rust_class(&jvm, &java_class).await?;
    assert_eq!(rust_class.name(), "java/lang/String");

    Ok(())
}

#[tokio::test]
async fn test_is_assignable_from() -> Result<()> {
    let jvm = test_jvm().await?;

    let string_class = jvm.resolve_class("java/lang/String").await?.java_class();
    let object_class = jvm.resolve_class("java/lang/Object").await?.java_class();

    let result: bool = jvm
        .invoke_virtual(&object_class, "isAssignableFrom", "(Ljava/lang/Class;)Z", (string_class.clone(),))
        .await?;
    assert!(result);

    let thread_class = jvm.resolve_class("java/lang/Thread").await?.java_class();

    let result: bool = jvm
        .invoke_virtual(&string_class, "isAssignableFrom", "(Ljava/lang/Class;)Z", (thread_class,))
        .await?;
    assert!(!result);

    Ok(())
}
