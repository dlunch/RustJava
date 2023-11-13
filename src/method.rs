use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use cafebabe::{attributes::AttributeData, MethodInfo};

pub struct Method {
    pub name: String,
    pub signature: String,
    pub body: Vec<u8>,
}

impl Method {
    pub fn from_methodinfo(method_info: &MethodInfo) -> Self {
        let name = method_info.name.to_string();
        let signature = method_info.descriptor.to_string();

        let body = Self::extract_body(method_info).unwrap();

        Self { name, signature, body }
    }

    fn extract_body(method_info: &MethodInfo) -> Option<Vec<u8>> {
        for attribute in &method_info.attributes {
            if attribute.name == "Code" {
                if let AttributeData::Code(x) = &attribute.data {
                    return Some(x.code.to_vec());
                }
            }
        }

        None
    }
}
