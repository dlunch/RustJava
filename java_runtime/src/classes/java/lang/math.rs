use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::MethodAccessFlags;
use jvm::{Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

// class java.lang.Math
pub struct Math;

impl Math {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Math",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("abs", "(I)I", Self::abs, MethodAccessFlags::STATIC),
                JavaMethodProto::new("abs", "(J)J", Self::abs_long, MethodAccessFlags::STATIC),
                JavaMethodProto::new("abs", "(F)F", Self::abs_float, MethodAccessFlags::STATIC),
                JavaMethodProto::new("abs", "(D)D", Self::abs_double, MethodAccessFlags::STATIC),
                JavaMethodProto::new("max", "(II)I", Self::max, MethodAccessFlags::STATIC),
                JavaMethodProto::new("max", "(JJ)J", Self::max_long, MethodAccessFlags::STATIC),
                JavaMethodProto::new("min", "(II)I", Self::min, MethodAccessFlags::STATIC),
                JavaMethodProto::new("min", "(JJ)J", Self::min_long, MethodAccessFlags::STATIC),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn abs(_: &Jvm, _: &mut RuntimeContext, x: i32) -> Result<i32> {
        tracing::debug!("java.lang.Math::abs({x:?})");

        Ok(x.abs())
    }

    async fn abs_long(_: &Jvm, _: &mut RuntimeContext, x: i64) -> Result<i64> {
        tracing::debug!("java.lang.Math::abs({x:?})");

        Ok(x.abs())
    }

    async fn abs_float(_: &Jvm, _: &mut RuntimeContext, x: f32) -> Result<f32> {
        tracing::debug!("java.lang.Math::abs({x:?})");

        Ok(x.abs())
    }

    async fn abs_double(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::abs({x:?})");

        Ok(x.abs())
    }

    async fn max(_: &Jvm, _: &mut RuntimeContext, x: i32, y: i32) -> Result<i32> {
        tracing::debug!("java.lang.Math::max({x:?}, {y:?})");

        Ok(x.max(y))
    }

    async fn max_long(_: &Jvm, _: &mut RuntimeContext, x: i64, y: i64) -> Result<i64> {
        tracing::debug!("java.lang.Math::max({x:?}, {y:?})");

        Ok(x.max(y))
    }

    async fn min(_: &Jvm, _: &mut RuntimeContext, x: i32, y: i32) -> Result<i32> {
        tracing::debug!("java.lang.Math::min({x:?}, {y:?})");

        Ok(x.min(y))
    }

    async fn min_long(_: &Jvm, _: &mut RuntimeContext, x: i64, y: i64) -> Result<i64> {
        tracing::debug!("java.lang.Math::min({x:?}, {y:?})");

        Ok(x.min(y))
    }
}
