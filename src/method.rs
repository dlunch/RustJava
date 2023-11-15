use alloc::{string::String, vec::Vec};

use classfile::{AttributeInfo, MethodInfo, Opcode};

pub struct Method {
    pub name: String,
    pub signature: String,
    pub body: Vec<Opcode>,
}

impl Method {
    pub fn from_methodinfo(method_info: &MethodInfo) -> Self {
        let body = Self::extract_body(method_info).unwrap();

        Self {
            name: method_info.name.clone(),
            signature: method_info.descriptor.clone(),
            body,
        }
    }

    fn extract_body(method_info: &MethodInfo) -> Option<Vec<Opcode>> {
        for attribute in &method_info.attributes {
            if let AttributeInfo::Code(x) = attribute {
                return Some(x.code.clone());
            }
        }

        None
    }
}
