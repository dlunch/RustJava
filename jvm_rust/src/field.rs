use alloc::{
    string::{String, ToString},
    sync::Arc,
};

use classfile::FieldInfo;
use java_class_proto::JavaFieldProto;
use java_constants::FieldAccessFlags;
use jvm::Field;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct FieldInner {
    name: String,
    descriptor: String,
    access_flags: FieldAccessFlags,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct FieldImpl {
    inner: Arc<FieldInner>,
}

impl FieldImpl {
    pub fn new(name: &str, descriptor: &str, access_flags: FieldAccessFlags) -> Self {
        Self {
            inner: Arc::new(FieldInner {
                name: name.to_string(),
                descriptor: descriptor.to_string(),
                access_flags,
            }),
        }
    }

    pub fn from_field_proto(proto: JavaFieldProto) -> Self {
        Self::new(&proto.name, &proto.descriptor, proto.access_flags)
    }

    pub fn from_field_info(field_info: FieldInfo) -> Self {
        Self {
            inner: Arc::new(FieldInner {
                name: field_info.name.to_string(),
                descriptor: field_info.descriptor.to_string(),
                access_flags: field_info.access_flags,
            }),
        }
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
}
