use alloc::{
    borrow::Cow,
    string::{String, ToString},
    sync::Arc,
};

use classfile::FieldInfo;
use jvm::Field;
use jvm_class_proto::JavaFieldProto;
use jvm_types::FieldAccessFlags;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct FieldInner {
    declaring_class: String,
    name: String,
    descriptor: String,
    access_flags: FieldAccessFlags,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct FieldImpl {
    inner: Arc<FieldInner>,
}

impl FieldImpl {
    pub fn new(declaring_class: &str, name: &str, descriptor: &str, access_flags: FieldAccessFlags) -> Self {
        Self {
            inner: Arc::new(FieldInner {
                declaring_class: declaring_class.to_string(),
                name: name.to_string(),
                descriptor: descriptor.to_string(),
                access_flags,
            }),
        }
    }

    pub fn from_field_proto(declaring_class: &str, proto: JavaFieldProto) -> Self {
        Self::new(declaring_class, &proto.name, &proto.descriptor, proto.access_flags)
    }

    pub fn from_field_info(declaring_class: &str, field_info: FieldInfo) -> Self {
        Self::new(declaring_class, &field_info.name, &field_info.descriptor, field_info.access_flags)
    }
}

impl Field for FieldImpl {
    fn name(&self) -> Cow<'_, str> {
        (&self.inner.name).into()
    }

    fn descriptor(&self) -> Cow<'_, str> {
        (&self.inner.descriptor).into()
    }

    fn access_flags(&self) -> FieldAccessFlags {
        self.inner.access_flags
    }
}
