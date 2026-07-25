use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

// class java.util.Random
pub struct Random;

impl Random {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Random",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/io/Serializable"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(J)V", Self::init_with_seed, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("next", "(I)I", Self::next, MethodAccessFlags::PROTECTED | MethodAccessFlags::SYNCHRONIZED),
                JavaMethodProto::new("nextBoolean", "()Z", Self::next_boolean, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("nextBytes", "([B)V", Self::next_bytes, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("nextInt", "()I", Self::next_int, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("nextInt", "(I)I", Self::next_int_with_bound, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("nextLong", "()J", Self::next_long, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("nextFloat", "()F", Self::next_float, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("nextDouble", "()D", Self::next_double, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "nextGaussian",
                    "()D",
                    Self::next_gaussian,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "setSeed",
                    "(J)V",
                    Self::set_seed,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
            ],
            fields: vec![
                JavaFieldProto::new("seed", "J", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("nextNextGaussian", "D", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("haveNextNextGaussian", "Z", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
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

    async fn next_boolean(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.Random::nextBoolean({this:?})");

        let value: i32 = jvm.invoke_virtual(&this, "next", "(I)I", (1,)).await?;
        Ok(value != 0)
    }

    async fn next_bytes(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, mut bytes: ClassInstanceRef<Array<i8>>) -> Result<()> {
        tracing::debug!("java.util.Random::nextBytes({this:?}, {bytes:?})");

        if bytes.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "bytes is null").await);
        }

        let length = jvm.array_length(&bytes).await?;
        let mut index = 0;
        while index < length {
            let mut random: i32 = jvm.invoke_virtual(&this, "nextInt", "()I", ()).await?;
            let chunk_length = core::cmp::min(length - index, 4);
            let mut chunk = alloc::vec::Vec::with_capacity(chunk_length);
            for _ in 0..chunk_length {
                chunk.push(random as i8);
                random >>= 8;
            }
            jvm.store_array(&mut bytes, index, chunk).await?;
            index += chunk_length;
        }

        Ok(())
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

        if (bound as u32).is_power_of_two() {
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

    async fn next_gaussian(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<f64> {
        tracing::debug!("java.util.Random::nextGaussian({this:?})");

        let have_next_next_gaussian: bool = jvm.get_field(&this, "haveNextNextGaussian", "Z").await?;
        if have_next_next_gaussian {
            let next_next_gaussian: f64 = jvm.get_field(&this, "nextNextGaussian", "D").await?;
            jvm.put_field(&mut this, "haveNextNextGaussian", "Z", false).await?;
            return Ok(next_next_gaussian);
        }

        let (first, second, radius_squared) = loop {
            let first = 2.0 * jvm.invoke_virtual::<_, f64>(&this, "nextDouble", "()D", ()).await? - 1.0;
            let second = 2.0 * jvm.invoke_virtual::<_, f64>(&this, "nextDouble", "()D", ()).await? - 1.0;
            let radius_squared = first * first + second * second;
            if radius_squared < 1.0 && radius_squared != 0.0 {
                break (first, second, radius_squared);
            }
        };
        let multiplier = libm::sqrt(-2.0 * libm::log(radius_squared) / radius_squared);
        jvm.put_field(&mut this, "nextNextGaussian", "D", second * multiplier).await?;
        jvm.put_field(&mut this, "haveNextNextGaussian", "Z", true).await?;
        Ok(first * multiplier)
    }

    async fn set_seed(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, seed: i64) -> Result<()> {
        tracing::debug!("java.util.Random::setSeed({this:?}, {seed:?})");

        let seed = (seed ^ 0x5DEECE66D) & ((1 << 48) - 1);

        jvm.put_field(&mut this, "seed", "J", seed).await?;
        jvm.put_field(&mut this, "haveNextNextGaussian", "Z", false).await?;

        Ok(())
    }
}
