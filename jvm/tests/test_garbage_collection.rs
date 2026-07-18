use jvm::{Array, ClassInstanceRef, JavaValue, Result as JvmResult, runtime::JavaLangString};

use std::collections::BTreeMap;

use test_utils::{TestRuntime, create_test_jvm, test_jvm};

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

    // Temporaries used in class loading should be garbage collected. The exact
    // count depends on the loaded type hierarchy and interface graph.
    assert!(garbage_count > 0);
    let remaining_garbage_count = jvm.collect_garbage()?;
    assert_eq!(remaining_garbage_count, 0);

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

#[tokio::test]
async fn global_references_are_independent_garbage_collection_roots() -> JvmResult<()> {
    let jvm = test_jvm().await?;

    jvm.collect_garbage()?;

    struct Object;

    jvm.push_native_frame();
    let object: ClassInstanceRef<Object> = jvm.new_class("java/lang/Object", "()V", ()).await?.into();
    let first = jvm.new_global_ref(&object).unwrap();
    let second = jvm.new_global_ref(&object).unwrap();
    jvm.pop_frame();

    assert_eq!(jvm.collect_garbage()?, 0);
    drop(first);
    assert_eq!(jvm.collect_garbage()?, 0);
    drop(second);
    assert_eq!(jvm.collect_garbage()?, 1);

    let null: ClassInstanceRef<Object> = None.into();
    assert!(jvm.new_global_ref(&null).is_none());

    Ok(())
}

#[tokio::test]
async fn array_load_result_is_a_local_reference() -> JvmResult<()> {
    let jvm = test_jvm().await?;

    struct Object;

    jvm.push_native_frame();
    let _: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 0).await?.into();
    jvm.pop_frame();
    jvm.collect_garbage()?;

    jvm.push_native_frame();
    let mut array: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 1).await?.into();
    let object: ClassInstanceRef<Object> = jvm.new_class("java/lang/Object", "()V", ()).await?.into();
    jvm.store_array(&mut array, 0, [object]).await?;
    let array = jvm.new_global_ref(&array).unwrap();
    jvm.pop_frame();

    assert_eq!(jvm.collect_garbage()?, 0);

    jvm.push_native_frame();
    let _: Vec<ClassInstanceRef<Object>> = jvm.load_array(&array, 0, 1).await?;
    let mut mutable_array = (*array).clone();
    jvm.store_array(&mut mutable_array, 0, [ClassInstanceRef::<Object>::new(None)]).await?;
    assert_eq!(jvm.collect_garbage()?, 0);
    jvm.pop_frame();

    assert_eq!(jvm.collect_garbage()?, 1);
    drop(array);
    assert_eq!(jvm.collect_garbage()?, 1);

    Ok(())
}

