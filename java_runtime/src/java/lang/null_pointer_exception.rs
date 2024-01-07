use alloc::vec;

use crate::RuntimeClassProto;

// class java.lang.NullPointerException
pub struct NullPointerException {}

impl NullPointerException {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/RuntimeException"),
            interfaces: vec![],
            methods: vec![],
            fields: vec![],
        }
    }
}
