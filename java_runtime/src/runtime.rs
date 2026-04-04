mod io;

use alloc::boxed::Box;
use core::time::Duration;

use dyn_clone::{DynClone, clone_trait_object};

use jvm::{ClassDefinition, Jvm, Result as JvmResult};

pub use io::{File, FileDescriptorId, FileSize, FileStat, FileType, IOError, IOResult};

#[async_trait::async_trait]
pub trait SpawnCallback: Sync + Send {
    async fn call(&self) -> JvmResult<()>;
}

#[async_trait::async_trait]
pub trait Runtime: Sync + Send + DynClone {
    async fn sleep(&self, duration: Duration);
    async fn r#yield(&self);
    fn spawn(&self, jvm: &Jvm, callback: Box<dyn SpawnCallback>);

    fn now(&self) -> u64; // unix time in millis
    fn current_task_id(&self) -> u64;

    fn stdin(&self) -> IOResult<FileDescriptorId>;
    fn stdout(&self) -> IOResult<FileDescriptorId>;
    fn stderr(&self) -> IOResult<FileDescriptorId>;

    async fn open(&self, path: &str, write: bool) -> IOResult<FileDescriptorId>;
    fn get_file(&self, fd: FileDescriptorId) -> IOResult<Box<dyn File>>;
    fn close_file(&self, fd: FileDescriptorId);
    async fn unlink(&self, path: &str) -> IOResult<()>;
    async fn metadata(&self, path: &str) -> IOResult<FileStat>;

    async fn find_rustjar_class(&self, jvm: &Jvm, classpath: &str, class: &str) -> JvmResult<Option<Box<dyn ClassDefinition>>>;
    async fn define_class(&self, jvm: &Jvm, data: &[u8]) -> JvmResult<Box<dyn ClassDefinition>>;
    async fn define_array_class(&self, jvm: &Jvm, element_type_name: &str) -> JvmResult<Box<dyn ClassDefinition>>;
}

clone_trait_object!(Runtime);
