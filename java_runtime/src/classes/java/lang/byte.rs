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

// public final class java.lang.Byte
pub struct Byte;

impl Byte {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Byte",
            parent_class: Some("java/lang/Number"),
            interfaces: vec!["java/lang/Comparable"],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "(B)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_string, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "parseByte",
                    "(Ljava/lang/String;)B",
                    Self::parse_byte,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "parseByte",
                    "(Ljava/lang/String;I)B",
                    Self::parse_byte_radix,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "valueOf",
                    "(Ljava/lang/String;)Ljava/lang/Byte;",
                    Self::value_of_string,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "valueOf",
                    "(Ljava/lang/String;I)Ljava/lang/Byte;",
                    Self::value_of_string_radix,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "decode",
                    "(Ljava/lang/String;)Ljava/lang/Byte;",
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
                    "(B)Ljava/lang/String;",
                    Self::to_string_static,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("compareTo", "(Ljava/lang/Byte;)I", Self::compare_to, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("compareTo", "(Ljava/lang/Object;)I", Self::compare_to_object, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new(
                    "MIN_VALUE",
                    "B",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "MAX_VALUE",
                    "B",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "TYPE",
                    "Ljava/lang/Class;",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new("value", "B", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL),
            ],
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::FINAL,
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        jvm.put_static_field("java/lang/Byte", "MIN_VALUE", "B", i8::MIN).await?;
        jvm.put_static_field("java/lang/Byte", "MAX_VALUE", "B", i8::MAX).await?;
        jvm.put_static_field(
            "java/lang/Byte",
            "TYPE",
            "Ljava/lang/Class;",
            JavaLangClass::from_rust_primitive(jvm, "byte").await?,
        )
        .await
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i8) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Number", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "value", "B", value).await
    }

    async fn init_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<String>) -> Result<()> {
        let value: i8 = jvm
            .invoke_static("java/lang/Byte", "parseByte", "(Ljava/lang/String;)B", (value,))
            .await?;
        let mut this = this;
        let _: () = jvm.invoke_special(&this, "java/lang/Number", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "value", "B", value).await
    }

    async fn parse_byte(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>) -> Result<i8> {
        let parsed: i32 = jvm
            .invoke_static("java/lang/Integer", "parseInt", "(Ljava/lang/String;)I", (value,))
            .await?;
        if !(i32::from(i8::MIN)..=i32::from(i8::MAX)).contains(&parsed) {
            return Err(jvm.exception("java/lang/NumberFormatException", "Value out of range").await);
        }
        Ok(parsed as i8)
    }

    async fn parse_byte_radix(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>, radix: i32) -> Result<i8> {
        let parsed: i32 = jvm
            .invoke_static("java/lang/Integer", "parseInt", "(Ljava/lang/String;I)I", (value, radix))
            .await?;
        if !(i32::from(i8::MIN)..=i32::from(i8::MAX)).contains(&parsed) {
            return Err(jvm.exception("java/lang/NumberFormatException", "Value out of range").await);
        }
        Ok(parsed as i8)
    }

    async fn value_of_string(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Self>> {
        let value: i8 = jvm
            .invoke_static("java/lang/Byte", "parseByte", "(Ljava/lang/String;)B", (value,))
            .await?;
        Ok(jvm.new_class("java/lang/Byte", "(B)V", (value,)).await?.into())
    }

    async fn value_of_string_radix(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>, radix: i32) -> Result<ClassInstanceRef<Self>> {
        let value: i8 = jvm
            .invoke_static("java/lang/Byte", "parseByte", "(Ljava/lang/String;I)B", (value, radix))
            .await?;
        Ok(jvm.new_class("java/lang/Byte", "(B)V", (value,)).await?.into())
    }

    async fn decode(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Self>> {
        let integer = jvm
            .invoke_static("java/lang/Integer", "decode", "(Ljava/lang/String;)Ljava/lang/Integer;", (value,))
            .await?;
        let value: i32 = jvm.invoke_virtual(&integer, "intValue", "()I", ()).await?;
        if !(i32::from(i8::MIN)..=i32::from(i8::MAX)).contains(&value) {
            return Err(jvm.exception("java/lang/NumberFormatException", "Value out of range").await);
        }
        Ok(jvm.new_class("java/lang/Byte", "(B)V", (value as i8,)).await?.into())
    }

    async fn byte_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i8> {
        jvm.get_field(&this, "value", "B").await
    }

    async fn short_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i16> {
        let value: i8 = jvm.invoke_virtual(&this, "byteValue", "()B", ()).await?;
        Ok(value as i16)
    }

    async fn int_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let value: i8 = jvm.invoke_virtual(&this, "byteValue", "()B", ()).await?;
        Ok(value as i32)
    }

    async fn long_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        let value: i8 = jvm.invoke_virtual(&this, "byteValue", "()B", ()).await?;
        Ok(value as i64)
    }

    async fn float_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        let value: i8 = jvm.invoke_virtual(&this, "byteValue", "()B", ()).await?;
        Ok(value as f32)
    }

    async fn double_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f64> {
        let value: i8 = jvm.invoke_virtual(&this, "byteValue", "()B", ()).await?;
        Ok(value as f64)
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let value: i8 = jvm.invoke_virtual(&this, "byteValue", "()B", ()).await?;
        Ok(JavaLangString::from_rust_string(jvm, &format!("{value}")).await?.into())
    }

    async fn to_string_static(jvm: &Jvm, _: &mut RuntimeContext, value: i8) -> Result<ClassInstanceRef<String>> {
        Ok(JavaLangString::from_rust_string(jvm, &format!("{value}")).await?.into())
    }

    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let value: i8 = jvm.invoke_virtual(&this, "byteValue", "()B", ()).await?;
        Ok(value as i32)
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if other.is_null() || !jvm.is_instance(&**other, "java/lang/Byte") {
            return Ok(false);
        }
        let left: i8 = jvm.invoke_virtual(&this, "byteValue", "()B", ()).await?;
        let right: i8 = jvm.invoke_virtual(&other, "byteValue", "()B", ()).await?;
        Ok(left == right)
    }

    async fn compare_to(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<i32> {
        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "other").await);
        }
        let left: i8 = jvm.invoke_virtual(&this, "byteValue", "()B", ()).await?;
        let right: i8 = jvm.invoke_virtual(&other, "byteValue", "()B", ()).await?;
        Ok(left.cmp(&right) as i32)
    }

    async fn compare_to_object(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<i32> {
        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "other").await);
        }
        if !jvm.is_instance(&**other, "java/lang/Byte") {
            return Err(jvm.exception("java/lang/ClassCastException", "not Byte").await);
        }
        let other = ClassInstanceRef::<Self>::from(other.instance);
        let left: i8 = jvm.invoke_virtual(&this, "byteValue", "()B", ()).await?;
        let right: i8 = jvm.invoke_virtual(&other, "byteValue", "()B", ()).await?;
        Ok(left.cmp(&right) as i32)
    }
}
