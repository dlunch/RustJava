use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::io::OutputStream};

use super::LogRecord;

// public class java.util.logging.ConsoleHandler
#[java_class(name = "java/util/logging/ConsoleHandler", extends = "java/util/logging/StreamHandler", public)]
pub struct ConsoleHandler;

#[java_methods]
impl ConsoleHandler {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.logging.ConsoleHandler::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/util/logging/StreamHandler", "<init>", "()V", ()).await?;
        let error: ClassInstanceRef<OutputStream> = jvm.get_static_field("java/lang/System", "err", "Ljava/io/PrintStream;").await?;
        jvm.invoke_special(
            &this,
            "java/util/logging/StreamHandler",
            "setOutputStream",
            "(Ljava/io/OutputStream;)V",
            (error,),
        )
        .await
    }

    #[method(public)]
    async fn close(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.logging.ConsoleHandler::close({this:?})");

        jvm.invoke_special(&this, "java/util/logging/StreamHandler", "flush", "()V", ()).await
    }

    #[method(public)]
    async fn publish(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, record: ClassInstanceRef<LogRecord>) -> Result<()> {
        tracing::debug!("java.util.logging.ConsoleHandler::publish({this:?}, {record:?})");

        let _: () = jvm
            .invoke_special(
                &this,
                "java/util/logging/StreamHandler",
                "publish",
                "(Ljava/util/logging/LogRecord;)V",
                (record,),
            )
            .await?;
        jvm.invoke_special(&this, "java/util/logging/StreamHandler", "flush", "()V", ()).await
    }
}
