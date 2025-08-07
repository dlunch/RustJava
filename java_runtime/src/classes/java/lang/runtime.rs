use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::MethodAccessFlags;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

// class java.lang.Runtime
pub struct Runtime;

impl Runtime {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Runtime",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getRuntime", "()Ljava/lang/Runtime;", Self::get_runtime, MethodAccessFlags::STATIC),
                JavaMethodProto::new("totalMemory", "()J", Self::total_memory, Default::default()),
                JavaMethodProto::new("freeMemory", "()J", Self::free_memory, Default::default()),
                JavaMethodProto::new("gc", "()V", Self::gc, Default::default()),
            ],
            fields: vec![],
        }
    }

    async fn init(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Runtime>) -> Result<()> {
        tracing::warn!("stub java.lang.Runtime::<init>({:?})", &this);

        Ok(())
    }

    async fn get_runtime(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.Runtime::getRuntime");

        let instance = jvm.new_class("java/lang/Runtime", "()V", []).await?;

        Ok(instance.into())
    }

    async fn total_memory(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Runtime>) -> Result<i64> {
        tracing::warn!("stub java.lang.Runtime::totalMemory({:?})", &this);

        Ok(0x100000) // TODO: hardcoded
    }

    async fn free_memory(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Runtime>) -> Result<i64> {
        tracing::warn!("stub java.lang.Runtime::freeMemory({:?})", &this);

        Ok(0x100000) // TODO: hardcoded
    }

    async fn gc(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Runtime>) -> Result<()> {
        tracing::debug!("java.lang.Runtime::gc({:?})", &this);

        jvm.collect_garbage()?;

        Ok(())
    }
}
