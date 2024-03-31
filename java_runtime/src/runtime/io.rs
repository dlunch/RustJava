use alloc::boxed::Box;

pub enum IOError {}
pub type FileSize = u64;

pub struct FileStat {
    pub size: FileSize,
    // TODO more..
}

#[async_trait::async_trait]
pub trait File {
    async fn input(&self, offset: FileSize) -> Result<Box<dyn InputStream>, IOError>;
    async fn output(&self, offset: FileSize) -> Result<Box<dyn OutputStream>, IOError>;
    async fn stat(&self) -> Result<FileStat, IOError>;
}

#[async_trait::async_trait]
pub trait OutputStream {
    async fn write(&self, buf: &[u8]) -> Result<usize, IOError>;
}

#[async_trait::async_trait]
pub trait InputStream {
    async fn read(&self, buf: &mut [u8]) -> Result<usize, IOError>;
}
