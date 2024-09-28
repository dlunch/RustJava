use alloc::vec;

use crate::RuntimeClassProto;

// class java.util.TimerTask
pub struct TimerTask;

impl TimerTask {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/TimerTask",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![],
            fields: vec![],
        }
    }
}
