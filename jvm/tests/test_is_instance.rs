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

#[tokio::test]
async fn test_is_instance_array_covariance() -> JvmResult<()> {
    let jvm = test_jvm().await?;

    let string_array = jvm.instantiate_array("Ljava/lang/String;", 1).await?;
    assert!(jvm.is_instance(&*string_array, "[Ljava/lang/Object;"));
    assert!(jvm.is_instance(&*string_array, "java/lang/Object"));
    assert!(jvm.is_instance(&*string_array, "java/lang/Cloneable"));
    assert!(jvm.is_instance(&*string_array, "java/io/Serializable"));
    assert!(jvm.is_instance(&*string_array, "[Ljava/lang/String;"));
    assert!(!jvm.is_instance(&*string_array, "[Ljava/lang/Integer;"));

    let int_array = jvm.instantiate_array("I", 1).await?;
    assert!(jvm.is_instance(&*int_array, "java/lang/Object"));
    assert!(!jvm.is_instance(&*int_array, "[Ljava/lang/Object;"));
    assert!(!jvm.is_instance(&*int_array, "[J"));

    let nested = jvm.instantiate_array("[Ljava/lang/String;", 1).await?;
    assert!(jvm.is_instance(&*nested, "[Ljava/lang/Object;"));
    assert!(jvm.is_instance(&*nested, "[[Ljava/lang/Object;"));
    assert!(jvm.is_instance(&*nested, "[[Ljava/lang/String;"));

    let object_array = jvm.instantiate_array("Ljava/lang/Object;", 1).await?;
    assert!(!jvm.is_instance(&*object_array, "[Ljava/lang/String;"));

    Ok(())
}
