use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::OutputStreamWriter,
        lang::{Exception, String},
    },
};

use super::{Filter, Formatter, Level, LogRecord};

// public abstract class java.util.logging.Handler
pub struct Handler;

impl Handler {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/logging/Handler",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PROTECTED),
                JavaMethodProto::new_abstract("close", "()V", MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new_abstract("flush", "()V", MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new_abstract(
                    "publish",
                    "(Ljava/util/logging/LogRecord;)V",
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT,
                ),
                JavaMethodProto::new("getEncoding", "()Ljava/lang/String;", Self::get_encoding, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getFilter", "()Ljava/util/logging/Filter;", Self::get_filter, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "getFormatter",
                    "()Ljava/util/logging/Formatter;",
                    Self::get_formatter,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("getLevel", "()Ljava/util/logging/Level;", Self::get_level, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "isLoggable",
                    "(Ljava/util/logging/LogRecord;)Z",
                    Self::is_loggable,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "reportError",
                    "(Ljava/lang/String;Ljava/lang/Exception;I)V",
                    Self::report_error,
                    MethodAccessFlags::PROTECTED,
                ),
                JavaMethodProto::new("setEncoding", "(Ljava/lang/String;)V", Self::set_encoding, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("setFilter", "(Ljava/util/logging/Filter;)V", Self::set_filter, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "setFormatter",
                    "(Ljava/util/logging/Formatter;)V",
                    Self::set_formatter,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("setLevel", "(Ljava/util/logging/Level;)V", Self::set_level, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("level", "Ljava/util/logging/Level;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("filter", "Ljava/util/logging/Filter;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("formatter", "Ljava/util/logging/Formatter;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("encoding", "Ljava/lang/String;", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::ABSTRACT,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.logging.Handler::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        let level: ClassInstanceRef<Level> = jvm
            .get_static_field("java/util/logging/Level", "ALL", "Ljava/util/logging/Level;")
            .await?;
        jvm.put_field(&mut this, "level", "Ljava/util/logging/Level;", level).await
    }

    async fn get_encoding(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.logging.Handler::getEncoding({this:?})");

        jvm.get_field(&this, "encoding", "Ljava/lang/String;").await
    }

    async fn get_filter(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Filter>> {
        tracing::debug!("java.util.logging.Handler::getFilter({this:?})");

        jvm.get_field(&this, "filter", "Ljava/util/logging/Filter;").await
    }

    async fn get_formatter(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Formatter>> {
        tracing::debug!("java.util.logging.Handler::getFormatter({this:?})");

        jvm.get_field(&this, "formatter", "Ljava/util/logging/Formatter;").await
    }

    async fn get_level(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Level>> {
        tracing::debug!("java.util.logging.Handler::getLevel({this:?})");

        jvm.get_field(&this, "level", "Ljava/util/logging/Level;").await
    }

    async fn is_loggable(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, record: ClassInstanceRef<LogRecord>) -> Result<bool> {
        tracing::debug!("java.util.logging.Handler::isLoggable({this:?}, {record:?})");

        if record.is_null() {
            return Ok(false);
        }

        let record_level: ClassInstanceRef<Level> = jvm.invoke_virtual(&record, "getLevel", "()Ljava/util/logging/Level;", ()).await?;
        let handler_level: ClassInstanceRef<Level> = jvm.get_field(&this, "level", "Ljava/util/logging/Level;").await?;
        let record_value: i32 = jvm.invoke_virtual(&record_level, "intValue", "()I", ()).await?;
        let handler_value: i32 = jvm.invoke_virtual(&handler_level, "intValue", "()I", ()).await?;
        if handler_value == i32::MAX || record_value < handler_value {
            return Ok(false);
        }

        let filter: ClassInstanceRef<Filter> = jvm.get_field(&this, "filter", "Ljava/util/logging/Filter;").await?;
        if filter.is_null() {
            return Ok(true);
        }

        jvm.invoke_virtual(&filter, "isLoggable", "(Ljava/util/logging/LogRecord;)Z", (record,))
            .await
    }

    async fn report_error(
        _: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        _: ClassInstanceRef<String>,
        _: ClassInstanceRef<Exception>,
        _: i32,
    ) -> Result<()> {
        tracing::debug!("java.util.logging.Handler::reportError({this:?})");

        Ok(())
    }

    async fn set_encoding(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, encoding: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.util.logging.Handler::setEncoding({this:?}, {encoding:?})");

        if !encoding.is_null() {
            OutputStreamWriter::validate_encoding(jvm, &encoding).await?;
        }
        jvm.put_field(&mut this, "encoding", "Ljava/lang/String;", encoding).await
    }

    async fn set_filter(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, filter: ClassInstanceRef<Filter>) -> Result<()> {
        tracing::debug!("java.util.logging.Handler::setFilter({this:?}, {filter:?})");

        jvm.put_field(&mut this, "filter", "Ljava/util/logging/Filter;", filter).await
    }

    async fn set_formatter(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        formatter: ClassInstanceRef<Formatter>,
    ) -> Result<()> {
        tracing::debug!("java.util.logging.Handler::setFormatter({this:?}, {formatter:?})");

        if formatter.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "formatter").await);
        }
        jvm.put_field(&mut this, "formatter", "Ljava/util/logging/Formatter;", formatter).await
    }

    async fn set_level(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, level: ClassInstanceRef<Level>) -> Result<()> {
        tracing::debug!("java.util.logging.Handler::setLevel({this:?}, {level:?})");

        if level.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "level").await);
        }
        jvm.put_field(&mut this, "level", "Ljava/util/logging/Level;", level).await
    }
}
