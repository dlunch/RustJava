use alloc::vec;

use java_runtime_base::{JavaClassProto, JavaMethodFlag, JavaMethodProto, JavaResult, JvmClassInstanceHandle};
use jvm::Jvm;

// class java.io.OutputStream
pub struct OutputStream {}

impl OutputStream {
    pub fn as_proto() -> JavaClassProto {
        JavaClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new("<init>", "()V", Self::init, JavaMethodFlag::NONE)],
            fields: vec![],
        }
    }

    async fn init(_: &mut Jvm, this: JvmClassInstanceHandle<Self>) -> JavaResult<()> {
        tracing::warn!("stub java.lang.OutputStream::<init>({:?})", &this);

        Ok(())
    }
}
