use alloc::vec;

use java_runtime_base::JavaClassProto;

// interface java.lang.Runnable
pub struct Runnable {}

impl Runnable {
    // TODO Create JavaInterfaceProto
    pub fn as_proto() -> JavaClassProto {
        JavaClassProto {
            parent_class: None,
            interfaces: vec![],
            methods: vec![],
            fields: vec![],
        }
    }
}
