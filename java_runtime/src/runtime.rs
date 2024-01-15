use alloc::{boxed::Box, string::String, vec::Vec};
use core::time::Duration;

use dyn_clone::{clone_trait_object, DynClone};

use jvm::JvmCallback;

#[async_trait::async_trait(?Send)]
pub trait Runtime: DynClone {
    async fn sleep(&self, duration: Duration);
    async fn r#yield(&self);
    fn spawn(&self, callback: Box<dyn JvmCallback>);

    fn now(&self) -> u64; // unix time in millis

    fn encode_str(&self, s: &str) -> Vec<u8>; // TODO implement java charset conversion
    fn decode_str(&self, bytes: &[u8]) -> String;

    fn load_resource(&self, name: &str) -> Option<Vec<u8>>; // TODO implement resource in classloader
    fn println(&self, s: &str); // TODO Properly implement printstream handler
}

clone_trait_object!(Runtime);

// for testing
#[cfg(test)]
pub mod test {
    use alloc::{boxed::Box, string::String, vec::Vec};
    use core::time::Duration;

    use jvm::JvmCallback;

    use crate::runtime::Runtime;

    #[derive(Clone)]
    pub struct DummyRuntime;

    #[async_trait::async_trait(?Send)]
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

        fn load_resource(&self, _name: &str) -> Option<Vec<u8>> {
            todo!()
        }

        fn println(&self, _s: &str) {
            todo!()
        }
    }
}
