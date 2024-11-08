use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

// class java.util.Random
pub struct Random;

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
            fields: vec![JavaFieldProto::new("seed", "J", Default::default())],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Random::<init>({:?})", &this);

        let default_seed = 0i64; // TODO
        let _: () = jvm.invoke_special(&this, "java/util/Random", "<init>", "(J)V", (default_seed,)).await?;

        Ok(())
    }

    async fn init_with_seed(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, seed: i64) -> Result<()> {
        tracing::debug!("java.util.Random::<init>({:?}, {:?})", &this, seed);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        // TODO constant
        let seed = (seed ^ 0x5DEECE66D) & ((1 << 48) - 1);

        jvm.put_field(&mut this, "seed", "J", seed).await?;

        Ok(())
    }

    async fn next_int(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.Random::nextInt({:?})", &this);

        let seed: i64 = jvm.get_field(&this, "seed", "J").await?;
        let next_seed = seed.wrapping_mul(0x5DEECE66D).wrapping_add(0xB) & 0xFFFFFFFFFFFF;

        jvm.put_field(&mut this, "seed", "J", next_seed).await?;

        let value = next_seed.wrapping_shr(16) as i32;

        Ok(value)
    }
}

#[cfg(test)]
mod test {
    use jvm::Result;

    use crate::test::test_jvm;

    #[tokio::test]
    async fn test_random() -> Result<()> {
        let jvm = test_jvm().await?;

        let seed = 42i64;
        let random = jvm.new_class("java/util/Random", "(J)V", (seed,)).await?;

        let next: i32 = jvm.invoke_virtual(&random, "nextInt", "()I", ()).await?;
        assert_eq!(next, -1170105035);

        let next: i32 = jvm.invoke_virtual(&random, "nextInt", "()I", ()).await?;
        assert_eq!(next, 234785527);

        Ok(())
    }
}
