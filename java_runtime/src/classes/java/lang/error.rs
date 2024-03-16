use alloc::vec;

use crate::RuntimeClassProto;

// class java.lang.Error
pub struct Error {}

impl Error {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/Throwable"),
            interfaces: vec![],
            methods: vec![],
            fields: vec![],
        }
    }
}
