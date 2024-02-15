use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, JvmResult};

use crate::{RuntimeClassProto, RuntimeContext};

// class java.util.Random
pub struct Random {}

impl Random {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("nextInt", "()I", Self::next_int, Default::default()),
            ],
            fields: vec![],
        }
    }

    async fn init(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> JvmResult<()> {
        tracing::warn!("stub java.util.Random::<init>({:?})", &this);

        Ok(())
    }

    async fn next_int(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> JvmResult<i32> {
        tracing::warn!("stub java.util.Random::nextInt({:?})", &this);

        let random = 12351352; // TODO

        Ok(random)
    }
}
