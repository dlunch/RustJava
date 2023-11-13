use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use crate::{method::Method, JvmResult};

use cafebabe::parse_class;

pub struct Class {
    pub name: String,

    pub methods: Vec<Method>,
}

impl Class {
    pub fn from_classfile(data: &[u8]) -> JvmResult<Self> {
        let class = parse_class(data)?;

        let name = class.this_class.to_string();
        let methods = class.methods.iter().map(Method::from_methodinfo).collect::<Vec<_>>();

        Ok(Self { name, methods })
    }

    pub fn method(&self, name: &str, signature: &str) -> Option<&Method> {
        self.methods.iter().find(|&method| method.name == name && method.signature == signature)
    }
}
