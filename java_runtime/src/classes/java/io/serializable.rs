use alloc::vec;

use java_constants::ClassAccessFlags;

use crate::RuntimeClassProto;

// interface java.io.Serializable
pub struct Serializable;

impl Serializable {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/Serializable",
            parent_class: None,
            interfaces: vec![],
            methods: vec![],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE,
        }
    }
}
