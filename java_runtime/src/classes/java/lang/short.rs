use alloc::{format, vec};

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

// public final class java.lang.Short
pub struct Short;

impl Short {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Short",
            parent_class: Some("java/lang/Number"),
            interfaces: vec!["java/lang/Comparable"],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "(S)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_string, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "parseShort",
                    "(Ljava/lang/String;)S",
                    Self::parse_short,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "parseShort",
                    "(Ljava/lang/String;I)S",
                    Self::parse_short_radix,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "valueOf",
                    "(Ljava/lang/String;)Ljava/lang/Short;",
                    Self::value_of_string,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "valueOf",
                    "(Ljava/lang/String;I)Ljava/lang/Short;",
                    Self::value_of_string_radix,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "decode",
                    "(Ljava/lang/String;)Ljava/lang/Short;",
                    Self::decode,
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
                    "(S)Ljava/lang/String;",
                    Self::to_string_static,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("compareTo", "(Ljava/lang/Short;)I", Self::compare_to, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("compareTo", "(Ljava/lang/Object;)I", Self::compare_to_object, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new(
                    "MIN_VALUE",
                    "S",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "MAX_VALUE",
                    "S",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "TYPE",
                    "Ljava/lang/Class;",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new("value", "S", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL),
            ],
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::FINAL,
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        jvm.put_static_field("java/lang/Short", "MIN_VALUE", "S", i16::MIN).await?;
        jvm.put_static_field("java/lang/Short", "MAX_VALUE", "S", i16::MAX).await?;
        jvm.put_static_field(
            "java/lang/Short",
            "TYPE",
            "Ljava/lang/Class;",
            JavaLangClass::from_rust_primitive(jvm, "short").await?,
        )
        .await
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i16) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Number", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "value", "S", value).await
    }

    async fn init_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<String>) -> Result<()> {
        let value: i16 = jvm
            .invoke_static("java/lang/Short", "parseShort", "(Ljava/lang/String;)S", (value,))
            .await?;
        let mut this = this;
        let _: () = jvm.invoke_special(&this, "java/lang/Number", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "value", "S", value).await
    }

    async fn parse_short(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>) -> Result<i16> {
        let parsed: i32 = jvm
            .invoke_static("java/lang/Integer", "parseInt", "(Ljava/lang/String;)I", (value,))
            .await?;
        if !(i32::from(i16::MIN)..=i32::from(i16::MAX)).contains(&parsed) {
            return Err(jvm.exception("java/lang/NumberFormatException", "Value out of range").await);
        }
        Ok(parsed as i16)
    }

    async fn parse_short_radix(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>, radix: i32) -> Result<i16> {
        let parsed: i32 = jvm
            .invoke_static("java/lang/Integer", "parseInt", "(Ljava/lang/String;I)I", (value, radix))
            .await?;
        if !(i32::from(i16::MIN)..=i32::from(i16::MAX)).contains(&parsed) {
            return Err(jvm.exception("java/lang/NumberFormatException", "Value out of range").await);
        }
        Ok(parsed as i16)
    }

    async fn value_of_string(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Self>> {
        let value: i16 = jvm
            .invoke_static("java/lang/Short", "parseShort", "(Ljava/lang/String;)S", (value,))
            .await?;
        Ok(jvm.new_class("java/lang/Short", "(S)V", (value,)).await?.into())
    }

    async fn value_of_string_radix(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>, radix: i32) -> Result<ClassInstanceRef<Self>> {
        let value: i16 = jvm
            .invoke_static("java/lang/Short", "parseShort", "(Ljava/lang/String;I)S", (value, radix))
            .await?;
        Ok(jvm.new_class("java/lang/Short", "(S)V", (value,)).await?.into())
    }

    async fn decode(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Self>> {
        let integer = jvm
            .invoke_static("java/lang/Integer", "decode", "(Ljava/lang/String;)Ljava/lang/Integer;", (value,))
            .await?;
        let value: i32 = jvm.invoke_virtual(&integer, "intValue", "()I", ()).await?;
        if !(i32::from(i16::MIN)..=i32::from(i16::MAX)).contains(&value) {
            return Err(jvm.exception("java/lang/NumberFormatException", "Value out of range").await);
        }
        Ok(jvm.new_class("java/lang/Short", "(S)V", (value as i16,)).await?.into())
    }

    async fn byte_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i8> {
        Ok((jvm.get_field::<i16>(&this, "value", "S").await?) as i8)
    }
    async fn short_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i16> {
        jvm.get_field(&this, "value", "S").await
    }
    async fn int_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(jvm.get_field::<i16>(&this, "value", "S").await? as i32)
    }
    async fn long_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        Ok(jvm.get_field::<i16>(&this, "value", "S").await? as i64)
    }
    async fn float_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        Ok(jvm.get_field::<i16>(&this, "value", "S").await? as f32)
    }
    async fn double_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f64> {
        Ok(jvm.get_field::<i16>(&this, "value", "S").await? as f64)
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let value: i16 = jvm.invoke_virtual(&this, "shortValue", "()S", ()).await?;
        Ok(JavaLangString::from_rust_string(jvm, &format!("{value}")).await?.into())
    }
    async fn to_string_static(jvm: &Jvm, _: &mut RuntimeContext, value: i16) -> Result<ClassInstanceRef<String>> {
        Ok(JavaLangString::from_rust_string(jvm, &format!("{value}")).await?.into())
    }
    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(jvm.get_field::<i16>(&this, "value", "S").await? as i32)
    }
    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if other.is_null() || !jvm.is_instance(&**other, "java/lang/Short") {
            return Ok(false);
        }
        Ok(jvm.invoke_virtual::<_, i16>(&this, "shortValue", "()S", ()).await?
            == jvm.invoke_virtual::<_, i16>(&other, "shortValue", "()S", ()).await?)
    }
    async fn compare_to(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<i32> {
        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "other").await);
        }
        let left: i16 = jvm.invoke_virtual(&this, "shortValue", "()S", ()).await?;
        let right: i16 = jvm.invoke_virtual(&other, "shortValue", "()S", ()).await?;
        Ok(left.cmp(&right) as i32)
    }
    async fn compare_to_object(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<i32> {
        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "other").await);
        }
        if !jvm.is_instance(&**other, "java/lang/Short") {
            return Err(jvm.exception("java/lang/ClassCastException", "not Short").await);
        }
        let other = ClassInstanceRef::<Self>::from(other.instance);
        let left: i16 = jvm.invoke_virtual(&this, "shortValue", "()S", ()).await?;
        let right: i16 = jvm.invoke_virtual(&other, "shortValue", "()S", ()).await?;
        Ok(left.cmp(&right) as i32)
    }
}
