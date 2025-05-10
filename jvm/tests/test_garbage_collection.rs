use jvm::{Result as JvmResult, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_garbage_collection() -> JvmResult<()> {
    let jvm = test_jvm().await?;

    // collect garbage before test to ensure a clean state
    jvm.collect_garbage()?;

    jvm.push_native_frame();
    let _string = JavaLangString::from_rust_string(&jvm, "test").await?;

    let garbage_count = jvm.collect_garbage()?;
    assert_eq!(garbage_count, 0);

    jvm.pop_frame();

    let garbage_count = jvm.collect_garbage()?;

    // java/lang/String, its internal [C, and [C used in creation should be garbage collected
    assert_eq!(garbage_count, 3);

    // load a class
    jvm.push_native_frame();
    let _ = jvm.resolve_class("java/util/Random").await?;
    jvm.pop_frame();

    let garbage_count = jvm.collect_garbage()?;

    assert_eq!(garbage_count, 3);

    // use loaded class
    jvm.push_native_frame();
    let _random = jvm.new_class("java/util/Random", "()V", ()).await?;
    jvm.pop_frame();

    let garbage_count = jvm.collect_garbage()?;
    assert_eq!(garbage_count, 1);

    // load another class
    jvm.push_native_frame();
    let _ = jvm.resolve_class("java/util/Vector").await?;
    jvm.pop_frame();

    let garbage_count = jvm.collect_garbage()?;

    // temporaries used in class loading should be garbage collected
    assert_eq!(garbage_count, 9);

    // use loaded class
    jvm.push_native_frame();
    let vector = jvm.new_class("java/util/Vector", "()V", ()).await?;
    let _: () = jvm
        .invoke_virtual(
            &vector,
            "addElement",
            "(Ljava/lang/Object;)V",
            (JavaLangString::from_rust_string(&jvm, "test").await?,),
        )
        .await?;

    // vector and string should not be garbage collected
    let garbage_count = jvm.collect_garbage()?;
    assert_eq!(garbage_count, 0);

    jvm.pop_frame();

    // vector, string, and its internal [C should be garbage collected
    let garbage_count = jvm.collect_garbage()?;
    assert_eq!(garbage_count, 5);

    Ok(())
}
