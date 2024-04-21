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
}

clone_trait_object!(Runtime);

// for testing
#[cfg(test)]
pub mod test {
    use alloc::boxed::Box;
    use core::time::Duration;

    use jvm::JvmCallback;

    use crate::runtime::{File, IOError, Runtime};

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

        fn stdin(&self) -> Result<Box<dyn File>, IOError> {
            Err(IOError::Unsupported)
        }

        fn stdout(&self) -> Result<Box<dyn File>, IOError> {
            Err(IOError::Unsupported)
        }

        fn stderr(&self) -> Result<Box<dyn File>, IOError> {
            Err(IOError::Unsupported)
        }

        async fn open(&self, _path: &str) -> Result<Box<dyn File>, IOError> {
            todo!()
        }
    }
}
