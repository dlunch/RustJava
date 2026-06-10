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

#[tokio::test]
async fn test_is_instance_interface() -> JvmResult<()> {
    let jvm = test_jvm().await?;

    let buffer = jvm.instantiate_array("B", 0).await?;
    let bais = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (buffer,)).await?;
    let dis = jvm.new_class("java/io/DataInputStream", "(Ljava/io/InputStream;)V", (bais,)).await?;

    assert!(jvm.is_instance(&*dis, "java/io/DataInput"));
    assert!(jvm.is_instance(&*dis, "java/io/FilterInputStream"));
    assert!(jvm.is_instance(&*dis, "java/lang/Object"));
    assert!(!jvm.is_instance(&*dis, "java/io/DataOutput"));

    Ok(())
}
