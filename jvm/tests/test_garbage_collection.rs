use jvm::{JavaValue, Result as JvmResult, runtime::JavaLangString};

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

    let garbage_count = jvm.collect_garbage()?;
    assert_eq!(garbage_count, 3);

    jvm.pop_frame();

    // vector, elementData, string, and its internal [C should be garbage collected
    let garbage_count = jvm.collect_garbage()?;
    assert_eq!(garbage_count, 5);

    Ok(())
}

#[tokio::test]
async fn test_garbage_collection_hashtable() -> JvmResult<()> {
    let jvm = test_jvm().await?;

    // collect garbage before test to ensure a clean state
    jvm.collect_garbage()?;

    jvm.push_native_frame();
    let hashtable = jvm.new_class("java/util/Hashtable", "()V", ()).await?;

    let key = JavaLangString::from_rust_string(&jvm, "key").await?;
    let value = JavaLangString::from_rust_string(&jvm, "value").await?;
    let _: JavaValue = jvm
        .invoke_virtual(
            &hashtable,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (key.clone(), value.clone()),
        )
        .await
        .unwrap();

    // all objects reachable through hashtable on the frame
    let garbage_count = jvm.collect_garbage()?;
    assert_eq!(garbage_count, 0);

    jvm.pop_frame();

    // hashtable, table array, entry, key string, key [C, value string, value [C, and 2 temporaries from string construction
    let garbage_count = jvm.collect_garbage()?;
    assert_eq!(garbage_count, 9);

    Ok(())
}
