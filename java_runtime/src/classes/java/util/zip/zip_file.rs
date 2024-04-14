use alloc::vec;

use crate::RuntimeClassProto;

// class java.util.zip.ZipFile
pub struct ZipFile {}

impl ZipFile {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![],
            fields: vec![],
        }
    }
}
