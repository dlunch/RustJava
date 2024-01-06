use alloc::vec;

use java_runtime_base::JavaClassProto;

// class java.lang.Throwable
pub struct Throwable {}

impl Throwable {
    pub fn as_proto() -> JavaClassProto {
        JavaClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![],
            fields: vec![],
        }
    }
}
