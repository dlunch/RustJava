use jvm::{Array, ClassInstance, ClassInstanceRef, ExternalRoots, JavaError, JavaValue, Jvm, runtime::JavaLangString};

use std::{
    collections::BTreeMap,
    future::{Future, poll_fn},
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
    task::Poll,
};

use test_utils::{TestRuntime, create_test_jvm, get_bootstrap_class_loader, test_jvm};

use anyhow::Result as JvmResult;

#[tokio::test]
async fn external_roots_trace_children_and_failed_discovery_does_not_sweep() -> JvmResult<()> {
    struct Roots(Arc<Mutex<Option<Vec<usize>>>>);

    impl ExternalRoots for Roots {
        fn roots(&self, objects: &[&dyn ClassInstance]) -> anyhow::Result<Vec<usize>> {
            let state = self.0.lock().unwrap();
            let roots = state.as_ref().ok_or_else(|| anyhow::anyhow!("root memory unreadable"))?;
            for identity in roots {
                assert!(objects.iter().any(|object| object.identity() == *identity));
            }
            Ok(roots.clone())
        }
    }

    let roots = Arc::new(Mutex::new(Some(Vec::new())));
    let jvm = Jvm::new(
        get_bootstrap_class_loader(Box::new(TestRuntime::new(BTreeMap::new()))),
        || 0,
        BTreeMap::new(),
        Some(Box::new(Roots(roots.clone()))),
    )
    .await?;
    jvm.collect_garbage()?;

    let string = {
        let _frame = jvm.push_native_frame(&[]);
        let string = JavaLangString::from_rust_string(&jvm, "external").await?;
        jvm.new_class("java/lang/Object", "()V", ()).await?;
        *roots.lock().unwrap() = Some(vec![string.identity()]);
        string
    };

    *roots.lock().unwrap() = None;
    assert_eq!(jvm.collect_garbage().unwrap_err().to_string(), "root memory unreadable");
    *roots.lock().unwrap() = Some(vec![string.identity()]);
    assert_eq!(jvm.collect_garbage()?, 1);
    {
        let _frame = jvm.push_native_frame(&[]);
        assert_eq!(JavaLangString::to_rust_string(&jvm, &string).await?, "external");
    }
    *roots.lock().unwrap() = Some(Vec::new());
    assert_eq!(jvm.collect_garbage()?, 2);

    let _frame = jvm.push_native_frame(&[]);
    let runtime: Box<dyn ClassInstance> = jvm.invoke_static("java/lang/Runtime", "getRuntime", "()Ljava/lang/Runtime;", ()).await?;
    *roots.lock().unwrap() = None;
    for result in [
        jvm.invoke_static::<_, ()>("java/lang/System", "gc", "()V", ()).await,
        jvm.invoke_virtual::<_, ()>(&runtime, "java/lang/Runtime", "gc", "()V", ()).await,
    ] {
        let JavaError::JavaException(exception) = result.unwrap_err();
        assert!(jvm.is_instance(&*exception, "java/lang/InternalError"));
    }
    Ok(())
}

#[tokio::test]
async fn native_frames_seed_arguments_discard_temporaries_and_remember_their_thread() -> JvmResult<()> {
    let current_thread = Arc::new(AtomicU64::new(0));
    let jvm = Jvm::new(
        get_bootstrap_class_loader(Box::new(TestRuntime::new(BTreeMap::new()))),
        {
            let current_thread = current_thread.clone();
            move || current_thread.load(Ordering::Relaxed)
        },
        BTreeMap::new(),
        None,
    )
    .await?;
    current_thread.store(1, Ordering::Relaxed);
    jvm.attach_thread(None).await?;
    current_thread.store(0, Ordering::Relaxed);
    jvm.collect_garbage()?;

    let argument = {
        let _frame = jvm.push_native_frame(&[]);
        JavaLangString::from_rust_string(&jvm, "argument").await?
    };
    let frame = jvm.push_native_frame(&[JavaValue::from(argument), JavaValue::Int(7), JavaValue::Object(None)]);
    jvm.new_class("java/lang/Object", "()V", ()).await?;
    tokio::task::yield_now().await;
    assert_eq!(jvm.collect_garbage()?, 0);

    current_thread.store(1, Ordering::Relaxed);
    let other_frame = jvm.push_native_frame(&[]);
    jvm.new_class("java/lang/Object", "()V", ()).await?;
    drop(frame);
    assert_eq!(jvm.collect_garbage()?, 3);
    drop(other_frame);
    assert_eq!(jvm.collect_garbage()?, 1);
    Ok(())
}

