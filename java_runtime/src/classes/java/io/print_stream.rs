use alloc::{string::ToString, vec};

use java_runtime_base::{JavaMethodFlag, JavaMethodProto, JavaResult, JvmClassInstanceHandle};
use jvm::Jvm;

use crate::{classes::java::lang::String, RuntimeClassProto, RuntimeContext};

// class java.io.PrintStream
pub struct PrintStream {}

impl PrintStream {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/io/OutputStream"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, JavaMethodFlag::NONE),
                JavaMethodProto::new("println", "(Ljava/lang/String;)V", Self::println_string, JavaMethodFlag::NONE),
                JavaMethodProto::new("println", "(I)V", Self::println_int, JavaMethodFlag::NONE),
            ],
            fields: vec![],
        }
    }

    async fn init(_: &mut Jvm, _: &mut RuntimeContext, this: JvmClassInstanceHandle<Self>) -> JavaResult<()> {
        tracing::warn!("stub java.lang.PrintStream::<init>({:?})", &this);

        Ok(())
    }

    async fn println_string(
        jvm: &mut Jvm,
        context: &mut RuntimeContext,
        this: JvmClassInstanceHandle<Self>,
        str: JvmClassInstanceHandle<String>,
    ) -> JavaResult<()> {
        tracing::warn!("stub java.lang.PrintStream::println({:?}, {:?})", &this, &str);

        let rust_str = String::to_rust_string(jvm, &str)?;
        context.println(&rust_str);

        Ok(())
    }

    async fn println_int(_: &mut Jvm, context: &mut RuntimeContext, this: JvmClassInstanceHandle<Self>, int: i32) -> JavaResult<()> {
        tracing::warn!("stub java.lang.PrintStream::println({:?}, {:?})", &this, &int);

        context.println(&int.to_string());

        Ok(())
    }
}
