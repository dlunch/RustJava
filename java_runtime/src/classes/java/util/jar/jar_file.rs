use alloc::vec;

use crate::RuntimeClassProto;

// class java.util.jar.JarFile
pub struct JarFile {}

impl JarFile {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/util/jar/JarFile"),
            interfaces: vec![],
            methods: vec![],
            fields: vec![],
        }
    }
}
