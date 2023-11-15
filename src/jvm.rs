use alloc::{boxed::Box, collections::BTreeMap, vec::Vec};

use crate::{
    class::Class,
    class_loader::ClassLoader,
    interpreter::Interpreter,
    thread::{ThreadContext, ThreadId},
    JvmResult,
};

#[derive(Default)]
pub struct Jvm {
    class_loaders: Vec<Box<dyn ClassLoader>>,
    thread_contexts: BTreeMap<ThreadId, ThreadContext>,
}

impl Jvm {
    pub fn new() -> Self {
        let thread_contexts = [(Self::current_thread_id(), ThreadContext::new())].into_iter().collect();

        Self {
            class_loaders: Vec::new(),
            thread_contexts,
        }
    }

    pub fn add_class_loader<T>(&mut self, class_loader: T)
    where
        T: ClassLoader + 'static,
    {
        self.class_loaders.push(Box::new(class_loader));
    }

    pub fn invoke_static_method(&mut self, class_name: &str, name: &str, signature: &str) -> JvmResult<()> {
        let class = self.resolve_class(class_name)?.unwrap();
        let method = class.method(name, signature).unwrap();

        self.current_thread_context().push_stack_frame();
        Interpreter::run(self, &class.constant_pool, &method.body)
    }

    pub(crate) fn current_thread_context(&mut self) -> &mut ThreadContext {
        self.thread_contexts.get_mut(&Jvm::current_thread_id()).unwrap()
    }

    fn current_thread_id() -> ThreadId {
        0 // TODO
    }

    fn resolve_class(&mut self, class_name: &str) -> JvmResult<Option<Class>> {
        for class_loader in &mut self.class_loaders {
            if let Some(x) = class_loader.load(class_name)? {
                return Ok(Some(x));
            }
        }

        Ok(None)
    }
}
