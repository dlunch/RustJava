use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::ClassAccessFlags;

use crate::RuntimeClassProto;
// interface java.io.DataInput
pub struct DataInput;

impl DataInput {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/DataInput",
            parent_class: None,
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new_abstract("readBoolean", "()Z", Default::default()),
                JavaMethodProto::new_abstract("readByte", "()B", Default::default()),
                JavaMethodProto::new_abstract("readChar", "()C", Default::default()),
                JavaMethodProto::new_abstract("readDouble", "()D", Default::default()),
                JavaMethodProto::new_abstract("readFloat", "()F", Default::default()),
                JavaMethodProto::new_abstract("readFully", "([B)V", Default::default()),
                JavaMethodProto::new_abstract("readFully", "([BII)V", Default::default()),
                JavaMethodProto::new_abstract("readInt", "()I", Default::default()),
                JavaMethodProto::new_abstract("readLong", "()J", Default::default()),
                JavaMethodProto::new_abstract("readShort", "()S", Default::default()),
                JavaMethodProto::new_abstract("readUnsignedShort", "()I", Default::default()),
                JavaMethodProto::new_abstract("readUTF", "()Ljava/lang/String;", Default::default()),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::INTERFACE,
        }
    }
}