#[tokio::test]
async fn field_and_method_results_are_local_references() -> JvmResult<()> {
    let jvm = test_jvm().await?;

    struct Object;
    struct Vector;

    jvm.push_native_frame();
    let _: ClassInstanceRef<Vector> = jvm.new_class("java/util/Vector", "()V", ()).await?.into();
    jvm.pop_frame();
    jvm.collect_garbage()?;

    jvm.push_native_frame();
    let vector: ClassInstanceRef<Vector> = jvm.new_class("java/util/Vector", "(I)V", (1,)).await?.into();
    let object: ClassInstanceRef<Object> = jvm.new_class("java/lang/Object", "()V", ()).await?.into();
    let _: () = jvm.invoke_virtual(&vector, "addElement", "(Ljava/lang/Object;)V", (object,)).await?;
    let vector = jvm.new_global_ref(&vector).unwrap();
    jvm.pop_frame();

    assert_eq!(jvm.collect_garbage()?, 0);

    jvm.push_native_frame();
    let _: ClassInstanceRef<Array<Object>> = jvm.get_field(&vector, "elementData", "[Ljava/lang/Object;").await?;
    let mut mutable_vector = (*vector).clone();
    jvm.put_field(
        &mut mutable_vector,
        "elementData",
        "[Ljava/lang/Object;",
        ClassInstanceRef::<Array<Object>>::new(None),
    )
    .await?;
    assert_eq!(jvm.collect_garbage()?, 0);
    jvm.pop_frame();

    assert_eq!(jvm.collect_garbage()?, 2);
    drop(vector);
    assert_eq!(jvm.collect_garbage()?, 1);

    jvm.push_native_frame();
    let vector: ClassInstanceRef<Vector> = jvm.new_class("java/util/Vector", "(I)V", (1,)).await?.into();
    let object: ClassInstanceRef<Object> = jvm.new_class("java/lang/Object", "()V", ()).await?.into();
    let _: () = jvm.invoke_virtual(&vector, "addElement", "(Ljava/lang/Object;)V", (object,)).await?;
    let vector = jvm.new_global_ref(&vector).unwrap();
    jvm.pop_frame();

    jvm.push_native_frame();
    let _: ClassInstanceRef<Object> = jvm.invoke_virtual(&vector, "remove", "(I)Ljava/lang/Object;", (0,)).await?;
    assert_eq!(jvm.collect_garbage()?, 0);
    jvm.pop_frame();

    assert_eq!(jvm.collect_garbage()?, 1);
    drop(vector);
    assert_eq!(jvm.collect_garbage()?, 2);

    Ok(())
}

#[tokio::test]
async fn static_field_result_is_a_local_reference() -> JvmResult<()> {
    let jvm = test_jvm().await?;

    struct Object;

    jvm.collect_garbage()?;

    jvm.push_native_frame();
    let _: ClassInstanceRef<Object> = jvm.get_static_field("java/lang/System", "out", "Ljava/io/PrintStream;").await?;
    jvm.put_static_field("java/lang/System", "out", "Ljava/io/PrintStream;", ClassInstanceRef::<Object>::new(None))
        .await?;
    assert_eq!(jvm.collect_garbage()?, 0);
    jvm.pop_frame();

    assert!(jvm.collect_garbage()? > 0);

    Ok(())
}

#[tokio::test]
async fn returned_exception_is_a_local_reference() -> JvmResult<()> {
    let jvm = test_jvm().await?;

    struct Vector;

    jvm.push_native_frame();
    let vector: ClassInstanceRef<Vector> = jvm.new_class("java/util/Vector", "()V", ()).await?.into();
    let _: jvm::JavaError = jvm
        .invoke_virtual::<_, ClassInstanceRef<()>>(&vector, "elementAt", "(I)Ljava/lang/Object;", (0,))
        .await
        .unwrap_err();
    jvm.pop_frame();
    jvm.collect_garbage()?;

    jvm.push_native_frame();
    let vector: ClassInstanceRef<Vector> = jvm.new_class("java/util/Vector", "()V", ()).await?.into();
    let _: jvm::JavaError = jvm
        .invoke_virtual::<_, ClassInstanceRef<()>>(&vector, "elementAt", "(I)Ljava/lang/Object;", (0,))
        .await
        .unwrap_err();

    assert_eq!(jvm.collect_garbage()?, 2);
    assert_eq!(jvm.collect_garbage()?, 0);
    jvm.pop_frame();
    assert_eq!(jvm.collect_garbage()?, 8);

    Ok(())
}

#[tokio::test]
async fn thread_start_keeps_the_thread_alive_until_spawn_callback_runs() -> JvmResult<()> {
    let runtime = TestRuntime::new_with_queued_spawns(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;

    jvm.collect_garbage()?;

    jvm.push_native_frame();
    let thread = jvm.new_class("java/lang/Thread", "()V", ()).await?;
    let _: () = jvm.invoke_virtual(&thread, "start", "()V", ()).await?;
    jvm.pop_frame();

    assert_eq!(jvm.collect_garbage()?, 1);
    assert_eq!(jvm.collect_garbage()?, 0);

    drop(runtime.take_spawn_callback().unwrap());
    assert_eq!(jvm.collect_garbage()?, 3);

    Ok(())
}
