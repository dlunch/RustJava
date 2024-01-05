use alloc::{
    rc::Rc,
    string::{String, ToString},
};

use classfile::{FieldAccessFlags, FieldInfo};

use jvm::{Field, JavaType};

#[derive(Debug)]
struct FieldDetail {
    name: String,
    descriptor: String,
    is_static: bool,
    index: usize,
}

#[derive(Clone, Debug)]
pub struct FieldImpl {
    detail: Rc<FieldDetail>,
}

impl FieldImpl {
    pub fn new(name: &str, descriptor: &str, is_static: bool, index: usize) -> Self {
        Self {
            detail: Rc::new(FieldDetail {
                name: name.to_string(),
                descriptor: descriptor.to_string(),
                is_static,
                index,
            }),
        }
    }

    pub fn from_fieldinfo(field_info: FieldInfo, index: usize) -> Self {
        Self {
            detail: Rc::new(FieldDetail {
                name: field_info.name.to_string(),
                descriptor: field_info.descriptor.to_string(),
                is_static: field_info.access_flags.contains(FieldAccessFlags::STATIC),
                index,
            }),
        }
    }

    pub fn index(&self) -> usize {
        self.detail.index
    }
}

impl Field for FieldImpl {
    fn name(&self) -> String {
        self.detail.name.clone()
    }

    fn descriptor(&self) -> String {
        self.detail.descriptor.clone()
    }

    fn is_static(&self) -> bool {
        self.detail.is_static
    }

    fn r#type(&self) -> JavaType {
        JavaType::parse(&self.detail.descriptor)
    }
}
