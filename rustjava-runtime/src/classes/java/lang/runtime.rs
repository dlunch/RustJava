use alloc::format;

use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::RuntimeContext;

// class java.lang.Runtime
#[java_class(name = "java/lang/Runtime", public, final)]
pub struct Runtime;

#[java_methods]
impl Runtime {
    #[method(constructor, private)]
    async fn init(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Runtime>) -> Result<()> {
        tracing::warn!("stub java.lang.Runtime::<init>({this:?})");

        Ok(())
    }

    #[method(static)]
    async fn get_runtime(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.Runtime::getRuntime");

        let instance = jvm.new_class("java/lang/Runtime", "()V", []).await?;

        Ok(instance.into())
    }

    #[method(public, native)]
    async fn total_memory(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Runtime>) -> Result<i64> {
        tracing::warn!("stub java.lang.Runtime::totalMemory({this:?})");

        Ok(0x100000) // TODO: hardcoded
    }

    #[method(public, native)]
    async fn free_memory(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Runtime>) -> Result<i64> {
        tracing::warn!("stub java.lang.Runtime::freeMemory({this:?})");

        Ok(0x100000) // TODO: hardcoded
    }

    #[method(public, native)]
    async fn gc(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Runtime>) -> Result<()> {
        tracing::debug!("java.lang.Runtime::gc({this:?})");

        if let Err(error) = jvm.collect_garbage() {
            return Err(jvm
                .exception("java/lang/InternalError", &format!("Garbage collection failed: {error:#}"))
                .await);
        }

        Ok(())
    }

    #[method(public)]
    async fn exit(_: &Jvm, context: &mut RuntimeContext, _: ClassInstanceRef<Self>, status: i32) -> Result<()> {
        tracing::debug!("java.lang.Runtime::exit({status})");

        context.exit(status);
        Ok(())
    }
}
