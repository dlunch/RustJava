use jvm::Result;

use test_utils::test_jvm;

#[tokio::test]
async fn test_cldc11_exception_and_error_hierarchy() -> Result<()> {
    let jvm = test_jvm().await?;

    for (class_name, parent_name) in [
        ("java/lang/IllegalAccessException", "java/lang/Exception"),
        ("java/lang/IllegalMonitorStateException", "java/lang/RuntimeException"),
        ("java/lang/IllegalThreadStateException", "java/lang/IllegalArgumentException"),
        ("java/lang/InstantiationException", "java/lang/Exception"),
        ("java/lang/VirtualMachineError", "java/lang/Error"),
        ("java/lang/OutOfMemoryError", "java/lang/VirtualMachineError"),
        ("java/io/InterruptedIOException", "java/io/IOException"),
        ("java/io/UnsupportedEncodingException", "java/io/IOException"),
        ("java/io/UTFDataFormatException", "java/io/IOException"),
    ] {
        let class = jvm.resolve_class(class_name).await?;
        assert!(jvm.is_inherited_from(&*class.definition, parent_name));

        if class_name != "java/lang/VirtualMachineError" {
            let instance = jvm.new_class(class_name, "()V", ()).await?;
            assert!(jvm.is_instance(&*instance, parent_name));
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_corrected_exception_names() -> Result<()> {
    let jvm = test_jvm().await?;

    let empty_stack = jvm.resolve_class("java/util/EmptyStackException").await?;
    assert_eq!(empty_stack.definition.name(), "java/util/EmptyStackException");

    let unknown_service = jvm.resolve_class("java/net/UnknownServiceException").await?;
    assert_eq!(unknown_service.definition.name(), "java/net/UnknownServiceException");

    Ok(())
}
