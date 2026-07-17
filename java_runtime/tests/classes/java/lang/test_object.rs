use core::{
    panic,
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

use alloc::{boxed::Box, collections::btree_map::BTreeMap, sync::Arc, vec};

use java_class_proto::JavaFieldProto;
use java_runtime::{Runtime, RuntimeClassProto, SpawnCallback, classes::java::lang::Object};
use jvm::{Array, ClassInstanceRef, JavaError, Jvm, Result};
use jvm_rust::ClassDefinitionImpl;

use test_utils::{TestRuntime, create_test_jvm};

struct CloneableObject;

impl CloneableObject {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "CloneableObject",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/lang/Cloneable"],
            methods: vec![],
            fields: vec![
                JavaFieldProto::new("value", "I", Default::default()),
                JavaFieldProto::new("reference", "Ljava/lang/Object;", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }
}

#[tokio::test]
async fn test_wait() -> Result<()> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;

    let notified = Arc::new(AtomicBool::new(false));

    let object = jvm.new_class("java/lang/Object", "()V", ()).await?;

    struct Notifier {
        jvm: Jvm,
        notified: Arc<AtomicBool>,
        runtime: TestRuntime,
        target: ClassInstanceRef<Object>,
    }

    #[async_trait::async_trait]
    impl SpawnCallback for Notifier {
        async fn call(&self) -> Result<()> {
            self.jvm.attach_thread(None).await?;

            self.runtime.sleep(Duration::from_millis(100)).await;
            self.notified.store(true, Ordering::Relaxed);
            self.jvm.monitor_enter(&self.target).await?;
            let _: () = self.jvm.invoke_virtual(&self.target, "notify", "()V", ()).await?;
            self.jvm.monitor_exit(&self.target).await?;

            self.jvm.detach_thread()?;

            Ok(())
        }
    }

    runtime.spawn(
        &jvm,
        Box::new(Notifier {
            jvm: jvm.clone(),
            notified: notified.clone(),
            runtime: runtime.clone(),
            target: object.clone().into(),
        }),
    );

    assert!(!notified.load(Ordering::Relaxed));
    jvm.monitor_enter(&object).await?;
    let _: () = jvm.invoke_virtual(&object, "wait", "()V", ()).await?;
    jvm.monitor_exit(&object).await?;
    assert!(notified.load(Ordering::Relaxed));

    Ok(())
}
#[tokio::test]
async fn test_wait_timeout() -> Result<()> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;

    let object = jvm.new_class("java/lang/Object", "()V", ()).await?;
    jvm.monitor_enter(&object).await?;
    let _: () = jvm.invoke_virtual(&object, "wait", "(J)V", (100i64,)).await?;
    jvm.monitor_exit(&object).await?;

    Ok(())
}

#[tokio::test]
async fn test_wait_and_notify_require_monitor_ownership() -> Result<()> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime).await?;
    let object = jvm.new_class("java/lang/Object", "()V", ()).await?;

    for result in [
        jvm.invoke_virtual::<_, ()>(&object, "notify", "()V", ()).await,
        jvm.invoke_virtual::<_, ()>(&object, "wait", "(J)V", (1i64,)).await,
    ] {
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("monitor ownership violation must throw IllegalMonitorStateException");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/IllegalMonitorStateException"));
    }

    Ok(())
}

#[tokio::test]
async fn test_clone_not_cloneable() -> Result<()> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;

    let object = jvm.new_class("java/lang/Object", "()V", ()).await?;

    let result: Result<ClassInstanceRef<Object>> = jvm.invoke_virtual(&object, "clone", "()Ljava/lang/Object;", ()).await;
    let Err(JavaError::JavaException(java_exception)) = result else {
        panic!("Expected JavaException, got {:?}", result);
    };

    let class_name = java_exception.class_definition().name();
    assert_eq!(class_name, "java/lang/CloneNotSupportedException");

    Ok(())
}

