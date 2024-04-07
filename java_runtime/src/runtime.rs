mod io;

use alloc::{boxed::Box, string::String, vec::Vec};
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

    fn encode_str(&self, s: &str) -> Vec<u8>; // TODO implement java charset conversion
    fn decode_str(&self, bytes: &[u8]) -> String;

    fn println(&mut self, s: &str); // TODO Properly implement PrintStream handler

    fn stdin(&self) -> Result<Box<dyn File>, IOError>;
    fn stdout(&self) -> Result<Box<dyn File>, IOError>;
    fn stderr(&self) -> Result<Box<dyn File>, IOError>;

    async fn open(&self, path: &str) -> Result<Box<dyn File>, IOError>;
}

clone_trait_object!(Runtime);

// for testing
#[cfg(test)]
pub mod test {
    use alloc::{boxed::Box, string::String, vec::Vec};
    use core::time::Duration;

    use jvm::JvmCallback;

    use crate::runtime::{File, FileStat, IOError, Runtime};

    struct DummyFile;

    #[async_trait::async_trait]
    impl File for DummyFile {
        async fn read(&mut self, _buf: &mut [u8]) -> Result<usize, IOError> {
            todo!()
        }

        async fn write(&mut self, _buf: &[u8]) -> Result<usize, IOError> {
            todo!()
        }

        async fn stat(&self) -> Result<FileStat, IOError> {
            todo!()
        }
    }

    #[derive(Clone)]
    pub struct DummyRuntime;

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

        fn encode_str(&self, _s: &str) -> Vec<u8> {
            todo!()
        }

        fn decode_str(&self, _bytes: &[u8]) -> String {
            todo!()
        }

        fn println(&mut self, _s: &str) {
            todo!()
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
}
