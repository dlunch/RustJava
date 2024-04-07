mod io;

use alloc::sync::Arc;
use core::time::Duration;
use std::{
    io::{stderr, stdin, stdout, Write},
    sync::RwLock,
};

use java_runtime::{File, IOError, Runtime};
use jvm::JvmCallback;

use self::io::FileImpl;

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
        Ok(Box::new(FileImpl::from_read(stdin())))
    }

    fn stdout(&self) -> Result<Box<dyn File>, IOError> {
        Ok(Box::new(FileImpl::from_write(stdout())))
    }

    fn stderr(&self) -> Result<Box<dyn File>, IOError> {
        Ok(Box::new(FileImpl::from_write(stderr())))
    }

    async fn open(&self, path: &str) -> Result<Box<dyn File>, IOError> {
        Ok(Box::new(FileImpl::from_path(path).unwrap()))
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