#[tokio::test]
async fn active_monitors_are_roots_until_release_including_wait_reacquisition() -> JvmResult<()> {
    let jvm = test_jvm().await?;
    jvm.collect_garbage()?;
    let object = {
        let _frame = jvm.push_native_frame(&[]);
        JavaLangString::from_rust_string(&jvm, "monitor").await?
    };
    jvm.monitor_enter(&object).await?;
    assert_eq!(jvm.collect_garbage()?, 0);
    let (wait, timeout) = jvm.object_wait_prepare(&object).await?;
    assert_eq!(jvm.collect_garbage()?, 0);
    timeout.clone().notify();
    assert_eq!(jvm.collect_garbage()?, 0);
    jvm.object_wait(wait).await?;
    assert_eq!(jvm.collect_garbage()?, 0);
    jvm.monitor_exit(&object).await?;
    assert_eq!(jvm.collect_garbage()?, 2);
    timeout.notify();
    Ok(())
}

#[tokio::test]
async fn synchronized_entry_roots_arguments_while_waiting_for_the_monitor() -> JvmResult<()> {
    let current_thread = Arc::new(AtomicU64::new(0));
    let jvm = Jvm::new(
        get_bootstrap_class_loader(Box::new(TestRuntime::new(BTreeMap::new()))),
        {
            let current_thread = current_thread.clone();
            move || current_thread.load(Ordering::Relaxed)
        },
        BTreeMap::new(),
        None,
    )
    .await?;
    current_thread.store(1, Ordering::Relaxed);
    jvm.attach_thread(None).await?;
    current_thread.store(0, Ordering::Relaxed);
    let vector = {
        let _frame = jvm.push_native_frame(&[]);
        let vector = jvm.new_class("java/util/Vector", "(I)V", (1,)).await?;
        jvm.monitor_enter(&vector).await?;
        vector
    };
    jvm.collect_garbage()?;
    let argument = {
        let _frame = jvm.push_native_frame(&[]);
        JavaLangString::from_rust_string(&jvm, "pending argument").await?
    };
    current_thread.store(1, Ordering::Relaxed);
    let mut call = Box::pin(jvm.invoke_virtual::<_, ()>(&vector, "java/util/Vector", "addElement", "(Ljava/lang/Object;)V", (argument,)));
    assert!(poll_fn(|context| Poll::Ready(call.as_mut().poll(context).is_pending())).await);

    current_thread.store(0, Ordering::Relaxed);
    assert_eq!(jvm.collect_garbage()?, 0);
    jvm.monitor_exit(&vector).await?;
    assert_eq!(jvm.collect_garbage()?, 0);
    current_thread.store(1, Ordering::Relaxed);
    call.await?;
    {
        let _frame = jvm.push_native_frame(&[]);
        let element: Box<dyn ClassInstance> = jvm
            .invoke_virtual(&vector, "java/util/Vector", "elementAt", "(I)Ljava/lang/Object;", (0,))
            .await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &element).await?, "pending argument");
    }
    assert_eq!(jvm.collect_garbage()?, 4);
    Ok(())
}

#[tokio::test]
async fn test_garbage_collection() -> JvmResult<()> {
    let jvm = test_jvm().await?;

    // collect garbage before test to ensure a clean state
    jvm.collect_garbage()?;

    let frame = jvm.push_native_frame(&[]);
    let _string = JavaLangString::from_rust_string(&jvm, "test").await?;

    let garbage_count = jvm.collect_garbage()?;
    assert_eq!(garbage_count, 0);

    drop(frame);

    let garbage_count = jvm.collect_garbage()?;

    // java/lang/String and its internal [C should be garbage collected
    assert_eq!(garbage_count, 2);

    // load a class
    let frame = jvm.push_native_frame(&[]);
    let _ = jvm.resolve_class("java/util/Random").await?;
    drop(frame);

    let garbage_count = jvm.collect_garbage()?;

    assert_eq!(garbage_count, 2);

    // use loaded class
    let frame = jvm.push_native_frame(&[]);
    let _random = jvm.new_class("java/util/Random", "()V", ()).await?;
    drop(frame);

    let garbage_count = jvm.collect_garbage()?;
    assert_eq!(garbage_count, 1);

    // load another class
    let frame = jvm.push_native_frame(&[]);
    let _ = jvm.resolve_class("java/util/Vector").await?;
    drop(frame);

    let garbage_count = jvm.collect_garbage()?;

    // Temporaries used in class loading should be garbage collected. The exact
    // count depends on the loaded type hierarchy and interface graph.
    assert!(garbage_count > 0);
    let remaining_garbage_count = jvm.collect_garbage()?;
    assert_eq!(remaining_garbage_count, 0);

    // use loaded class
    let frame = jvm.push_native_frame(&[]);
    let vector = jvm.new_class("java/util/Vector", "()V", ()).await?;
    let _: () = jvm
        .invoke_virtual(
            &vector,
            &vector.class_definition().name(),
            "addElement",
            "(Ljava/lang/Object;)V",
            (JavaLangString::from_rust_string(&jvm, "test").await?,),
        )
        .await?;

    let garbage_count = jvm.collect_garbage()?;
    assert!(garbage_count > 0);
    assert_eq!(jvm.collect_garbage()?, 0);

    drop(frame);

    // vector, elementData, string, and its internal [C should be garbage collected
    let garbage_count = jvm.collect_garbage()?;
    assert_eq!(garbage_count, 4);

    Ok(())
}

