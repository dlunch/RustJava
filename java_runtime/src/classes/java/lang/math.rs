use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::util::Random};

// public final class java.lang.Math
pub struct Math;

impl Math {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Math",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("abs", "(I)I", Self::abs, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new("abs", "(J)J", Self::abs_long, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new("abs", "(F)F", Self::abs_float, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new("abs", "(D)D", Self::abs_double, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new("ceil", "(D)D", Self::ceil, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new("floor", "(D)D", Self::floor, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new("sqrt", "(D)D", Self::sqrt, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new("sin", "(D)D", Self::sin, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new("cos", "(D)D", Self::cos, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new("tan", "(D)D", Self::tan, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "toDegrees",
                    "(D)D",
                    Self::to_degrees,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "toRadians",
                    "(D)D",
                    Self::to_radians,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("max", "(II)I", Self::max, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new("max", "(JJ)J", Self::max_long, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new("max", "(FF)F", Self::max_float, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new("max", "(DD)D", Self::max_double, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new("min", "(II)I", Self::min, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new("min", "(JJ)J", Self::min_long, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new("min", "(FF)F", Self::min_float, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new("min", "(DD)D", Self::min_double, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new("acos", "(D)D", Self::acos, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new("asin", "(D)D", Self::asin, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new("atan", "(D)D", Self::atan, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new("atan2", "(DD)D", Self::atan2, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new("exp", "(D)D", Self::exp, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new("log", "(D)D", Self::log, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new("pow", "(DD)D", Self::pow, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new("rint", "(D)D", Self::rint, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "IEEEremainder",
                    "(DD)D",
                    Self::ieee_remainder,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("round", "(F)I", Self::round_float, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new("round", "(D)J", Self::round_double, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "random",
                    "()D",
                    Self::random,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC | MethodAccessFlags::SYNCHRONIZED,
                ),
            ],
            fields: vec![
                JavaFieldProto::new("E", "D", FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("PI", "D", FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new(
                    "randomNumberGenerator",
                    "Ljava/util/Random;",
                    FieldAccessFlags::PRIVATE | FieldAccessFlags::STATIC,
                ),
            ],
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::FINAL,
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        jvm.put_static_field("java/lang/Math", "E", "D", core::f64::consts::E).await?;
        jvm.put_static_field("java/lang/Math", "PI", "D", core::f64::consts::PI).await
    }

    async fn abs(_: &Jvm, _: &mut RuntimeContext, x: i32) -> Result<i32> {
        tracing::debug!("java.lang.Math::abs({x:?})");
        Ok(x.wrapping_abs())
    }

    async fn abs_long(_: &Jvm, _: &mut RuntimeContext, x: i64) -> Result<i64> {
        tracing::debug!("java.lang.Math::abs({x:?})");
        Ok(x.wrapping_abs())
    }

    async fn abs_float(_: &Jvm, _: &mut RuntimeContext, x: f32) -> Result<f32> {
        tracing::debug!("java.lang.Math::abs({x:?})");
        Ok(libm::fabsf(x))
    }

    async fn abs_double(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::abs({x:?})");
        Ok(libm::fabs(x))
    }

    async fn ceil(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::ceil({x:?})");
        Ok(libm::ceil(x))
    }

    async fn floor(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::floor({x:?})");
        Ok(libm::floor(x))
    }

    async fn sqrt(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::sqrt({x:?})");
        Ok(libm::sqrt(x))
    }

    async fn sin(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::sin({x:?})");
        Ok(libm::sin(x))
    }

    async fn cos(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::cos({x:?})");
        Ok(libm::cos(x))
    }

    async fn tan(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::tan({x:?})");
        Ok(libm::tan(x))
    }

    async fn to_degrees(_: &Jvm, _: &mut RuntimeContext, radians: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::toDegrees({radians:?})");
        Ok(radians * 180.0 / core::f64::consts::PI)
    }

    async fn to_radians(_: &Jvm, _: &mut RuntimeContext, degrees: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::toRadians({degrees:?})");
        Ok(degrees / 180.0 * core::f64::consts::PI)
    }

    async fn max(_: &Jvm, _: &mut RuntimeContext, x: i32, y: i32) -> Result<i32> {
        tracing::debug!("java.lang.Math::max({x:?}, {y:?})");
        Ok(x.max(y))
    }

    async fn max_long(_: &Jvm, _: &mut RuntimeContext, x: i64, y: i64) -> Result<i64> {
        tracing::debug!("java.lang.Math::max({x:?}, {y:?})");
        Ok(x.max(y))
    }

    async fn max_float(_: &Jvm, _: &mut RuntimeContext, x: f32, y: f32) -> Result<f32> {
        tracing::debug!("java.lang.Math::max({x:?}, {y:?})");

        if x.is_nan() {
            return Ok(x);
        }
        if y.is_nan() {
            return Ok(y);
        }
        if x == 0.0 && y == 0.0 {
            return Ok(if x.is_sign_positive() || y.is_sign_positive() { 0.0 } else { -0.0 });
        }
        Ok(if x >= y { x } else { y })
    }

    async fn max_double(_: &Jvm, _: &mut RuntimeContext, x: f64, y: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::max({x:?}, {y:?})");

        if x.is_nan() {
            return Ok(x);
        }
        if y.is_nan() {
            return Ok(y);
        }
        if x == 0.0 && y == 0.0 {
            return Ok(if x.is_sign_positive() || y.is_sign_positive() { 0.0 } else { -0.0 });
        }
        Ok(if x >= y { x } else { y })
    }

    async fn min(_: &Jvm, _: &mut RuntimeContext, x: i32, y: i32) -> Result<i32> {
        tracing::debug!("java.lang.Math::min({x:?}, {y:?})");
        Ok(x.min(y))
    }

    async fn min_long(_: &Jvm, _: &mut RuntimeContext, x: i64, y: i64) -> Result<i64> {
        tracing::debug!("java.lang.Math::min({x:?}, {y:?})");
        Ok(x.min(y))
    }

    async fn min_float(_: &Jvm, _: &mut RuntimeContext, x: f32, y: f32) -> Result<f32> {
        tracing::debug!("java.lang.Math::min({x:?}, {y:?})");

        if x.is_nan() {
            return Ok(x);
        }
        if y.is_nan() {
            return Ok(y);
        }
        if x == 0.0 && y == 0.0 {
            return Ok(if x.is_sign_negative() || y.is_sign_negative() { -0.0 } else { 0.0 });
        }
        Ok(if x <= y { x } else { y })
    }

    async fn min_double(_: &Jvm, _: &mut RuntimeContext, x: f64, y: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::min({x:?}, {y:?})");

        if x.is_nan() {
            return Ok(x);
        }
        if y.is_nan() {
            return Ok(y);
        }
        if x == 0.0 && y == 0.0 {
            return Ok(if x.is_sign_negative() || y.is_sign_negative() { -0.0 } else { 0.0 });
        }
        Ok(if x <= y { x } else { y })
    }

    async fn acos(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::acos({x:?})");
        Ok(libm::acos(x))
    }

    async fn asin(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::asin({x:?})");
        Ok(libm::asin(x))
    }

    async fn atan(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::atan({x:?})");
        Ok(libm::atan(x))
    }

    async fn atan2(_: &Jvm, _: &mut RuntimeContext, y: f64, x: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::atan2({y:?}, {x:?})");
        Ok(libm::atan2(y, x))
    }

    async fn exp(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::exp({x:?})");
        Ok(libm::exp(x))
    }

    async fn log(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::log({x:?})");
        Ok(libm::log(x))
    }

    async fn pow(_: &Jvm, _: &mut RuntimeContext, x: f64, y: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::pow({x:?}, {y:?})");

        if y == 0.0 {
            return Ok(1.0);
        }
        if y.is_nan() {
            return Ok(f64::NAN);
        }
        if (x == 1.0 || x == -1.0) && y.is_infinite() {
            return Ok(f64::NAN);
        }
        Ok(libm::pow(x, y))
    }

    async fn rint(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::rint({x:?})");
        Ok(libm::rint(x))
    }

    async fn ieee_remainder(_: &Jvm, _: &mut RuntimeContext, x: f64, y: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::IEEEremainder({x:?}, {y:?})");
        Ok(libm::remainder(x, y))
    }

    async fn round_float(_: &Jvm, _: &mut RuntimeContext, x: f32) -> Result<i32> {
        tracing::debug!("java.lang.Math::round({x:?})");

        if x.is_nan() {
            return Ok(0);
        }
        if x >= i32::MAX as f32 {
            return Ok(i32::MAX);
        }
        if x <= i32::MIN as f32 {
            return Ok(i32::MIN);
        }
        Ok(libm::floorf(x + 0.5) as i32)
    }

    async fn round_double(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<i64> {
        tracing::debug!("java.lang.Math::round({x:?})");

        if x.is_nan() {
            return Ok(0);
        }
        if x >= i64::MAX as f64 {
            return Ok(i64::MAX);
        }
        if x <= i64::MIN as f64 {
            return Ok(i64::MIN);
        }
        Ok(libm::floor(x + 0.5) as i64)
    }

    async fn random(jvm: &Jvm, _: &mut RuntimeContext) -> Result<f64> {
        tracing::debug!("java.lang.Math::random()");

        let mut random: ClassInstanceRef<Random> = jvm
            .get_static_field("java/lang/Math", "randomNumberGenerator", "Ljava/util/Random;")
            .await?;
        if random.is_null() {
            random = jvm.new_class("java/util/Random", "()V", ()).await?.into();
            jvm.put_static_field("java/lang/Math", "randomNumberGenerator", "Ljava/util/Random;", random.clone())
                .await?;
        }
        jvm.invoke_virtual(&random, "nextDouble", "()D", ()).await
    }
}
