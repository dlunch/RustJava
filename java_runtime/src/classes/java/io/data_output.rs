use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::ClassAccessFlags;

use crate::RuntimeClassProto;

// interface java.io.DataOutput
pub struct DataOutput;

impl DataOutput {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/DataOutput",
            parent_class: None,
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new_abstract("write", "(I)V", Default::default()),
                JavaMethodProto::new_abstract("writeByte", "(I)V", Default::default()),
                JavaMethodProto::new_abstract("writeBoolean", "(Z)V", Default::default()),
                JavaMethodProto::new_abstract("writeInt", "(I)V", Default::default()),
                JavaMethodProto::new_abstract("writeShort", "(I)V", Default::default()),
                JavaMethodProto::new_abstract("writeLong", "(J)V", Default::default()),
                JavaMethodProto::new_abstract("writeChars", "(Ljava/lang/String;)V", Default::default()),
                JavaMethodProto::new_abstract("writeUTF", "(Ljava/lang/String;)V", Default::default()),
                JavaMethodProto::new_abstract("close", "()V", Default::default()),
                JavaMethodProto::new_abstract("flush", "()V", Default::default()),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE,
        }
    }
}