#[tokio::test]
async fn test_garbage_collection_hashtable() -> JvmResult<()> {
    let jvm = test_jvm().await?;

    // collect garbage before test to ensure a clean state
    jvm.collect_garbage()?;

    let frame = jvm.push_native_frame(&[]);
    let hashtable = jvm.new_class("java/util/Hashtable", "()V", ()).await?;

    let key = JavaLangString::from_rust_string(&jvm, "key").await?;
    let value = JavaLangString::from_rust_string(&jvm, "value").await?;
    let _: JavaValue = jvm
        .invoke_virtual(
            &hashtable,
            &hashtable.class_definition().name(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (key.clone(), value.clone()),
        )
        .await
        .unwrap();

    // all objects reachable through hashtable on the frame
    let garbage_count = jvm.collect_garbage()?;
    assert_eq!(garbage_count, 0);

    drop(frame);

    // hashtable, table array, entry, key string, key [C, value string, value [C
    let garbage_count = jvm.collect_garbage()?;
    assert_eq!(garbage_count, 7);

    Ok(())
}

#[tokio::test]
async fn substring_keeps_shared_array_alive_after_parent_collection() -> JvmResult<()> {
    let jvm = test_jvm().await?;

    jvm.collect_garbage()?;

    struct String;

    let frame = jvm.push_native_frame(&[]);
    let parent = JavaLangString::from_rust_string(&jvm, "HelloWorld").await?;
    let child: ClassInstanceRef<String> = jvm
        .invoke_virtual(&parent, &parent.class_definition().name(), "substring", "(II)Ljava/lang/String;", (2, 5))
        .await?;
    let child = jvm.new_global_ref(&child).unwrap();
    drop(frame);

    // only the parent String is collected; the shared [C stays reachable through the child's value field
    assert_eq!(jvm.collect_garbage()?, 1);

    let frame = jvm.push_native_frame(&[]);
    assert_eq!(JavaLangString::to_rust_string(&jvm, &child).await?, "llo");
    drop(frame);
    jvm.collect_garbage()?;

    drop(child);
    assert_eq!(jvm.collect_garbage()?, 2);

    Ok(())
}

#[tokio::test]
async fn global_references_are_independent_garbage_collection_roots() -> JvmResult<()> {
    let jvm = test_jvm().await?;

    let frame = jvm.push_native_frame(&[]);
    jvm.resolve_class("[Ljava/lang/Object;").await?;
    drop(frame);
    jvm.collect_garbage()?;

    struct Object;

    let frame = jvm.push_native_frame(&[]);
    let mut object: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 2).await?.into();
    let references = [object.clone(), object.clone()];
    jvm.store_array(&mut object, 0, references).await?;
    let first = jvm.new_global_ref(&object).unwrap();
    let second = jvm.new_global_ref(&object).unwrap();
    drop(frame);

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

    let frame = jvm.push_native_frame(&[]);
    let _: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 0).await?.into();
    drop(frame);
    jvm.collect_garbage()?;

    let frame = jvm.push_native_frame(&[]);
    let mut array: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 1).await?.into();
    let object: ClassInstanceRef<Object> = jvm.new_class("java/lang/Object", "()V", ()).await?.into();
    jvm.store_array(&mut array, 0, [object]).await?;
    let array = jvm.new_global_ref(&array).unwrap();
    drop(frame);

    assert_eq!(jvm.collect_garbage()?, 0);

    let frame = jvm.push_native_frame(&[]);
    let _: Vec<ClassInstanceRef<Object>> = jvm.load_array(&array, 0, 1).await?;
    let mut mutable_array = (*array).clone();
    jvm.store_array(&mut mutable_array, 0, [ClassInstanceRef::<Object>::new(None)]).await?;
    assert_eq!(jvm.collect_garbage()?, 0);
    drop(frame);

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

    let frame = jvm.push_native_frame(&[]);
    let _: ClassInstanceRef<Vector> = jvm.new_class("java/util/Vector", "()V", ()).await?.into();
    drop(frame);
    jvm.collect_garbage()?;

    let frame = jvm.push_native_frame(&[]);
    let vector: ClassInstanceRef<Vector> = jvm.new_class("java/util/Vector", "(I)V", (1,)).await?.into();
    let object: ClassInstanceRef<Object> = jvm.new_class("java/lang/Object", "()V", ()).await?.into();
    let _: () = jvm
        .invoke_virtual(&vector, "java/util/Vector", "addElement", "(Ljava/lang/Object;)V", (object,))
        .await?;
    let vector = jvm.new_global_ref(&vector).unwrap();
    drop(frame);

    assert_eq!(jvm.collect_garbage()?, 0);

    let frame = jvm.push_native_frame(&[]);
    let _: ClassInstanceRef<Array<Object>> = jvm.get_field(&vector, "java/util/Vector", "elementData", "[Ljava/lang/Object;").await?;
    let mut mutable_vector = (*vector).clone();
    jvm.put_field(
        &mut mutable_vector,
        "java/util/Vector",
        "elementData",
        "[Ljava/lang/Object;",
        ClassInstanceRef::<Array<Object>>::new(None),
    )
    .await?;
    assert_eq!(jvm.collect_garbage()?, 0);
    drop(frame);

    assert_eq!(jvm.collect_garbage()?, 2);
    drop(vector);
    assert_eq!(jvm.collect_garbage()?, 1);

    let frame = jvm.push_native_frame(&[]);
    let vector: ClassInstanceRef<Vector> = jvm.new_class("java/util/Vector", "(I)V", (1,)).await?.into();
    let object: ClassInstanceRef<Object> = jvm.new_class("java/lang/Object", "()V", ()).await?.into();
    let _: () = jvm
        .invoke_virtual(&vector, "java/util/Vector", "addElement", "(Ljava/lang/Object;)V", (object,))
        .await?;
    let vector = jvm.new_global_ref(&vector).unwrap();
    drop(frame);

    let frame = jvm.push_native_frame(&[]);
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&vector, "java/util/Vector", "remove", "(I)Ljava/lang/Object;", (0,))
        .await?;
    assert_eq!(jvm.collect_garbage()?, 0);
    drop(frame);

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

    let frame = jvm.push_native_frame(&[]);
    let _: ClassInstanceRef<Object> = jvm.get_static_field("java/lang/System", "out", "Ljava/io/PrintStream;").await?;
    jvm.put_static_field("java/lang/System", "out", "Ljava/io/PrintStream;", ClassInstanceRef::<Object>::new(None))
        .await?;
    assert_eq!(jvm.collect_garbage()?, 0);
    drop(frame);

    assert!(jvm.collect_garbage()? > 0);

    Ok(())
}

