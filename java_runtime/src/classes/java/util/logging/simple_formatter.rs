use alloc::{format, string::String as RustString, vec};

use java_class_proto::JavaMethodProto;
use java_constants::{ClassAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::lang::{String, Throwable},
};

use super::{Level, LogRecord};

// public class java.util.logging.SimpleFormatter
pub struct SimpleFormatter;

impl SimpleFormatter {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/logging/SimpleFormatter",
            parent_class: Some("java/util/logging/Formatter"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "format",
                    "(Ljava/util/logging/LogRecord;)Ljava/lang/String;",
                    Self::format,
                    MethodAccessFlags::PUBLIC,
                ),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.logging.SimpleFormatter::<init>({this:?})");

        jvm.invoke_special(&this, "java/util/logging/Formatter", "<init>", "()V", ()).await
    }

    async fn format(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        record: ClassInstanceRef<LogRecord>,
    ) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.logging.SimpleFormatter::format({this:?}, {record:?})");

        if record.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "record").await);
        }

        let source_class: ClassInstanceRef<String> = jvm.invoke_virtual(&record, "getSourceClassName", "()Ljava/lang/String;", ()).await?;
        let source_method: ClassInstanceRef<String> = jvm.invoke_virtual(&record, "getSourceMethodName", "()Ljava/lang/String;", ()).await?;
        let logger_name: ClassInstanceRef<String> = jvm.invoke_virtual(&record, "getLoggerName", "()Ljava/lang/String;", ()).await?;
        let mut source = if source_class.is_null() {
            if logger_name.is_null() {
                RustString::new()
            } else {
                JavaLangString::to_rust_string(jvm, &logger_name).await?
            }
        } else {
            JavaLangString::to_rust_string(jvm, &source_class).await?
        };
        if !source_method.is_null() {
            if !source.is_empty() {
                source.push(' ');
            }
            source.push_str(&JavaLangString::to_rust_string(jvm, &source_method).await?);
        }

        let level: ClassInstanceRef<Level> = jvm.invoke_virtual(&record, "getLevel", "()Ljava/util/logging/Level;", ()).await?;
        let level_name: ClassInstanceRef<String> = jvm.invoke_virtual(&level, "getName", "()Ljava/lang/String;", ()).await?;
        let level_name = JavaLangString::to_rust_string(jvm, &level_name).await?;
        let message: ClassInstanceRef<String> = jvm
            .invoke_virtual(
                &this,
                "formatMessage",
                "(Ljava/util/logging/LogRecord;)Ljava/lang/String;",
                (record.clone(),),
            )
            .await?;
        let message = if message.is_null() {
            RustString::from("null")
        } else {
            JavaLangString::to_rust_string(jvm, &message).await?
        };

        let mut formatted = if source.is_empty() {
            format!("{level_name}: {message}\n")
        } else {
            format!("{source} {level_name}: {message}\n")
        };
        let thrown: ClassInstanceRef<Throwable> = jvm.invoke_virtual(&record, "getThrown", "()Ljava/lang/Throwable;", ()).await?;
        if !thrown.is_null() {
            let thrown: ClassInstanceRef<String> = jvm.invoke_virtual(&thrown, "toString", "()Ljava/lang/String;", ()).await?;
            if thrown.is_null() {
                formatted.push_str("null\n");
            } else {
                formatted.push_str(&JavaLangString::to_rust_string(jvm, &thrown).await?);
                formatted.push('\n');
            }
        }

        Ok(JavaLangString::from_rust_string(jvm, &formatted).await?.into())
    }
}
