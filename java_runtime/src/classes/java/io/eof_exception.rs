use alloc::vec;

use crate::RuntimeClassProto;

// class java.io.EOFException
pub struct EOFException {}

impl EOFException {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/io/IOException"),
            interfaces: vec![],
            methods: vec![],
            fields: vec![],
        }
    }
}
