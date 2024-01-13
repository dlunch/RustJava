use alloc::{string::ToString, vec};

use java_class_proto::{JavaMethodProto, JavaResult};
use jvm::{ClassInstanceRef, Jvm};

use crate::{classes::java::lang::String, RuntimeClassProto, RuntimeContext};

// class java.io.PrintStream
pub struct PrintStream {}

impl PrintStream {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/io/OutputStream"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("println", "(Ljava/lang/String;)V", Self::println_string, Default::default()),
                JavaMethodProto::new("println", "(I)V", Self::println_int, Default::default()),
            ],
            fields: vec![],
        }
    }

    async fn init(_: &mut Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> JavaResult<()> {
        tracing::warn!("stub java.lang.PrintStream::<init>({:?})", &this);

        Ok(())
    }

    async fn println_string(
        jvm: &mut Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        str: ClassInstanceRef<String>,
    ) -> JavaResult<()> {
        tracing::warn!("stub java.lang.PrintStream::println({:?}, {:?})", &this, &str);

        let rust_str = String::to_rust_string(jvm, &str)?;
        context.println(&rust_str);

        Ok(())
    }

    async fn println_int(_: &mut Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, int: i32) -> JavaResult<()> {
        tracing::warn!("stub java.lang.PrintStream::println({:?}, {:?})", &this, &int);

        context.println(&int.to_string());

        Ok(())
    }
}
