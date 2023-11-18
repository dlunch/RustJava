use alloc::{
    collections::BTreeMap,
    rc::Rc,
    string::{String, ToString},
    vec::Vec,
};
use core::cell::RefCell;

use crate::{field::Field, method::Method, value::JavaValue, JvmResult};

use classfile::ClassInfo;

pub struct ClassDefinition {
    pub name: String,
    pub methods: Vec<Method>,
    pub fields: Vec<Field>,
}

impl ClassDefinition {
    pub fn from_classfile(data: &[u8]) -> JvmResult<Self> {
        let class = ClassInfo::parse(data)?;

        Ok(Self {
            name: class.this_class.to_string(),
            methods: class.methods.into_iter().map(Method::from_methodinfo).collect::<Vec<_>>(),
            fields: class.fields.into_iter().map(Field::from_fieldinfo).collect::<Vec<_>>(),
        })
    }

    pub fn method(&self, name: &str, descriptor: &str) -> Option<&Method> {
        self.methods.iter().find(|&method| method.name == name && method.descriptor == descriptor)
    }
}

pub struct Class {
    pub class_definition: ClassDefinition,
    pub storage: BTreeMap<String, JavaValue>,
}

pub struct ClassInstance {
    pub class: Rc<RefCell<Class>>,
    pub storage: BTreeMap<String, JavaValue>,
}
