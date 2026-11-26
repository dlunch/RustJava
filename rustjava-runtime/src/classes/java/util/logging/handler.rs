use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{
        io::OutputStreamWriter,
        lang::{Exception, String},
    },
};

use super::{Filter, Formatter, Level, LogRecord};

// public abstract class java.util.logging.Handler
#[java_class(name = "java/util/logging/Handler", public, abstract)]
pub struct Handler {
    #[field(private)]
    level: ClassInstanceRef<Level>,
    #[field(private)]
    filter: ClassInstanceRef<Filter>,
    #[field(private)]
    formatter: ClassInstanceRef<Formatter>,
    #[field(private)]
    encoding: ClassInstanceRef<String>,
}

#[java_methods]
impl Handler {
    #[method(constructor, protected)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.logging.Handler::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        let level: ClassInstanceRef<Level> = jvm
            .get_static_field("java/util/logging/Level", "ALL", "Ljava/util/logging/Level;")
            .await?;
        jvm.put_field(&mut this, "java/util/logging/Handler", "level", "Ljava/util/logging/Level;", level)
            .await
    }

    #[method(public)]
    async fn close(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()>;

    #[method(public)]
    async fn flush(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()>;

    #[method(public)]
    async fn publish(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, record: ClassInstanceRef<LogRecord>) -> Result<()>;

    #[method(public)]
    async fn get_encoding(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.logging.Handler::getEncoding({this:?})");

        jvm.get_field(&this, "java/util/logging/Handler", "encoding", "Ljava/lang/String;").await
    }

    #[method(public)]
    async fn get_filter(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Filter>> {
        tracing::debug!("java.util.logging.Handler::getFilter({this:?})");

        jvm.get_field(&this, "java/util/logging/Handler", "filter", "Ljava/util/logging/Filter;")
            .await
    }

    #[method(public)]
    async fn get_formatter(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Formatter>> {
        tracing::debug!("java.util.logging.Handler::getFormatter({this:?})");

        jvm.get_field(&this, "java/util/logging/Handler", "formatter", "Ljava/util/logging/Formatter;")
            .await
    }

    #[method(public)]
    async fn get_level(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Level>> {
        tracing::debug!("java.util.logging.Handler::getLevel({this:?})");

        jvm.get_field(&this, "java/util/logging/Handler", "level", "Ljava/util/logging/Level;")
            .await
    }

    #[method(public)]
    async fn is_loggable(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, record: ClassInstanceRef<LogRecord>) -> Result<bool> {
        tracing::debug!("java.util.logging.Handler::isLoggable({this:?}, {record:?})");

        if record.is_null() {
            return Ok(false);
        }

        let record_level: ClassInstanceRef<Level> = jvm
            .invoke_virtual(&record, "java/util/logging/LogRecord", "getLevel", "()Ljava/util/logging/Level;", ())
            .await?;
        let handler_level: ClassInstanceRef<Level> = jvm
            .get_field(&this, "java/util/logging/Handler", "level", "Ljava/util/logging/Level;")
            .await?;
        let record_value: i32 = jvm
            .invoke_virtual(&record_level, "java/util/logging/Level", "intValue", "()I", ())
            .await?;
        let handler_value: i32 = jvm
            .invoke_virtual(&handler_level, "java/util/logging/Level", "intValue", "()I", ())
            .await?;
        if handler_value == i32::MAX || record_value < handler_value {
            return Ok(false);
        }

        let filter: ClassInstanceRef<Filter> = jvm
            .get_field(&this, "java/util/logging/Handler", "filter", "Ljava/util/logging/Filter;")
            .await?;
        if filter.is_null() {
            return Ok(true);
        }

        jvm.invoke_virtual(
            &filter,
            &filter.class_definition().name(),
            "isLoggable",
            "(Ljava/util/logging/LogRecord;)Z",
            (record,),
        )
        .await
    }

    #[method(protected)]
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

    #[method(public)]
    async fn set_encoding(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, encoding: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.util.logging.Handler::setEncoding({this:?}, {encoding:?})");

        if !encoding.is_null() {
            OutputStreamWriter::validate_encoding(jvm, &encoding).await?;
        }
        jvm.put_field(&mut this, "java/util/logging/Handler", "encoding", "Ljava/lang/String;", encoding)
            .await
    }

    #[method(public)]
    async fn set_filter(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, filter: ClassInstanceRef<Filter>) -> Result<()> {
        tracing::debug!("java.util.logging.Handler::setFilter({this:?}, {filter:?})");

        jvm.put_field(&mut this, "java/util/logging/Handler", "filter", "Ljava/util/logging/Filter;", filter)
            .await
    }

    #[method(public)]
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
        jvm.put_field(
            &mut this,
            "java/util/logging/Handler",
            "formatter",
            "Ljava/util/logging/Formatter;",
            formatter,
        )
        .await
    }

    #[method(public)]
    async fn set_level(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, level: ClassInstanceRef<Level>) -> Result<()> {
        tracing::debug!("java.util.logging.Handler::setLevel({this:?}, {level:?})");

        if level.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "level").await);
        }
        jvm.put_field(&mut this, "java/util/logging/Handler", "level", "Ljava/util/logging/Level;", level)
            .await
    }
}
