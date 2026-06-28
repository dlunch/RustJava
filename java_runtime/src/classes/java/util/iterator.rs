use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::ClassAccessFlags;

use crate::RuntimeClassProto;

// interface java.util.Iterator
pub struct Iterator;

impl Iterator {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Iterator",
            parent_class: None,
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new_abstract("hasNext", "()Z", Default::default()),
                JavaMethodProto::new_abstract("next", "()Ljava/lang/Object;", Default::default()),
                JavaMethodProto::new_abstract("remove", "()V", Default::default()),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE,
        }
    }
}
