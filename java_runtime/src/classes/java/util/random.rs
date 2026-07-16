use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::MethodAccessFlags;
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
                JavaMethodProto::new("next", "(I)I", Self::next, MethodAccessFlags::PROTECTED),
                JavaMethodProto::new("nextInt", "()I", Self::next_int, Default::default()),
                JavaMethodProto::new("nextInt", "(I)I", Self::next_int_with_bound, Default::default()),
                JavaMethodProto::new("nextLong", "()J", Self::next_long, Default::default()),
                JavaMethodProto::new("nextFloat", "()F", Self::next_float, Default::default()),
                JavaMethodProto::new("nextDouble", "()D", Self::next_double, Default::default()),
                JavaMethodProto::new("setSeed", "(J)V", Self::set_seed, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("seed", "J", Default::default())],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Random::<init>({this:?})");

        let default_seed: i64 = jvm.invoke_static("java/lang/System", "currentTimeMillis", "()J", ()).await?;
        let _: () = jvm.invoke_special(&this, "java/util/Random", "<init>", "(J)V", (default_seed,)).await?;

        Ok(())
    }

    async fn init_with_seed(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, seed: i64) -> Result<()> {
        tracing::debug!("java.util.Random::<init>({this:?}, {seed:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        let _: () = jvm.invoke_virtual(&this, "setSeed", "(J)V", (seed,)).await?;

        Ok(())
    }

    async fn next(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, bits: i32) -> Result<i32> {
        tracing::debug!("java.util.Random::next({this:?}, {bits:?})");

        let seed: i64 = jvm.get_field(&this, "seed", "J").await?;
        let next_seed = seed.wrapping_mul(0x5DEECE66D).wrapping_add(0xB) & 0xFFFFFFFFFFFF;

        jvm.put_field(&mut this, "seed", "J", next_seed).await?;

        let value = (next_seed as u64).wrapping_shr(((48 - bits) & 63) as u32) as i32;

        Ok(value)
    }

    async fn next_int(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.Random::nextInt({this:?})");
        jvm.invoke_virtual(&this, "next", "(I)I", (32,)).await
    }

    async fn next_int_with_bound(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, bound: i32) -> Result<i32> {
        tracing::debug!("java.util.Random::nextInt({this:?}, {bound:?})");

        if bound <= 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "n must be positive").await);
        }

        if bound & -bound == bound {
            let bits: i32 = jvm.invoke_virtual(&this, "next", "(I)I", (31,)).await?;
            return Ok(((bound as i64 * bits as i64) >> 31) as i32);
        }

        loop {
            let bits: i32 = jvm.invoke_virtual(&this, "next", "(I)I", (31,)).await?;
            let value = bits % bound;
            if bits.wrapping_sub(value).wrapping_add(bound - 1) >= 0 {
                return Ok(value);
            }
        }
    }

    async fn next_long(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        tracing::debug!("java.util.Random::nextLong({this:?})");

        let high: i32 = jvm.invoke_virtual(&this, "next", "(I)I", (32,)).await?;
        let low: i32 = jvm.invoke_virtual(&this, "next", "(I)I", (32,)).await?;
        Ok((high as i64).wrapping_shl(32).wrapping_add(low as i64))
    }

    async fn next_float(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        tracing::debug!("java.util.Random::nextFloat({this:?})");

        let bits: i32 = jvm.invoke_virtual(&this, "next", "(I)I", (24,)).await?;
        Ok(bits as f32 / (1u32 << 24) as f32)
    }

    async fn next_double(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f64> {
        tracing::debug!("java.util.Random::nextDouble({this:?})");

        let high: i32 = jvm.invoke_virtual(&this, "next", "(I)I", (26,)).await?;
        let low: i32 = jvm.invoke_virtual(&this, "next", "(I)I", (27,)).await?;
        Ok(((high as i64) << 27 | low as i64) as f64 / (1u64 << 53) as f64)
    }

    async fn set_seed(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, seed: i64) -> Result<()> {
        tracing::debug!("java.util.Random::setSeed({this:?}, {seed:?})");

        let seed = (seed ^ 0x5DEECE66D) & ((1 << 48) - 1);

        jvm.put_field(&mut this, "seed", "J", seed).await?;

        Ok(())
    }
}
