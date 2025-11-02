use alloc::vec;

use java_constants::ClassAccessFlags;

use crate::RuntimeClassProto;

// interface java.lang.Cloneable
pub struct Cloneable;

impl Cloneable {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Cloneable",
            parent_class: None,
            interfaces: vec![],
            methods: vec![],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE,
        }
    }
}
