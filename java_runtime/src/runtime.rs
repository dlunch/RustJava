mod io;

use alloc::boxed::Box;
use core::time::Duration;

use dyn_clone::{clone_trait_object, DynClone};

use jvm::ClassDefinition;

use crate::RuntimeClassProto;

pub use io::{File, FileSize, FileStat, IOError};

#[async_trait::async_trait]
pub trait SpawnCallback: Sync + Send {
    async fn call(&self);
}

#[async_trait::async_trait]
pub trait Runtime: Sync + Send + DynClone {
    async fn sleep(&self, duration: Duration);
    async fn r#yield(&self);
    fn spawn(&self, callback: Box<dyn SpawnCallback>);

    fn now(&self) -> u64; // unix time in millis
    fn current_task_id(&self) -> u64;

    fn stdin(&self) -> Result<Box<dyn File>, IOError>;
    fn stdout(&self) -> Result<Box<dyn File>, IOError>;
    fn stderr(&self) -> Result<Box<dyn File>, IOError>;

    async fn open(&self, path: &str) -> Result<Box<dyn File>, IOError>;
    async fn stat(&self, path: &str) -> Result<FileStat, IOError>;

    async fn define_class_rust(&self, name: &str, proto: RuntimeClassProto) -> jvm::Result<Box<dyn ClassDefinition>>;
    async fn define_class_java(&self, data: &[u8]) -> jvm::Result<Box<dyn ClassDefinition>>;
    async fn define_array_class(&self, element_type_name: &str) -> jvm::Result<Box<dyn ClassDefinition>>;
}

clone_trait_object!(Runtime);

// test helpers
#[cfg(test)]
pub mod test {
    extern crate std;

    use alloc::{boxed::Box, collections::BTreeMap, string::String, vec::Vec};
    use core::{
        cmp::min,
        sync::atomic::{AtomicU64, Ordering},
        time::Duration,
    };

    use jvm::ClassDefinition;
    use jvm_rust::{ArrayClassDefinitionImpl, ClassDefinitionImpl};

    use crate::{
        runtime::{File, IOError, Runtime, SpawnCallback},
        FileSize, FileStat, RuntimeClassProto,
    };

    #[derive(Clone)]
    pub struct DummyRuntime {
        filesystem: BTreeMap<String, Vec<u8>>,
    }

    impl DummyRuntime {
        pub fn new(filesystem: BTreeMap<String, Vec<u8>>) -> Self {
            Self { filesystem }
        }
    }

    tokio::task_local! {
        static TASK_ID: u64;
    }

    static LAST_TASK_ID: AtomicU64 = AtomicU64::new(1);

    #[async_trait::async_trait]
    impl Runtime for DummyRuntime {
        async fn sleep(&self, _duration: Duration) {
            todo!()
        }

        async fn r#yield(&self) {
            todo!()
        }

        fn spawn(&self, callback: Box<dyn SpawnCallback>) {
            let task_id = LAST_TASK_ID.fetch_add(1, Ordering::SeqCst);
            tokio::spawn(async move {
                TASK_ID
                    .scope(task_id, async move {
                        callback.call().await;
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

        fn stdin(&self) -> Result<Box<dyn File>, IOError> {
            Err(IOError::NotFound)
        }

        fn stdout(&self) -> Result<Box<dyn File>, IOError> {
            Err(IOError::NotFound)
        }

        fn stderr(&self) -> Result<Box<dyn File>, IOError> {
            Err(IOError::NotFound)
        }

        async fn open(&self, path: &str) -> Result<Box<dyn File>, IOError> {
            let entry = self.filesystem.get(path);
            if let Some(data) = entry {
                Ok(Box::new(DummyFile::new(data.clone())) as Box<_>)
            } else {
                Err(IOError::NotFound)
            }
        }

        async fn stat(&self, path: &str) -> Result<FileStat, IOError> {
            let entry = self.filesystem.get(path);
            if let Some(data) = entry {
                Ok(FileStat {
                    size: data.len() as FileSize,
                })
            } else {
                Err(IOError::NotFound)
            }
        }

        async fn define_class_rust(&self, name: &str, proto: RuntimeClassProto) -> jvm::Result<Box<dyn ClassDefinition>> {
            Ok(Box::new(ClassDefinitionImpl::from_class_proto(
                name,
                proto,
                Box::new(self.clone()) as Box<_>,
            )))
        }

        async fn define_class_java(&self, data: &[u8]) -> jvm::Result<Box<dyn ClassDefinition>> {
            ClassDefinitionImpl::from_classfile(data).map(|x| Box::new(x) as Box<_>)
        }

        async fn define_array_class(&self, element_type_name: &str) -> jvm::Result<Box<dyn ClassDefinition>> {
            Ok(Box::new(ArrayClassDefinitionImpl::new(element_type_name)))
        }
    }

    #[derive(Clone)]
    struct DummyFile {
        data: Vec<u8>,
    }

    impl DummyFile {
        pub fn new(data: Vec<u8>) -> Self {
            Self { data }
        }
    }

    #[async_trait::async_trait]
    impl File for DummyFile {
        async fn read(&mut self, buf: &mut [u8]) -> Result<usize, IOError> {
            let len = min(buf.len(), self.data.len());
            buf[..len].copy_from_slice(&self.data[..len]);
            self.data = self.data[len..].to_vec();

            Ok(len)
        }

        async fn write(&mut self, _buf: &[u8]) -> Result<usize, IOError> {
            Err(IOError::Unsupported)
        }
    }
}
