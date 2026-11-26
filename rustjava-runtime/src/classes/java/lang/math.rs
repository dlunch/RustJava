use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::util::Random};

// public final class java.lang.Math
#[java_class(name = "java/lang/Math", public, final)]
pub struct Math {
    #[field(public, static, final)]
    E: f64,
    #[field(public, static, final)]
    PI: f64,
    #[field(private, static)]
    randomNumberGenerator: ClassInstanceRef<Random>,
}

#[java_methods]
impl Math {
    #[method(static_initializer)]
    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        jvm.put_static_field("java/lang/Math", "E", "D", core::f64::consts::E).await?;
        jvm.put_static_field("java/lang/Math", "PI", "D", core::f64::consts::PI).await
    }

    #[method(public, static)]
    async fn abs(_: &Jvm, _: &mut RuntimeContext, x: i32) -> Result<i32> {
        tracing::debug!("java.lang.Math::abs({x:?})");
        Ok(x.wrapping_abs())
    }

    #[method(name = "abs", public, static)]
    async fn abs_long(_: &Jvm, _: &mut RuntimeContext, x: i64) -> Result<i64> {
        tracing::debug!("java.lang.Math::abs({x:?})");
        Ok(x.wrapping_abs())
    }

    #[method(name = "abs", public, static)]
    async fn abs_float(_: &Jvm, _: &mut RuntimeContext, x: f32) -> Result<f32> {
        tracing::debug!("java.lang.Math::abs({x:?})");
        Ok(libm::fabsf(x))
    }

    #[method(name = "abs", public, static)]
    async fn abs_double(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::abs({x:?})");
        Ok(libm::fabs(x))
    }

    #[method(public, static)]
    async fn ceil(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::ceil({x:?})");
        Ok(libm::ceil(x))
    }

    #[method(public, static)]
    async fn floor(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::floor({x:?})");
        Ok(libm::floor(x))
    }

    #[method(public, static)]
    async fn sqrt(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::sqrt({x:?})");
        Ok(libm::sqrt(x))
    }

    #[method(public, static)]
    async fn sin(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::sin({x:?})");
        Ok(libm::sin(x))
    }

    #[method(public, static)]
    async fn cos(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::cos({x:?})");
        Ok(libm::cos(x))
    }

    #[method(public, static)]
    async fn tan(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::tan({x:?})");
        Ok(libm::tan(x))
    }

    #[method(public, static)]
    async fn to_degrees(_: &Jvm, _: &mut RuntimeContext, radians: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::toDegrees({radians:?})");
        Ok(radians * 180.0 / core::f64::consts::PI)
    }

    #[method(public, static)]
    async fn to_radians(_: &Jvm, _: &mut RuntimeContext, degrees: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::toRadians({degrees:?})");
        Ok(degrees / 180.0 * core::f64::consts::PI)
    }

    #[method(public, static)]
    async fn max(_: &Jvm, _: &mut RuntimeContext, x: i32, y: i32) -> Result<i32> {
        tracing::debug!("java.lang.Math::max({x:?}, {y:?})");
        Ok(x.max(y))
    }

    #[method(name = "max", public, static)]
    async fn max_long(_: &Jvm, _: &mut RuntimeContext, x: i64, y: i64) -> Result<i64> {
        tracing::debug!("java.lang.Math::max({x:?}, {y:?})");
        Ok(x.max(y))
    }

    #[method(name = "max", public, static)]
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

    #[method(name = "max", public, static)]
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

    #[method(public, static)]
    async fn min(_: &Jvm, _: &mut RuntimeContext, x: i32, y: i32) -> Result<i32> {
        tracing::debug!("java.lang.Math::min({x:?}, {y:?})");
        Ok(x.min(y))
    }

    #[method(name = "min", public, static)]
    async fn min_long(_: &Jvm, _: &mut RuntimeContext, x: i64, y: i64) -> Result<i64> {
        tracing::debug!("java.lang.Math::min({x:?}, {y:?})");
        Ok(x.min(y))
    }

    #[method(name = "min", public, static)]
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

    #[method(name = "min", public, static)]
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

    #[method(public, static)]
    async fn acos(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::acos({x:?})");
        Ok(libm::acos(x))
    }

    #[method(public, static)]
    async fn asin(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::asin({x:?})");
        Ok(libm::asin(x))
    }

    #[method(public, static)]
    async fn atan(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::atan({x:?})");
        Ok(libm::atan(x))
    }

    #[method(public, static)]
    async fn atan2(_: &Jvm, _: &mut RuntimeContext, y: f64, x: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::atan2({y:?}, {x:?})");
        Ok(libm::atan2(y, x))
    }

    #[method(public, static)]
    async fn exp(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::exp({x:?})");
        Ok(libm::exp(x))
    }

    #[method(public, static)]
    async fn log(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::log({x:?})");
        Ok(libm::log(x))
    }

    #[method(public, static)]
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

    #[method(public, static)]
    async fn rint(_: &Jvm, _: &mut RuntimeContext, x: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::rint({x:?})");
        Ok(libm::rint(x))
    }

    #[method(name = "IEEEremainder", public, static)]
    async fn ieee_remainder(_: &Jvm, _: &mut RuntimeContext, x: f64, y: f64) -> Result<f64> {
        tracing::debug!("java.lang.Math::IEEEremainder({x:?}, {y:?})");
        Ok(libm::remainder(x, y))
    }

    #[method(name = "round", public, static)]
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

    #[method(name = "round", public, static)]
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

    #[method(public, static, synchronized)]
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
        jvm.invoke_virtual(&random, "java/util/Random", "nextDouble", "()D", ()).await
    }
}
