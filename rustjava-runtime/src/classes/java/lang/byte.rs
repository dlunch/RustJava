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

// public final class java.lang.Byte
#[java_class(name = "java/lang/Byte", extends = "java/lang/Number", implements("java/lang/Comparable"), public, final)]
pub struct Byte {
    #[field(public, static, final)]
    MIN_VALUE: i8,
    #[field(public, static, final)]
    MAX_VALUE: i8,
    #[field(public, static, final)]
    TYPE: ClassInstanceRef<Class>,
    #[field(private, final)]
    value: i8,
}

#[java_methods]
impl Byte {
    #[method(static_initializer)]
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

    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i8) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Number", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/lang/Byte", "value", "B", value).await
    }

    #[method(constructor, public)]
    async fn init_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<String>) -> Result<()> {
        let value: i8 = jvm
            .invoke_static("java/lang/Byte", "parseByte", "(Ljava/lang/String;)B", (value,))
            .await?;
        let mut this = this;
        let _: () = jvm.invoke_special(&this, "java/lang/Number", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/lang/Byte", "value", "B", value).await
    }

    #[method(public, static)]
    async fn parse_byte(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>) -> Result<i8> {
        let parsed: i32 = jvm
            .invoke_static("java/lang/Integer", "parseInt", "(Ljava/lang/String;)I", (value,))
            .await?;
        if !(i32::from(i8::MIN)..=i32::from(i8::MAX)).contains(&parsed) {
            return Err(jvm.exception("java/lang/NumberFormatException", "Value out of range").await);
        }
        Ok(parsed as i8)
    }

    #[method(name = "parseByte", public, static)]
    async fn parse_byte_radix(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>, radix: i32) -> Result<i8> {
        let parsed: i32 = jvm
            .invoke_static("java/lang/Integer", "parseInt", "(Ljava/lang/String;I)I", (value, radix))
            .await?;
        if !(i32::from(i8::MIN)..=i32::from(i8::MAX)).contains(&parsed) {
            return Err(jvm.exception("java/lang/NumberFormatException", "Value out of range").await);
        }
        Ok(parsed as i8)
    }

    #[method(name = "valueOf", public, static)]
    async fn value_of_string(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Self>> {
        let value: i8 = jvm
            .invoke_static("java/lang/Byte", "parseByte", "(Ljava/lang/String;)B", (value,))
            .await?;
        Ok(jvm.new_class("java/lang/Byte", "(B)V", (value,)).await?.into())
    }

    #[method(public, static)]
    async fn value_of(jvm: &Jvm, _: &mut RuntimeContext, value: i8) -> Result<ClassInstanceRef<Self>> {
        Ok(jvm.new_class("java/lang/Byte", "(B)V", (value,)).await?.into())
    }

    #[method(name = "valueOf", public, static)]
    async fn value_of_string_radix(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>, radix: i32) -> Result<ClassInstanceRef<Self>> {
        let value: i8 = jvm
            .invoke_static("java/lang/Byte", "parseByte", "(Ljava/lang/String;I)B", (value, radix))
            .await?;
        Ok(jvm.new_class("java/lang/Byte", "(B)V", (value,)).await?.into())
    }

    #[method(public, static)]
    async fn decode(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Self>> {
        let integer: Box<dyn ClassInstance> = jvm
            .invoke_static("java/lang/Integer", "decode", "(Ljava/lang/String;)Ljava/lang/Integer;", (value,))
            .await?;
        let value: i32 = jvm.invoke_virtual(&integer, "java/lang/Integer", "intValue", "()I", ()).await?;
        if !(i32::from(i8::MIN)..=i32::from(i8::MAX)).contains(&value) {
            return Err(jvm.exception("java/lang/NumberFormatException", "Value out of range").await);
        }
        Ok(jvm.new_class("java/lang/Byte", "(B)V", (value as i8,)).await?.into())
    }

    #[method(public)]
    async fn byte_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i8> {
        jvm.get_field(&this, "java/lang/Byte", "value", "B").await
    }

    #[method(public)]
    async fn short_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i16> {
        let value: i8 = jvm.invoke_virtual(&this, "java/lang/Byte", "byteValue", "()B", ()).await?;
        Ok(value as i16)
    }

    #[method(public)]
    async fn int_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let value: i8 = jvm.invoke_virtual(&this, "java/lang/Byte", "byteValue", "()B", ()).await?;
        Ok(value as i32)
    }

    #[method(public)]
    async fn long_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        let value: i8 = jvm.invoke_virtual(&this, "java/lang/Byte", "byteValue", "()B", ()).await?;
        Ok(value as i64)
    }

    #[method(public)]
    async fn float_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32> {
        let value: i8 = jvm.invoke_virtual(&this, "java/lang/Byte", "byteValue", "()B", ()).await?;
        Ok(value as f32)
    }

    #[method(public)]
    async fn double_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f64> {
        let value: i8 = jvm.invoke_virtual(&this, "java/lang/Byte", "byteValue", "()B", ()).await?;
        Ok(value as f64)
    }

    #[method(public)]
    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let value: i8 = jvm.invoke_virtual(&this, "java/lang/Byte", "byteValue", "()B", ()).await?;
        Ok(JavaLangString::from_rust_string(jvm, &format!("{value}")).await?.into())
    }

    #[method(name = "toString", public, static)]
    async fn to_string_static(jvm: &Jvm, _: &mut RuntimeContext, value: i8) -> Result<ClassInstanceRef<String>> {
        Ok(JavaLangString::from_rust_string(jvm, &format!("{value}")).await?.into())
    }

    #[method(public)]
    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let value: i8 = jvm.invoke_virtual(&this, "java/lang/Byte", "byteValue", "()B", ()).await?;
        Ok(value as i32)
    }

    #[method(public)]
    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if other.is_null() || !jvm.is_instance(&**other, "java/lang/Byte") {
            return Ok(false);
        }
        let left: i8 = jvm.invoke_virtual(&this, "java/lang/Byte", "byteValue", "()B", ()).await?;
        let right: i8 = jvm.invoke_virtual(&other, "java/lang/Byte", "byteValue", "()B", ()).await?;
        Ok(left == right)
    }

    #[method(public)]
    async fn compare_to(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<i32> {
        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "other").await);
        }
        let left: i8 = jvm.invoke_virtual(&this, "java/lang/Byte", "byteValue", "()B", ()).await?;
        let right: i8 = jvm.invoke_virtual(&other, "java/lang/Byte", "byteValue", "()B", ()).await?;
        Ok(left.cmp(&right) as i32)
    }

    #[method(name = "compareTo", public)]
    async fn compare_to_object(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<i32> {
        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "other").await);
        }
        if !jvm.is_instance(&**other, "java/lang/Byte") {
            return Err(jvm.exception("java/lang/ClassCastException", "not Byte").await);
        }
        let other = ClassInstanceRef::<Self>::from(other.instance);
        let left: i8 = jvm.invoke_virtual(&this, "java/lang/Byte", "byteValue", "()B", ()).await?;
        let right: i8 = jvm.invoke_virtual(&other, "java/lang/Byte", "byteValue", "()B", ()).await?;
        Ok(left.cmp(&right) as i32)
    }
}
