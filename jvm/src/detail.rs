use alloc::boxed::Box;

use crate::{Class, Jvm, JvmResult, ThreadContext, ThreadId};

#[async_trait::async_trait(?Send)]
pub trait JvmDetail {
    async fn define_class(&self, jvm: &Jvm, name: &str, data: &[u8]) -> JvmResult<Box<dyn Class>>;
    async fn define_array_class(&self, jvm: &Jvm, element_type_name: &str) -> JvmResult<Box<dyn Class>>;

    fn thread_context(&mut self, thread_id: ThreadId) -> Box<dyn ThreadContext>;
}
