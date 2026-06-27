use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::ClassAccessFlags;

use crate::RuntimeClassProto;

// interface java.util.Map$Entry
pub struct MapEntry;

impl MapEntry {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Map$Entry",
            parent_class: None,
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new_abstract("getKey", "()Ljava/lang/Object;", Default::default()),
                JavaMethodProto::new_abstract("getValue", "()Ljava/lang/Object;", Default::default()),
                JavaMethodProto::new_abstract("setValue", "(Ljava/lang/Object;)Ljava/lang/Object;", Default::default()),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE,
        }
    }
}
