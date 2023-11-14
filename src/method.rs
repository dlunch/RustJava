use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use classfile::{AttributeInfoCode, ClassInfo, MethodInfo};

pub struct Method {
    pub name: String,
    pub signature: String,
    pub body: Vec<u8>,
}

impl Method {
    pub fn from_methodinfo(class: &ClassInfo, method_info: &MethodInfo) -> Self {
        let name = class.constant_utf8(method_info.name_index).unwrap().to_string();
        let signature = class.constant_utf8(method_info.descriptor_index).unwrap().to_string();

        let body = Self::extract_body(class, method_info).unwrap();

        Self { name, signature, body }
    }

    fn extract_body(class_file: &ClassInfo, method_info: &MethodInfo) -> Option<Vec<u8>> {
        for attribute in &method_info.attributes {
            let name = class_file.constant_utf8(attribute.attribute_name_index).unwrap();
            if name == "Code" {
                let attribute = AttributeInfoCode::parse(&attribute.info).unwrap();

                return Some(attribute.code);
            }
        }

        None
    }
}
