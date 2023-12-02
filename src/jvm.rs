use alloc::{
    boxed::Box,
    collections::BTreeMap,
    rc::Rc,
    string::{String, ToString},
    vec::Vec,
};
use core::cell::RefCell;

use crate::{
    class::Class,
    class_instance::ClassInstance,
    class_loader::ClassLoader,
    thread::{ThreadContext, ThreadId},
    value::JavaValue,
    ClassDefinition, JvmResult,
};

#[derive(Default)]
pub struct Jvm {
    class_loaders: Vec<Box<dyn ClassLoader>>,
    thread_contexts: BTreeMap<ThreadId, ThreadContext>,
    loaded_classes: BTreeMap<String, Rc<RefCell<Class>>>,
    class_instances: Vec<Rc<RefCell<ClassInstance>>>,
}

impl Jvm {
    pub fn new() -> Self {
        let thread_contexts = [(Self::current_thread_id(), ThreadContext::new())].into_iter().collect();

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
        let class_definition = &class.borrow().class_definition;
        let method = class_definition.method(name, signature).unwrap();

        self.current_thread_context().push_stack_frame();

        method.run(self, Vec::new())
    }

    pub fn invoke_method(&mut self, class_instance: Rc<RefCell<ClassInstance>>, name: &str, signature: &str) -> JvmResult<JavaValue> {
        let class = &class_instance.borrow().class;
        let class_definition = &class.borrow().class_definition;
        let method = class_definition.method(name, signature).unwrap();

        self.current_thread_context().push_stack_frame();

        method.run(self, Vec::new())
    }

    pub fn instantiate_class(&mut self, class_name: &str) -> JvmResult<Rc<RefCell<ClassInstance>>> {
        let class = self.find_class(class_name)?.unwrap();

        let class_instance = ClassInstance::new(class);

        self.class_instances.push(class_instance.clone());

        Ok(class_instance)
    }

    pub fn instantiate_array(&mut self, element_type_name: &str, _count: usize) -> JvmResult<Rc<RefCell<ClassInstance>>> {
        let class = self.find_array_class(element_type_name)?.unwrap();

        let class_instance = ClassInstance::new(class);

        self.class_instances.push(class_instance.clone());

        Ok(class_instance)
    }

    pub fn get_static_field(&mut self, _class_name: &str, _field_name: &str) -> JvmResult<JavaValue> {
        Ok(JavaValue::Void) // TODO
    }

    pub(crate) fn current_thread_context(&mut self) -> &mut ThreadContext {
        self.thread_contexts.get_mut(&Jvm::current_thread_id()).unwrap()
    }

    fn current_thread_id() -> ThreadId {
        0 // TODO
    }

    fn find_class(&mut self, class_name: &str) -> JvmResult<Option<Rc<RefCell<Class>>>> {
        if self.loaded_classes.contains_key(class_name) {
            return Ok(self.loaded_classes.get(class_name).cloned());
        }

        for class_loader in &mut self.class_loaders {
            if let Some(x) = class_loader.load(class_name)? {
                self.loaded_classes.insert(
                    class_name.to_string(),
                    Rc::new(RefCell::new(Class {
                        class_definition: x,
                        storage: Vec::new(),
                    })),
                );

                return Ok(self.loaded_classes.get(class_name).cloned());
            }
        }

        Ok(None)
    }

    fn find_array_class(&mut self, element_type_name: &str) -> JvmResult<Option<Rc<RefCell<Class>>>> {
        let class_name = ClassDefinition::array_class_name(element_type_name);

        if self.loaded_classes.contains_key(&class_name) {
            return Ok(self.loaded_classes.get(&class_name).cloned());
        }

        let class_definition = ClassDefinition::array_class_definition(element_type_name);
        self.loaded_classes.insert(
            class_name.to_string(),
            Rc::new(RefCell::new(Class {
                class_definition,
                storage: Vec::new(),
            })),
        );

        Ok(self.loaded_classes.get(&class_name).cloned())
    }
}
