use alloc::{
    collections::BTreeMap,
    rc::Rc,
    string::{String, ToString},
    vec::Vec,
};
use core::cell::RefCell;

use crate::{method::Method, value::JavaValue, JvmResult};

use classfile::ClassInfo;

pub struct Class {
    pub name: String,
    pub methods: Vec<Method>,
}

impl Class {
    pub fn from_classfile(data: &[u8]) -> JvmResult<Self> {
        let class = ClassInfo::parse(data)?;

        Ok(Self {
            name: class.this_class.to_string(),
            methods: class.methods.into_iter().map(Method::from_methodinfo).collect::<Vec<_>>(),
        })
    }

    pub fn method(&self, name: &str, descriptor: &str) -> Option<&Method> {
        self.methods.iter().find(|&method| method.name == name && method.descriptor == descriptor)
    }
}

pub struct LoadedClass {
    pub class: Class,
    pub storage: BTreeMap<String, JavaValue>,
}

pub struct ClassInstance {
    pub class: Rc<RefCell<LoadedClass>>,
    pub storage: BTreeMap<String, JavaValue>,
}
