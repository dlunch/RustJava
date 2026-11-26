use alloc::{format, string::String as RustString};

use jvm::{
    ClassInstanceRef, Jvm, Result,
    runtime::{JavaLangClass, JavaLangString},
};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::lang::{Class, Object, String},
};

// public final class java.lang.Float
#[java_class(name = "java/lang/Float", extends = "java/lang/Number", implements("java/lang/Comparable"), public, final)]
pub struct Float {
    #[field(public, static, final)]
    POSITIVE_INFINITY: f32,
    #[field(public, static, final)]
    NEGATIVE_INFINITY: f32,
    #[field(public, static, final)]
    NaN: f32,
    #[field(public, static, final)]
    MAX_VALUE: f32,
    #[field(public, static, final)]
    MIN_VALUE: f32,
    #[field(public, static, final)]
    TYPE: ClassInstanceRef<Class>,
    #[field(private, final)]
    value: f32,
}

#[java_methods]
impl Float {
    #[method(static_initializer)]
    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        jvm.put_static_field("java/lang/Float", "POSITIVE_INFINITY", "F", f32::INFINITY).await?;
        jvm.put_static_field("java/lang/Float", "NEGATIVE_INFINITY", "F", f32::NEG_INFINITY)
            .await?;
        jvm.put_static_field("java/lang/Float", "NaN", "F", f32::NAN).await?;
        jvm.put_static_field("java/lang/Float", "MAX_VALUE", "F", f32::MAX).await?;
        jvm.put_static_field("java/lang/Float", "MIN_VALUE", "F", f32::from_bits(1)).await?;
        jvm.put_static_field(
            "java/lang/Float",
            "TYPE",
            "Ljava/lang/Class;",
            JavaLangClass::from_rust_primitive(jvm, "float").await?,
        )
        .await
    }

    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: f32) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Number", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/lang/Float", "value", "F", value).await
    }

    #[method(constructor, public)]
    async fn init_double(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: f64) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Number", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/lang/Float", "value", "F", value as f32).await
    }

    #[method(constructor, public)]
    async fn init_string(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: ClassInstanceRef<String>) -> Result<()> {
        if value.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "value").await);
        }
        let value = JavaLangString::to_rust_string(jvm, &value).await?;
        let value = Self::parse_value(jvm, &value).await?;
        let _: () = jvm.invoke_special(&this, "java/lang/Number", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/lang/Float", "value", "F", value).await
    }

    async fn parse_value(jvm: &Jvm, value: &str) -> Result<f32> {
        let trimmed = value.trim_matches(|character| character <= '\u{0020}');
        let parsed = match trimmed {
            "NaN" | "+NaN" | "-NaN" => Some(f32::NAN),
            "Infinity" | "+Infinity" => Some(f32::INFINITY),
            "-Infinity" => Some(f32::NEG_INFINITY),
            _ => {
                let number = match trimmed.as_bytes().last() {
                    Some(b'f' | b'F' | b'd' | b'D') => &trimmed[..trimmed.len() - 1],
                    _ => trimmed,
                };
                let bytes = number.as_bytes();
                let mut index = 0;
                if matches!(bytes.first(), Some(b'+' | b'-')) {
                    index += 1;
                }

                let mut digits = 0;
                while index < bytes.len() && bytes[index].is_ascii_digit() {
                    index += 1;
                    digits += 1;
                }
                if index < bytes.len() && bytes[index] == b'.' {
                    index += 1;
                    while index < bytes.len() && bytes[index].is_ascii_digit() {
                        index += 1;
                        digits += 1;
                    }
                }

                if digits == 0 {
                    None
                } else {
                    if index < bytes.len() && matches!(bytes[index], b'e' | b'E') {
                        index += 1;
                        if index < bytes.len() && matches!(bytes[index], b'+' | b'-') {
                            index += 1;
                        }
                        let exponent_start = index;
                        while index < bytes.len() && bytes[index].is_ascii_digit() {
                            index += 1;
                        }
                        if exponent_start == index {
                            index = bytes.len() + 1;
                        }
                    }
                    if index == bytes.len() { number.parse::<f32>().ok() } else { None }
                }
            }
        };

        match parsed {
            Some(value) => Ok(value),
            None => Err(jvm
                .exception("java/lang/NumberFormatException", &format!("For input string: \"{value}\""))
                .await),
        }
    }

    #[method(public, static)]
    async fn parse_float(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>) -> Result<f32> {
        if value.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "value").await);
        }
        let value = JavaLangString::to_rust_string(jvm, &value).await?;
        Self::parse_value(jvm, &value).await
    }

    #[method(name = "valueOf", public, static)]
    async fn value_of_string(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Self>> {
        if value.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "value").await);
        }
        let value = JavaLangString::to_rust_string(jvm, &value).await?;
        let value = Self::parse_value(jvm, &value).await?;
        Ok(jvm.new_class("java/lang/Float", "(F)V", (value,)).await?.into())
    }

    #[method(public, static)]
    async fn value_of(jvm: &Jvm, _: &mut RuntimeContext, value: f32) -> Result<ClassInstanceRef<Self>> {
        Ok(jvm.new_class("java/lang/Float", "(F)V", (value,)).await?.into())
    }

    #[method(public)]
    async fn byte_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i8> {
        let value: f32 = jvm.invoke_virtual(&this, "java/lang/Float", "floatValue", "()F", ()).await?;
        Ok((value as i32) as i8)
    }

    #[method(public)]
    async fn short_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i16> {
        let value: f32 = jvm.invoke_virtual(&this, "java/lang/Float", "floatValue", "()F", ()).await?;
        Ok((value as i32) as i16)
    }

    #[method(public)]
    async fn int_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let value: f32 = jvm.invoke_virtual(&this, "java/lang/Float", "floatValue", "()F", ()).await?;
        Ok(value as i32)
    }

    #[method(public)]
    async fn long_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        let value: f32 = jvm.invoke_virtual(&this, "java/lang/Float", "floatValue", "()F", ()).await?;
        Ok(value as i64)
    }

    #[method(public)]
    async fn float_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "java/lang/Float", "value", "F").await
    }

    #[method(public)]
    async fn double_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f64> {
        let value: f32 = jvm.invoke_virtual(&this, "java/lang/Float", "floatValue", "()F", ()).await?;
        Ok(value as f64)
    }

    fn format_value(value: f32) -> RustString {
        if value.is_nan() {
            return "NaN".into();
        }
        if value == f32::INFINITY {
            return "Infinity".into();
        }
        if value == f32::NEG_INFINITY {
            return "-Infinity".into();
        }
        if value.to_bits() == 1 {
            return "1.4E-45".into();
        }
        if value.to_bits() == 0x8000_0001 {
            return "-1.4E-45".into();
        }

        let magnitude = value.abs();
        let mut result = if value != 0.0 && !(0.001..10_000_000.0).contains(&magnitude) {
            format!("{value:e}")
        } else {
            format!("{value}")
        };
        if let Some(exponent_index) = result.find('e') {
            let exponent = result.split_off(exponent_index + 1);
            result.pop();
            if !result.contains('.') {
                result.push_str(".0");
            }
            result.push('E');
            result.push_str(exponent.strip_prefix('+').unwrap_or(&exponent));
        } else if !result.contains('.') {
            result.push_str(".0");
        }
        result
    }

    #[method(public)]
    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let value: f32 = jvm.invoke_virtual(&this, "java/lang/Float", "floatValue", "()F", ()).await?;
        Ok(JavaLangString::from_rust_string(jvm, &Self::format_value(value)).await?.into())
    }

    #[method(name = "toString", public, static)]
    async fn to_string_static(jvm: &Jvm, _: &mut RuntimeContext, value: f32) -> Result<ClassInstanceRef<String>> {
        Ok(JavaLangString::from_rust_string(jvm, &Self::format_value(value)).await?.into())
    }

    #[method(name = "isNaN", public)]
    async fn is_nan(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let value: f32 = jvm.invoke_virtual(&this, "java/lang/Float", "floatValue", "()F", ()).await?;
        Ok(value.is_nan())
    }

    #[method(name = "isNaN", public, static)]
    async fn is_nan_static(_: &Jvm, _: &mut RuntimeContext, value: f32) -> Result<bool> {
        Ok(value.is_nan())
    }

    #[method(public)]
    async fn is_infinite(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let value: f32 = jvm.invoke_virtual(&this, "java/lang/Float", "floatValue", "()F", ()).await?;
        Ok(value.is_infinite())
    }

    #[method(name = "isInfinite", public, static)]
    async fn is_infinite_static(_: &Jvm, _: &mut RuntimeContext, value: f32) -> Result<bool> {
        Ok(value.is_infinite())
    }

    #[method(public, static)]
    async fn float_to_int_bits(_: &Jvm, _: &mut RuntimeContext, value: f32) -> Result<i32> {
        Ok(if value.is_nan() { 0x7fc0_0000 } else { value.to_bits() as i32 })
    }

    #[method(public, static)]
    async fn int_bits_to_float(_: &Jvm, _: &mut RuntimeContext, bits: i32) -> Result<f32> {
        Ok(f32::from_bits(bits as u32))
    }

    #[method(public)]
    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let value: f32 = jvm.invoke_virtual(&this, "java/lang/Float", "floatValue", "()F", ()).await?;
        Ok(if value.is_nan() { 0x7fc0_0000 } else { value.to_bits() as i32 })
    }

    #[method(public)]
    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if other.is_null() || !jvm.is_instance(&**other, "java/lang/Float") {
            return Ok(false);
        }
        let left: f32 = jvm.invoke_virtual(&this, "java/lang/Float", "floatValue", "()F", ()).await?;
        let right: f32 = jvm.invoke_virtual(&other, "java/lang/Float", "floatValue", "()F", ()).await?;
        let left_bits = if left.is_nan() { 0x7fc0_0000 } else { left.to_bits() as i32 };
        let right_bits = if right.is_nan() { 0x7fc0_0000 } else { right.to_bits() as i32 };
        Ok(left_bits == right_bits)
    }

    #[method(public)]
    async fn compare_to(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<i32> {
        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "other").await);
        }
        let left: f32 = jvm.invoke_virtual(&this, "java/lang/Float", "floatValue", "()F", ()).await?;
        let right: f32 = jvm.invoke_virtual(&other, "java/lang/Float", "floatValue", "()F", ()).await?;
        if left < right {
            return Ok(-1);
        }
        if left > right {
            return Ok(1);
        }
        let left_bits = if left.is_nan() { 0x7fc0_0000 } else { left.to_bits() as i32 };
        let right_bits = if right.is_nan() { 0x7fc0_0000 } else { right.to_bits() as i32 };
        Ok(left_bits.cmp(&right_bits) as i32)
    }

    #[method(name = "compareTo", public)]
    async fn compare_to_object(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<i32> {
        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "other").await);
        }
        if !jvm.is_instance(&**other, "java/lang/Float") {
            return Err(jvm.exception("java/lang/ClassCastException", "not Float").await);
        }
        let other = ClassInstanceRef::<Self>::from(other.instance);
        let left: f32 = jvm.invoke_virtual(&this, "java/lang/Float", "floatValue", "()F", ()).await?;
        let right: f32 = jvm.invoke_virtual(&other, "java/lang/Float", "floatValue", "()F", ()).await?;
        if left < right {
            return Ok(-1);
        }
        if left > right {
            return Ok(1);
        }
        let left_bits = if left.is_nan() { 0x7fc0_0000 } else { left.to_bits() as i32 };
        let right_bits = if right.is_nan() { 0x7fc0_0000 } else { right.to_bits() as i32 };
        Ok(left_bits.cmp(&right_bits) as i32)
    }
}
