use alloc::vec;

use crate::RuntimeClassProto;

// class java.lang.IllegalArgumentException
pub struct IllegalArgumentException {}

impl IllegalArgumentException {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/RuntimeException"),
            interfaces: vec![],
            methods: vec![],
            fields: vec![],
        }
    }
}
