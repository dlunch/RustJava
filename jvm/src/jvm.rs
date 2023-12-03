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
    thread::{ThreadContext, ThreadContextProvider, ThreadId},
    JavaValue, JvmResult,
};

pub type ClassInstanceRef = Rc<RefCell<Box<dyn ClassInstance>>>;
pub type ClassRef = Rc<RefCell<Box<dyn Class>>>;

pub struct Jvm {
    class_loader: Box<dyn ClassLoader>,
    thread_contexts: BTreeMap<ThreadId, Box<dyn ThreadContext>>,
    loaded_classes: BTreeMap<String, ClassRef>,
    class_instances: Vec<ClassInstanceRef>,
}

impl Jvm {
    pub fn new<T>(class_loader: T, context_provider: &dyn ThreadContextProvider) -> Self
    where
        T: ClassLoader + 'static,
    {
        let thread_contexts = [(Self::current_thread_id(), context_provider.thread_context(Self::current_thread_id()))]
            .into_iter()
            .collect();

        Self {
            class_loader: Box::new(class_loader),
            thread_contexts,
            loaded_classes: BTreeMap::new(),
            class_instances: Vec::new(),
        }
    }

    pub fn instantiate_class(&mut self, class_name: &str, init_descriptor: &str, init_param: &[JavaValue]) -> JvmResult<ClassInstanceRef> {
        let class = self.resolve_class(class_name)?.unwrap();
        let class = class.borrow();

        let instance = Rc::new(RefCell::new(class.instantiate()));

        self.class_instances.push(instance.clone());

        let method = class.method("<init>", init_descriptor).unwrap();
        method.run(self, init_param)?;

        Ok(instance)
    }

    pub fn instantiate_array(&mut self, element_type_name: &str, length: usize) -> JvmResult<ClassInstanceRef> {
        let array_class = self.class_loader.load_array_class(element_type_name)?.unwrap();

        let instance = Rc::new(RefCell::new(array_class.instantiate_array(length)));

        self.class_instances.push(instance.clone());

        Ok(instance)
    }

    pub fn get_static_field(&mut self, class_name: &str, name: &str, descriptor: &str) -> JvmResult<JavaValue> {
        let class = self.resolve_class(class_name)?.unwrap();
        let class = class.borrow();

        let field = class.field(name, descriptor, true).unwrap();

        class.get_static_field(&*field)
    }

    pub fn put_static_field(&mut self, class_name: &str, name: &str, descriptor: &str, value: JavaValue) -> JvmResult<()> {
        let class = self.resolve_class(class_name)?.unwrap();
        let mut class = class.borrow_mut();

        let field = class.field(name, descriptor, true).unwrap();

        class.put_static_field(&*field, value)
    }

    pub fn invoke_static_method(&mut self, class_name: &str, name: &str, descriptor: &str, args: &[JavaValue]) -> JvmResult<JavaValue> {
        let class = self.resolve_class(class_name)?.unwrap();
        let class = class.borrow();
        let method = class.method(name, descriptor).unwrap();

        method.run(self, args)
    }

    pub fn invoke_method(
        &mut self,
        instance: &ClassInstanceRef,
        _class_name: &str,
        name: &str,
        descriptor: &str,
        args: &[JavaValue],
    ) -> JvmResult<JavaValue> {
        let class = self.resolve_class(instance.borrow().class_name())?.unwrap();
        let class = class.borrow();
        let method = class.method(name, descriptor).unwrap();

        method.run(self, args)
    }

    pub fn store_array(&mut self, array: &ClassInstanceRef, offset: usize, values: &[JavaValue]) -> JvmResult<()> {
        let mut array = array.borrow_mut();
        let array = array.as_array_instance_mut().unwrap();

        array.store(offset, values)
    }

    pub fn current_thread_context(&mut self) -> &mut dyn ThreadContext {
        self.thread_contexts.get_mut(&Jvm::current_thread_id()).unwrap().as_mut()
    }

    fn resolve_class(&mut self, class_name: &str) -> JvmResult<Option<ClassRef>> {
        if self.loaded_classes.contains_key(class_name) {
            return Ok(self.loaded_classes.get(class_name).cloned());
        }

        if let Some(x) = self.class_loader.load(class_name)? {
            self.load_class(class_name, x)?;

            return Ok(self.loaded_classes.get(class_name).cloned());
        }

        Ok(None)
    }

    fn load_class(&mut self, class_name: &str, class: Box<dyn Class>) -> JvmResult<()> {
        let class = Rc::new(RefCell::new(class));
        self.loaded_classes.insert(class_name.to_string(), class.clone());

        let clinit = class.borrow().method("<clinit>", "()V");

        if let Some(x) = clinit {
            x.run(self, &[])?;
        }

        Ok(())
    }

    fn current_thread_id() -> ThreadId {
        0 // TODO
    }
}
