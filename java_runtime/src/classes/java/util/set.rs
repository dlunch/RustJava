use alloc::vec;

use java_constants::ClassAccessFlags;

use crate::RuntimeClassProto;

// interface java.util.Set
pub struct Set;

impl Set {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Set",
            parent_class: None,
            interfaces: vec!["java/util/Collection"],
            methods: vec![],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE,
        }
    }
}
