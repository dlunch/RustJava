use alloc::vec;

use java_runtime_base::{JavaMethodFlag, JavaMethodProto, JavaResult};
use jvm::Jvm;

use crate::{JavaClassProto, JavaContext};

// class java.lang.Math
pub struct Math {}

impl Math {
    pub fn as_proto() -> JavaClassProto {
        JavaClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new("abs", "(I)I", Self::abs, JavaMethodFlag::STATIC)],
            fields: vec![],
        }
    }

    async fn abs(_: &mut Jvm, _: &JavaContext, x: i32) -> JavaResult<i32> {
        tracing::debug!("java.lang.Math::abs({:?})", x);

        Ok(x.abs())
    }
}
