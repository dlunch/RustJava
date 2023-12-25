use alloc::{boxed::Box, collections::BTreeMap, rc::Rc, vec::Vec};
use core::cell::RefCell;

use crate::{
    class::Class,
    class_instance::ClassInstance,
    class_loader::ClassLoader,
    class_registry::ClassRegistry,
    thread::{ThreadContext, ThreadContextProvider, ThreadId},
    JavaValue, JvmResult,
};

pub type ClassInstanceRef = Rc<RefCell<Box<dyn ClassInstance>>>;
pub type ClassRef = Rc<RefCell<Box<dyn Class>>>;

pub trait JvmDetail {
    fn class_loader(&mut self) -> &mut dyn ClassLoader;
    fn class_registry_mut(&mut self) -> &mut dyn ClassRegistry;
    fn class_registry(&self) -> &dyn ClassRegistry;
    fn thread_context_provider(&self) -> &dyn ThreadContextProvider;
}

pub struct Jvm {
    detail: Box<dyn JvmDetail>,
    thread_contexts: BTreeMap<ThreadId, Box<dyn ThreadContext>>,
}

impl Jvm {
    pub fn new<T>(detail: T) -> Self
    where
        T: JvmDetail + 'static,
    {
        let thread_context = detail.thread_context_provider().thread_context(Self::current_thread_id());
        let thread_contexts = [(Self::current_thread_id(), thread_context)].into_iter().collect();

        Self {
            detail: Box::new(detail),
            thread_contexts,
        }
    }

    pub async fn instantiate_class(&mut self, class_name: &str) -> JvmResult<ClassInstanceRef> {
        let class = self.resolve_class(class_name).await?.unwrap();
        let class = class.borrow();

        let instance = Rc::new(RefCell::new(class.instantiate()));

        Ok(instance)
    }

    pub fn instantiate_array(&mut self, element_type_name: &str, length: usize) -> JvmResult<ClassInstanceRef> {
        let array_class = self.detail.class_loader().load_array_class(element_type_name)?.unwrap();

        let instance = Rc::new(RefCell::new(array_class.instantiate_array(length)));

        Ok(instance)
    }

    pub async fn get_static_field(&mut self, class_name: &str, name: &str, descriptor: &str) -> JvmResult<JavaValue> {
        let class = self.resolve_class(class_name).await?.unwrap();
        let class = class.borrow();

        let field = class.field(name, descriptor, true).unwrap();

        class.get_static_field(&*field)
    }

    pub async fn put_static_field(&mut self, class_name: &str, name: &str, descriptor: &str, value: JavaValue) -> JvmResult<()> {
        let class = self.resolve_class(class_name).await?.unwrap();
        let mut class = class.borrow_mut();

        let field = class.field(name, descriptor, true).unwrap();

        class.put_static_field(&*field, value)
    }

    pub fn get_field(&mut self, instance: &ClassInstanceRef, name: &str, descriptor: &str) -> JvmResult<JavaValue> {
        let instance = instance.borrow();
        let class = self.get_class(&instance.class_name()).unwrap();
        let field = class.borrow().field(name, descriptor, false).unwrap();

        instance.get_field(&*field)
    }

    pub fn put_field(&mut self, instance: &ClassInstanceRef, name: &str, descriptor: &str, value: JavaValue) -> JvmResult<()> {
        let mut instance = instance.borrow_mut();
        let class = self.get_class(&instance.class_name()).unwrap();
        let field = class.borrow().field(name, descriptor, false).unwrap();

        instance.put_field(&*field, value)
    }

    pub async fn invoke_static_method(&mut self, class_name: &str, name: &str, descriptor: &str, args: &[JavaValue]) -> JvmResult<JavaValue> {
        let class = self.resolve_class(class_name).await?.unwrap();
        let class = class.borrow();
        let method = class.method(name, descriptor).unwrap();

        method.run(self, args).await
    }

    pub async fn invoke_method(
        &mut self,
        instance: &ClassInstanceRef,
        _class_name: &str,
        name: &str,
        descriptor: &str,
        args: &[JavaValue],
    ) -> JvmResult<JavaValue> {
        let class = self.resolve_class(&instance.borrow().class_name()).await?.unwrap();
        let class = class.borrow();
        let method = class.method(name, descriptor).unwrap();

        let args = [JavaValue::Object(Some(instance.clone()))]
            .iter()
            .chain(args.iter())
            .cloned()
            .collect::<Vec<_>>();

        method.run(self, &args).await
    }

    pub fn store_array(&mut self, array: &ClassInstanceRef, offset: usize, values: &[JavaValue]) -> JvmResult<()> {
        let mut array = array.borrow_mut();
        let array = array.as_array_instance_mut().unwrap();

        array.store(offset, values)
    }

    pub fn load_array(&self, array: &ClassInstanceRef, offset: usize, count: usize) -> JvmResult<Vec<JavaValue>> {
        let array = array.borrow();
        let array = array.as_array_instance().unwrap();

        array.load(offset, count)
    }

    pub fn array_length(&self, array: &ClassInstanceRef) -> JvmResult<usize> {
        let array = array.borrow();
        let array = array.as_array_instance().unwrap();

        Ok(array.length())
    }

    pub fn current_thread_context(&mut self) -> &mut dyn ThreadContext {
        self.thread_contexts.get_mut(&Jvm::current_thread_id()).unwrap().as_mut()
    }

    fn get_class(&self, class_name: &str) -> Option<ClassRef> {
        self.detail.class_registry().get_class(class_name).unwrap()
    }

    async fn resolve_class(&mut self, class_name: &str) -> JvmResult<Option<ClassRef>> {
        let class = self.get_class(class_name);
        if let Some(x) = class {
            return Ok(Some(x));
        }

        if let Some(x) = self.detail.class_loader().load(class_name)? {
            self.load_class(class_name, x).await?;
            let class = self.get_class(class_name).unwrap();

            return Ok(Some(class));
        }

        Ok(None)
    }

    async fn load_class(&mut self, class_name: &str, class: Box<dyn Class>) -> JvmResult<()> {
        self.detail.class_registry_mut().register_class(class);
        let class = self.detail.class_registry().get_class(class_name)?.unwrap();
        let class = class.borrow();

        let clinit = class.method("<clinit>", "()V");
        drop(class);

        if let Some(x) = clinit {
            x.run(self, &[]).await?;
        }

        Ok(())
    }

    fn current_thread_id() -> ThreadId {
        0 // TODO
    }
}
