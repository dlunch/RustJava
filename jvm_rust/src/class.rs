use alloc::{
    boxed::Box,
    collections::BTreeMap,
    rc::Rc,
    string::{String, ToString},
    vec::Vec,
};
use core::{
    cell::RefCell,
    ops::{Deref, DerefMut},
};

use classfile::ClassInfo;
use java_class_proto::JavaClassProto;
use java_constants::FieldAccessFlags;
use jvm::{Class, ClassInstance, Field, JavaValue, JvmResult, Method};

use crate::{class_instance::ClassInstanceImpl, field::FieldImpl, method::MethodImpl};

#[derive(Debug)]
struct ClassInner {
    name: String,
    super_class_name: Option<String>,
    methods: Vec<MethodImpl>,
    fields: Vec<FieldImpl>,
    storage: RefCell<BTreeMap<FieldImpl, JavaValue>>, // TODO we should use field offset or something
}

#[derive(Debug, Clone)]
pub struct ClassImpl {
    inner: Rc<ClassInner>,
}

impl ClassImpl {
    pub fn new(name: &str, super_class_name: Option<String>, methods: Vec<MethodImpl>, fields: Vec<FieldImpl>) -> Self {
        Self {
            inner: Rc::new(ClassInner {
                name: name.to_string(),
                super_class_name,
                methods,
                fields,
                storage: RefCell::new(BTreeMap::new()),
            }),
        }
    }

    pub fn from_class_proto<C, Context>(name: &str, proto: JavaClassProto<C>, context: Context) -> Self
    where
        C: ?Sized + 'static,
        Context: DerefMut + Deref<Target = C> + Clone + 'static,
    {
        let methods = proto
            .methods
            .into_iter()
            .map(|x| MethodImpl::from_method_proto(x, context.clone()))
            .collect::<Vec<_>>();

        let fields = proto.fields.into_iter().map(FieldImpl::from_field_proto).collect::<Vec<_>>();

        Self::new(name, proto.parent_class.map(|x| x.to_string()), methods, fields)
    }

    pub fn from_classfile(data: &[u8]) -> JvmResult<Self> {
        let class = ClassInfo::parse(data)?;

        let fields = class.fields.into_iter().map(FieldImpl::from_fieldinfo).collect::<Vec<_>>();

        let methods = class.methods.into_iter().map(MethodImpl::from_methodinfo).collect::<Vec<_>>();

        Ok(Self::new(&class.this_class, class.super_class.map(|x| x.to_string()), methods, fields))
    }

    pub fn fields(&self) -> &[FieldImpl] {
        &self.inner.fields
    }
}

impl Class for ClassImpl {
    fn name(&self) -> String {
        self.inner.name.clone()
    }

    fn super_class_name(&self) -> Option<String> {
        self.inner.super_class_name.as_ref().map(|x| x.to_string())
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
            .find(|&field| {
                field.name() == name && field.descriptor() == descriptor && field.access_flags().contains(FieldAccessFlags::STATIC) == is_static
            })
            .map(|x| Box::new(x.clone()) as Box<dyn Field>)
    }

    fn get_static_field(&self, field: &dyn Field) -> JvmResult<JavaValue> {
        let field = field.as_any().downcast_ref::<FieldImpl>().unwrap();

        let storage = self.inner.storage.borrow();
        let value = storage.get(field);

        if let Some(x) = value {
            Ok(x.clone())
        } else {
            Ok(field.r#type().default())
        }
    }

    fn put_static_field(&mut self, field: &dyn Field, value: JavaValue) -> JvmResult<()> {
        let field = field.as_any().downcast_ref::<FieldImpl>().unwrap();

        self.inner.storage.borrow_mut().insert(field.clone(), value);

        Ok(())
    }
}
