use alloc::{boxed::Box, string::String, vec::Vec};
use core::time::Duration;

use dyn_clone::{clone_trait_object, DynClone};

use jvm::{Class, JvmCallback};

use crate::RuntimeClassProto;

#[async_trait::async_trait(?Send)]
pub trait Runtime: DynClone {
    async fn sleep(&self, duration: Duration);
    async fn r#yield(&self);
    fn spawn(&self, callback: Box<dyn JvmCallback>);

    fn define_class(&self, name: &str, data: &[u8]) -> Box<dyn Class>;
    fn define_class_proto(&self, name: &str, proto: RuntimeClassProto) -> Box<dyn Class>;

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

    use jvm::{Class, JvmCallback};

    use crate::{Runtime, RuntimeClassProto};

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

        fn define_class(&self, _name: &str, _data: &[u8]) -> Box<dyn Class> {
            todo!()
        }

        fn define_class_proto(&self, _name: &str, _proto: RuntimeClassProto) -> Box<dyn Class> {
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
