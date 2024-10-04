use alloc::boxed::Box;

use dyn_clone::{clone_trait_object, DynClone};

#[derive(Debug)]
pub enum IOError {
    Unsupported,
    NotFound,
}
pub type FileSize = u64;

pub struct FileStat {
    pub size: FileSize,
    // TODO more..
}

#[async_trait::async_trait]
pub trait File: Send + DynClone {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, IOError>;
    async fn write(&mut self, buf: &[u8]) -> Result<usize, IOError>;
    async fn seek(&mut self, pos: FileSize) -> Result<(), IOError>;
}

clone_trait_object!(File);