#[tokio::test]
async fn test_clone_creates_shallow_object_and_array_copies() -> Result<()> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;
    jvm.register_class(
        Box::new(ClassDefinitionImpl::from_class_proto(
            CloneableObject::as_proto(),
            Box::new(runtime) as Box<_>,
        )),
        None,
    )
    .await?;

    let mut original = jvm.instantiate_class("CloneableObject").await?;
    let reference = jvm.new_class("java/lang/Object", "()V", ()).await?;
    jvm.put_field(&mut original, "value", "I", 7i32).await?;
    jvm.put_field(&mut original, "reference", "Ljava/lang/Object;", reference.clone()).await?;

    let mut cloned: ClassInstanceRef<CloneableObject> = jvm.invoke_virtual(&original, "clone", "()Ljava/lang/Object;", ()).await?;
    assert_ne!(original.identity(), cloned.identity());
    assert_eq!(jvm.get_field::<i32>(&cloned, "value", "I").await?, 7);
    let cloned_reference = jvm
        .get_field::<ClassInstanceRef<Object>>(&cloned, "reference", "Ljava/lang/Object;")
        .await?;
    assert_eq!(reference.identity(), cloned_reference.identity());

    jvm.put_field(&mut cloned, "value", "I", 9i32).await?;
    assert_eq!(jvm.get_field::<i32>(&original, "value", "I").await?, 7);

    let mut array = jvm.instantiate_array("I", 2).await?;
    jvm.store_array(&mut array, 0, [1i32, 2i32]).await?;
    let mut cloned_array: ClassInstanceRef<Array<i32>> = jvm.invoke_virtual(&array, "clone", "()Ljava/lang/Object;", ()).await?;
    assert_ne!(array.identity(), cloned_array.identity());
    jvm.store_array(&mut cloned_array, 0, [9i32]).await?;
    assert_eq!(jvm.load_array::<i32>(&array, 0, 2).await?, [1, 2]);
    assert_eq!(jvm.load_array::<i32>(&cloned_array, 0, 2).await?, [9, 2]);

    let mut reference_array = jvm.instantiate_array("Ljava/lang/Object;", 1).await?;
    jvm.store_array(&mut reference_array, 0, [reference.clone()]).await?;
    let mut cloned_reference_array: ClassInstanceRef<Array<Object>> =
        jvm.invoke_virtual(&reference_array, "clone", "()Ljava/lang/Object;", ()).await?;
    let cloned_element = jvm.load_array::<ClassInstanceRef<Object>>(&cloned_reference_array, 0, 1).await?;
    assert_eq!(cloned_element[0].identity(), reference.identity());
    let replacement: ClassInstanceRef<Object> = jvm.new_class("java/lang/Object", "()V", ()).await?.into();
    jvm.store_array(&mut cloned_reference_array, 0, [replacement]).await?;
    let original_element = jvm.load_array::<ClassInstanceRef<Object>>(&reference_array, 0, 1).await?;
    assert_eq!(original_element[0].identity(), reference.identity());

    Ok(())
}

#[tokio::test]
async fn test_hash_code_is_stable_for_same_object() -> Result<()> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime).await?;

    let object = jvm.new_class("java/lang/Object", "()V", ()).await?;

    let first: i32 = jvm.invoke_virtual(&object, "hashCode", "()I", ()).await?;
    let second: i32 = jvm.invoke_virtual(&object, "hashCode", "()I", ()).await?;

    assert_eq!(first, second);

    Ok(())
}

#[tokio::test]
async fn test_hash_code_is_not_constant_across_objects() -> Result<()> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime).await?;

    let mut hashes = hashbrown::HashSet::new();

    for _ in 0..32 {
        let object = jvm.new_class("java/lang/Object", "()V", ()).await?;
        let hash: i32 = jvm.invoke_virtual(&object, "hashCode", "()I", ()).await?;
        hashes.insert(hash);
    }

    assert!(hashes.len() > 1);

    Ok(())
}
