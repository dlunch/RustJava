use alloc::{boxed::Box, collections::BTreeMap};

use jvm::{ClassDefinition, Jvm, JvmDetail, JvmResult, ThreadContext, ThreadId};

use crate::{array_class_definition::ArrayClassDefinitionImpl, thread::ThreadContextImpl, ClassDefinitionImpl};

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
    async fn define_class(&self, _jvm: &Jvm, _name: &str, data: &[u8]) -> JvmResult<Box<dyn ClassDefinition>> {
        ClassDefinitionImpl::from_classfile(data).map(|x| Box::new(x) as Box<_>)
    }

    async fn define_array_class(&self, _jvm: &Jvm, element_type_name: &str) -> JvmResult<Box<dyn ClassDefinition>> {
        Ok(Box::new(ArrayClassDefinitionImpl::new(element_type_name)))
    }

    fn thread_context(&mut self, thread_id: ThreadId) -> Box<dyn ThreadContext> {
        let thread_context = self
            .thread_contexts
            .entry(thread_id)
            .or_insert_with(|| Box::new(ThreadContextImpl::new()));

        thread_context.clone()
    }
}
