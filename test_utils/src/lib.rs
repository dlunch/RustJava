extern crate alloc;

use alloc::{boxed::Box, collections::BTreeMap, string::String, sync::Arc, vec::Vec};
use core::{
    cmp::min,
    sync::atomic::{AtomicU32, AtomicU64, Ordering},
    time::Duration,
};
use std::{
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};

use jvm::{ClassDefinition, Jvm, Result};
use jvm_rust::{ArrayClassDefinitionImpl, ClassDefinitionImpl};

use java_runtime::{
    File, FileDescriptorId, FileSize, FileStat, FileType, IOError, IOResult, RT_RUSTJAR, Runtime, SpawnCallback, get_bootstrap_class_loader,
    get_runtime_class_proto,
};

pub struct TestRuntime {
    filesystem: BTreeMap<String, Vec<u8>>,
    file_table: Arc<Mutex<BTreeMap<u32, Box<dyn File>>>>,
    next_fd: Arc<AtomicU32>,
}

impl Clone for TestRuntime {
    fn clone(&self) -> Self {
        Self {
            filesystem: self.filesystem.clone(),
            file_table: self.file_table.clone(),
            next_fd: self.next_fd.clone(),
        }
    }
}

impl TestRuntime {
    pub fn new(filesystem: BTreeMap<String, Vec<u8>>) -> Self {
        Self {
            filesystem,
            file_table: Arc::new(Mutex::new(BTreeMap::new())),
            next_fd: Arc::new(AtomicU32::new(1)),
        }
    }

    fn register_file(&self, file: Box<dyn File>) -> FileDescriptorId {
        let fd = self.next_fd.fetch_add(1, Ordering::SeqCst);
        self.file_table.lock().unwrap().insert(fd, file);
        FileDescriptorId::new(fd)
    }
}

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
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(Duration::from_secs(0)).as_millis() as u64
    }

    fn current_task_id(&self) -> u64 {
        TASK_ID.try_with(|x| *x).unwrap_or(0)
    }

    fn stdin(&self) -> IOResult<FileDescriptorId> {
        Err(IOError::NotFound)
    }

    fn stdout(&self) -> IOResult<FileDescriptorId> {
        Err(IOError::NotFound)
    }

    fn stderr(&self) -> IOResult<FileDescriptorId> {
        Err(IOError::NotFound)
    }

    async fn open(&self, path: &str, _write: bool) -> IOResult<FileDescriptorId> {
        let entry = self.filesystem.get(path);
        if let Some(data) = entry {
            let file = Box::new(DummyFile::new(data.clone())) as Box<dyn File>;
            Ok(self.register_file(file))
        } else {
            Err(IOError::NotFound)
        }
    }

    fn get_file(&self, fd: FileDescriptorId) -> IOResult<Box<dyn File>> {
        self.file_table.lock().unwrap().get(&fd.id()).cloned().ok_or(IOError::NotFound)
    }

    fn close_file(&self, fd: FileDescriptorId) {
        self.file_table.lock().unwrap().remove(&fd.id());
    }

    async fn unlink(&self, _path: &str) -> IOResult<()> {
        Err(IOError::NotFound)
    }

    async fn metadata(&self, path: &str) -> IOResult<FileStat> {
        let entry = self.filesystem.get(path);
        if let Some(data) = entry {
            Ok(FileStat {
                size: data.len() as FileSize,
                r#type: FileType::File,
            })
        } else {
            Err(IOError::NotFound)
        }
    }

    async fn find_rustjar_class(&self, _jvm: &Jvm, classpath: &str, class: &str) -> jvm::Result<Option<Box<dyn ClassDefinition>>> {
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

    async fn define_class(&self, _jvm: &Jvm, data: &[u8]) -> jvm::Result<Box<dyn ClassDefinition>> {
        ClassDefinitionImpl::from_classfile(data).map(|x| Box::new(x) as Box<_>)
    }

    async fn define_array_class(&self, _jvm: &Jvm, element_type_name: &str) -> jvm::Result<Box<dyn ClassDefinition>> {
        Ok(Box::new(ArrayClassDefinitionImpl::new(element_type_name)))
    }
}

#[derive(Clone)]
struct DummyFile {
    data: Vec<u8>,
    pos: FileSize,
}

impl DummyFile {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data, pos: 0 }
    }
}

#[async_trait::async_trait]
impl File for DummyFile {
    async fn read(&mut self, buf: &mut [u8]) -> IOResult<usize> {
        let len = min(buf.len(), self.data.len());
        buf[..len].copy_from_slice(&self.data[..len]);
        self.data = self.data[len..].to_vec();

        Ok(len)
    }

    async fn write(&mut self, _buf: &[u8]) -> IOResult<usize> {
        Err(IOError::Unsupported)
    }

    async fn seek(&mut self, pos: FileSize) -> IOResult<()> {
        self.pos = pos;

        Ok(())
    }

    async fn tell(&self) -> IOResult<FileSize> {
        Ok(self.pos as _)
    }

    async fn set_len(&mut self, _len: FileSize) -> IOResult<()> {
        Err(IOError::Unsupported)
    }

    async fn metadata(&self) -> IOResult<FileStat> {
        Ok(FileStat {
            size: self.data.len() as FileSize,
            r#type: FileType::File,
        })
    }
}

pub async fn create_test_jvm<R>(runtime: R) -> Result<Jvm>
where
    R: Runtime + Clone + 'static,
{
    let bootstrap_class_loader = get_bootstrap_class_loader(Box::new(runtime.clone()));

    let properties = [("java.class.path", RT_RUSTJAR)].into_iter().collect();

    Jvm::new(bootstrap_class_loader, move || runtime.current_task_id(), properties).await
}

pub async fn test_jvm() -> Result<Jvm> {
    let runtime = TestRuntime::new(BTreeMap::new());
    create_test_jvm(runtime).await
}

pub async fn test_jvm_filesystem(filesystem: BTreeMap<String, Vec<u8>>) -> Result<Jvm> {
    let runtime = TestRuntime::new(filesystem);
    create_test_jvm(runtime).await
}
