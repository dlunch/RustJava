use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::{ClassAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::io::OutputStream};

use super::LogRecord;

// public class java.util.logging.ConsoleHandler
pub struct ConsoleHandler;

impl ConsoleHandler {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/logging/ConsoleHandler",
            parent_class: Some("java/util/logging/StreamHandler"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("close", "()V", Self::close, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("publish", "(Ljava/util/logging/LogRecord;)V", Self::publish, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

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

    async fn close(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.logging.ConsoleHandler::close({this:?})");

        jvm.invoke_special(&this, "java/util/logging/StreamHandler", "flush", "()V", ()).await
    }

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
