use alloc::boxed::Box;

use crate::{Class, JvmResult, ThreadContext, ThreadId};

pub trait JvmDetail {
    fn define_class(&self, name: &str, data: &[u8]) -> JvmResult<Box<dyn Class>>;
    fn define_array_class(&self, element_type_name: &str) -> JvmResult<Box<dyn Class>>;

    fn thread_context(&mut self, thread_id: ThreadId) -> &mut dyn ThreadContext;
}
