mod io;

use alloc::sync::Arc;
use core::{
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};
use std::{
    fs,
    io::{stderr, stdin, Write},
    sync::Mutex,
};

use java_runtime::{File, FileStat, IOError, Runtime, RuntimeClassProto, SpawnCallback};
use jvm::{ClassDefinition, Jvm};
use jvm_rust::{ArrayClassDefinitionImpl, ClassDefinitionImpl};

use self::io::FileImpl;

tokio::task_local! {
    static TASK_ID: u64;
}

static LAST_TASK_ID: AtomicU64 = AtomicU64::new(1);

struct WriteWrapper<T>
where
    T: Sync + Send + Write + 'static,
{
    write: Arc<Mutex<T>>,
}

impl<T> Write for WriteWrapper<T>
where
    T: Sync + Send + Write + 'static,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.write.lock().unwrap().write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.write.lock().unwrap().flush()
    }
}

impl<T> Clone for WriteWrapper<T>
where
    T: Sync + Send + Write + 'static,
{
    fn clone(&self) -> Self {
        Self { write: self.write.clone() }
    }
}

pub struct RuntimeImpl<T>
where
    T: Sync + Send + Write + 'static,
{
    stdout: WriteWrapper<T>,
}

impl<T> RuntimeImpl<T>
where
    T: Sync + Send + Write + 'static,
{
    pub fn new(stdout: T) -> Self {
        Self {
            stdout: WriteWrapper {
                write: Arc::new(Mutex::new(stdout)),
            },
        }
    }
}

#[async_trait::async_trait]
impl<T> Runtime for RuntimeImpl<T>
where
    T: Sync + Send + Write + 'static,
{
    async fn sleep(&self, _duration: Duration) {
        todo!()
    }

    async fn r#yield(&self) {
        todo!()
    }

    fn spawn(&self, callback: Box<dyn SpawnCallback>) {
        let task_id = LAST_TASK_ID.fetch_add(1, Ordering::SeqCst) + 1;
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
        Ok(Box::new(FileImpl::from_read(stdin())))
    }

    fn stdout(&self) -> Result<Box<dyn File>, IOError> {
        Ok(Box::new(FileImpl::from_write(self.stdout.clone())))
    }

    fn stderr(&self) -> Result<Box<dyn File>, IOError> {
        Ok(Box::new(FileImpl::from_write(stderr())))
    }

    async fn open(&self, path: &str) -> Result<Box<dyn File>, IOError> {
        Ok(Box::new(FileImpl::from_path(path).unwrap()))
    }

    async fn stat(&self, path: &str) -> Result<FileStat, IOError> {
        let metadata = fs::metadata(path);
        if let Ok(metadata) = metadata {
            Ok(FileStat { size: metadata.len() })
        } else {
            Err(IOError::NotFound) // TODO error conversion
        }
    }

    async fn define_class_rust(&self, _jvm: &Jvm, proto: RuntimeClassProto) -> jvm::Result<Box<dyn ClassDefinition>> {
        Ok(Box::new(ClassDefinitionImpl::from_class_proto(proto, Box::new(self.clone()) as Box<_>)))
    }

    async fn define_class_java(&self, _jvm: &Jvm, data: &[u8]) -> jvm::Result<Box<dyn ClassDefinition>> {
        ClassDefinitionImpl::from_classfile(data).map(|x| Box::new(x) as Box<_>)
    }

    async fn define_array_class(&self, _jvm: &Jvm, element_type_name: &str) -> jvm::Result<Box<dyn ClassDefinition>> {
        Ok(Box::new(ArrayClassDefinitionImpl::new(element_type_name)))
    }
}

impl<T> Clone for RuntimeImpl<T>
where
    T: Sync + Send + Write + 'static,
{
    fn clone(&self) -> Self {
        Self { stdout: self.stdout.clone() }
    }
}
