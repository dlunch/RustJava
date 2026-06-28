use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::ClassAccessFlags;

use crate::RuntimeClassProto;

// interface java.util.List
pub struct List;

impl List {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/List",
            parent_class: None,
            interfaces: vec!["java/util/Collection"],
            methods: vec![
                JavaMethodProto::new_abstract("get", "(I)Ljava/lang/Object;", Default::default()),
                JavaMethodProto::new_abstract("set", "(ILjava/lang/Object;)Ljava/lang/Object;", Default::default()),
                JavaMethodProto::new_abstract("add", "(ILjava/lang/Object;)V", Default::default()),
                JavaMethodProto::new_abstract("remove", "(I)Ljava/lang/Object;", Default::default()),
                JavaMethodProto::new_abstract("indexOf", "(Ljava/lang/Object;)I", Default::default()),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE,
        }
    }
}
