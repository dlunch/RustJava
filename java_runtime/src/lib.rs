#![no_std]
extern crate alloc;

pub mod classes;
mod init;
mod runtime;

pub use self::{
    init::initialize,
    runtime::{File, FileSize, FileStat, IOError, Runtime},
};

pub(crate) type RuntimeContext = dyn runtime::Runtime;
pub(crate) type RuntimeClassProto = java_class_proto::JavaClassProto<dyn runtime::Runtime>;

#[cfg(test)]
pub mod test {
    use alloc::{boxed::Box, collections::BTreeMap, string::String, vec::Vec};
    use core::future::ready;

    use jvm::{Jvm, Result};
    use jvm_rust::{ClassDefinitionImpl, JvmDetailImpl};

    use crate::{initialize, runtime::test::DummyRuntime};

    pub async fn test_jvm() -> Result<Jvm> {
        let runtime = DummyRuntime::new(BTreeMap::new());
        create_test_jvm(runtime).await
    }

    pub async fn test_jvm_filesystem(filesystem: BTreeMap<String, Vec<u8>>) -> Result<Jvm> {
        let runtime = DummyRuntime::new(filesystem);
        create_test_jvm(runtime).await
    }

    async fn create_test_jvm(runtime: DummyRuntime) -> Result<Jvm> {
        let jvm = Jvm::new(JvmDetailImpl).await?;

        initialize(&jvm, move |name, proto| {
            ready(Box::new(ClassDefinitionImpl::from_class_proto(name, proto, Box::new(runtime.clone()) as Box<_>)) as Box<_>)
        })
        .await?;

        Ok(jvm)
    }
}
