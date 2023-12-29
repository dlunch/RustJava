use alloc::{boxed::Box, rc::Rc, vec::Vec};
use core::cell::RefCell;

use crate::{
    class::Class,
    class_instance::ClassInstance,
    detail::JvmDetail,
    field::Field,
    thread::{ThreadContext, ThreadId},
    value::JavaValue,
    JvmResult,
};

pub type ClassInstanceRef = Rc<RefCell<Box<dyn ClassInstance>>>;
pub type ClassRef = Rc<RefCell<Box<dyn Class>>>;

pub struct Jvm {
    detail: Box<dyn JvmDetail>,
}

impl Jvm {
    pub fn new<T>(detail: T) -> Self
    where
        T: JvmDetail + 'static,
    {
        Self { detail: Box::new(detail) }
    }

    pub async fn instantiate_class(&mut self, class_name: &str) -> JvmResult<ClassInstanceRef> {
        let class = self.resolve_class(class_name).await?.unwrap();
        let class = class.borrow();

        let instance = Rc::new(RefCell::new(class.instantiate()));

        Ok(instance)
    }

    pub async fn instantiate_array(&mut self, element_type_name: &str, length: usize) -> JvmResult<ClassInstanceRef> {
        let array_class = self.detail.load_array_class(element_type_name).await?.unwrap();

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

    pub fn get_field(&self, instance: &ClassInstanceRef, name: &str, descriptor: &str) -> JvmResult<JavaValue> {
        let instance = instance.borrow();
        let field = self.find_field(&instance.class_name(), name, descriptor).unwrap();

        instance.get_field(&*field)
    }

    pub fn put_field(&mut self, instance: &ClassInstanceRef, name: &str, descriptor: &str, value: JavaValue) -> JvmResult<()> {
        let mut instance = instance.borrow_mut();
        let field = self.find_field(&instance.class_name(), name, descriptor).unwrap();

        instance.put_field(&*field, value)
    }

    pub async fn invoke_static_method<T>(&mut self, class_name: &str, name: &str, descriptor: &str, args: T) -> JvmResult<JavaValue>
    where
        T: InvokeArg,
    {
        let class = self.resolve_class(class_name).await?.unwrap();
        let class = class.borrow();
        let method = class.method(name, descriptor).unwrap();

        method.run(self, args.into_arg()).await
    }

    pub async fn invoke_method<T>(
        &mut self,
        instance: &ClassInstanceRef,
        _class_name: &str,
        name: &str,
        descriptor: &str,
        args: T,
    ) -> JvmResult<JavaValue>
    where
        T: InvokeArg,
    {
        let class = self.resolve_class(&instance.borrow().class_name()).await?.unwrap();
        let class = class.borrow();
        let method = class.method(name, descriptor).unwrap();

        let args = [JavaValue::Object(Some(instance.clone()))]
            .into_iter()
            .chain(args.into_iter())
            .collect::<Vec<_>>();

        method.run(self, args.into_boxed_slice()).await
    }

    // non-virtual
    pub async fn invoke_special<T>(
        &mut self,
        instance: &ClassInstanceRef,
        class_name: &str,
        name: &str,
        descriptor: &str,
        args: T,
    ) -> JvmResult<JavaValue>
    where
        T: InvokeArg,
    {
        let class = self.resolve_class(class_name).await?.unwrap();
        let class = class.borrow();
        let method = class.method(name, descriptor).unwrap();

        let args = [JavaValue::Object(Some(instance.clone()))]
            .into_iter()
            .chain(args.into_iter())
            .collect::<Vec<_>>();

        method.run(self, args.into_boxed_slice()).await
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
        self.detail.thread_context(Jvm::current_thread_id())
    }

    // temporary until we have working gc
    pub fn destroy(&mut self, instance: ClassInstanceRef) -> JvmResult<()> {
        let instance = Rc::into_inner(instance).unwrap().into_inner();

        instance.destroy();

        Ok(())
    }

    fn get_class(&self, class_name: &str) -> Option<ClassRef> {
        self.detail.get_class(class_name).unwrap()
    }

    async fn resolve_class(&mut self, class_name: &str) -> JvmResult<Option<ClassRef>> {
        let class = self.get_class(class_name);
        if let Some(x) = class {
            return Ok(Some(x));
        }

        if let Some(x) = self.detail.load_class(class_name).await? {
            let class = x.borrow();
            let clinit = class.method("<clinit>", "()V");
            drop(class);

            if let Some(x) = clinit {
                x.run(self, Box::new([])).await?;
            }

            let class = self.get_class(class_name).unwrap();

            return Ok(Some(class));
        }

        Ok(None)
    }

    fn find_field(&self, class_name: &str, name: &str, descriptor: &str) -> Option<Box<dyn Field>> {
        let class = self.get_class(class_name).unwrap();
        let field = class.borrow().field(name, descriptor, false);

        if let Some(x) = field {
            Some(x)
        } else if let Some(x) = class.borrow().super_class_name() {
            self.find_field(&x, name, descriptor)
        } else {
            None
        }
    }

    fn current_thread_id() -> ThreadId {
        0 // TODO
    }
}

pub trait InvokeArg: IntoIterator<Item = JavaValue> {
    fn into_arg(self) -> Box<[JavaValue]>;
}

impl InvokeArg for Vec<JavaValue> {
    fn into_arg(self) -> Box<[JavaValue]> {
        self.into_boxed_slice()
    }
}

impl<const N: usize> InvokeArg for [JavaValue; N] {
    fn into_arg(self) -> Box<[JavaValue]> {
        self.into()
    }
}
