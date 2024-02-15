#![no_std]
extern crate alloc;

pub mod classes;
mod init;
mod runtime;

pub use self::{init::initialize, runtime::Runtime};

pub(crate) type RuntimeContext = dyn runtime::Runtime;
pub(crate) type RuntimeClassProto = java_class_proto::JavaClassProto<dyn runtime::Runtime>;

#[cfg(test)]
pub mod test {
    use alloc::boxed::Box;
    use core::future::ready;

    use jvm::{Jvm, JvmResult};
    use jvm_rust::{ClassDefinitionImpl, JvmDetailImpl};

    use crate::{initialize, runtime::test::DummyRuntime};

    pub async fn test_jvm() -> JvmResult<Jvm> {
        let jvm = Jvm::new(JvmDetailImpl).await?;

        initialize(&jvm, |name, proto| {
            ready(Box::new(ClassDefinitionImpl::from_class_proto(name, proto, Box::new(DummyRuntime) as Box<_>)) as Box<_>)
        })
        .await?;

        Ok(jvm)
    }
}
