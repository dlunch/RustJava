use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::lang::{Object, String},
};

// public class java.util.logging.Level
pub struct Level;

impl Level {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/logging/Level",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;I)V", Self::init, MethodAccessFlags::PROTECTED),
                JavaMethodProto::new(
                    "parse",
                    "(Ljava/lang/String;)Ljava/util/logging/Level;",
                    Self::parse,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("getName", "()Ljava/lang/String;", Self::get_name, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("intValue", "()I", Self::int_value, MethodAccessFlags::PUBLIC | MethodAccessFlags::FINAL),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "toString",
                    "()Ljava/lang/String;",
                    Self::to_string,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::FINAL,
                ),
            ],
            fields: vec![
                JavaFieldProto::new(
                    "OFF",
                    "Ljava/util/logging/Level;",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "SEVERE",
                    "Ljava/util/logging/Level;",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "WARNING",
                    "Ljava/util/logging/Level;",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "INFO",
                    "Ljava/util/logging/Level;",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "CONFIG",
                    "Ljava/util/logging/Level;",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "FINE",
                    "Ljava/util/logging/Level;",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "FINER",
                    "Ljava/util/logging/Level;",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "FINEST",
                    "Ljava/util/logging/Level;",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "ALL",
                    "Ljava/util/logging/Level;",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new("name", "Ljava/lang/String;", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL),
                JavaFieldProto::new("value", "I", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        tracing::debug!("java.util.logging.Level::<clinit>()");

        let off: ClassInstanceRef<Self> = jvm
            .new_class(
                "java/util/logging/Level",
                "(Ljava/lang/String;I)V",
                (JavaLangString::from_rust_string(jvm, "OFF").await?, i32::MAX),
            )
            .await?
            .into();
        jvm.put_static_field("java/util/logging/Level", "OFF", "Ljava/util/logging/Level;", off)
            .await?;

        let severe: ClassInstanceRef<Self> = jvm
            .new_class(
                "java/util/logging/Level",
                "(Ljava/lang/String;I)V",
                (JavaLangString::from_rust_string(jvm, "SEVERE").await?, 1000i32),
            )
            .await?
            .into();
        jvm.put_static_field("java/util/logging/Level", "SEVERE", "Ljava/util/logging/Level;", severe)
            .await?;

        let warning: ClassInstanceRef<Self> = jvm
            .new_class(
                "java/util/logging/Level",
                "(Ljava/lang/String;I)V",
                (JavaLangString::from_rust_string(jvm, "WARNING").await?, 900i32),
            )
            .await?
            .into();
        jvm.put_static_field("java/util/logging/Level", "WARNING", "Ljava/util/logging/Level;", warning)
            .await?;

        let info: ClassInstanceRef<Self> = jvm
            .new_class(
                "java/util/logging/Level",
                "(Ljava/lang/String;I)V",
                (JavaLangString::from_rust_string(jvm, "INFO").await?, 800i32),
            )
            .await?
            .into();
        jvm.put_static_field("java/util/logging/Level", "INFO", "Ljava/util/logging/Level;", info)
            .await?;

        let config: ClassInstanceRef<Self> = jvm
            .new_class(
                "java/util/logging/Level",
                "(Ljava/lang/String;I)V",
                (JavaLangString::from_rust_string(jvm, "CONFIG").await?, 700i32),
            )
            .await?
            .into();
        jvm.put_static_field("java/util/logging/Level", "CONFIG", "Ljava/util/logging/Level;", config)
            .await?;

        let fine: ClassInstanceRef<Self> = jvm
            .new_class(
                "java/util/logging/Level",
                "(Ljava/lang/String;I)V",
                (JavaLangString::from_rust_string(jvm, "FINE").await?, 500i32),
            )
            .await?
            .into();
        jvm.put_static_field("java/util/logging/Level", "FINE", "Ljava/util/logging/Level;", fine)
            .await?;

        let finer: ClassInstanceRef<Self> = jvm
            .new_class(
                "java/util/logging/Level",
                "(Ljava/lang/String;I)V",
                (JavaLangString::from_rust_string(jvm, "FINER").await?, 400i32),
            )
            .await?
            .into();
        jvm.put_static_field("java/util/logging/Level", "FINER", "Ljava/util/logging/Level;", finer)
            .await?;

        let finest: ClassInstanceRef<Self> = jvm
            .new_class(
                "java/util/logging/Level",
                "(Ljava/lang/String;I)V",
                (JavaLangString::from_rust_string(jvm, "FINEST").await?, 300i32),
            )
            .await?
            .into();
        jvm.put_static_field("java/util/logging/Level", "FINEST", "Ljava/util/logging/Level;", finest)
            .await?;

        let all: ClassInstanceRef<Self> = jvm
            .new_class(
                "java/util/logging/Level",
                "(Ljava/lang/String;I)V",
                (JavaLangString::from_rust_string(jvm, "ALL").await?, i32::MIN),
            )
            .await?
            .into();
        jvm.put_static_field("java/util/logging/Level", "ALL", "Ljava/util/logging/Level;", all)
            .await
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, name: ClassInstanceRef<String>, value: i32) -> Result<()> {
        tracing::debug!("java.util.logging.Level::<init>({this:?}, {name:?}, {value})");

        if name.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "name").await);
        }

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "name", "Ljava/lang/String;", name).await?;
        jvm.put_field(&mut this, "value", "I", value).await
    }

    async fn parse(jvm: &Jvm, _: &mut RuntimeContext, name: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.util.logging.Level::parse({name:?})");

        if name.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "name").await);
        }

        let parsed_name = JavaLangString::to_rust_string(jvm, &name).await?;
        let field_name = match parsed_name.as_str() {
            "OFF" => Some("OFF"),
            "SEVERE" => Some("SEVERE"),
            "WARNING" => Some("WARNING"),
            "INFO" => Some("INFO"),
            "CONFIG" => Some("CONFIG"),
            "FINE" => Some("FINE"),
            "FINER" => Some("FINER"),
            "FINEST" => Some("FINEST"),
            "ALL" => Some("ALL"),
            _ => None,
        };
        if let Some(field_name) = field_name {
            return jvm
                .get_static_field("java/util/logging/Level", field_name, "Ljava/util/logging/Level;")
                .await;
        }

        let Ok(value) = parsed_name.parse::<i32>() else {
            return Err(jvm.exception("java/lang/IllegalArgumentException", &parsed_name).await);
        };
        let field_name = match value {
            i32::MAX => Some("OFF"),
            1000 => Some("SEVERE"),
            900 => Some("WARNING"),
            800 => Some("INFO"),
            700 => Some("CONFIG"),
            500 => Some("FINE"),
            400 => Some("FINER"),
            300 => Some("FINEST"),
            i32::MIN => Some("ALL"),
            _ => None,
        };
        if let Some(field_name) = field_name {
            return jvm
                .get_static_field("java/util/logging/Level", field_name, "Ljava/util/logging/Level;")
                .await;
        }

        Ok(jvm
            .new_class("java/util/logging/Level", "(Ljava/lang/String;I)V", (name, value))
            .await?
            .into())
    }

    async fn get_name(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.logging.Level::getName({this:?})");

        jvm.get_field(&this, "name", "Ljava/lang/String;").await
    }

    async fn int_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.logging.Level::intValue({this:?})");

        jvm.get_field(&this, "value", "I").await
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.logging.Level::equals({this:?}, {other:?})");

        if other.is_null() || !jvm.is_instance(&**other, "java/util/logging/Level") {
            return Ok(false);
        }

        let other: ClassInstanceRef<Self> = ClassInstanceRef::new(other.instance);
        let value: i32 = jvm.get_field(&this, "value", "I").await?;
        let other_value: i32 = jvm.get_field(&other, "value", "I").await?;
        Ok(value == other_value)
    }

    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.logging.Level::hashCode({this:?})");

        jvm.get_field(&this, "value", "I").await
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.logging.Level::toString({this:?})");

        jvm.get_field(&this, "name", "Ljava/lang/String;").await
    }
}
