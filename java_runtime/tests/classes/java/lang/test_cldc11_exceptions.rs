use java_constants::{ClassAccessFlags, MethodAccessFlags};
use java_runtime::{classes::java::lang::String, get_runtime_class_proto};
use jvm::{ClassInstanceRef, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_cldc11_exception_and_error_hierarchy() -> Result<()> {
    let jvm = test_jvm().await?;

    for (class_name, parent_name) in [
        ("java/lang/IllegalAccessException", "java/lang/Exception"),
        ("java/lang/IllegalMonitorStateException", "java/lang/RuntimeException"),
        ("java/lang/IllegalThreadStateException", "java/lang/IllegalArgumentException"),
        ("java/lang/InstantiationException", "java/lang/Exception"),
        ("java/lang/VerifyError", "java/lang/LinkageError"),
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

#[tokio::test]
async fn test_runtime_api_exception_and_error_hierarchy() -> Result<()> {
    let jvm = test_jvm().await?;

    for (class_name, parent_name, constructor_descriptors) in [
        (
            "java/lang/IllegalStateException",
            "java/lang/RuntimeException",
            &["()V", "(Ljava/lang/String;)V"][..],
        ),
        (
            "java/lang/StackOverflowError",
            "java/lang/VirtualMachineError",
            &["()V", "(Ljava/lang/String;)V"][..],
        ),
        (
            "java/lang/InternalError",
            "java/lang/VirtualMachineError",
            &["()V", "(Ljava/lang/String;)V"][..],
        ),
        (
            "java/lang/UnknownError",
            "java/lang/VirtualMachineError",
            &["()V", "(Ljava/lang/String;)V"][..],
        ),
        (
            "java/lang/IllegalAccessError",
            "java/lang/IncompatibleClassChangeError",
            &["()V", "(Ljava/lang/String;)V"][..],
        ),
        ("java/lang/ThreadDeath", "java/lang/Error", &["()V"][..]),
        (
            "java/lang/NoSuchFieldException",
            "java/lang/Exception",
            &["()V", "(Ljava/lang/String;)V"][..],
        ),
        (
            "java/lang/NoSuchMethodException",
            "java/lang/Exception",
            &["()V", "(Ljava/lang/String;)V"][..],
        ),
        (
            "java/lang/ClassCircularityError",
            "java/lang/LinkageError",
            &["()V", "(Ljava/lang/String;)V"][..],
        ),
        (
            "java/util/ConcurrentModificationException",
            "java/lang/RuntimeException",
            &["()V", "(Ljava/lang/String;)V"][..],
        ),
    ] {
        let proto = get_runtime_class_proto(class_name).unwrap_or_else(|| panic!("missing {class_name}"));
        assert_eq!(proto.access_flags, ClassAccessFlags::PUBLIC, "{class_name}");
        assert_eq!(proto.parent_class, Some(parent_name), "{class_name}");
        assert!(proto.interfaces.is_empty(), "{class_name}");
        assert!(proto.fields.is_empty(), "{class_name}");
        assert_eq!(proto.methods.len(), constructor_descriptors.len(), "{class_name}");
        for descriptor in constructor_descriptors {
            let constructor = proto
                .methods
                .iter()
                .find(|method| method.name == "<init>" && method.descriptor == *descriptor)
                .unwrap_or_else(|| panic!("missing {class_name}.<init>{descriptor}"));
            assert_eq!(constructor.access_flags, MethodAccessFlags::PUBLIC, "{class_name}.<init>{descriptor}");
        }

        let class = jvm.resolve_class(class_name).await?;
        assert_eq!(class.definition.super_class_name().as_deref(), Some(parent_name));
        assert!(jvm.is_inherited_from(&*class.definition, parent_name));

        let instance = jvm.new_class(class_name, "()V", ()).await?;
        assert!(jvm.is_instance(&*instance, parent_name));
    }

    Ok(())
}

#[tokio::test]
async fn test_runtime_api_exception_and_error_message_constructors() -> Result<()> {
    let jvm = test_jvm().await?;

    for class_name in [
        "java/lang/IllegalStateException",
        "java/lang/StackOverflowError",
        "java/lang/InternalError",
        "java/lang/UnknownError",
        "java/lang/IllegalAccessError",
        "java/lang/NoSuchFieldException",
        "java/lang/NoSuchMethodException",
        "java/lang/ClassCircularityError",
        "java/util/ConcurrentModificationException",
    ] {
        let message = JavaLangString::from_rust_string(&jvm, class_name).await?;
        let instance = jvm.new_class(class_name, "(Ljava/lang/String;)V", (message,)).await?;
        let message: ClassInstanceRef<String> = jvm.invoke_virtual(&instance, "getMessage", "()Ljava/lang/String;", ()).await?;

        assert_eq!(JavaLangString::to_rust_string(&jvm, &message).await?, class_name);
    }

    Ok(())
}
