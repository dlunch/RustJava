use alloc::{
    boxed::Box,
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

use classfile::{AttributeInfo, MethodInfo, Opcode};
use jvm::{JavaValue, Jvm, JvmResult, Method};

use crate::interpreter::Interpreter;

pub enum MethodBody {
    ByteCode(BTreeMap<u32, Opcode>),
    Rust(Box<dyn Fn() -> JavaValue>),
}

pub struct MethodImpl {
    name: String,
    descriptor: String,
    body: MethodBody,
}

impl MethodImpl {
    pub fn new(name: &str, descriptor: &str, body: MethodBody) -> Self {
        Self {
            name: name.to_string(),
            descriptor: descriptor.to_string(),
            body,
        }
    }

    pub fn from_methodinfo(method_info: MethodInfo) -> Self {
        Self {
            name: method_info.name.to_string(),
            descriptor: method_info.descriptor.to_string(),
            body: MethodBody::ByteCode(Self::extract_body(method_info.attributes).unwrap()),
        }
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

impl Method for MethodImpl {
    fn name(&self) -> &str {
        &self.name
    }

    fn descriptor(&self) -> &str {
        &self.descriptor
    }

    fn run(&self, jvm: &mut Jvm, _parameters: &[JavaValue]) -> JvmResult<JavaValue> {
        Ok(match &self.body {
            MethodBody::ByteCode(x) => Interpreter::run(jvm, x)?,
            MethodBody::Rust(x) => x(),
        })
    }
}
