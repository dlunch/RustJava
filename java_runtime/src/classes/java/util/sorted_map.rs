use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::{ClassAccessFlags, MethodAccessFlags};

use crate::RuntimeClassProto;

// interface java.util.SortedMap
pub struct SortedMap;

impl SortedMap {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/SortedMap",
            parent_class: None,
            interfaces: vec!["java/util/Map"],
            methods: vec![
                JavaMethodProto::new_abstract(
                    "comparator",
                    "()Ljava/util/Comparator;",
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT,
                ),
                JavaMethodProto::new_abstract(
                    "subMap",
                    "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;",
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT,
                ),
                JavaMethodProto::new_abstract(
                    "headMap",
                    "(Ljava/lang/Object;)Ljava/util/SortedMap;",
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT,
                ),
                JavaMethodProto::new_abstract(
                    "tailMap",
                    "(Ljava/lang/Object;)Ljava/util/SortedMap;",
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT,
                ),
                JavaMethodProto::new_abstract(
                    "firstKey",
                    "()Ljava/lang/Object;",
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT,
                ),
                JavaMethodProto::new_abstract("lastKey", "()Ljava/lang/Object;", MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::INTERFACE | ClassAccessFlags::ABSTRACT,
        }
    }
}
