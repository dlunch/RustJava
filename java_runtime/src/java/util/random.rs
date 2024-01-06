use alloc::vec;

use java_runtime_base::{JavaClassProto, JavaMethodFlag, JavaMethodProto, JavaResult, JvmClassInstanceHandle};
use jvm::Jvm;

// class java.util.Random
pub struct Random {}

impl Random {
    pub fn as_proto() -> JavaClassProto {
        JavaClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, JavaMethodFlag::NONE),
                JavaMethodProto::new("nextInt", "()I", Self::next_int, JavaMethodFlag::NONE),
            ],
            fields: vec![],
        }
    }

    async fn init(_: &mut Jvm, this: JvmClassInstanceHandle<Self>) -> JavaResult<()> {
        tracing::warn!("stub java.util.Random::<init>({:?})", &this);

        Ok(())
    }

    async fn next_int(_: &mut Jvm, this: JvmClassInstanceHandle<Self>) -> JavaResult<i32> {
        tracing::warn!("stub java.util.Random::nextInt({:?})", &this);

        let random = 12351352; // TODO

        Ok(random)
    }
}
