use alloc::vec;

use java_constants::ClassAccessFlags;

use crate::RuntimeClassProto;

// interface java.lang.Comparable
pub struct Comparable;

impl Comparable {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Comparable",
            parent_class: None,
            interfaces: vec![],
            methods: vec![],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE,
        }
    }
}
