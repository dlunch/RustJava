use alloc::vec;

use java_class_proto::{JavaMethodProto, JavaResult};
use java_constants::MethodAccessFlags;
use jvm::Jvm;

use crate::{RuntimeClassProto, RuntimeContext};

// class java.lang.Math
pub struct Math {}

impl Math {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new("abs", "(I)I", Self::abs, MethodAccessFlags::STATIC)],
            fields: vec![],
        }
    }

    async fn abs(_: &Jvm, _: &mut RuntimeContext, x: i32) -> JavaResult<i32> {
        tracing::debug!("java.lang.Math::abs({:?})", x);

        Ok(x.abs())
    }
}
