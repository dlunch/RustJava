use alloc::{boxed::Box, format};

use jvm::{
    ClassInstance, ClassInstanceRef, Jvm, Result,
    runtime::{JavaLangClass, JavaLangString},
};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::lang::{Class, Object, String},
};

// public final class java.lang.Short
#[java_class(name = "java/lang/Short", extends = "java/lang/Number", implements("java/lang/Comparable"), public, final)]
pub struct Short {
    #[field(public, static, final)]
    MIN_VALUE: i16,
    #[field(public, static, final)]
    MAX_VALUE: i16,
    #[field(public, static, final)]
    TYPE: ClassInstanceRef<Class>,
    #[field(private, final)]
    value: i16,
}

#[java_methods]
impl Short {
    #[method(static_initializer)]
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

    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i16) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Number", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/lang/Short", "value", "S", value).await
    }

    #[method(constructor, public)]
    async fn init_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<String>) -> Result<()> {
        let value: i16 = jvm
            .invoke_static("java/lang/Short", "parseShort", "(Ljava/lang/String;)S", (value,))
            .await?;
        let mut this = this;
        let _: () = jvm.invoke_special(&this, "java/lang/Number", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/lang/Short", "value", "S", value).await
    }

    #[method(public, static)]
    async fn parse_short(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>) -> Result<i16> {
        let parsed: i32 = jvm
            .invoke_static("java/lang/Integer", "parseInt", "(Ljava/lang/String;)I", (value,))
            .await?;
        if !(i32::from(i16::MIN)..=i32::from(i16::MAX)).contains(&parsed) {
            return Err(jvm.exception("java/lang/NumberFormatException", "Value out of range").await);
        }
        Ok(parsed as i16)
    }

    #[method(name = "parseShort", public, static)]
    async fn parse_short_radix(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>, radix: i32) -> Result<i16> {
        let parsed: i32 = jvm
            .invoke_static("java/lang/Integer", "parseInt", "(Ljava/lang/String;I)I", (value, radix))
            .await?;
        if !(i32::from(i16::MIN)..=i32::from(i16::MAX)).contains(&parsed) {
            return Err(jvm.exception("java/lang/NumberFormatException", "Value out of range").await);
        }
        Ok(parsed as i16)
    }

    #[method(name = "valueOf", public, static)]
    async fn value_of_string(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Self>> {
        let value: i16 = jvm
            .invoke_static("java/lang/Short", "parseShort", "(Ljava/lang/String;)S", (value,))
            .await?;
        Ok(jvm.new_class("java/lang/Short", "(S)V", (value,)).await?.into())
    }

    #[method(public, static)]
    async fn value_of(jvm: &Jvm, _: &mut RuntimeContext, value: i16) -> Result<ClassInstanceRef<Self>> {
        Ok(jvm.new_class("java/lang/Short", "(S)V", (value,)).await?.into())
    }

    #[method(name = "valueOf", public, static)]
    async fn value_of_string_radix(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>, radix: i32) -> Result<ClassInstanceRef<Self>> {
        let value: i16 = jvm
            .invoke_static("java/lang/Short", "parseShort", "(Ljava/lang/String;I)S", (value, radix))
            .await?;
        Ok(jvm.new_class("java/lang/Short", "(S)V", (value,)).await?.into())
    }

    #[method(public, static)]
    async fn decode(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Self>> {
        let integer: Box<dyn ClassInstance> = jvm
            .invoke_static("java/lang/Integer", "decode", "(Ljava/lang/String;)Ljava/lang/Integer;", (value,))
            .await?;
        let value: i32 = jvm.invoke_virtual(&integer, "java/lang/Integer", "intValue", "()I", ()).await?;
        if !(i32::from(i16::MIN)..=i32::from(i16::MAX)).contains(&value) {
            return Err(jvm.exception("java/lang/NumberFormatException", "Value out of range").await);
        }
        Ok(jvm.new_class("java/lang/Short", "(S)V", (value as i16,)).await?.into())
    }

    #[method(public)]
    async fn byte_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i8> {
        Ok((jvm.get_field::<i16>(&this, "java/lang/Short", "value", "S").await?) as i8)
    }
    #[method(public)]
    async fn short_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i16> {
        jvm.get_field(&this, "java/lang/Short", "value", "S").await
    }
    #[method(public)]
    async fn int_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(jvm.get_field::<i16>(&this, "java/lang/Short", "value", "S").await? as i32)
    }
    #[method(public)]
    async fn long_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        Ok(jvm.get_field::<i16>(&this, "java/lang/Short", "value", "S").await? as i64)
    }
    #[method(public)]
    async fn float_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        Ok(jvm.get_field::<i16>(&this, "java/lang/Short", "value", "S").await? as f32)
    }
    #[method(public)]
    async fn double_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f64> {
        Ok(jvm.get_field::<i16>(&this, "java/lang/Short", "value", "S").await? as f64)
    }

    #[method(public)]
    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let value: i16 = jvm.invoke_virtual(&this, "java/lang/Short", "shortValue", "()S", ()).await?;
        Ok(JavaLangString::from_rust_string(jvm, &format!("{value}")).await?.into())
    }
    #[method(name = "toString", public, static)]
    async fn to_string_static(jvm: &Jvm, _: &mut RuntimeContext, value: i16) -> Result<ClassInstanceRef<String>> {
        Ok(JavaLangString::from_rust_string(jvm, &format!("{value}")).await?.into())
    }
    #[method(public)]
    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(jvm.get_field::<i16>(&this, "java/lang/Short", "value", "S").await? as i32)
    }
    #[method(public)]
    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if other.is_null() || !jvm.is_instance(&**other, "java/lang/Short") {
            return Ok(false);
        }
        Ok(jvm.invoke_virtual::<_, i16>(&this, "java/lang/Short", "shortValue", "()S", ()).await?
            == jvm.invoke_virtual::<_, i16>(&other, "java/lang/Short", "shortValue", "()S", ()).await?)
    }
    #[method(public)]
    async fn compare_to(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<i32> {
        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "other").await);
        }
        let left: i16 = jvm.invoke_virtual(&this, "java/lang/Short", "shortValue", "()S", ()).await?;
        let right: i16 = jvm.invoke_virtual(&other, "java/lang/Short", "shortValue", "()S", ()).await?;
        Ok(left.cmp(&right) as i32)
    }
    #[method(name = "compareTo", public)]
    async fn compare_to_object(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<i32> {
        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "other").await);
        }
        if !jvm.is_instance(&**other, "java/lang/Short") {
            return Err(jvm.exception("java/lang/ClassCastException", "not Short").await);
        }
        let other = ClassInstanceRef::<Self>::from(other.instance);
        let left: i16 = jvm.invoke_virtual(&this, "java/lang/Short", "shortValue", "()S", ()).await?;
        let right: i16 = jvm.invoke_virtual(&other, "java/lang/Short", "shortValue", "()S", ()).await?;
        Ok(left.cmp(&right) as i32)
    }
}
