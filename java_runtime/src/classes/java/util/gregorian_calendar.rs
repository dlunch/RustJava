use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, JvmResult};

use crate::{RuntimeClassProto, RuntimeContext};

// class java.util.GregorianCalendar
pub struct GregorianCalendar {}

impl GregorianCalendar {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/util/Calendar"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new("<init>", "()V", Self::init, Default::default())],
            fields: vec![],
        }
    }

    async fn init(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> JvmResult<()> {
        tracing::warn!("stub java.util.GregorianCalendar::<init>({:?})", &this);

        Ok(())
    }
}
