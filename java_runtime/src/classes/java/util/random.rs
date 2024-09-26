use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

// class java.util.Random
pub struct Random {}

impl Random {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Random",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(J)V", Self::init_with_seed, Default::default()),
                JavaMethodProto::new("nextInt", "()I", Self::next_int, Default::default()),
            ],
            fields: vec![],
        }
    }

    async fn init(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::warn!("stub java.util.Random::<init>({:?})", &this);

        Ok(())
    }

    async fn init_with_seed(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, seed: i64) -> Result<()> {
        tracing::warn!("stub java.util.Random::<init>({:?}, {:?})", &this, seed);

        Ok(())
    }

    async fn next_int(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::warn!("stub java.util.Random::nextInt({:?})", &this);

        let random = 12351352; // TODO

        Ok(random)
    }
}
