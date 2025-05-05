use java_runtime::classes::java::lang::Object;
use jvm::{ClassInstanceRef, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_stack_push_pop() -> Result<()> {
    let jvm = test_jvm().await?;

    let stack = jvm.new_class("java/util/Stack", "()V", ()).await?;

    let element1 = JavaLangString::from_rust_string(&jvm, "testValue1").await?;
    let element2 = JavaLangString::from_rust_string(&jvm, "testValue2").await?;

    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&stack, "push", "(Ljava/lang/Object;)Ljava/lang/Object;", (element1.clone(),))
        .await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&stack, "push", "(Ljava/lang/Object;)Ljava/lang/Object;", (element2.clone(),))
        .await?;

    let size: i32 = jvm.invoke_virtual(&stack, "size", "()I", ()).await?;
    assert_eq!(size, 2);

    let popped: ClassInstanceRef<Object> = jvm.invoke_virtual(&stack, "pop", "()Ljava/lang/Object;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &popped).await?, "testValue2");

    let popped: ClassInstanceRef<Object> = jvm.invoke_virtual(&stack, "pop", "()Ljava/lang/Object;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &popped).await?, "testValue1");

    Ok(())
}

#[tokio::test]
async fn test_stack_peek() -> Result<()> {
    let jvm = test_jvm().await?;

    let stack = jvm.new_class("java/util/Stack", "()V", ()).await?;

    let element1 = JavaLangString::from_rust_string(&jvm, "testValue1").await?;
    let element2 = JavaLangString::from_rust_string(&jvm, "testValue2").await?;

    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&stack, "push", "(Ljava/lang/Object;)Ljava/lang/Object;", (element1.clone(),))
        .await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&stack, "push", "(Ljava/lang/Object;)Ljava/lang/Object;", (element2.clone(),))
        .await?;

    let size: i32 = jvm.invoke_virtual(&stack, "size", "()I", ()).await?;
    assert_eq!(size, 2);

    let peek: ClassInstanceRef<Object> = jvm.invoke_virtual(&stack, "peek", "()Ljava/lang/Object;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &peek).await?, "testValue2");

    let peek: ClassInstanceRef<Object> = jvm.invoke_virtual(&stack, "pop", "()Ljava/lang/Object;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &peek).await?, "testValue2");

    Ok(())
}

#[tokio::test]
async fn test_stack_search() -> Result<()> {
    let jvm = test_jvm().await?;

    let stack = jvm.new_class("java/util/Stack", "()V", ()).await?;

    let element1 = JavaLangString::from_rust_string(&jvm, "testValue1").await?;
    let element2 = JavaLangString::from_rust_string(&jvm, "testValue2").await?;
    let element3 = JavaLangString::from_rust_string(&jvm, "testValue3").await?;
    let element4 = JavaLangString::from_rust_string(&jvm, "testValue3").await?;

    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&stack, "push", "(Ljava/lang/Object;)Ljava/lang/Object;", (element1.clone(),))
        .await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&stack, "push", "(Ljava/lang/Object;)Ljava/lang/Object;", (element2.clone(),))
        .await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&stack, "push", "(Ljava/lang/Object;)Ljava/lang/Object;", (element3.clone(),))
        .await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&stack, "push", "(Ljava/lang/Object;)Ljava/lang/Object;", (element1.clone(),))
        .await?;

    let size: i32 = jvm.invoke_virtual(&stack, "size", "()I", ()).await?;
    assert_eq!(size, 4);

    let peek: i32 = jvm.invoke_virtual(&stack, "search", "(Ljava/lang/Object;)I", (element2.clone(),)).await?;
    assert_eq!(peek, 3);

    let peek: i32 = jvm.invoke_virtual(&stack, "search", "(Ljava/lang/Object;)I", (element1.clone(),)).await?;
    assert_eq!(peek, 1);

    let peek: i32 = jvm.invoke_virtual(&stack, "search", "(Ljava/lang/Object;)I", (element4.clone(),)).await?;
    assert_eq!(peek, -1);

    Ok(())
}
