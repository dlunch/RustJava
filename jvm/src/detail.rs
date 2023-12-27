use alloc::boxed::Box;

use crate::{ArrayClass, ClassRef, JvmResult, ThreadContext, ThreadId};

#[async_trait::async_trait(?Send)]
pub trait JvmDetail {
    async fn load_class(&mut self, class_name: &str) -> JvmResult<Option<ClassRef>>;
    async fn load_array_class(&mut self, element_type_name: &str) -> JvmResult<Option<Box<dyn ArrayClass>>>;

    fn get_class(&self, class_name: &str) -> JvmResult<Option<ClassRef>>;

    fn thread_context(&mut self, thread_id: ThreadId) -> &mut dyn ThreadContext;
}