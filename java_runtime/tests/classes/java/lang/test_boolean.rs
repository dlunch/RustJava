use java_runtime::classes::java::lang::{Boolean, Object, String};
use jvm::{ClassInstanceRef, JavaError, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_boolean_string_constants_and_type() -> Result<()> {
    let jvm = test_jvm().await?;

    let true_value: ClassInstanceRef<Boolean> = jvm.new_class("java/lang/Boolean", "(Z)V", (true,)).await?.into();
    assert!(jvm.invoke_virtual::<_, bool>(&true_value, "booleanValue", "()Z", ()).await?);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&true_value, "hashCode", "()I", ()).await?, 1231);

    let text: ClassInstanceRef<String> = jvm.invoke_virtual(&true_value, "toString", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "true");

    let mixed_case = JavaLangString::from_rust_string(&jvm, "TrUe").await?;
    let parsed: ClassInstanceRef<Boolean> = jvm
        .invoke_static("java/lang/Boolean", "valueOf", "(Ljava/lang/String;)Ljava/lang/Boolean;", (mixed_case,))
        .await?;
    assert!(jvm.invoke_virtual::<_, bool>(&parsed, "booleanValue", "()Z", ()).await?);

    let padded = JavaLangString::from_rust_string(&jvm, " true").await?;
    let parsed: ClassInstanceRef<Boolean> = jvm
        .invoke_static("java/lang/Boolean", "valueOf", "(Ljava/lang/String;)Ljava/lang/Boolean;", (padded,))
        .await?;
    assert!(!jvm.invoke_virtual::<_, bool>(&parsed, "booleanValue", "()Z", ()).await?);

    let null_string: ClassInstanceRef<String> = None.into();
    let parsed: ClassInstanceRef<Boolean> = jvm
        .invoke_static(
            "java/lang/Boolean",
            "valueOf",
            "(Ljava/lang/String;)Ljava/lang/Boolean;",
            (null_string.clone(),),
        )
        .await?;
    assert!(!jvm.invoke_virtual::<_, bool>(&parsed, "booleanValue", "()Z", ()).await?);

    let from_null: ClassInstanceRef<Boolean> = jvm.new_class("java/lang/Boolean", "(Ljava/lang/String;)V", (null_string,)).await?.into();
    assert!(!jvm.invoke_virtual::<_, bool>(&from_null, "booleanValue", "()Z", ()).await?);

    let true_constant: ClassInstanceRef<Boolean> = jvm.get_static_field("java/lang/Boolean", "TRUE", "Ljava/lang/Boolean;").await?;
    let false_constant: ClassInstanceRef<Boolean> = jvm.get_static_field("java/lang/Boolean", "FALSE", "Ljava/lang/Boolean;").await?;
    assert!(jvm.invoke_virtual::<_, bool>(&true_constant, "booleanValue", "()Z", ()).await?);
    assert!(!jvm.invoke_virtual::<_, bool>(&false_constant, "booleanValue", "()Z", ()).await?);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&false_constant, "hashCode", "()I", ()).await?, 1237);
    assert!(
        jvm.invoke_virtual::<_, bool>(&true_constant, "equals", "(Ljava/lang/Object;)Z", (true_value,))
            .await?
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(&false_constant, "equals", "(Ljava/lang/Object;)Z", (None,))
            .await?
    );
    let object = jvm.new_class("java/lang/Object", "()V", ()).await?;
    assert!(
        !jvm.invoke_virtual::<_, bool>(&false_constant, "equals", "(Ljava/lang/Object;)Z", (object,))
            .await?
    );
    assert!(!jvm.is_instance(&**true_constant, "java/lang/Comparable"));
    assert!(jvm.is_instance(&**true_constant, "java/io/Serializable"));

    let typ = jvm.get_static_field("java/lang/Boolean", "TYPE", "Ljava/lang/Class;").await?;
    let name: ClassInstanceRef<String> = jvm.invoke_virtual(&typ, "getName", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &name).await?, "boolean");
    assert!(jvm.invoke_virtual::<_, bool>(&typ, "isPrimitive", "()Z", ()).await?);

    Ok(())
}

#[tokio::test]
async fn test_boolean_property_and_primitive_value_of_exclusion() -> Result<()> {
    let jvm = test_jvm().await?;

    let key = JavaLangString::from_rust_string(&jvm, "rustjava.boolean.test").await?;
    let value = JavaLangString::from_rust_string(&jvm, "TRUE").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/lang/System",
            "setProperty",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
            (key.clone(), value),
        )
        .await?;
    assert!(
        jvm.invoke_static::<_, bool>("java/lang/Boolean", "getBoolean", "(Ljava/lang/String;)Z", (key,))
            .await?
    );

    let missing = JavaLangString::from_rust_string(&jvm, "rustjava.boolean.missing").await?;
    assert!(
        !jvm.invoke_static::<_, bool>("java/lang/Boolean", "getBoolean", "(Ljava/lang/String;)Z", (missing,))
            .await?
    );

    assert!(
        !jvm.invoke_static::<_, bool>("java/lang/Boolean", "getBoolean", "(Ljava/lang/String;)Z", (None,))
            .await?
    );
    let false_key = JavaLangString::from_rust_string(&jvm, "rustjava.boolean.false").await?;
    let false_value = JavaLangString::from_rust_string(&jvm, "not-true").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/lang/System",
            "setProperty",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
            (false_key.clone(), false_value),
        )
        .await?;
    assert!(
        !jvm.invoke_static::<_, bool>("java/lang/Boolean", "getBoolean", "(Ljava/lang/String;)Z", (false_key,))
            .await?
    );

    let result: Result<ClassInstanceRef<Boolean>> = jvm.invoke_static("java/lang/Boolean", "valueOf", "(Z)Ljava/lang/Boolean;", (true,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Boolean.valueOf(boolean) must remain outside the Java 1.2 API");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NoSuchMethodError"));

    let left = jvm.new_class("java/lang/Boolean", "(Z)V", (false,)).await?;
    let right = jvm.new_class("java/lang/Boolean", "(Z)V", (true,)).await?;
    let result: Result<i32> = jvm.invoke_virtual(&left, "compareTo", "(Ljava/lang/Boolean;)I", (right.clone(),)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Boolean typed compareTo must remain outside the Java 1.2 API");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NoSuchMethodError"));
    let result: Result<i32> = jvm.invoke_virtual(&left, "compareTo", "(Ljava/lang/Object;)I", (right,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Boolean raw compareTo must remain outside the Java 1.2 API");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NoSuchMethodError"));

    Ok(())
}
