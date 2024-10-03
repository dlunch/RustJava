use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::MethodAccessFlags;
use jvm::{Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

// class java.lang.Math
pub struct Math;

impl Math {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Math",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("abs", "(I)I", Self::abs, MethodAccessFlags::STATIC),
                JavaMethodProto::new("max", "(II)I", Self::max, MethodAccessFlags::STATIC),
                JavaMethodProto::new("min", "(II)I", Self::min, MethodAccessFlags::STATIC),
            ],
            fields: vec![],
        }
    }

    async fn abs(_: &Jvm, _: &mut RuntimeContext, x: i32) -> Result<i32> {
        tracing::debug!("java.lang.Math::abs({:?})", x);

        Ok(x.abs())
    }

    async fn max(_: &Jvm, _: &mut RuntimeContext, x: i32, y: i32) -> Result<i32> {
        tracing::debug!("java.lang.Math::max({:?}, {:?})", x, y);

        Ok(x.max(y))
    }

    async fn min(_: &Jvm, _: &mut RuntimeContext, x: i32, y: i32) -> Result<i32> {
        tracing::debug!("java.lang.Math::min({:?}, {:?})", x, y);

        Ok(x.min(y))
    }
}

#[cfg(test)]
mod test {
    use jvm::Result;

    use crate::test::test_jvm;

    #[tokio::test]
    async fn test_abs() -> Result<()> {
        let jvm = test_jvm().await?;

        assert_eq!(42i32, jvm.invoke_static("java/lang/Math", "abs", "(I)I", (42,)).await?);
        assert_eq!(42i32, jvm.invoke_static("java/lang/Math", "abs", "(I)I", (-42,)).await?);

        Ok(())
    }
}
