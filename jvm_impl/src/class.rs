use alloc::{
    boxed::Box,
    rc::Rc,
    string::{String, ToString},
    vec::Vec,
};
use core::cell::RefCell;

use dyn_clone::clone_box;

use classfile::ClassInfo;
use java_runtime_base::JavaClassProto;
use jvm::{Class, ClassInstance, Field, JavaValue, JvmResult, Method};

use crate::{class_instance::ClassInstanceImpl, field::FieldImpl, method::MethodImpl};

#[derive(Debug)]
struct ClassInner {
    name: String,
    super_class: Option<Box<dyn Class>>,
    methods: Vec<MethodImpl>,
    fields: Vec<FieldImpl>,
    storage: RefCell<Vec<JavaValue>>,
}

#[derive(Debug, Clone)]
pub struct ClassImpl {
    inner: Rc<ClassInner>,
}

impl ClassImpl {
    pub fn new(name: &str, super_class: Option<Box<dyn Class>>, methods: Vec<MethodImpl>, fields: Vec<FieldImpl>) -> Self {
        let storage = fields.iter().filter(|x| x.is_static()).map(|x| x.r#type().default()).collect();

        Self {
            inner: Rc::new(ClassInner {
                name: name.to_string(),
                super_class,
                methods,
                fields,
                storage: RefCell::new(storage),
            }),
        }
    }

    pub fn from_class_proto<C>(name: &str, proto: JavaClassProto<C>, context: C) -> Self
    where
        C: Clone + 'static,
    {
        let methods = proto
            .methods
            .into_iter()
            .map(|x| MethodImpl::from_method_proto(x, context.clone()))
            .collect::<Vec<_>>();
        let fields = proto
            .fields
            .into_iter()
            .enumerate()
            .map(|(i, x)| FieldImpl::from_field_proto(x, i))
            .collect::<Vec<_>>();

        Self::new(name, None, methods, fields)
    }

    pub fn from_classfile(data: &[u8]) -> JvmResult<Self> {
        let class = ClassInfo::parse(data)?;

        let fields = class
            .fields
            .into_iter()
            .enumerate()
            .map(|(i, x)| FieldImpl::from_fieldinfo(x, i))
            .collect::<Vec<_>>();

        let methods = class.methods.into_iter().map(MethodImpl::from_methodinfo).collect::<Vec<_>>();

        let super_class = None; // TODO

        Ok(Self::new(&class.this_class, super_class, methods, fields))
    }

    pub fn fields(&self) -> &[FieldImpl] {
        &self.inner.fields
    }
}

impl Class for ClassImpl {
    fn name(&self) -> String {
        self.inner.name.clone()
    }

    fn super_class(&self) -> Option<Box<dyn Class>> {
        self.inner.super_class.as_ref().map(|x| clone_box(&**x))
    }

    fn instantiate(&self) -> Box<dyn ClassInstance> {
        Box::new(ClassInstanceImpl::new(self))
    }

    fn method(&self, name: &str, descriptor: &str) -> Option<Box<dyn Method>> {
        self.inner
            .methods
            .iter()
            .find(|&method| method.name() == name && method.descriptor() == descriptor)
            .map(|x| Box::new(x.clone()) as Box<dyn Method>)
    }

    fn field(&self, name: &str, descriptor: &str, is_static: bool) -> Option<Box<dyn Field>> {
        self.inner
            .fields
            .iter()
            .find(|&field| field.name() == name && field.descriptor() == descriptor && field.is_static() == is_static)
            .map(|x| Box::new(x.clone()) as Box<dyn Field>)
    }

    fn get_static_field(&self, field: &dyn Field) -> JvmResult<JavaValue> {
        let field = field.as_any().downcast_ref::<FieldImpl>().unwrap();

        Ok(self.inner.storage.borrow()[field.index()].clone())
    }

    fn put_static_field(&mut self, field: &dyn Field, value: JavaValue) -> JvmResult<()> {
        let field = field.as_any().downcast_ref::<FieldImpl>().unwrap();

        self.inner.storage.borrow_mut()[field.index()] = value;

        Ok(())
    }
}
