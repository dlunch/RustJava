use alloc::vec;

use java_class_proto::JavaMethodProto;

use crate::RuntimeClassProto;

// interface java.util.Enumeration
pub struct Enumeration;

impl Enumeration {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Enumeration",
            parent_class: None,
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new_abstract("hasMoreElements", "()Z", Default::default()),
                JavaMethodProto::new_abstract("nextElement", "()Ljava/lang/Object;", Default::default()),
            ],
            fields: vec![],
        }
    }
}
