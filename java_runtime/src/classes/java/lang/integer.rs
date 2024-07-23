use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::MethodAccessFlags;
use jvm::{runtime::JavaLangString, ClassInstanceRef, Jvm, Result};

use crate::{classes::java::lang::String, RuntimeClassProto, RuntimeContext};

// class java.lang.Integer
pub struct Integer {}

impl Integer {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new(
                "parseInt",
                "(Ljava/lang/String;)I",
                Self::parse_int,
                MethodAccessFlags::STATIC,
            )],
            fields: vec![],
        }
    }

    async fn parse_int(jvm: &Jvm, _: &mut RuntimeContext, s: ClassInstanceRef<String>) -> Result<i32> {
        tracing::debug!("java.lang.Integer::parseInt({:?})", &s);

        let s = JavaLangString::to_rust_string(jvm, &s).await?;

        Ok(s.parse().unwrap())
    }
}

#[cfg(test)]
mod test {
    use jvm::{runtime::JavaLangString, Result};

    use crate::test::test_jvm;

    #[futures_test::test]
    async fn test_parse_int() -> Result<()> {
        let jvm = test_jvm().await?;

        let string = JavaLangString::from_rust_string(&jvm, "42").await?;
        assert_eq!(
                    42i32,
        let _:() = jvm.invoke_static("java/lang/Integer", "parseInt", "(Ljava/lang/String;)I", (string,))
                        .await?
                );

        Ok(())
    }
}
