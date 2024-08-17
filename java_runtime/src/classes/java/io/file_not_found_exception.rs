use alloc::vec;

use crate::RuntimeClassProto;

// class java.io.FileNotFoundException
pub struct FileNotFoundException {}

impl FileNotFoundException {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/FileNotFoundException",
            parent_class: Some("java/io/IOException"),
            interfaces: vec![],
            methods: vec![],
            fields: vec![],
        }
    }
}
