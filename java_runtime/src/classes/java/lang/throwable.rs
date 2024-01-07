use alloc::vec;

use crate::RuntimeClassProto;

// class java.lang.Throwable
pub struct Throwable {}

impl Throwable {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![],
            fields: vec![],
        }
    }
}
