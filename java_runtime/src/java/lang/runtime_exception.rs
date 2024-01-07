use alloc::vec;

use crate::RuntimeClassProto;

// class java.lang.RuntimeException
pub struct RuntimeException {}

impl RuntimeException {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/Exception"),
            interfaces: vec![],
            methods: vec![],
            fields: vec![],
        }
    }
}
