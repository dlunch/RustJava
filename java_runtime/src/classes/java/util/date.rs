use alloc::vec;

use crate::RuntimeClassProto;

// class java.util.Date
pub struct Date {}

impl Date {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Date",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![],
            fields: vec![],
        }
    }
}
