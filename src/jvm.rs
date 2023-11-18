use alloc::{
    boxed::Box,
    collections::BTreeMap,
    rc::Rc,
    string::{String, ToString},
    vec::Vec,
};

use crate::{
    class::LoadedClass,
    class_loader::ClassLoader,
    thread::{ThreadContext, ThreadId},
    JvmResult,
};

#[derive(Default)]
pub struct Jvm {
    class_loaders: Vec<Box<dyn ClassLoader>>,
    thread_contexts: BTreeMap<ThreadId, ThreadContext>,
    loaded_classes: BTreeMap<String, Rc<LoadedClass>>,
}

impl Jvm {
    pub fn new() -> Self {
        let thread_contexts = [(Self::current_thread_id(), ThreadContext::new())].into_iter().collect();

        Self {
            class_loaders: Vec::new(),
            thread_contexts,
            loaded_classes: BTreeMap::new(),
        }
    }

    pub fn add_class_loader<T>(&mut self, class_loader: T)
    where
        T: ClassLoader + 'static,
    {
        self.class_loaders.push(Box::new(class_loader));
    }

    pub fn invoke_static_method(&mut self, class_name: &str, name: &str, signature: &str) -> JvmResult<()> {
        let loaded_class = self.find_class(class_name)?.unwrap();
        let class = &loaded_class.class;
        let method = class.method(name, signature).unwrap();

        self.current_thread_context().push_stack_frame();

        method.run(self, Vec::new())?;

        Ok(())
    }

    pub(crate) fn current_thread_context(&mut self) -> &mut ThreadContext {
        self.thread_contexts.get_mut(&Jvm::current_thread_id()).unwrap()
    }

    fn current_thread_id() -> ThreadId {
        0 // TODO
    }

    fn find_class(&mut self, class_name: &str) -> JvmResult<Option<Rc<LoadedClass>>> {
        if self.loaded_classes.contains_key(class_name) {
            return Ok(self.loaded_classes.get(class_name).cloned());
        }

        for class_loader in &mut self.class_loaders {
            if let Some(x) = class_loader.load(class_name)? {
                self.loaded_classes.insert(
                    class_name.to_string(),
                    Rc::new(LoadedClass {
                        class: x,
                        storage: BTreeMap::new(),
                    }),
                );

                return Ok(self.loaded_classes.get(class_name).cloned());
            }
        }

        Ok(None)
    }
}