#[tokio::test]
async fn returned_exception_is_a_local_reference() -> JvmResult<()> {
    let jvm = test_jvm().await?;

    struct Vector;

    let frame = jvm.push_native_frame(&[]);
    let vector: ClassInstanceRef<Vector> = jvm.new_class("java/util/Vector", "()V", ()).await?.into();
    let _: jvm::JavaError = jvm
        .invoke_virtual::<_, ClassInstanceRef<()>>(&vector, "java/util/Vector", "elementAt", "(I)Ljava/lang/Object;", (0,))
        .await
        .unwrap_err();
    drop(frame);
    jvm.collect_garbage()?;

    let frame = jvm.push_native_frame(&[]);
    let vector: ClassInstanceRef<Vector> = jvm.new_class("java/util/Vector", "()V", ()).await?.into();
    let _: jvm::JavaError = jvm
        .invoke_virtual::<_, ClassInstanceRef<()>>(&vector, "java/util/Vector", "elementAt", "(I)Ljava/lang/Object;", (0,))
        .await
        .unwrap_err();

    assert_eq!(jvm.collect_garbage()?, 0);
    drop(frame);
    assert_eq!(jvm.collect_garbage()?, 8);

    Ok(())
}

#[tokio::test]
async fn thread_start_keeps_the_thread_alive_until_spawn_callback_runs() -> JvmResult<()> {
    let runtime = TestRuntime::new_with_queued_spawns(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;

    jvm.collect_garbage()?;

    let frame = jvm.push_native_frame(&[]);
    let thread = jvm.new_class("java/lang/Thread", "()V", ()).await?;
    let _: () = jvm.invoke_virtual(&thread, &thread.class_definition().name(), "start", "()V", ()).await?;
    drop(frame);

    assert_eq!(jvm.collect_garbage()?, 0);

    drop(runtime.take_spawn_callback().unwrap());
    assert_eq!(jvm.collect_garbage()?, 3);

    Ok(())
}
