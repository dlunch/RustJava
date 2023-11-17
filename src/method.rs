use alloc::{
    boxed::Box,
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

use classfile::{AttributeInfo, MethodInfo, Opcode};

use crate::{interpreter::Interpreter, value::JavaValue, Jvm, JvmResult};

pub enum MethodBody {
    ByteCode(BTreeMap<u32, Opcode>),
    Rust(Box<dyn Fn() -> JavaValue>),
}

pub struct Method {
    pub name: String,
    pub descriptor: String,
    pub body: MethodBody,
}

impl Method {
    pub fn from_methodinfo(method_info: MethodInfo) -> Self {
        Self {
            name: method_info.name.to_string(),
            descriptor: method_info.descriptor.to_string(),
            body: MethodBody::ByteCode(Self::extract_body(method_info.attributes).unwrap()),
        }
    }

    pub fn run(&self, jvm: &mut Jvm, _parameters: Vec<JavaValue>) -> JvmResult<JavaValue> {
        Ok(match &self.body {
            MethodBody::ByteCode(x) => Interpreter::run(jvm, x)?,
            MethodBody::Rust(x) => x(),
        })
    }

    fn extract_body(attributes: Vec<AttributeInfo>) -> Option<BTreeMap<u32, Opcode>> {
        for attribute in attributes {
            if let AttributeInfo::Code(x) = attribute {
                return Some(x.code);
            }
        }

        None
    }
}
