use alloc::{boxed::Box, collections::BTreeMap};

use jvm::{Class, JvmDetail, JvmResult, ThreadContext, ThreadId};

use crate::{array_class::ArrayClassImpl, thread::ThreadContextImpl, ClassImpl};

#[derive(Default)]
pub struct JvmDetailImpl {
    thread_contexts: BTreeMap<ThreadId, Box<dyn ThreadContext>>,
}

impl JvmDetailImpl {
    pub fn new() -> Self {
        Self {
            thread_contexts: BTreeMap::new(),
        }
    }
}

#[async_trait::async_trait(?Send)]
impl JvmDetail for JvmDetailImpl {
    async fn define_class(&self, _name: &str, data: &[u8]) -> JvmResult<Box<dyn Class>> {
        ClassImpl::from_classfile(data).map(|x| Box::new(x) as Box<_>)
    }

    async fn define_array_class(&self, element_type_name: &str) -> JvmResult<Box<dyn Class>> {
        Ok(Box::new(ArrayClassImpl::new(element_type_name)))
    }

    fn thread_context(&mut self, thread_id: ThreadId) -> Box<dyn ThreadContext> {
        let thread_context = self
            .thread_contexts
            .entry(thread_id)
            .or_insert_with(|| Box::new(ThreadContextImpl::new()));

        thread_context.clone()
    }
}
