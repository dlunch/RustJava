use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::lang::{Object, String},
};

// public class java.util.logging.Level
#[java_class(
    name = "java/util/logging/Level",
    extends = "java/lang/Object",
    implements("java/io/Serializable"),
    public
)]
pub struct Level {
    #[field(public, static, final)]
    OFF: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    SEVERE: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    WARNING: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    INFO: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    CONFIG: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    FINE: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    FINER: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    FINEST: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    ALL: ClassInstanceRef<Self>,
    #[field(private, final)]
    name: ClassInstanceRef<String>,
    #[field(private, final)]
    value: i32,
}

#[java_methods]
impl Level {
    #[method(static_initializer)]
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

    #[method(constructor, protected)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, name: ClassInstanceRef<String>, value: i32) -> Result<()> {
        tracing::debug!("java.util.logging.Level::<init>({this:?}, {name:?}, {value})");

        if name.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "name").await);
        }

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/util/logging/Level", "name", "Ljava/lang/String;", name)
            .await?;
        jvm.put_field(&mut this, "java/util/logging/Level", "value", "I", value).await
    }

    #[method(public, static)]
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

    #[method(public)]
    async fn get_name(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.logging.Level::getName({this:?})");

        jvm.get_field(&this, "java/util/logging/Level", "name", "Ljava/lang/String;").await
    }

    #[method(public, final)]
    async fn int_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.logging.Level::intValue({this:?})");

        jvm.get_field(&this, "java/util/logging/Level", "value", "I").await
    }

    #[method(public)]
    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.logging.Level::equals({this:?}, {other:?})");

        if other.is_null() || !jvm.is_instance(&**other, "java/util/logging/Level") {
            return Ok(false);
        }

        let other: ClassInstanceRef<Self> = ClassInstanceRef::new(other.instance);
        let value: i32 = jvm.get_field(&this, "java/util/logging/Level", "value", "I").await?;
        let other_value: i32 = jvm.get_field(&other, "java/util/logging/Level", "value", "I").await?;
        Ok(value == other_value)
    }

    #[method(public)]
    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.logging.Level::hashCode({this:?})");

        jvm.get_field(&this, "java/util/logging/Level", "value", "I").await
    }

    #[method(public, final)]
    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.logging.Level::toString({this:?})");

        jvm.get_field(&this, "java/util/logging/Level", "name", "Ljava/lang/String;").await
    }
}
