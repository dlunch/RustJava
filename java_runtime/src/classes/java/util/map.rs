use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::ClassAccessFlags;

use crate::RuntimeClassProto;

// interface java.util.Map
pub struct Map;

impl Map {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Map",
            parent_class: None,
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new_abstract("size", "()I", Default::default()),
                JavaMethodProto::new_abstract("isEmpty", "()Z", Default::default()),
                JavaMethodProto::new_abstract("containsKey", "(Ljava/lang/Object;)Z", Default::default()),
                JavaMethodProto::new_abstract("containsValue", "(Ljava/lang/Object;)Z", Default::default()),
                JavaMethodProto::new_abstract("get", "(Ljava/lang/Object;)Ljava/lang/Object;", Default::default()),
                JavaMethodProto::new_abstract("put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", Default::default()),
                JavaMethodProto::new_abstract("remove", "(Ljava/lang/Object;)Ljava/lang/Object;", Default::default()),
                JavaMethodProto::new_abstract("clear", "()V", Default::default()),
                JavaMethodProto::new_abstract("keySet", "()Ljava/util/Set;", Default::default()),
                JavaMethodProto::new_abstract("values", "()Ljava/util/Collection;", Default::default()),
                JavaMethodProto::new_abstract("entrySet", "()Ljava/util/Set;", Default::default()),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE,
        }
    }
}
