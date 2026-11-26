use alloc::{format, string::String as RustString};

use jvm::{
    ClassInstanceRef, JavaChar, JavaError, Jvm, Result,
    runtime::{JavaLangClass, JavaLangString},
};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::lang::{Character, Class, Object, String},
};

// public final class java.lang.Integer
#[java_class(
    name = "java/lang/Integer",
    extends = "java/lang/Number",
    implements("java/lang/Comparable"),
    public,
    final
)]
pub struct Integer {
    #[field(public, static, final)]
    MIN_VALUE: i32,
    #[field(public, static, final)]
    MAX_VALUE: i32,
    #[field(public, static, final)]
    TYPE: ClassInstanceRef<Class>,
    #[field(private, final)]
    value: i32,
}

#[java_methods]
impl Integer {
    #[method(static_initializer)]
    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        jvm.put_static_field("java/lang/Integer", "MIN_VALUE", "I", i32::MIN).await?;
        jvm.put_static_field("java/lang/Integer", "MAX_VALUE", "I", i32::MAX).await?;
        jvm.put_static_field(
            "java/lang/Integer",
            "TYPE",
            "Ljava/lang/Class;",
            JavaLangClass::from_rust_primitive(jvm, "int").await?,
        )
        .await
    }

    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Number", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/lang/Integer", "value", "I", value).await
    }

    #[method(constructor, public)]
    async fn init_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<String>) -> Result<()> {
        if value.is_null() {
            return Err(jvm.exception("java/lang/NumberFormatException", "null").await);
        }
        let value = JavaLangString::to_rust_string(jvm, &value).await?;
        let value = Self::parse_value(jvm, &value, 10).await?;
        let mut this = this;
        let _: () = jvm.invoke_special(&this, "java/lang/Number", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/lang/Integer", "value", "I", value).await
    }

    fn parse_value_raw(value: &str, radix: u32) -> Option<i32> {
        if !(2..=36).contains(&radix) || value.is_empty() {
            return None;
        }
        let mut chars = value.chars();
        let negative = match chars.next() {
            Some('-') => true,
            Some('+') => false,
            Some(first) => {
                chars = value.chars();
                let _ = first;
                false
            }
            None => return None,
        };
        let limit = if negative { i64::from(i32::MIN) } else { -i64::from(i32::MAX) };
        let mut result = 0i64;
        let mut count = 0;
        for value in chars {
            let value = JavaChar::try_from(u32::from(value)).ok()?;
            let digit = i64::from(Character::digit_value(value, radix as i32));
            if digit < 0 {
                return None;
            }
            if result < (limit + digit) / i64::from(radix) {
                return None;
            }
            result = result * i64::from(radix) - digit;
            count += 1;
        }
        if count == 0 || (!negative && result == i64::from(i32::MIN)) {
            return None;
        }
        Some(if negative { result as i32 } else { -result as i32 })
    }

    async fn parse_value(jvm: &Jvm, value: &str, radix: u32) -> Result<i32> {
        match Self::parse_value_raw(value, radix) {
            Some(value) => Ok(value),
            None => Err(jvm
                .exception("java/lang/NumberFormatException", &format!("For input string: \"{value}\""))
                .await),
        }
    }

    #[method(public, static)]
    async fn parse_int(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>) -> Result<i32> {
        if value.is_null() {
            return Err(jvm.exception("java/lang/NumberFormatException", "null").await);
        }
        let value = JavaLangString::to_rust_string(jvm, &value).await?;
        Self::parse_value(jvm, &value, 10).await
    }

    #[method(name = "parseInt", public, static)]
    async fn parse_int_radix(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>, radix: i32) -> Result<i32> {
        if value.is_null() {
            return Err(jvm.exception("java/lang/NumberFormatException", "null").await);
        }
        let value = JavaLangString::to_rust_string(jvm, &value).await?;
        Self::parse_value(jvm, &value, radix as u32).await
    }

    #[method(public, static)]
    async fn value_of(jvm: &Jvm, _: &mut RuntimeContext, value: i32) -> Result<ClassInstanceRef<Self>> {
        Ok(jvm.new_class("java/lang/Integer", "(I)V", (value,)).await?.into())
    }

    #[method(name = "valueOf", public, static)]
    async fn value_of_string(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Self>> {
        if value.is_null() {
            return Err(jvm.exception("java/lang/NumberFormatException", "null").await);
        }
        let value = JavaLangString::to_rust_string(jvm, &value).await?;
        let value = Self::parse_value(jvm, &value, 10).await?;
        Ok(jvm.new_class("java/lang/Integer", "(I)V", (value,)).await?.into())
    }

    #[method(name = "valueOf", public, static)]
    async fn value_of_string_radix(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>, radix: i32) -> Result<ClassInstanceRef<Self>> {
        if value.is_null() {
            return Err(jvm.exception("java/lang/NumberFormatException", "null").await);
        }
        let value = JavaLangString::to_rust_string(jvm, &value).await?;
        let value = Self::parse_value(jvm, &value, radix as u32).await?;
        Ok(jvm.new_class("java/lang/Integer", "(I)V", (value,)).await?.into())
    }

    #[method(public, static)]
    async fn decode(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Self>> {
        if value.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "value").await);
        }
        let value = JavaLangString::to_rust_string(jvm, &value).await?;
        let (sign, body) = if let Some(body) = value.strip_prefix('-') {
            ("-", body)
        } else if let Some(body) = value.strip_prefix('+') {
            ("+", body)
        } else {
            ("", value.as_str())
        };
        let (radix, body) = if let Some(body) = body
            .strip_prefix("0x")
            .or_else(|| body.strip_prefix("0X"))
            .or_else(|| body.strip_prefix('#'))
        {
            (16, body)
        } else if body.starts_with('0') && body.len() > 1 {
            (8, &body[1..])
        } else {
            (10, body)
        };
        if body.starts_with('-') || body.starts_with('+') {
            return Err(jvm.exception("java/lang/NumberFormatException", "Sign character in wrong position").await);
        }
        let mut signed = RustString::from(sign);
        signed.push_str(body);
        let parsed = Self::parse_value(jvm, &signed, radix).await?;
        Ok(jvm.new_class("java/lang/Integer", "(I)V", (parsed,)).await?.into())
    }

    #[method(public, static)]
    async fn get_integer(jvm: &Jvm, _: &mut RuntimeContext, key: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Self>> {
        let default: ClassInstanceRef<Self> = None.into();
        jvm.invoke_static(
            "java/lang/Integer",
            "getInteger",
            "(Ljava/lang/String;Ljava/lang/Integer;)Ljava/lang/Integer;",
            (key, default),
        )
        .await
    }

    #[method(name = "getInteger", public, static)]
    async fn get_integer_value_default(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        key: ClassInstanceRef<String>,
        default: i32,
    ) -> Result<ClassInstanceRef<Self>> {
        let default = jvm.new_class("java/lang/Integer", "(I)V", (default,)).await?;
        jvm.invoke_static(
            "java/lang/Integer",
            "getInteger",
            "(Ljava/lang/String;Ljava/lang/Integer;)Ljava/lang/Integer;",
            (key, default),
        )
        .await
    }

    #[method(name = "getInteger", public, static)]
    async fn get_integer_object_default(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        key: ClassInstanceRef<String>,
        default: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<Self>> {
        if key.is_null() {
            return Ok(default);
        }
        let value: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/System", "getProperty", "(Ljava/lang/String;)Ljava/lang/String;", (key,))
            .await?;
        if value.is_null() {
            return Ok(default);
        }
        match jvm
            .invoke_static("java/lang/Integer", "decode", "(Ljava/lang/String;)Ljava/lang/Integer;", (value,))
            .await
        {
            Ok(value) => Ok(value),
            Err(JavaError::JavaException(exception)) if jvm.is_instance(&*exception, "java/lang/NumberFormatException") => Ok(default),
            Err(error) => Err(error),
        }
    }

    #[method(public)]
    async fn int_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "java/lang/Integer", "value", "I").await
    }

    #[method(public)]
    async fn long_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        let value: i32 = jvm.invoke_virtual(&this, "java/lang/Integer", "intValue", "()I", ()).await?;
        Ok(value as i64)
    }

    #[method(public)]
    async fn float_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        let value: i32 = jvm.invoke_virtual(&this, "java/lang/Integer", "intValue", "()I", ()).await?;
        Ok(value as f32)
    }

    #[method(public)]
    async fn double_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f64> {
        let value: i32 = jvm.invoke_virtual(&this, "java/lang/Integer", "intValue", "()I", ()).await?;
        Ok(value as f64)
    }

    fn format_value(value: i64, radix: u32) -> RustString {
        let radix = if (2..=36).contains(&radix) { radix } else { 10 };
        if value == 0 {
            return "0".into();
        }
        let negative = value < 0;
        let mut magnitude = value.unsigned_abs();
        let mut result = RustString::new();
        while magnitude != 0 {
            let digit = (magnitude % u64::from(radix)) as u32;
            result.push(char::from_digit(digit, radix).unwrap_or('0'));
            magnitude /= u64::from(radix);
        }
        if negative {
            result.push('-');
        }
        result.chars().rev().collect()
    }

    #[method(public)]
    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let value: i32 = jvm.invoke_virtual(&this, "java/lang/Integer", "intValue", "()I", ()).await?;
        Ok(JavaLangString::from_rust_string(jvm, &Self::format_value(value as i64, 10)).await?.into())
    }

    #[method(name = "toString", public, static)]
    async fn to_string_static(jvm: &Jvm, _: &mut RuntimeContext, value: i32) -> Result<ClassInstanceRef<String>> {
        Ok(JavaLangString::from_rust_string(jvm, &Self::format_value(value as i64, 10)).await?.into())
    }

    #[method(name = "toString", public, static)]
    async fn to_string_radix_static(jvm: &Jvm, _: &mut RuntimeContext, value: i32, radix: i32) -> Result<ClassInstanceRef<String>> {
        Ok(JavaLangString::from_rust_string(jvm, &Self::format_value(value as i64, radix as u32))
            .await?
            .into())
    }

    #[method(public, static)]
    async fn to_binary_string(jvm: &Jvm, _: &mut RuntimeContext, value: i32) -> Result<ClassInstanceRef<String>> {
        let text = if value < 0 {
            format!("{:032b}", value as u32)
        } else {
            Self::format_value(value as i64, 2)
        };
        Ok(JavaLangString::from_rust_string(jvm, &text).await?.into())
    }

    #[method(public, static)]
    async fn to_octal_string(jvm: &Jvm, _: &mut RuntimeContext, value: i32) -> Result<ClassInstanceRef<String>> {
        let text = if value < 0 {
            format!("{:011o}", value as u32)
        } else {
            Self::format_value(value as i64, 8)
        };
        Ok(JavaLangString::from_rust_string(jvm, &text).await?.into())
    }

    #[method(public, static)]
    async fn to_hex_string(jvm: &Jvm, _: &mut RuntimeContext, value: i32) -> Result<ClassInstanceRef<String>> {
        let text = if value < 0 {
            format!("{:08x}", value as u32)
        } else {
            Self::format_value(value as i64, 16)
        };
        Ok(JavaLangString::from_rust_string(jvm, &text).await?.into())
    }

    #[method(public)]
    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "java/lang/Integer", "value", "I").await
    }

    #[method(public)]
    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if other.is_null() || !jvm.is_instance(&**other, "java/lang/Integer") {
            return Ok(false);
        }
        let this_value: i32 = jvm.get_field(&this, "java/lang/Integer", "value", "I").await?;
        let other_value: i32 = jvm.get_field(&other, "java/lang/Integer", "value", "I").await?;
        Ok(this_value == other_value)
    }

    #[method(public)]
    async fn compare_to(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<i32> {
        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "other").await);
        }
        let this_value: i32 = jvm.invoke_virtual(&this, "java/lang/Integer", "intValue", "()I", ()).await?;
        let other_value: i32 = jvm.invoke_virtual(&other, "java/lang/Integer", "intValue", "()I", ()).await?;
        Ok(this_value.cmp(&other_value) as i32)
    }

    #[method(name = "compareTo", public)]
    async fn compare_to_object(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<i32> {
        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "other").await);
        }
        if !jvm.is_instance(&**other, "java/lang/Integer") {
            return Err(jvm.exception("java/lang/ClassCastException", "java/lang/Object is not Integer").await);
        }
        let other = ClassInstanceRef::<Self>::from(other.instance);
        let this_value: i32 = jvm.invoke_virtual(&this, "java/lang/Integer", "intValue", "()I", ()).await?;
        let other_value: i32 = jvm.invoke_virtual(&other, "java/lang/Integer", "intValue", "()I", ()).await?;
        Ok(this_value.cmp(&other_value) as i32)
    }
}
