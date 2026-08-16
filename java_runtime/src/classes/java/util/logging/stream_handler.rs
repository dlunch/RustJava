use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, JavaError, Jvm, Result};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::{OutputStream, OutputStreamWriter},
        lang::{Exception, String},
    },
};

use super::{Formatter, Level, LogRecord, SimpleFormatter};

// public class java.util.logging.StreamHandler
pub struct StreamHandler;

impl StreamHandler {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/logging/StreamHandler",
            parent_class: Some("java/util/logging/Handler"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/io/OutputStream;Ljava/util/logging/Formatter;)V",
                    Self::init_with_output,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("close", "()V", Self::close, MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED),
                JavaMethodProto::new("flush", "()V", Self::flush, MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED),
                JavaMethodProto::new(
                    "isLoggable",
                    "(Ljava/util/logging/LogRecord;)Z",
                    Self::is_loggable,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "publish",
                    "(Ljava/util/logging/LogRecord;)V",
                    Self::publish,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "setEncoding",
                    "(Ljava/lang/String;)V",
                    Self::set_encoding,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "setOutputStream",
                    "(Ljava/io/OutputStream;)V",
                    Self::set_output_stream,
                    MethodAccessFlags::PROTECTED | MethodAccessFlags::SYNCHRONIZED,
                ),
            ],
            fields: vec![
                JavaFieldProto::new("output", "Ljava/io/OutputStream;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("writer", "Ljava/io/OutputStreamWriter;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("headerWritten", "Z", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.logging.StreamHandler::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/util/logging/Handler", "<init>", "()V", ()).await?;
        let formatter: ClassInstanceRef<SimpleFormatter> = jvm.new_class("java/util/logging/SimpleFormatter", "()V", ()).await?.into();
        let level: ClassInstanceRef<Level> = jvm
            .get_static_field("java/util/logging/Level", "INFO", "Ljava/util/logging/Level;")
            .await?;
        let _: () = jvm
            .invoke_special(
                &this,
                "java/util/logging/Handler",
                "setFormatter",
                "(Ljava/util/logging/Formatter;)V",
                (formatter,),
            )
            .await?;
        let _: () = jvm
            .invoke_special(&this, "java/util/logging/Handler", "setLevel", "(Ljava/util/logging/Level;)V", (level,))
            .await?;
        jvm.put_field(&mut this, "headerWritten", "Z", false).await
    }

    async fn init_with_output(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        output: ClassInstanceRef<OutputStream>,
        formatter: ClassInstanceRef<Formatter>,
    ) -> Result<()> {
        tracing::debug!("java.util.logging.StreamHandler::<init>({this:?}, {output:?}, {formatter:?})");

        if output.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "output").await);
        }
        if formatter.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "formatter").await);
        }

        let _: () = jvm.invoke_special(&this, "java/util/logging/StreamHandler", "<init>", "()V", ()).await?;
        let _: () = jvm
            .invoke_special(
                &this,
                "java/util/logging/Handler",
                "setFormatter",
                "(Ljava/util/logging/Formatter;)V",
                (formatter,),
            )
            .await?;
        jvm.invoke_special(
            &this,
            "java/util/logging/StreamHandler",
            "setOutputStream",
            "(Ljava/io/OutputStream;)V",
            (output,),
        )
        .await
    }

    async fn close(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.logging.StreamHandler::close({this:?})");

        let writer: ClassInstanceRef<OutputStreamWriter> = jvm.get_field(&this, "writer", "Ljava/io/OutputStreamWriter;").await?;
        if writer.is_null() {
            return Ok(());
        }

        Self::write_tail(jvm, &this).await?;
        let _: () = jvm.invoke_virtual(&writer, "java/io/OutputStreamWriter", "flush", "()V", ()).await?;
        let _: () = jvm.invoke_virtual(&writer, "java/io/OutputStreamWriter", "close", "()V", ()).await?;
        let output: ClassInstanceRef<OutputStream> = None.into();
        let writer: ClassInstanceRef<OutputStreamWriter> = None.into();
        jvm.put_field(&mut this, "output", "Ljava/io/OutputStream;", output).await?;
        jvm.put_field(&mut this, "writer", "Ljava/io/OutputStreamWriter;", writer).await?;
        jvm.put_field(&mut this, "headerWritten", "Z", false).await
    }

    async fn flush(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.logging.StreamHandler::flush({this:?})");

        let writer: ClassInstanceRef<OutputStreamWriter> = jvm.get_field(&this, "writer", "Ljava/io/OutputStreamWriter;").await?;
        if writer.is_null() {
            return Ok(());
        }
        jvm.invoke_virtual(&writer, "java/io/OutputStreamWriter", "flush", "()V", ()).await
    }

    async fn is_loggable(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, record: ClassInstanceRef<LogRecord>) -> Result<bool> {
        tracing::debug!("java.util.logging.StreamHandler::isLoggable({this:?}, {record:?})");

        let output: ClassInstanceRef<OutputStream> = jvm.get_field(&this, "output", "Ljava/io/OutputStream;").await?;
        if output.is_null() || record.is_null() {
            return Ok(false);
        }

        jvm.invoke_special(
            &this,
            "java/util/logging/Handler",
            "isLoggable",
            "(Ljava/util/logging/LogRecord;)Z",
            (record,),
        )
        .await
    }

    async fn publish(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, record: ClassInstanceRef<LogRecord>) -> Result<()> {
        tracing::debug!("java.util.logging.StreamHandler::publish({this:?}, {record:?})");

        let loggable: bool = jvm
            .invoke_virtual(
                &this,
                "java/util/logging/StreamHandler",
                "isLoggable",
                "(Ljava/util/logging/LogRecord;)Z",
                (record.clone(),),
            )
            .await?;
        if !loggable {
            return Ok(());
        }

        if let Err(JavaError::JavaException(exception)) = Self::write_head(jvm, &this).await {
            if !jvm.is_instance(&*exception, "java/lang/Exception") {
                return Err(JavaError::JavaException(exception));
            }
            let message: ClassInstanceRef<String> = None.into();
            let exception: ClassInstanceRef<Exception> = exception.into();
            let _: () = jvm
                .invoke_virtual(
                    &this,
                    "java/util/logging/StreamHandler",
                    "reportError",
                    "(Ljava/lang/String;Ljava/lang/Exception;I)V",
                    (message, exception, 1),
                )
                .await?;
            return Ok(());
        }
        let formatter: ClassInstanceRef<Formatter> = jvm.get_field(&this, "formatter", "Ljava/util/logging/Formatter;").await?;
        let formatted: ClassInstanceRef<String> = match jvm
            .invoke_virtual(
                &formatter,
                "java/util/logging/Formatter",
                "format",
                "(Ljava/util/logging/LogRecord;)Ljava/lang/String;",
                (record,),
            )
            .await
        {
            Ok(formatted) => formatted,
            Err(JavaError::JavaException(exception)) => {
                if !jvm.is_instance(&*exception, "java/lang/Exception") {
                    return Err(JavaError::JavaException(exception));
                }
                let message: ClassInstanceRef<String> = None.into();
                let exception: ClassInstanceRef<Exception> = exception.into();
                let _: () = jvm
                    .invoke_virtual(
                        &this,
                        "java/util/logging/StreamHandler",
                        "reportError",
                        "(Ljava/lang/String;Ljava/lang/Exception;I)V",
                        (message, exception, 5),
                    )
                    .await?;
                return Ok(());
            }
        };
        let writer: ClassInstanceRef<OutputStreamWriter> = jvm.get_field(&this, "writer", "Ljava/io/OutputStreamWriter;").await?;
        if let Err(JavaError::JavaException(exception)) = jvm
            .invoke_virtual::<_, ()>(&writer, "java/io/OutputStreamWriter", "write", "(Ljava/lang/String;)V", (formatted,))
            .await
        {
            if !jvm.is_instance(&*exception, "java/lang/Exception") {
                return Err(JavaError::JavaException(exception));
            }
            let message: ClassInstanceRef<String> = None.into();
            let exception: ClassInstanceRef<Exception> = exception.into();
            let _: () = jvm
                .invoke_virtual(
                    &this,
                    "java/util/logging/StreamHandler",
                    "reportError",
                    "(Ljava/lang/String;Ljava/lang/Exception;I)V",
                    (message, exception, 1),
                )
                .await?;
        }
        Ok(())
    }

    async fn set_encoding(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, encoding: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.util.logging.StreamHandler::setEncoding({this:?}, {encoding:?})");

        let _: () = jvm
            .invoke_special(
                &this,
                "java/util/logging/Handler",
                "setEncoding",
                "(Ljava/lang/String;)V",
                (encoding.clone(),),
            )
            .await?;

        let output: ClassInstanceRef<OutputStream> = jvm.get_field(&this, "output", "Ljava/io/OutputStream;").await?;
        if output.is_null() {
            return Ok(());
        }

        let writer: ClassInstanceRef<OutputStreamWriter> = jvm.get_field(&this, "writer", "Ljava/io/OutputStreamWriter;").await?;
        let _: () = jvm.invoke_virtual(&writer, "java/io/OutputStreamWriter", "flush", "()V", ()).await?;
        let writer: ClassInstanceRef<OutputStreamWriter> = if encoding.is_null() {
            jvm.new_class("java/io/OutputStreamWriter", "(Ljava/io/OutputStream;)V", (output,))
                .await?
                .into()
        } else {
            jvm.new_class(
                "java/io/OutputStreamWriter",
                "(Ljava/io/OutputStream;Ljava/lang/String;)V",
                (output, encoding),
            )
            .await?
            .into()
        };
        jvm.put_field(&mut this, "writer", "Ljava/io/OutputStreamWriter;", writer).await
    }

    async fn set_output_stream(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        output: ClassInstanceRef<OutputStream>,
    ) -> Result<()> {
        tracing::debug!("java.util.logging.StreamHandler::setOutputStream({this:?}, {output:?})");

        if output.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "output").await);
        }

        let current_writer: ClassInstanceRef<OutputStreamWriter> = jvm.get_field(&this, "writer", "Ljava/io/OutputStreamWriter;").await?;
        if !current_writer.is_null() {
            Self::write_tail(jvm, &this).await?;
            let _: () = jvm
                .invoke_virtual(&current_writer, "java/io/OutputStreamWriter", "flush", "()V", ())
                .await?;
            let _: () = jvm
                .invoke_virtual(&current_writer, "java/io/OutputStreamWriter", "close", "()V", ())
                .await?;
        }

        let encoding: ClassInstanceRef<String> = jvm.get_field(&this, "encoding", "Ljava/lang/String;").await?;
        let writer: ClassInstanceRef<OutputStreamWriter> = if encoding.is_null() {
            jvm.new_class("java/io/OutputStreamWriter", "(Ljava/io/OutputStream;)V", (output.clone(),))
                .await?
                .into()
        } else {
            jvm.new_class(
                "java/io/OutputStreamWriter",
                "(Ljava/io/OutputStream;Ljava/lang/String;)V",
                (output.clone(), encoding),
            )
            .await?
            .into()
        };
        jvm.put_field(&mut this, "output", "Ljava/io/OutputStream;", output).await?;
        jvm.put_field(&mut this, "writer", "Ljava/io/OutputStreamWriter;", writer).await?;
        jvm.put_field(&mut this, "headerWritten", "Z", false).await
    }

    async fn write_head(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<()> {
        if jvm.get_field::<bool>(this, "headerWritten", "Z").await? {
            return Ok(());
        }

        let formatter: ClassInstanceRef<Formatter> = jvm.get_field(this, "formatter", "Ljava/util/logging/Formatter;").await?;
        let head: ClassInstanceRef<String> = jvm
            .invoke_virtual(
                &formatter,
                "java/util/logging/Formatter",
                "getHead",
                "(Ljava/util/logging/Handler;)Ljava/lang/String;",
                (this.clone(),),
            )
            .await?;
        let writer: ClassInstanceRef<OutputStreamWriter> = jvm.get_field(this, "writer", "Ljava/io/OutputStreamWriter;").await?;
        let _: () = jvm
            .invoke_virtual(&writer, "java/io/OutputStreamWriter", "write", "(Ljava/lang/String;)V", (head,))
            .await?;
        let mut this = this.clone();
        jvm.put_field(&mut this, "headerWritten", "Z", true).await
    }

    async fn write_tail(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<()> {
        Self::write_head(jvm, this).await?;
        let formatter: ClassInstanceRef<Formatter> = jvm.get_field(this, "formatter", "Ljava/util/logging/Formatter;").await?;
        let tail: ClassInstanceRef<String> = jvm
            .invoke_virtual(
                &formatter,
                "java/util/logging/Formatter",
                "getTail",
                "(Ljava/util/logging/Handler;)Ljava/lang/String;",
                (this.clone(),),
            )
            .await?;
        let writer: ClassInstanceRef<OutputStreamWriter> = jvm.get_field(this, "writer", "Ljava/io/OutputStreamWriter;").await?;
        jvm.invoke_virtual(&writer, "java/io/OutputStreamWriter", "write", "(Ljava/lang/String;)V", (tail,))
            .await
    }
}
