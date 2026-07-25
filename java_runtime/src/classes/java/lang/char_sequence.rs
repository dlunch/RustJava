use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::{ClassAccessFlags, MethodAccessFlags};

use crate::RuntimeClassProto;

// public interface java.lang.CharSequence
pub struct CharSequence;

impl CharSequence {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/CharSequence",
            parent_class: None,
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new_abstract("length", "()I", MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new_abstract("charAt", "(I)C", MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new_abstract(
                    "subSequence",
                    "(II)Ljava/lang/CharSequence;",
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT,
                ),
                JavaMethodProto::new_abstract(
                    "toString",
                    "()Ljava/lang/String;",
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT,
                ),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::INTERFACE | ClassAccessFlags::ABSTRACT,
        }
    }
}
