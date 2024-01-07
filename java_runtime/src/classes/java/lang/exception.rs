use alloc::vec;

use crate::RuntimeClassProto;

// class java.lang.Exception
pub struct Exception {}

impl Exception {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/Throwable"),
            interfaces: vec![],
            methods: vec![],
            fields: vec![],
        }
    }
}
