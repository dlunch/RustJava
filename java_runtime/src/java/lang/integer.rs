use alloc::vec;

use java_runtime_base::{JavaClassProto, JavaContext, JavaMethodFlag, JavaMethodProto, JavaResult, JvmClassInstanceHandle};

use crate::java::lang::String;

// class java.lang.Integer
pub struct Integer {}

impl Integer {
    pub fn as_proto() -> JavaClassProto {
        JavaClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new(
                "parseInt",
                "(Ljava/lang/String;)I",
                Self::parse_int,
                JavaMethodFlag::STATIC,
            )],
            fields: vec![],
        }
    }

    async fn parse_int(context: &mut dyn JavaContext, s: JvmClassInstanceHandle<String>) -> JavaResult<i32> {
        tracing::debug!("java.lang.Integer::parseInt({:?})", &s);

        let s = String::to_rust_string(context, &s)?;

        Ok(s.parse()?)
    }
}
