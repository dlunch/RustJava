#![no_std]
extern crate alloc;

pub mod classes;
mod loader;
mod runtime;

pub use self::{
    loader::{get_bootstrap_class_loader, get_runtime_class_proto},
    runtime::{File, FileSize, FileStat, IOError, IOResult, Runtime, SpawnCallback},
};

pub(crate) type RuntimeContext = dyn runtime::Runtime;
pub(crate) type RuntimeClassProto = java_class_proto::JavaClassProto<dyn runtime::Runtime>;

pub static RT_RUSTJAR: &str = "rt.rustjar";

#[cfg(test)]
pub mod test {
    use alloc::{boxed::Box, collections::BTreeMap, string::String, vec::Vec};

    use jvm::{Jvm, Result};

    use crate::{get_bootstrap_class_loader, runtime::test::TestRuntime, Runtime, RT_RUSTJAR};

    pub async fn test_jvm() -> Result<Jvm> {
        let runtime = TestRuntime::new(BTreeMap::new());
        create_test_jvm(runtime).await
    }

    pub async fn test_jvm_filesystem(filesystem: BTreeMap<String, Vec<u8>>) -> Result<Jvm> {
        let runtime = TestRuntime::new(filesystem);
        create_test_jvm(runtime).await
    }

    pub async fn create_test_jvm<R>(runtime: R) -> Result<Jvm>
    where
        R: Runtime + Clone + 'static,
    {
        let bootstrap_class_loader = get_bootstrap_class_loader(Box::new(runtime.clone()));

        let properties = [("java.class.path", RT_RUSTJAR)].into_iter().collect();

        Jvm::new(bootstrap_class_loader, move || runtime.current_task_id(), properties).await
    }
}
