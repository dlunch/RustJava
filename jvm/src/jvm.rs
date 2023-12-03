use alloc::{
    boxed::Box,
    collections::BTreeMap,
    format,
    rc::Rc,
    string::{String, ToString},
    vec::Vec,
};
use core::cell::RefCell;

use crate::{
    class::Class,
    class_instance::ClassInstance,
    class_loader::ClassLoader,
    thread::{ThreadContext, ThreadContextProvider, ThreadId},
    value::JavaValue,
    JvmResult,
};

#[derive(Default)]
pub struct Jvm {
    class_loaders: Vec<Box<dyn ClassLoader>>,
    thread_contexts: BTreeMap<ThreadId, Box<dyn ThreadContext>>,
    loaded_classes: BTreeMap<String, Rc<RefCell<Box<dyn Class>>>>,
    class_instances: Vec<Rc<RefCell<Box<dyn ClassInstance>>>>,
}

impl Jvm {
    pub fn new(context_provider: &dyn ThreadContextProvider) -> Self {
        let thread_contexts = [(Self::current_thread_id(), context_provider.thread_context(Self::current_thread_id()))]
            .into_iter()
            .collect();

        Self {
            class_loaders: Vec::new(),
            thread_contexts,
            loaded_classes: BTreeMap::new(),
            class_instances: Vec::new(),
        }
    }

    pub fn add_class_loader<T>(&mut self, class_loader: T)
    where
        T: ClassLoader + 'static,
    {
        self.class_loaders.push(Box::new(class_loader));
    }

    pub fn invoke_static_method(&mut self, class_name: &str, name: &str, signature: &str) -> JvmResult<JavaValue> {
        let class = self.find_class(class_name)?.unwrap();
        let class = class.borrow();
        let method = class.method(name, signature).unwrap();

        method.run(self, &Vec::new())
    }

    pub fn invoke_method(&mut self, class_instance: &Rc<RefCell<Box<dyn ClassInstance>>>, name: &str, signature: &str) -> JvmResult<JavaValue> {
        let class_instance = class_instance.borrow();
        let class_name = class_instance.class_name();
        let class = self.find_class(class_name)?.unwrap();
        let class = class.borrow();

        let method = class.method(name, signature).unwrap();

        method.run(self, &Vec::new())
    }

    pub fn instantiate_class(&mut self, class_name: &str) -> JvmResult<Rc<RefCell<Box<dyn ClassInstance>>>> {
        let class = self.find_class(class_name)?.unwrap();

        let class_instance = Rc::new(RefCell::new(class.borrow().instantiate()));

        self.class_instances.push(class_instance.clone());

        Ok(class_instance)
    }

    pub fn instantiate_array(&mut self, element_type_name: &str, _count: usize) -> JvmResult<Rc<RefCell<Box<dyn ClassInstance>>>> {
        let class_name = format!("[{}", element_type_name);

        let class_instance = self.instantiate_class(&class_name)?;

        Ok(class_instance)
    }

    pub fn get_static_field(&mut self, class_name: &str, field_name: &str, descriptor: &str) -> JvmResult<JavaValue> {
        let class = self.find_class(class_name)?.unwrap();
        let class = class.borrow();
        let field = class.field(field_name, descriptor, true).unwrap();

        class.get_static_field(field)
    }

    pub fn current_thread_context(&mut self) -> &mut dyn ThreadContext {
        self.thread_contexts.get_mut(&Jvm::current_thread_id()).unwrap().as_mut()
    }

    fn current_thread_id() -> ThreadId {
        0 // TODO
    }

    #[allow(clippy::type_complexity)] // TODO
    fn find_class(&mut self, class_name: &str) -> JvmResult<Option<Rc<RefCell<Box<dyn Class>>>>> {
        if self.loaded_classes.contains_key(class_name) {
            return Ok(self.loaded_classes.get(class_name).cloned());
        }

        for class_loader in &mut self.class_loaders {
            if let Some(x) = class_loader.load(class_name)? {
                self.loaded_classes.insert(class_name.to_string(), Rc::new(RefCell::new(x)));

                return Ok(self.loaded_classes.get(class_name).cloned());
            }
        }

        Ok(None)
    }
}
