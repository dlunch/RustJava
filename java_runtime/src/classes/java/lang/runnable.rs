use alloc::vec;

use java_class_proto::JavaMethodProto;

use crate::RuntimeClassProto;

// interface java.lang.Runnable
pub struct Runnable;

impl Runnable {
    // TODO Create JavaInterfaceProto
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Runnable",
            parent_class: None,
            interfaces: vec![],
            methods: vec![JavaMethodProto::new_abstract("run", "()V", Default::default())],
            fields: vec![],
            access_flags: Default::default(),
        }
    }
}
