use alloc::vec;

use crate::RuntimeClassProto;

// class java.util.Timer
pub struct Timer {}

impl Timer {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Timer",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![],
            fields: vec![],
        }
    }
}
