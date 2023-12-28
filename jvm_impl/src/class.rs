use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};

use classfile::ClassInfo;
use jvm::{Class, ClassInstance, Field, JavaValue, JvmResult, Method};

use crate::{class_instance::ClassInstanceImpl, field::FieldImpl, method::MethodImpl};

#[derive(Debug)]
pub struct ClassImpl {
    name: String,
    methods: Vec<MethodImpl>,
    fields: Vec<FieldImpl>,
    storage: Vec<JavaValue>,
}

impl ClassImpl {
    pub fn new(name: &str, methods: Vec<MethodImpl>, fields: Vec<FieldImpl>) -> Self {
        let storage = fields.iter().filter(|x| x.is_static()).map(|x| x.r#type().default()).collect();

        Self {
            name: name.to_string(),
            methods,
            fields,
            storage,
        }
    }

    pub fn from_classfile(data: &[u8]) -> JvmResult<Self> {
        let class = ClassInfo::parse(data)?;

        let fields = class
            .fields
            .into_iter()
            .scan(0, |index, field| {
                let field = FieldImpl::from_fieldinfo(field, *index);
                *index += 1;

                Some(field)
            })
            .collect::<Vec<_>>();

        let methods = class.methods.into_iter().map(MethodImpl::from_methodinfo).collect::<Vec<_>>();

        Ok(Self::new(&class.this_class, methods, fields))
    }

    pub fn fields(&self) -> &[FieldImpl] {
        &self.fields
    }
}

impl Class for ClassImpl {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn super_class_name(&self) -> Option<String> {
        Some("java/lang/Object".to_string()) // TODO
    }

    fn instantiate(&self) -> Box<dyn ClassInstance> {
        Box::new(ClassInstanceImpl::new(self))
    }

    fn method(&self, name: &str, descriptor: &str) -> Option<Box<dyn Method>> {
        self.methods
            .iter()
            .find(|&method| method.name() == name && method.descriptor() == descriptor)
            .map(|x| Box::new(x.clone()) as Box<dyn Method>)
    }

    fn field(&self, name: &str, descriptor: &str, is_static: bool) -> Option<Box<dyn Field>> {
        self.fields
            .iter()
            .find(|&field| field.name() == name && field.descriptor() == descriptor && field.is_static() == is_static)
            .map(|x| Box::new(x.clone()) as Box<dyn Field>)
    }

    fn get_static_field(&self, field: &dyn Field) -> JvmResult<JavaValue> {
        let field = field.as_any().downcast_ref::<FieldImpl>().unwrap();

        Ok(self.storage[field.index()].clone())
    }

    fn put_static_field(&mut self, field: &dyn Field, value: JavaValue) -> JvmResult<()> {
        let field = field.as_any().downcast_ref::<FieldImpl>().unwrap();

        self.storage[field.index()] = value;

        Ok(())
    }
}
