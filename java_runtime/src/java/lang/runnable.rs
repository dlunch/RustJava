use alloc::vec;

use crate::RuntimeClassProto;

// interface java.lang.Runnable
pub struct Runnable {}

impl Runnable {
    // TODO Create JavaInterfaceProto
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: None,
            interfaces: vec![],
            methods: vec![],
            fields: vec![],
        }
    }
}
