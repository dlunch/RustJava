use alloc::boxed::Box;

use dyn_clone::{DynClone, clone_trait_object};

#[derive(Debug)]
pub enum IOError {
    Unsupported,
    NotFound,
}

pub type IOResult<T> = Result<T, IOError>;
pub type FileSize = u64;

#[derive(Eq, PartialEq)]
pub enum FileType {
    File,
    Directory,
}

pub struct FileStat {
    pub size: FileSize,
    pub r#type: FileType,
}

#[async_trait::async_trait]
pub trait File: Send + DynClone {
    async fn read(&mut self, buf: &mut [u8]) -> IOResult<usize>;
    async fn write(&mut self, buf: &[u8]) -> IOResult<usize>;
    async fn seek(&mut self, pos: FileSize) -> IOResult<()>;
    async fn tell(&self) -> IOResult<FileSize>;
    async fn set_len(&mut self, len: FileSize) -> IOResult<()>;
    async fn metadata(&self) -> IOResult<FileStat>;
}

clone_trait_object!(File);
