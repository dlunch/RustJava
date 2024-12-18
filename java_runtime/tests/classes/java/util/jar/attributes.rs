use java_runtime::classes::java::lang::String;
use jvm::{runtime::JavaLangString, ClassInstanceRef, Result};

use test_utils::test_jvm;

#[tokio::test]
async fn test_attribute_get_set() -> Result<()> {
    let jvm = test_jvm().await?;

    let attributes = jvm.new_class("java/util/jar/Attributes", "()V", ()).await?;

    let name = JavaLangString::from_rust_string(&jvm, "Name").await?;
    let value = JavaLangString::from_rust_string(&jvm, "Value").await?;

    let old: ClassInstanceRef<String> = jvm
        .invoke_virtual(
            &attributes,
            "putValue",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
            (name.clone(), value),
        )
        .await?;
    assert!(old.is_null());

    let value = jvm
        .invoke_virtual(&attributes, "getValue", "(Ljava/lang/String;)Ljava/lang/String;", (name,))
        .await?;

    let value = JavaLangString::to_rust_string(&jvm, &value).await?;
    assert_eq!(value, "Value");

    Ok(())
}
