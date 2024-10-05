use alloc::boxed::Box;

use dyn_clone::{clone_trait_object, DynClone};

#[derive(Debug)]
pub enum IOError {
    Unsupported,
    NotFound,
}

pub type IOResult<T> = Result<T, IOError>;
pub type FileSize = u64;

pub struct FileStat {
    pub size: FileSize,
    // TODO more..
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
