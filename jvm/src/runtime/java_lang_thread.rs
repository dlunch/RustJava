use alloc::boxed::Box;

use crate::{jvm::Jvm, Result};

pub struct JavaLangThread {}

impl JavaLangThread {
    pub async fn current_thread_id(jvm: &Jvm) -> Result<u64> {
        // TODO we should not use jvm.invoke_static as it's called via invoke methods on jvm

        let thread_class = jvm.resolve_class("java/lang/Thread").await?;
        let method = thread_class.definition.method("currentThreadId", "()J").unwrap();
        let id: i64 = method.run(jvm, Box::new([])).await?.into();

        Ok(id as _)
    }
}
