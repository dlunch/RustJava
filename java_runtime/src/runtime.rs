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
