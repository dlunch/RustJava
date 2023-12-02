use alloc::string::{String, ToString};

use classfile::{FieldAccessFlags, FieldInfo};

pub struct Field {
    pub name: String,
    pub descriptor: String,
    pub is_static: bool,
    pub index: usize,
}

impl Field {
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
