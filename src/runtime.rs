mod io;

use alloc::{collections::BTreeMap, sync::Arc};
use core::{
    sync::atomic::{AtomicU32, AtomicU64, Ordering},
    time::Duration,
};
use std::{
    fs,
    io::{Write, stderr, stdin},
    sync::Mutex,
};

use java_runtime::{File, FileDescriptorId, FileStat, FileType, IOError, IOResult, RT_RUSTJAR, Runtime, SpawnCallback, get_runtime_class_proto};
use jvm::{ClassDefinition, Jvm};
use jvm_rust::{ArrayClassDefinitionImpl, ClassDefinitionImpl};

use self::io::{FileImpl, InputStreamFile, WriteStreamFile};

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
    file_table: Arc<Mutex<BTreeMap<u32, Box<dyn File>>>>,
    next_fd: Arc<AtomicU32>,
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

    fn spawn(&self, _jvm: &Jvm, callback: Box<dyn SpawnCallback>) {
        let task_id = LAST_TASK_ID.fetch_add(1, Ordering::SeqCst) + 1;
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

    fn stdin(&self) -> IOResult<FileDescriptorId> {
        let file = Box::new(InputStreamFile::new(stdin()));
        Ok(self.register_file(file))
    }

    fn stdout(&self) -> IOResult<FileDescriptorId> {
        let file = Box::new(WriteStreamFile::new(self.stdout.clone()));
        Ok(self.register_file(file))
    }

    fn stderr(&self) -> IOResult<FileDescriptorId> {
        let file = Box::new(WriteStreamFile::new(stderr()));
        Ok(self.register_file(file))
    }

    async fn open(&self, path: &str, write: bool) -> IOResult<FileDescriptorId> {
        let file = Box::new(FileImpl::new(path, write));
        Ok(self.register_file(file))
    }

    fn get_file(&self, fd: FileDescriptorId) -> IOResult<Box<dyn File>> {
        self.file_table.lock().unwrap().get(&fd.id()).cloned().ok_or(IOError::NotFound)
    }

    fn close_file(&self, fd: FileDescriptorId) {
        self.file_table.lock().unwrap().remove(&fd.id());
    }

    async fn unlink(&self, path: &str) -> IOResult<()> {
        fs::remove_file(path).map_err(|_| IOError::NotFound) // TODO error conversion
    }

    async fn metadata(&self, path: &str) -> IOResult<FileStat> {
        let metadata = fs::metadata(path);
        if let Ok(metadata) = metadata {
            let file_type = if metadata.is_dir() { FileType::Directory } else { FileType::File };

            Ok(FileStat {
                size: metadata.len(),
                r#type: file_type,
            })
        } else {
            Err(IOError::NotFound) // TODO error conversion
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

impl<T> Clone for RuntimeImpl<T>
where
    T: Sync + Send + Write + 'static,
{
    fn clone(&self) -> Self {
        Self {
            stdout: self.stdout.clone(),
            file_table: self.file_table.clone(),
            next_fd: self.next_fd.clone(),
        }
    }
}
