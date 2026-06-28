use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::ClassAccessFlags;

use crate::RuntimeClassProto;

// interface java.util.Collection
pub struct Collection;

impl Collection {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Collection",
            parent_class: None,
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new_abstract("size", "()I", Default::default()),
                JavaMethodProto::new_abstract("isEmpty", "()Z", Default::default()),
                JavaMethodProto::new_abstract("contains", "(Ljava/lang/Object;)Z", Default::default()),
                JavaMethodProto::new_abstract("iterator", "()Ljava/util/Iterator;", Default::default()),
                JavaMethodProto::new_abstract("toArray", "()[Ljava/lang/Object;", Default::default()),
                JavaMethodProto::new_abstract("add", "(Ljava/lang/Object;)Z", Default::default()),
                JavaMethodProto::new_abstract("remove", "(Ljava/lang/Object;)Z", Default::default()),
                JavaMethodProto::new_abstract("clear", "()V", Default::default()),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE,
        }
    }
}
