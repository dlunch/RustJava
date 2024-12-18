extern crate alloc;
extern crate std;

use alloc::boxed::Box;
use core::{
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

use java_runtime::{get_bootstrap_class_loader, get_runtime_class_proto, File, FileStat, IOError, IOResult, Runtime, SpawnCallback, RT_RUSTJAR};
use jvm::{ClassDefinition, Jvm, Result as JvmResult};
use jvm_rust::{ArrayClassDefinitionImpl, ClassDefinitionImpl};

#[derive(Clone)]
pub struct TestRuntime;

tokio::task_local! {
    static TASK_ID: u64;
}

static LAST_TASK_ID: AtomicU64 = AtomicU64::new(1);

#[async_trait::async_trait]
impl Runtime for TestRuntime {
    async fn sleep(&self, duration: Duration) {
        tokio::time::sleep(duration).await;
    }

    async fn r#yield(&self) {
        todo!()
    }

    fn spawn(&self, _jvm: &Jvm, callback: Box<dyn SpawnCallback>) {
        let task_id = LAST_TASK_ID.fetch_add(1, Ordering::SeqCst);
        tokio::spawn(async move {
            TASK_ID
                .scope(task_id, async move {
                    callback.call().await.unwrap();
                })
                .await;
        });
    }

    fn now(&self) -> u64 {
        todo!()
    }

    fn current_task_id(&self) -> u64 {
        TASK_ID.try_with(|x| *x).unwrap_or(0)
    }

    fn stdin(&self) -> IOResult<Box<dyn File>> {
        Err(IOError::NotFound)
    }

    fn stdout(&self) -> IOResult<Box<dyn File>> {
        Err(IOError::NotFound)
    }

    fn stderr(&self) -> IOResult<Box<dyn File>> {
        Err(IOError::NotFound)
    }

    async fn open(&self, _path: &str, _write: bool) -> IOResult<Box<dyn File>> {
        Err(IOError::NotFound)
    }

    async fn unlink(&self, _path: &str) -> IOResult<()> {
        Err(IOError::NotFound)
    }

    async fn metadata(&self, _path: &str) -> IOResult<FileStat> {
        Err(IOError::NotFound)
    }

    async fn find_rustjar_class(&self, _jvm: &Jvm, classpath: &str, class: &str) -> JvmResult<Option<Box<dyn ClassDefinition>>> {
        if classpath == RT_RUSTJAR {
            let proto = get_runtime_class_proto(class);
            if let Some(proto) = proto {
                return Ok(Some(Box::new(ClassDefinitionImpl::from_class_proto(
                    proto,
                    Box::new(self.clone()) as Box<_>,
                ))));
            }
        }

        Ok(None)
    }

    async fn define_class(&self, _jvm: &Jvm, data: &[u8]) -> JvmResult<Box<dyn ClassDefinition>> {
        ClassDefinitionImpl::from_classfile(data).map(|x| Box::new(x) as Box<_>)
    }

    async fn define_array_class(&self, _jvm: &Jvm, element_type_name: &str) -> JvmResult<Box<dyn ClassDefinition>> {
        Ok(Box::new(ArrayClassDefinitionImpl::new(element_type_name)))
    }
}

pub async fn test_jvm() -> JvmResult<Jvm> {
    let bootstrap_class_loader = get_bootstrap_class_loader(Box::new(TestRuntime));

    let properties = [("java.class.path", RT_RUSTJAR)].into_iter().collect();

    Jvm::new(bootstrap_class_loader, move || TestRuntime.current_task_id(), properties).await
}
