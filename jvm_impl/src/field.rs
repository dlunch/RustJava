use alloc::string::{String, ToString};

use classfile::{FieldAccessFlags, FieldInfo};

use jvm::{Field, JavaType};

pub struct FieldImpl {
    pub name: String,
    pub descriptor: String,
    pub is_static: bool,
    pub index: usize,
}

impl FieldImpl {
    pub fn new(name: &str, descriptor: &str, is_static: bool, index: usize) -> Self {
        Self {
            name: name.to_string(),
            descriptor: descriptor.to_string(),
            is_static,
            index,
        }
    }

    pub fn from_fieldinfo(field_info: FieldInfo, index: usize) -> Self {
        Self {
            name: field_info.name.to_string(),
            descriptor: field_info.descriptor.to_string(),
            is_static: field_info.access_flags.contains(FieldAccessFlags::STATIC),
            index,
        }
    }
}

impl Field for FieldImpl {
    fn name(&self) -> &str {
        &self.name
    }

    fn descriptor(&self) -> &str {
        &self.descriptor
    }

    fn is_static(&self) -> bool {
        self.is_static
    }

    fn r#type(&self) -> JavaType {
        JavaType::parse(&self.descriptor)
    }
}
