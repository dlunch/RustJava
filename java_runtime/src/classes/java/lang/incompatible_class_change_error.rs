use alloc::vec;

use crate::RuntimeClassProto;

// class java.lang.IncompatibleClassChangeError
pub struct IncompatibleClassChangeError {}

impl IncompatibleClassChangeError {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/LinkageError"),
            interfaces: vec![],
            methods: vec![],
            fields: vec![],
        }
    }
}
