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

        let s = JavaLangString::to_rust_string(jvm, s.into())?;

        Ok(s.parse().unwrap())
    }
}
