mod io;

use alloc::boxed::Box;
use core::time::Duration;

use dyn_clone::{clone_trait_object, DynClone};

use jvm::JvmCallback;

pub use io::{File, FileSize, FileStat, IOError};

#[async_trait::async_trait]
pub trait Runtime: Sync + Send + DynClone {
    async fn sleep(&self, duration: Duration);
    async fn r#yield(&self);
    fn spawn(&self, callback: Box<dyn JvmCallback>);

    fn now(&self) -> u64; // unix time in millis

    fn stdin(&self) -> Result<Box<dyn File>, IOError>;
    fn stdout(&self) -> Result<Box<dyn File>, IOError>;
    fn stderr(&self) -> Result<Box<dyn File>, IOError>;

    async fn open(&self, path: &str) -> Result<Box<dyn File>, IOError>;
    async fn stat(&self, path: &str) -> Result<FileStat, IOError>;
}

clone_trait_object!(Runtime);

// for testing
#[cfg(test)]
pub mod test {
    use alloc::{boxed::Box, collections::BTreeMap, string::String, vec::Vec};
    use core::{cmp::min, time::Duration};

    use jvm::JvmCallback;

    use crate::{
        runtime::{File, IOError, Runtime},
        FileSize, FileStat,
    };

    #[derive(Clone)]
    pub struct DummyRuntime {
        filesystem: BTreeMap<String, Vec<u8>>,
    }

    impl DummyRuntime {
        pub fn new(filesystem: BTreeMap<String, Vec<u8>>) -> Self {
            Self { filesystem }
        }
    }

    #[async_trait::async_trait]
    impl Runtime for DummyRuntime {
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
            Err(IOError::NoSuchFile)
        }

        fn stdout(&self) -> Result<Box<dyn File>, IOError> {
            Err(IOError::NoSuchFile)
        }

        fn stderr(&self) -> Result<Box<dyn File>, IOError> {
            Err(IOError::NoSuchFile)
        }

        async fn open(&self, path: &str) -> Result<Box<dyn File>, IOError> {
            let entry = self.filesystem.get(path);
            if let Some(data) = entry {
                Ok(Box::new(DummyFile::new(data.clone())) as Box<_>)
            } else {
                Err(IOError::NoSuchFile)
            }
        }

        async fn stat(&self, path: &str) -> Result<FileStat, IOError> {
            let entry = self.filesystem.get(path);
            if let Some(data) = entry {
                Ok(FileStat {
                    size: data.len() as FileSize,
                })
            } else {
                Err(IOError::NoSuchFile)
            }
        }
    }

    #[derive(Clone)]
    struct DummyFile {
        data: Vec<u8>,
    }

    impl DummyFile {
        pub fn new(data: Vec<u8>) -> Self {
            Self { data }
        }
    }

    #[async_trait::async_trait]
    impl File for DummyFile {
        async fn read(&mut self, buf: &mut [u8]) -> Result<usize, IOError> {
            let len = min(buf.len(), self.data.len());
            buf[..len].copy_from_slice(&self.data[..len]);
            self.data = self.data[len..].to_vec();

            Ok(len)
        }

        async fn write(&mut self, _buf: &[u8]) -> Result<usize, IOError> {
            Err(IOError::Unsupported)
        }
    }
}
