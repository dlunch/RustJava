use alloc::{format, string::String as RustString, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{
    ClassInstanceRef, Jvm, Result,
    runtime::{JavaLangClass, JavaLangString},
};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::lang::{Object, String},
};

// public final class java.lang.Float
pub struct Float;

impl Float {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Float",
            parent_class: Some("java/lang/Number"),
            interfaces: vec!["java/lang/Comparable"],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "(F)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(D)V", Self::init_double, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_string, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "parseFloat",
                    "(Ljava/lang/String;)F",
                    Self::parse_float,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "valueOf",
                    "(Ljava/lang/String;)Ljava/lang/Float;",
                    Self::value_of_string,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("byteValue", "()B", Self::byte_value, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("shortValue", "()S", Self::short_value, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("intValue", "()I", Self::int_value, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("longValue", "()J", Self::long_value, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("floatValue", "()F", Self::float_value, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("doubleValue", "()D", Self::double_value, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "toString",
                    "(F)Ljava/lang/String;",
                    Self::to_string_static,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("isNaN", "()Z", Self::is_nan, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "isNaN",
                    "(F)Z",
                    Self::is_nan_static,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("isInfinite", "()Z", Self::is_infinite, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "isInfinite",
                    "(F)Z",
                    Self::is_infinite_static,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "floatToIntBits",
                    "(F)I",
                    Self::float_to_int_bits,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "intBitsToFloat",
                    "(I)F",
                    Self::int_bits_to_float,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("compareTo", "(Ljava/lang/Float;)I", Self::compare_to, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("compareTo", "(Ljava/lang/Object;)I", Self::compare_to_object, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new(
                    "POSITIVE_INFINITY",
                    "F",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "NEGATIVE_INFINITY",
                    "F",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new("NaN", "F", FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new(
                    "MAX_VALUE",
                    "F",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "MIN_VALUE",
                    "F",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "TYPE",
                    "Ljava/lang/Class;",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new("value", "F", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL),
            ],
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::FINAL,
        }
    }

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

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: f32) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Number", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "value", "F", value).await
    }

    async fn init_double(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: f64) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Number", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "value", "F", value as f32).await
    }

    async fn init_string(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: ClassInstanceRef<String>) -> Result<()> {
        if value.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "value").await);
        }
        let value = JavaLangString::to_rust_string(jvm, &value).await?;
        let value = Self::parse_value(jvm, &value).await?;
        let _: () = jvm.invoke_special(&this, "java/lang/Number", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "value", "F", value).await
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

    async fn parse_float(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>) -> Result<f32> {
        if value.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "value").await);
        }
        let value = JavaLangString::to_rust_string(jvm, &value).await?;
        Self::parse_value(jvm, &value).await
    }

    async fn value_of_string(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Self>> {
        if value.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "value").await);
        }
        let value = JavaLangString::to_rust_string(jvm, &value).await?;
        let value = Self::parse_value(jvm, &value).await?;
        Ok(jvm.new_class("java/lang/Float", "(F)V", (value,)).await?.into())
    }

    async fn byte_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i8> {
        let value: f32 = jvm.invoke_virtual(&this, "floatValue", "()F", ()).await?;
        Ok((value as i32) as i8)
    }

    async fn short_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i16> {
        let value: f32 = jvm.invoke_virtual(&this, "floatValue", "()F", ()).await?;
        Ok((value as i32) as i16)
    }

    async fn int_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let value: f32 = jvm.invoke_virtual(&this, "floatValue", "()F", ()).await?;
        Ok(value as i32)
    }

    async fn long_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        let value: f32 = jvm.invoke_virtual(&this, "floatValue", "()F", ()).await?;
        Ok(value as i64)
    }

    async fn float_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        jvm.get_field(&this, "value", "F").await
    }

    async fn double_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f64> {
        let value: f32 = jvm.invoke_virtual(&this, "floatValue", "()F", ()).await?;
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

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let value: f32 = jvm.invoke_virtual(&this, "floatValue", "()F", ()).await?;
        Ok(JavaLangString::from_rust_string(jvm, &Self::format_value(value)).await?.into())
    }

    async fn to_string_static(jvm: &Jvm, _: &mut RuntimeContext, value: f32) -> Result<ClassInstanceRef<String>> {
        Ok(JavaLangString::from_rust_string(jvm, &Self::format_value(value)).await?.into())
    }

    async fn is_nan(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let value: f32 = jvm.invoke_virtual(&this, "floatValue", "()F", ()).await?;
        Ok(value.is_nan())
    }

    async fn is_nan_static(_: &Jvm, _: &mut RuntimeContext, value: f32) -> Result<bool> {
        Ok(value.is_nan())
    }

    async fn is_infinite(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let value: f32 = jvm.invoke_virtual(&this, "floatValue", "()F", ()).await?;
        Ok(value.is_infinite())
    }

    async fn is_infinite_static(_: &Jvm, _: &mut RuntimeContext, value: f32) -> Result<bool> {
        Ok(value.is_infinite())
    }

    async fn float_to_int_bits(_: &Jvm, _: &mut RuntimeContext, value: f32) -> Result<i32> {
        Ok(if value.is_nan() { 0x7fc0_0000 } else { value.to_bits() as i32 })
    }

    async fn int_bits_to_float(_: &Jvm, _: &mut RuntimeContext, bits: i32) -> Result<f32> {
        Ok(f32::from_bits(bits as u32))
    }

    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let value: f32 = jvm.invoke_virtual(&this, "floatValue", "()F", ()).await?;
        Ok(if value.is_nan() { 0x7fc0_0000 } else { value.to_bits() as i32 })
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if other.is_null() || !jvm.is_instance(&**other, "java/lang/Float") {
            return Ok(false);
        }
        let left: f32 = jvm.invoke_virtual(&this, "floatValue", "()F", ()).await?;
        let right: f32 = jvm.invoke_virtual(&other, "floatValue", "()F", ()).await?;
        let left_bits = if left.is_nan() { 0x7fc0_0000 } else { left.to_bits() as i32 };
        let right_bits = if right.is_nan() { 0x7fc0_0000 } else { right.to_bits() as i32 };
        Ok(left_bits == right_bits)
    }

    async fn compare_to(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<i32> {
        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "other").await);
        }
        let left: f32 = jvm.invoke_virtual(&this, "floatValue", "()F", ()).await?;
        let right: f32 = jvm.invoke_virtual(&other, "floatValue", "()F", ()).await?;
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

    async fn compare_to_object(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<i32> {
        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "other").await);
        }
        if !jvm.is_instance(&**other, "java/lang/Float") {
            return Err(jvm.exception("java/lang/ClassCastException", "not Float").await);
        }
        let other = ClassInstanceRef::<Self>::from(other.instance);
        let left: f32 = jvm.invoke_virtual(&this, "floatValue", "()F", ()).await?;
        let right: f32 = jvm.invoke_virtual(&other, "floatValue", "()F", ()).await?;
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
