use alloc::boxed::Box;

use crate::{ArrayClass, Class, JvmResult, ThreadContext, ThreadId};

#[async_trait::async_trait(?Send)]
pub trait JvmDetail {
    async fn load_class(&mut self, class_name: &str) -> JvmResult<Option<Box<dyn Class>>>;
    async fn load_array_class(&mut self, element_type_name: &str) -> JvmResult<Option<Box<dyn ArrayClass>>>;

    fn thread_context(&mut self, thread_id: ThreadId) -> &mut dyn ThreadContext;
}
