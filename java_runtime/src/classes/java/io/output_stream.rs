use alloc::vec;

use java_class_proto::{JavaMethodProto, JavaResult};
use jvm::{ClassInstanceRef, Jvm};

use crate::{RuntimeClassProto, RuntimeContext};

// class java.io.OutputStream
pub struct OutputStream {}

impl OutputStream {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new("<init>", "()V", Self::init, Default::default())],
            fields: vec![],
        }
    }

    async fn init(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> JavaResult<()> {
        tracing::warn!("stub java.lang.OutputStream::<init>({:?})", &this);

        Ok(())
    }
}
