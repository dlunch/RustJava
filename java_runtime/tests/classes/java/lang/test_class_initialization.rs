use alloc::{boxed::Box, collections::BTreeMap, sync::Arc, vec};
use core::{
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};
use std::sync::Mutex;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use java_runtime::{Runtime, RuntimeClassProto, RuntimeContext, SpawnCallback};
use jvm::{JavaError, Jvm, Result};
use jvm_rust::ClassDefinitionImpl;
use test_utils::{TestRuntime, create_test_jvm};

struct ConcurrentInitialization;

static FAILING_INITIALIZATION_CALLS: AtomicUsize = AtomicUsize::new(0);

impl ConcurrentInitialization {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "ConcurrentInitialization",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC)],
            fields: vec![
                JavaFieldProto::new("count", "I", FieldAccessFlags::STATIC),
                JavaFieldProto::new("value", "I", FieldAccessFlags::STATIC),
            ],
            access_flags: Default::default(),
        }
    }

    async fn clinit(jvm: &Jvm, context: &mut RuntimeContext) -> Result<()> {
        let count: i32 = jvm.get_static_field("ConcurrentInitialization", "count", "I").await?;
        jvm.put_static_field("ConcurrentInitialization", "count", "I", count + 1).await?;
        context.sleep(Duration::from_millis(50)).await;
        jvm.put_static_field("ConcurrentInitialization", "value", "I", 42i32).await
    }
}

struct RecursiveInitialization;

impl RecursiveInitialization {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "RecursiveInitialization",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC)],
            fields: vec![
                JavaFieldProto::new("count", "I", FieldAccessFlags::STATIC),
                JavaFieldProto::new("value", "I", FieldAccessFlags::STATIC),
            ],
            access_flags: Default::default(),
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let count: i32 = jvm.get_static_field("RecursiveInitialization", "count", "I").await?;
        jvm.put_static_field("RecursiveInitialization", "count", "I", count + 1).await?;
        jvm.put_static_field("RecursiveInitialization", "value", "I", 7i32).await
    }
}

struct FailingInitialization;

impl FailingInitialization {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "FailingInitialization",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC)],
            fields: vec![JavaFieldProto::new("value", "I", FieldAccessFlags::STATIC)],
            access_flags: Default::default(),
        }
    }

    async fn clinit(jvm: &Jvm, context: &mut RuntimeContext) -> Result<()> {
        FAILING_INITIALIZATION_CALLS.fetch_add(1, Ordering::SeqCst);
        context.sleep(Duration::from_millis(50)).await;
        Err(jvm.exception("java/lang/IllegalArgumentException", "initialization failed").await)
    }
}

#[tokio::test]
async fn class_initialization_waits_for_the_owner_thread() -> Result<()> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;
    jvm.register_class(
        Box::new(ClassDefinitionImpl::from_class_proto(
            ConcurrentInitialization::as_proto(),
            Box::new(runtime.clone()) as Box<_>,
        )),
        None,
    )
    .await?;

    struct ReadValue {
        jvm: Jvm,
        completed: Arc<AtomicUsize>,
        failures: Arc<AtomicUsize>,
        values: Arc<Mutex<alloc::vec::Vec<i32>>>,
    }

    #[async_trait::async_trait]
    impl SpawnCallback for ReadValue {
        async fn call(&self) -> Result<()> {
            self.jvm.attach_thread(None).await?;
            match self.jvm.get_static_field("ConcurrentInitialization", "value", "I").await {
                Ok(value) => self.values.lock().unwrap().push(value),
                Err(_) => {
                    self.failures.fetch_add(1, Ordering::SeqCst);
                }
            }
            self.jvm.detach_thread()?;
            self.completed.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    let completed = Arc::new(AtomicUsize::new(0));
    let failures = Arc::new(AtomicUsize::new(0));
    let values = Arc::new(Mutex::new(alloc::vec::Vec::new()));
    for _ in 0..2 {
        runtime.spawn(
            &jvm,
            Box::new(ReadValue {
                jvm: jvm.clone(),
                completed: completed.clone(),
                failures: failures.clone(),
                values: values.clone(),
            }),
        );
    }

    for _ in 0..100 {
        if completed.load(Ordering::SeqCst) == 2 {
            break;
        }
        runtime.sleep(Duration::from_millis(5)).await;
    }

    assert_eq!(completed.load(Ordering::SeqCst), 2);
    assert_eq!(failures.load(Ordering::SeqCst), 0);
    assert_eq!(*values.lock().unwrap(), [42, 42]);
    assert_eq!(jvm.get_static_field::<i32>("ConcurrentInitialization", "count", "I").await?, 1);

    Ok(())
}

#[tokio::test]
async fn class_initialization_allows_same_thread_recursion() -> Result<()> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;
    jvm.register_class(
        Box::new(ClassDefinitionImpl::from_class_proto(
            RecursiveInitialization::as_proto(),
            Box::new(runtime) as Box<_>,
        )),
        None,
    )
    .await?;

    assert_eq!(jvm.get_static_field::<i32>("RecursiveInitialization", "value", "I").await?, 7);
    assert_eq!(jvm.get_static_field::<i32>("RecursiveInitialization", "count", "I").await?, 1);

    Ok(())
}

#[tokio::test]
async fn failed_class_initialization_wakes_waiters_and_becomes_erroneous() -> Result<()> {
    FAILING_INITIALIZATION_CALLS.store(0, Ordering::SeqCst);
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;
    jvm.register_class(
        Box::new(ClassDefinitionImpl::from_class_proto(
            FailingInitialization::as_proto(),
            Box::new(runtime.clone()) as Box<_>,
        )),
        None,
    )
    .await?;

    struct ReadFailingValue {
        jvm: Jvm,
        completed: Arc<AtomicUsize>,
        errors: Arc<Mutex<alloc::vec::Vec<alloc::string::String>>>,
    }

    #[async_trait::async_trait]
    impl SpawnCallback for ReadFailingValue {
        async fn call(&self) -> Result<()> {
            self.jvm.attach_thread(None).await?;
            if let Err(JavaError::JavaException(exception)) = self.jvm.get_static_field::<i32>("FailingInitialization", "value", "I").await {
                self.errors.lock().unwrap().push(exception.class_definition().name());
            }
            self.jvm.detach_thread()?;
            self.completed.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    let completed = Arc::new(AtomicUsize::new(0));
    let errors = Arc::new(Mutex::new(alloc::vec::Vec::new()));
    for _ in 0..2 {
        runtime.spawn(
            &jvm,
            Box::new(ReadFailingValue {
                jvm: jvm.clone(),
                completed: completed.clone(),
                errors: errors.clone(),
            }),
        );
    }

    for _ in 0..100 {
        if completed.load(Ordering::SeqCst) == 2 {
            break;
        }
        runtime.sleep(Duration::from_millis(5)).await;
    }

    let mut errors = errors.lock().unwrap().clone();
    errors.sort();
    assert_eq!(completed.load(Ordering::SeqCst), 2);
    assert_eq!(FAILING_INITIALIZATION_CALLS.load(Ordering::SeqCst), 1);
    assert_eq!(errors, ["java/lang/ExceptionInInitializerError", "java/lang/NoClassDefFoundError"]);

    let result = jvm.get_static_field::<i32>("FailingInitialization", "value", "I").await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("an erroneous class must remain erroneous");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NoClassDefFoundError"));

    Ok(())
}
