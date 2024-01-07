use alloc::vec;

use java_runtime_base::{JavaMethodFlag, JavaMethodProto, JavaResult, JvmClassInstanceHandle};
use jvm::Jvm;

use crate::{RuntimeClassProto, RuntimeContext};

// class java.lang.Runtime
pub struct Runtime {}

impl Runtime {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, JavaMethodFlag::NONE),
                JavaMethodProto::new("getRuntime", "()Ljava/lang/Runtime;", Self::get_runtime, JavaMethodFlag::STATIC),
                JavaMethodProto::new("totalMemory", "()J", Self::total_memory, JavaMethodFlag::NONE),
                JavaMethodProto::new("freeMemory", "()J", Self::free_memory, JavaMethodFlag::NONE),
                JavaMethodProto::new("gc", "()V", Self::gc, JavaMethodFlag::NONE),
            ],
            fields: vec![],
        }
    }

    async fn init(_: &mut Jvm, _: &mut RuntimeContext, this: JvmClassInstanceHandle<Runtime>) -> JavaResult<()> {
        tracing::warn!("stub java.lang.Runtime::<init>({:?})", &this);

        Ok(())
    }

    async fn get_runtime(jvm: &mut Jvm, _: &mut RuntimeContext) -> JavaResult<JvmClassInstanceHandle<Self>> {
        tracing::debug!("java.lang.Runtime::getRuntime");

        let instance = jvm.new_class("java/lang/Runtime", "()V", []).await?;

        Ok(instance.into())
    }

    async fn total_memory(_: &mut Jvm, _: &mut RuntimeContext, this: JvmClassInstanceHandle<Runtime>) -> JavaResult<i64> {
        tracing::warn!("stub java.lang.Runtime::totalMemory({:?})", &this);

        Ok(0x100000) // TODO: hardcoded
    }

    async fn free_memory(_: &mut Jvm, _: &mut RuntimeContext, this: JvmClassInstanceHandle<Runtime>) -> JavaResult<i64> {
        tracing::warn!("stub java.lang.Runtime::freeMemory({:?})", &this);

        Ok(0x100000) // TODO: hardcoded
    }

    async fn gc(_: &mut Jvm, _: &mut RuntimeContext, this: JvmClassInstanceHandle<Runtime>) -> JavaResult<()> {
        tracing::warn!("stub java.lang.Runtime::gc({:?})", &this);

        Ok(())
    }
}
