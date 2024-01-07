use alloc::vec;

use crate::RuntimeClassProto;

// class java.io.IOException
pub struct IOException {}

impl IOException {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/Exception"),
            interfaces: vec![],
            methods: vec![],
            fields: vec![],
        }
    }
}
