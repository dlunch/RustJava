use alloc::vec;

use java_runtime_base::{JavaClassProto, JavaContext, JavaMethodFlag, JavaMethodProto, JavaResult, JvmClassInstanceHandle};

// class java.util.Vector
pub struct Vector {}

impl Vector {
    pub fn as_proto() -> JavaClassProto {
        JavaClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new("<init>", "()V", Self::init, JavaMethodFlag::NONE)],
            fields: vec![],
        }
    }

    async fn init(_: &mut dyn JavaContext, this: JvmClassInstanceHandle<Self>) -> JavaResult<()> {
        tracing::warn!("stub java.util.Vector::<init>({:?})", &this);

        Ok(())
    }
}
