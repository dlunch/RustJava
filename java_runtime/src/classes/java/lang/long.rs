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

// public final class java.lang.Long
pub struct Long;

impl Long {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Long",
            parent_class: Some("java/lang/Number"),
            interfaces: vec!["java/lang/Comparable"],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "(J)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_string, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "parseLong",
                    "(Ljava/lang/String;)J",
                    Self::parse_long,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "parseLong",
                    "(Ljava/lang/String;I)J",
                    Self::parse_long_radix,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "valueOf",
                    "(Ljava/lang/String;)Ljava/lang/Long;",
                    Self::value_of_string,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "valueOf",
                    "(Ljava/lang/String;I)Ljava/lang/Long;",
                    Self::value_of_string_radix,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "decode",
                    "(Ljava/lang/String;)Ljava/lang/Long;",
                    Self::decode,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "getLong",
                    "(Ljava/lang/String;)Ljava/lang/Long;",
                    Self::get_long,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "getLong",
                    "(Ljava/lang/String;J)Ljava/lang/Long;",
                    Self::get_long_value_default,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "getLong",
                    "(Ljava/lang/String;Ljava/lang/Long;)Ljava/lang/Long;",
                    Self::get_long_object_default,
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
                    "(J)Ljava/lang/String;",
                    Self::to_string_static,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "toString",
                    "(JI)Ljava/lang/String;",
                    Self::to_string_radix,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "toBinaryString",
                    "(J)Ljava/lang/String;",
                    Self::to_binary_string,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "toOctalString",
                    "(J)Ljava/lang/String;",
                    Self::to_octal_string,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "toHexString",
                    "(J)Ljava/lang/String;",
                    Self::to_hex_string,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("compareTo", "(Ljava/lang/Long;)I", Self::compare_to, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("compareTo", "(Ljava/lang/Object;)I", Self::compare_to_object, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new(
                    "MIN_VALUE",
                    "J",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "MAX_VALUE",
                    "J",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "TYPE",
                    "Ljava/lang/Class;",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new("value", "J", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL),
            ],
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::FINAL,
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        jvm.put_static_field("java/lang/Long", "MIN_VALUE", "J", i64::MIN).await?;
        jvm.put_static_field("java/lang/Long", "MAX_VALUE", "J", i64::MAX).await?;
        jvm.put_static_field(
            "java/lang/Long",
            "TYPE",
            "Ljava/lang/Class;",
            JavaLangClass::from_rust_primitive(jvm, "long").await?,
        )
        .await
    }
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i64) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Number", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "value", "J", value).await
    }
    async fn init_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<String>) -> Result<()> {
        let value: i64 = jvm
            .invoke_static("java/lang/Long", "parseLong", "(Ljava/lang/String;)J", (value,))
            .await?;
        let mut this = this;
        let _: () = jvm.invoke_special(&this, "java/lang/Number", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "value", "J", value).await
    }
    fn parse_raw(value: &str, radix: u32) -> Option<i64> {
        if !(2..=36).contains(&radix) || value.is_empty() {
            return None;
        }
        let negative = value.starts_with('-');
        let body = value.strip_prefix('-').or_else(|| value.strip_prefix('+')).unwrap_or(value);
        if body.is_empty() {
            return None;
        }
        let limit = if negative { i128::from(i64::MIN) } else { -i128::from(i64::MAX) };
        let mut result = 0i128;
        for value in body.chars() {
            let value = JavaChar::try_from(u32::from(value)).ok()?;
            let digit = i128::from(Character::digit_value(value, radix as i32));
            if digit < 0 {
                return None;
            }
            if result < (limit + digit) / i128::from(radix) {
                return None;
            }
            result = result * i128::from(radix) - digit;
        }
        if !negative && result == i128::from(i64::MIN) {
            return None;
        }
        Some(if negative { result as i64 } else { -result as i64 })
    }
    async fn parse_long(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>) -> Result<i64> {
        if value.is_null() {
            return Err(jvm.exception("java/lang/NumberFormatException", "null").await);
        }
        let value = JavaLangString::to_rust_string(jvm, &value).await?;
        match Self::parse_raw(&value, 10) {
            Some(value) => Ok(value),
            None => Err(jvm
                .exception("java/lang/NumberFormatException", &format!("For input string: \"{value}\""))
                .await),
        }
    }
    async fn parse_long_radix(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>, radix: i32) -> Result<i64> {
        if value.is_null() {
            return Err(jvm.exception("java/lang/NumberFormatException", "null").await);
        }
        let value = JavaLangString::to_rust_string(jvm, &value).await?;
        match Self::parse_raw(&value, radix as u32) {
            Some(value) => Ok(value),
            None => Err(jvm
                .exception("java/lang/NumberFormatException", &format!("For input string: \"{value}\""))
                .await),
        }
    }
    async fn value_of_string(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Self>> {
        let value: i64 = jvm
            .invoke_static("java/lang/Long", "parseLong", "(Ljava/lang/String;)J", (value,))
            .await?;
        Ok(jvm.new_class("java/lang/Long", "(J)V", (value,)).await?.into())
    }
    async fn value_of_string_radix(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>, radix: i32) -> Result<ClassInstanceRef<Self>> {
        let value: i64 = jvm
            .invoke_static("java/lang/Long", "parseLong", "(Ljava/lang/String;I)J", (value, radix))
            .await?;
        Ok(jvm.new_class("java/lang/Long", "(J)V", (value,)).await?.into())
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
        let value = match Self::parse_raw(&signed, radix) {
            Some(value) => value,
            None => return Err(jvm.exception("java/lang/NumberFormatException", "invalid long").await),
        };
        Ok(jvm.new_class("java/lang/Long", "(J)V", (value,)).await?.into())
    }
    async fn get_long(jvm: &Jvm, _: &mut RuntimeContext, key: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Self>> {
        let default: ClassInstanceRef<Self> = None.into();
        jvm.invoke_static(
            "java/lang/Long",
            "getLong",
            "(Ljava/lang/String;Ljava/lang/Long;)Ljava/lang/Long;",
            (key, default),
        )
        .await
    }
    async fn get_long_value_default(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        key: ClassInstanceRef<String>,
        default: i64,
    ) -> Result<ClassInstanceRef<Self>> {
        let default = jvm.new_class("java/lang/Long", "(J)V", (default,)).await?;
        jvm.invoke_static(
            "java/lang/Long",
            "getLong",
            "(Ljava/lang/String;Ljava/lang/Long;)Ljava/lang/Long;",
            (key, default),
        )
        .await
    }
    async fn get_long_object_default(
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
            .invoke_static("java/lang/Long", "decode", "(Ljava/lang/String;)Ljava/lang/Long;", (value,))
            .await
        {
            Ok(value) => Ok(value),
            Err(JavaError::JavaException(exception)) if jvm.is_instance(&*exception, "java/lang/NumberFormatException") => Ok(default),
            Err(error) => Err(error),
        }
    }
    async fn byte_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i8> {
        Ok(jvm.get_field::<i64>(&this, "value", "J").await? as i8)
    }
    async fn short_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i16> {
        Ok(jvm.get_field::<i64>(&this, "value", "J").await? as i16)
    }
    async fn int_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(jvm.get_field::<i64>(&this, "value", "J").await? as i32)
    }
    async fn long_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        jvm.get_field(&this, "value", "J").await
    }
    async fn float_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        Ok(jvm.get_field::<i64>(&this, "value", "J").await? as f32)
    }
    async fn double_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f64> {
        Ok(jvm.get_field::<i64>(&this, "value", "J").await? as f64)
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
            result.push(char::from_digit((magnitude % u64::from(radix)) as u32, radix).unwrap_or('0'));
            magnitude /= u64::from(radix);
        }
        if negative {
            result.push('-');
        }
        result.chars().rev().collect()
    }
    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let value: i64 = jvm.invoke_virtual(&this, "longValue", "()J", ()).await?;
        Ok(JavaLangString::from_rust_string(jvm, &Self::format_value(value, 10)).await?.into())
    }
    async fn to_string_static(jvm: &Jvm, _: &mut RuntimeContext, value: i64) -> Result<ClassInstanceRef<String>> {
        Ok(JavaLangString::from_rust_string(jvm, &Self::format_value(value, 10)).await?.into())
    }
    async fn to_string_radix(jvm: &Jvm, _: &mut RuntimeContext, value: i64, radix: i32) -> Result<ClassInstanceRef<String>> {
        Ok(JavaLangString::from_rust_string(jvm, &Self::format_value(value, radix as u32))
            .await?
            .into())
    }
    async fn to_binary_string(jvm: &Jvm, _: &mut RuntimeContext, value: i64) -> Result<ClassInstanceRef<String>> {
        let text = if value < 0 {
            format!("{:064b}", value as u64)
        } else {
            Self::format_value(value, 2)
        };
        Ok(JavaLangString::from_rust_string(jvm, &text).await?.into())
    }
    async fn to_octal_string(jvm: &Jvm, _: &mut RuntimeContext, value: i64) -> Result<ClassInstanceRef<String>> {
        let text = if value < 0 {
            format!("{:022o}", value as u64)
        } else {
            Self::format_value(value, 8)
        };
        Ok(JavaLangString::from_rust_string(jvm, &text).await?.into())
    }
    async fn to_hex_string(jvm: &Jvm, _: &mut RuntimeContext, value: i64) -> Result<ClassInstanceRef<String>> {
        let text = if value < 0 {
            format!("{:016x}", value as u64)
        } else {
            Self::format_value(value, 16)
        };
        Ok(JavaLangString::from_rust_string(jvm, &text).await?.into())
    }
    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let value: i64 = jvm.invoke_virtual(&this, "longValue", "()J", ()).await?;
        Ok((value ^ (value >> 32)) as i32)
    }
    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if other.is_null() || !jvm.is_instance(&**other, "java/lang/Long") {
            return Ok(false);
        }
        Ok(
            jvm.invoke_virtual::<_, i64>(&this, "longValue", "()J", ()).await?
                == jvm.invoke_virtual::<_, i64>(&other, "longValue", "()J", ()).await?,
        )
    }
    async fn compare_to(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<i32> {
        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "other").await);
        }
        let left: i64 = jvm.invoke_virtual(&this, "longValue", "()J", ()).await?;
        let right: i64 = jvm.invoke_virtual(&other, "longValue", "()J", ()).await?;
        Ok(left.cmp(&right) as i32)
    }
    async fn compare_to_object(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<i32> {
        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "other").await);
        }
        if !jvm.is_instance(&**other, "java/lang/Long") {
            return Err(jvm.exception("java/lang/ClassCastException", "not Long").await);
        }
        let other = ClassInstanceRef::<Self>::from(other.instance);
        let left: i64 = jvm.invoke_virtual(&this, "longValue", "()J", ()).await?;
        let right: i64 = jvm.invoke_virtual(&other, "longValue", "()J", ()).await?;
        Ok(left.cmp(&right) as i32)
    }
}
