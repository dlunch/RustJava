use core::{
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

use alloc::{boxed::Box, collections::btree_map::BTreeMap, sync::Arc};

use java_runtime::{Runtime, SpawnCallback, classes::java::lang::Object};
use jvm::{ClassInstanceRef, Jvm, Result};

use test_utils::{TestRuntime, create_test_jvm};

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
            self.jvm.attach_thread()?;

            self.runtime.sleep(Duration::from_millis(100)).await;
            self.notified.store(true, Ordering::Relaxed);
            let _: () = self.jvm.invoke_virtual(&self.target, "notify", "()V", ()).await?;

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
    let _: () = jvm.invoke_virtual(&object, "wait", "()V", ()).await?;
    assert!(notified.load(Ordering::Relaxed));

    Ok(())
}
#[tokio::test]
async fn test_wait_timeout() -> Result<()> {
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
            self.jvm.attach_thread()?;

            self.runtime.sleep(Duration::from_millis(1000)).await;
            self.notified.store(true, Ordering::Relaxed);
            let _: () = self.jvm.invoke_virtual(&self.target, "notify", "()V", ()).await?;

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
    let _: () = jvm.invoke_virtual(&object, "wait", "(J)V", (100i64,)).await?;
    assert!(!notified.load(Ordering::Relaxed));

    Ok(())
}
