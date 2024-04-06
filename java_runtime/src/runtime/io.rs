use alloc::boxed::Box;

#[derive(Debug)]
pub enum IOError {}
pub type FileSize = u64;

pub struct FileStat {
    pub size: FileSize,
    // TODO more..
}

#[async_trait::async_trait]
pub trait File: Send {
    async fn read(&self, offset: FileSize, buf: &mut [u8]) -> Result<usize, IOError>;
    async fn write(&self, offset: FileSize, buf: &[u8]) -> Result<usize, IOError>;
    async fn stat(&self) -> Result<FileStat, IOError>;
}
