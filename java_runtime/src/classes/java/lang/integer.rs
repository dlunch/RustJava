use alloc::{format, string::String as RustString, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{
    ClassInstanceRef, JavaChar, JavaError, Jvm, Result,
    runtime::{JavaLangClass, JavaLangString},
};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::lang::{Character, Object, String},
};

// public final class java.lang.Integer
pub struct Integer;

impl Integer {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Integer",
            parent_class: Some("java/lang/Number"),
            interfaces: vec!["java/lang/Comparable"],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "(I)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_string, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "parseInt",
                    "(Ljava/lang/String;)I",
                    Self::parse_int,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "parseInt",
                    "(Ljava/lang/String;I)I",
                    Self::parse_int_radix,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "valueOf",
                    "(I)Ljava/lang/Integer;",
                    Self::value_of,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "valueOf",
                    "(Ljava/lang/String;)Ljava/lang/Integer;",
                    Self::value_of_string,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "valueOf",
                    "(Ljava/lang/String;I)Ljava/lang/Integer;",
                    Self::value_of_string_radix,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "decode",
                    "(Ljava/lang/String;)Ljava/lang/Integer;",
                    Self::decode,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "getInteger",
                    "(Ljava/lang/String;)Ljava/lang/Integer;",
                    Self::get_integer,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "getInteger",
                    "(Ljava/lang/String;I)Ljava/lang/Integer;",
                    Self::get_integer_value_default,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "getInteger",
                    "(Ljava/lang/String;Ljava/lang/Integer;)Ljava/lang/Integer;",
                    Self::get_integer_object_default,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("intValue", "()I", Self::int_value, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("longValue", "()J", Self::long_value, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("floatValue", "()F", Self::float_value, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("doubleValue", "()D", Self::double_value, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "toString",
                    "(I)Ljava/lang/String;",
                    Self::to_string_static,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "toString",
                    "(II)Ljava/lang/String;",
                    Self::to_string_radix_static,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "toBinaryString",
                    "(I)Ljava/lang/String;",
                    Self::to_binary_string,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "toOctalString",
                    "(I)Ljava/lang/String;",
                    Self::to_octal_string,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "toHexString",
                    "(I)Ljava/lang/String;",
                    Self::to_hex_string,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("compareTo", "(Ljava/lang/Integer;)I", Self::compare_to, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("compareTo", "(Ljava/lang/Object;)I", Self::compare_to_object, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new(
                    "MIN_VALUE",
                    "I",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "MAX_VALUE",
                    "I",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "TYPE",
                    "Ljava/lang/Class;",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new("value", "I", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL),
            ],
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::FINAL,
        }
    }

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

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Number", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "value", "I", value).await
    }

    async fn init_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<String>) -> Result<()> {
        if value.is_null() {
            return Err(jvm.exception("java/lang/NumberFormatException", "null").await);
        }
        let value = JavaLangString::to_rust_string(jvm, &value).await?;
        let value = Self::parse_value(jvm, &value, 10).await?;
        let mut this = this;
        let _: () = jvm.invoke_special(&this, "java/lang/Number", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "value", "I", value).await
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

    async fn parse_int(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>) -> Result<i32> {
        if value.is_null() {
            return Err(jvm.exception("java/lang/NumberFormatException", "null").await);
        }
        let value = JavaLangString::to_rust_string(jvm, &value).await?;
        Self::parse_value(jvm, &value, 10).await
    }

    async fn parse_int_radix(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>, radix: i32) -> Result<i32> {
        if value.is_null() {
            return Err(jvm.exception("java/lang/NumberFormatException", "null").await);
        }
        let value = JavaLangString::to_rust_string(jvm, &value).await?;
        Self::parse_value(jvm, &value, radix as u32).await
    }

    async fn value_of(jvm: &Jvm, _: &mut RuntimeContext, value: i32) -> Result<ClassInstanceRef<Self>> {
        Ok(jvm.new_class("java/lang/Integer", "(I)V", (value,)).await?.into())
    }

    async fn value_of_string(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Self>> {
        if value.is_null() {
            return Err(jvm.exception("java/lang/NumberFormatException", "null").await);
        }
        let value = JavaLangString::to_rust_string(jvm, &value).await?;
        let value = Self::parse_value(jvm, &value, 10).await?;
        Ok(jvm.new_class("java/lang/Integer", "(I)V", (value,)).await?.into())
    }

    async fn value_of_string_radix(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>, radix: i32) -> Result<ClassInstanceRef<Self>> {
        if value.is_null() {
            return Err(jvm.exception("java/lang/NumberFormatException", "null").await);
        }
        let value = JavaLangString::to_rust_string(jvm, &value).await?;
        let value = Self::parse_value(jvm, &value, radix as u32).await?;
        Ok(jvm.new_class("java/lang/Integer", "(I)V", (value,)).await?.into())
    }

    async fn decode(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Self>> {
        if value.is_null() {
            return Err(jvm.exception("java/lang/NumberFormatException", "null").await);
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

    async fn int_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "value", "I").await
    }

    async fn long_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        let value: i32 = jvm.invoke_virtual(&this, "intValue", "()I", ()).await?;
        Ok(value as i64)
    }

    async fn float_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        let value: i32 = jvm.invoke_virtual(&this, "intValue", "()I", ()).await?;
        Ok(value as f32)
    }

    async fn double_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f64> {
        let value: i32 = jvm.invoke_virtual(&this, "intValue", "()I", ()).await?;
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

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let value: i32 = jvm.invoke_virtual(&this, "intValue", "()I", ()).await?;
        Ok(JavaLangString::from_rust_string(jvm, &Self::format_value(value as i64, 10)).await?.into())
    }

    async fn to_string_static(jvm: &Jvm, _: &mut RuntimeContext, value: i32) -> Result<ClassInstanceRef<String>> {
        Ok(JavaLangString::from_rust_string(jvm, &Self::format_value(value as i64, 10)).await?.into())
    }

    async fn to_string_radix_static(jvm: &Jvm, _: &mut RuntimeContext, value: i32, radix: i32) -> Result<ClassInstanceRef<String>> {
        Ok(JavaLangString::from_rust_string(jvm, &Self::format_value(value as i64, radix as u32))
            .await?
            .into())
    }

    async fn to_binary_string(jvm: &Jvm, _: &mut RuntimeContext, value: i32) -> Result<ClassInstanceRef<String>> {
        let text = if value < 0 {
            format!("{:032b}", value as u32)
        } else {
            Self::format_value(value as i64, 2)
        };
        Ok(JavaLangString::from_rust_string(jvm, &text).await?.into())
    }

    async fn to_octal_string(jvm: &Jvm, _: &mut RuntimeContext, value: i32) -> Result<ClassInstanceRef<String>> {
        let text = if value < 0 {
            format!("{:011o}", value as u32)
        } else {
            Self::format_value(value as i64, 8)
        };
        Ok(JavaLangString::from_rust_string(jvm, &text).await?.into())
    }

    async fn to_hex_string(jvm: &Jvm, _: &mut RuntimeContext, value: i32) -> Result<ClassInstanceRef<String>> {
        let text = if value < 0 {
            format!("{:08x}", value as u32)
        } else {
            Self::format_value(value as i64, 16)
        };
        Ok(JavaLangString::from_rust_string(jvm, &text).await?.into())
    }

    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "value", "I").await
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if other.is_null() || !jvm.is_instance(&**other, "java/lang/Integer") {
            return Ok(false);
        }
        let this_value: i32 = jvm.invoke_virtual(&this, "intValue", "()I", ()).await?;
        let other_value: i32 = jvm.invoke_virtual(&other, "intValue", "()I", ()).await?;
        Ok(this_value == other_value)
    }

    async fn compare_to(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<i32> {
        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "other").await);
        }
        let this_value: i32 = jvm.invoke_virtual(&this, "intValue", "()I", ()).await?;
        let other_value: i32 = jvm.invoke_virtual(&other, "intValue", "()I", ()).await?;
        Ok(this_value.cmp(&other_value) as i32)
    }

    async fn compare_to_object(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<i32> {
        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "other").await);
        }
        if !jvm.is_instance(&**other, "java/lang/Integer") {
            return Err(jvm.exception("java/lang/ClassCastException", "java/lang/Object is not Integer").await);
        }
        let other = ClassInstanceRef::<Self>::from(other.instance);
        let this_value: i32 = jvm.invoke_virtual(&this, "intValue", "()I", ()).await?;
        let other_value: i32 = jvm.invoke_virtual(&other, "intValue", "()I", ()).await?;
        Ok(this_value.cmp(&other_value) as i32)
    }
}
