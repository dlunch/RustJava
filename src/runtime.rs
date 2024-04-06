use alloc::sync::Arc;
use core::time::Duration;
use std::{io::Write, sync::RwLock};

use java_runtime::{File, FileSize, FileStat, IOError, Runtime};
use jvm::JvmCallback;

// TODO
struct DummyFile;

#[async_trait::async_trait]
impl File for DummyFile {
    async fn read(&self, _offset: FileSize, _buf: &mut [u8]) -> Result<usize, IOError> {
        todo!()
    }

    async fn write(&self, _offset: FileSize, _buf: &[u8]) -> Result<usize, IOError> {
        todo!()
    }

    async fn stat(&self) -> Result<FileStat, IOError> {
        todo!()
    }
}

pub struct RuntimeImpl<T>
where
    T: Sync + Send + Write,
{
    stdout: Arc<RwLock<T>>,
}

impl<T> RuntimeImpl<T>
where
    T: Sync + Send + Write,
{
    pub fn new(stdout: T) -> Self {
        Self {
            stdout: Arc::new(RwLock::new(stdout)),
        }
    }
}

#[async_trait::async_trait]
impl<T> Runtime for RuntimeImpl<T>
where
    T: Sync + Send + Write,
{
    async fn sleep(&self, _duration: Duration) {
        todo!()
    }

    async fn r#yield(&self) {
        todo!()
    }

    fn spawn(&self, _callback: Box<dyn JvmCallback>) {
        todo!()
    }

    fn now(&self) -> u64 {
        todo!()
    }

    fn encode_str(&self, _s: &str) -> Vec<u8> {
        todo!()
    }

    fn decode_str(&self, bytes: &[u8]) -> String {
        let end = bytes.iter().position(|x| *x == 0).unwrap_or(bytes.len());
        String::from_utf8(bytes[..end].to_vec()).unwrap()
    }

    fn println(&mut self, s: &str) {
        writeln!(self.stdout.write().unwrap(), "{}", s).unwrap();
    }

    fn stdin(&self) -> Result<Box<dyn File>, IOError> {
        Ok(Box::new(DummyFile))
    }

    fn stdout(&self) -> Result<Box<dyn File>, IOError> {
        Ok(Box::new(DummyFile))
    }

    fn stderr(&self) -> Result<Box<dyn File>, IOError> {
        Ok(Box::new(DummyFile))
    }

    async fn open(&self, _path: &str) -> Result<Box<dyn File>, IOError> {
        todo!()
    }
}

impl<T> Clone for RuntimeImpl<T>
where
    T: Sync + Send + Write,
{
    fn clone(&self) -> Self {
        Self { stdout: self.stdout.clone() }
    }
}
