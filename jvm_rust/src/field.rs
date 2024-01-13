use alloc::{
    rc::Rc,
    string::{String, ToString},
};

use classfile::FieldInfo;
use java_class_proto::JavaFieldProto;
use java_constants::FieldAccessFlags;
use jvm::{Field, JavaType};

#[derive(Debug)]
struct FieldInner {
    name: String,
    descriptor: String,
    access_flags: FieldAccessFlags,
    index: usize,
}

#[derive(Clone, Debug)]
pub struct FieldImpl {
    inner: Rc<FieldInner>,
}

impl FieldImpl {
    pub fn new(name: &str, descriptor: &str, access_flags: FieldAccessFlags, index: usize) -> Self {
        Self {
            inner: Rc::new(FieldInner {
                name: name.to_string(),
                descriptor: descriptor.to_string(),
                access_flags,
                index,
            }),
        }
    }

    pub fn from_field_proto(proto: JavaFieldProto, index: usize) -> Self {
        Self::new(&proto.name, &proto.descriptor, proto.access_flags, index)
    }

    pub fn from_fieldinfo(field_info: FieldInfo, index: usize) -> Self {
        Self {
            inner: Rc::new(FieldInner {
                name: field_info.name.to_string(),
                descriptor: field_info.descriptor.to_string(),
                access_flags: field_info.access_flags,
                index,
            }),
        }
    }

    pub fn index(&self) -> usize {
        self.inner.index
    }
}

impl Field for FieldImpl {
    fn name(&self) -> String {
        self.inner.name.clone()
    }

    fn descriptor(&self) -> String {
        self.inner.descriptor.clone()
    }

    fn access_flags(&self) -> FieldAccessFlags {
        self.inner.access_flags
    }

    fn r#type(&self) -> JavaType {
        JavaType::parse(&self.inner.descriptor)
    }
}
