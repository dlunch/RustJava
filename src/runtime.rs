mod io;

use alloc::sync::Arc;
use core::time::Duration;
use std::{
    fs,
    io::{stderr, stdin, Write},
    sync::Mutex,
};

use java_runtime::{File, FileStat, IOError, Runtime};
use jvm::JvmCallback;

use self::io::FileImpl;

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

    fn spawn(&self, _callback: Box<dyn JvmCallback>) {
        todo!()
    }

    fn now(&self) -> u64 {
        todo!()
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
}

impl<T> Clone for RuntimeImpl<T>
where
    T: Sync + Send + Write + 'static,
{
    fn clone(&self) -> Self {
        Self { stdout: self.stdout.clone() }
    }
}
