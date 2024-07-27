#![no_std]
extern crate alloc;

pub mod classes;
mod loader;
mod runtime;

pub use self::{
    loader::get_bootstrap_class_loader,
    runtime::{File, FileSize, FileStat, IOError, Runtime, SpawnCallback},
};

pub(crate) type RuntimeContext = dyn runtime::Runtime;
pub type RuntimeClassProto = java_class_proto::JavaClassProto<dyn runtime::Runtime>;

#[cfg(test)]
pub mod test {
    use alloc::{boxed::Box, collections::BTreeMap, string::String, vec::Vec};

    use jvm::{Jvm, Result};

    use crate::{get_bootstrap_class_loader, runtime::test::DummyRuntime, Runtime};

    pub async fn test_jvm() -> Result<Jvm> {
        let runtime = DummyRuntime::new(BTreeMap::new());
        create_test_jvm(runtime).await
    }

    pub async fn test_jvm_filesystem(filesystem: BTreeMap<String, Vec<u8>>) -> Result<Jvm> {
        let runtime = DummyRuntime::new(filesystem);
        create_test_jvm(runtime).await
    }

    pub async fn create_test_jvm<R>(runtime: R) -> Result<Jvm>
    where
        R: Runtime + 'static,
    {
        let bootstrap_class_loader = get_bootstrap_class_loader(Box::new(runtime));

        Jvm::new(bootstrap_class_loader, BTreeMap::new()).await
    }
}
