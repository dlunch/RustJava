use alloc::vec;

use crate::RuntimeClassProto;

// class java.lang.LinkageError
pub struct LinkageError {}

impl LinkageError {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/Error"),
            interfaces: vec![],
            methods: vec![],
            fields: vec![],
        }
    }
}
