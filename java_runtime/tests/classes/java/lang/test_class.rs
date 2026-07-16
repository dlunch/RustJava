use java_runtime::classes::java::lang::{Class, String};
use jvm::{
    ClassInstanceRef, JavaError, Result,
    runtime::{JavaLangClass, JavaLangString},
};

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

#[tokio::test]
async fn test_for_name() -> Result<()> {
    let jvm = test_jvm().await?;

    let class_name = JavaLangString::from_rust_string(&jvm, "java.lang.String").await?;
    let class: ClassInstanceRef<Class> = jvm
        .invoke_static("java/lang/Class", "forName", "(Ljava/lang/String;)Ljava/lang/Class;", (class_name,))
        .await?;

    let rust_class = JavaLangClass::to_rust_class(&jvm, &class).await?;
    assert_eq!(rust_class.name(), "java/lang/String");

    let result: Result<ClassInstanceRef<Class>> = jvm
        .invoke_static("java/lang/Class", "forName", "(Ljava/lang/String;)Ljava/lang/Class;", (None,))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Class.forName(null) must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    Ok(())
}

#[tokio::test]
async fn test_primitive_class_api() -> Result<()> {
    let jvm = test_jvm().await?;

    for name in ["boolean", "byte", "char", "short", "int", "long", "float", "double"] {
        let primitive = JavaLangClass::from_rust_primitive(&jvm, name).await?;
        let primitive_name = JavaLangClass::name(&jvm, &primitive).await?;
        assert_eq!(primitive_name, name);

        let virtual_name: ClassInstanceRef<String> = jvm.invoke_virtual(&primitive, "getName", "()Ljava/lang/String;", ()).await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &virtual_name).await?, name);

        let is_primitive: bool = jvm.invoke_virtual(&primitive, "isPrimitive", "()Z", ()).await?;
        assert!(is_primitive);

        let class_name = JavaLangString::from_rust_string(&jvm, name).await?;
        let result: Result<ClassInstanceRef<Class>> = jvm
            .invoke_static("java/lang/Class", "forName", "(Ljava/lang/String;)Ljava/lang/Class;", (class_name,))
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("Class.forName must reject primitive source names");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/ClassNotFoundException"));
    }

    let object_class = jvm.resolve_class("java/lang/Object").await?.java_class();
    let string_class = jvm.resolve_class("java/lang/String").await?.java_class();
    let primitive = JavaLangClass::from_rust_primitive(&jvm, "int").await?;
    let other_primitive = JavaLangClass::from_rust_primitive(&jvm, "long").await?;

    let result: bool = jvm
        .invoke_virtual(&primitive, "isAssignableFrom", "(Ljava/lang/Class;)Z", (primitive.clone(),))
        .await?;
    assert!(result);

    let result: bool = jvm
        .invoke_virtual(&primitive, "isAssignableFrom", "(Ljava/lang/Class;)Z", (other_primitive,))
        .await?;
    assert!(!result);

    let result: bool = jvm
        .invoke_virtual(&primitive, "isAssignableFrom", "(Ljava/lang/Class;)Z", (string_class.clone(),))
        .await?;
    assert!(!result);

    let result: bool = jvm
        .invoke_virtual(&object_class, "isAssignableFrom", "(Ljava/lang/Class;)Z", (primitive,))
        .await?;
    assert!(!result);

    let result: Result<bool> = jvm
        .invoke_virtual(&object_class, "isAssignableFrom", "(Ljava/lang/Class;)Z", (None,))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Class.isAssignableFrom(null) must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    Ok(())
}

#[tokio::test]
async fn test_wrapper_type_fields_survive_gc() -> Result<()> {
    let jvm = test_jvm().await?;

    for (wrapper, primitive_name) in [
        ("java/lang/Boolean", "boolean"),
        ("java/lang/Byte", "byte"),
        ("java/lang/Character", "char"),
        ("java/lang/Short", "short"),
        ("java/lang/Integer", "int"),
        ("java/lang/Long", "long"),
        ("java/lang/Float", "float"),
        ("java/lang/Double", "double"),
    ] {
        let typ = jvm.get_static_field(wrapper, "TYPE", "Ljava/lang/Class;").await?;
        let name: ClassInstanceRef<String> = jvm.invoke_virtual(&typ, "getName", "()Ljava/lang/String;", ()).await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &name).await?, primitive_name);
        assert!(jvm.invoke_virtual::<_, bool>(&typ, "isPrimitive", "()Z", ()).await?);
    }

    let _: () = jvm.invoke_static("java/lang/System", "gc", "()V", ()).await?;

    for wrapper in [
        "java/lang/Boolean",
        "java/lang/Byte",
        "java/lang/Character",
        "java/lang/Short",
        "java/lang/Integer",
        "java/lang/Long",
        "java/lang/Float",
        "java/lang/Double",
    ] {
        let typ = jvm.get_static_field(wrapper, "TYPE", "Ljava/lang/Class;").await?;
        assert!(jvm.invoke_virtual::<_, bool>(&typ, "isPrimitive", "()Z", ()).await?);
    }

    Ok(())
}
