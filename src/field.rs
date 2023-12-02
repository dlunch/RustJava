use alloc::string::{String, ToString};

use classfile::FieldInfo;

pub struct Field {
    pub name: String,
    pub descriptor: String,
    pub index: usize,
}

impl Field {
    pub fn from_fieldinfo(field_info: FieldInfo, index: usize) -> Self {
        Self {
            name: field_info.name.to_string(),
            descriptor: field_info.descriptor.to_string(),
            index,
        }
    }
}
