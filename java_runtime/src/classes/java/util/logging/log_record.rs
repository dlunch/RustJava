use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::lang::{Object, String, Throwable},
};

use super::Level;

// public class java.util.logging.LogRecord
pub struct LogRecord;

impl LogRecord {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/logging/LogRecord",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/io/Serializable"],
            methods: vec![
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/util/logging/Level;Ljava/lang/String;)V",
                    Self::init,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "allocateSequenceNumber",
                    "()J",
                    Self::allocate_sequence_number,
                    MethodAccessFlags::PRIVATE | MethodAccessFlags::STATIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new("getLoggerName", "()Ljava/lang/String;", Self::get_logger_name, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("setLoggerName", "(Ljava/lang/String;)V", Self::set_logger_name, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "getResourceBundle",
                    "()Ljava/util/ResourceBundle;",
                    Self::get_resource_bundle,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "setResourceBundle",
                    "(Ljava/util/ResourceBundle;)V",
                    Self::set_resource_bundle,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "getResourceBundleName",
                    "()Ljava/lang/String;",
                    Self::get_resource_bundle_name,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "setResourceBundleName",
                    "(Ljava/lang/String;)V",
                    Self::set_resource_bundle_name,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("getLevel", "()Ljava/util/logging/Level;", Self::get_level, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("setLevel", "(Ljava/util/logging/Level;)V", Self::set_level, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getSequenceNumber", "()J", Self::get_sequence_number, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("setSequenceNumber", "(J)V", Self::set_sequence_number, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "getSourceClassName",
                    "()Ljava/lang/String;",
                    Self::get_source_class_name,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "setSourceClassName",
                    "(Ljava/lang/String;)V",
                    Self::set_source_class_name,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "getSourceMethodName",
                    "()Ljava/lang/String;",
                    Self::get_source_method_name,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "setSourceMethodName",
                    "(Ljava/lang/String;)V",
                    Self::set_source_method_name,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("getMessage", "()Ljava/lang/String;", Self::get_message, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("setMessage", "(Ljava/lang/String;)V", Self::set_message, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getThreadID", "()I", Self::get_thread_id, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("setThreadID", "(I)V", Self::set_thread_id, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getMillis", "()J", Self::get_millis, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("setMillis", "(J)V", Self::set_millis, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getThrown", "()Ljava/lang/Throwable;", Self::get_thrown, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("setThrown", "(Ljava/lang/Throwable;)V", Self::set_thrown, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getParameters", "()[Ljava/lang/Object;", Self::get_parameters, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("setParameters", "([Ljava/lang/Object;)V", Self::set_parameters, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("nextSequenceNumber", "J", FieldAccessFlags::PRIVATE | FieldAccessFlags::STATIC),
                JavaFieldProto::new("loggerName", "Ljava/lang/String;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("resourceBundle", "Ljava/util/ResourceBundle;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("resourceBundleName", "Ljava/lang/String;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("level", "Ljava/util/logging/Level;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("sequenceNumber", "J", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("sourceClassName", "Ljava/lang/String;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("sourceMethodName", "Ljava/lang/String;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("message", "Ljava/lang/String;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("threadID", "I", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("millis", "J", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("thrown", "Ljava/lang/Throwable;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("parameters", "[Ljava/lang/Object;", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        level: ClassInstanceRef<Level>,
        message: ClassInstanceRef<String>,
    ) -> Result<()> {
        tracing::debug!("java.util.logging.LogRecord::<init>({this:?}, {level:?}, {message:?})");

        if level.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "level").await);
        }

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        let sequence_number: i64 = jvm
            .invoke_static("java/util/logging/LogRecord", "allocateSequenceNumber", "()J", ())
            .await?;
        let current_thread = jvm.current_java_thread();
        let thread_id: i64 = jvm.get_field(&current_thread, "id", "J").await?;

        jvm.put_field(&mut this, "level", "Ljava/util/logging/Level;", level).await?;
        jvm.put_field(&mut this, "message", "Ljava/lang/String;", message).await?;
        jvm.put_field(&mut this, "sequenceNumber", "J", sequence_number).await?;
        jvm.put_field(&mut this, "millis", "J", context.now() as i64).await?;
        jvm.put_field(&mut this, "threadID", "I", thread_id as i32).await
    }

    async fn allocate_sequence_number(jvm: &Jvm, _: &mut RuntimeContext) -> Result<i64> {
        let sequence_number: i64 = jvm.get_static_field("java/util/logging/LogRecord", "nextSequenceNumber", "J").await?;
        jvm.put_static_field("java/util/logging/LogRecord", "nextSequenceNumber", "J", sequence_number.wrapping_add(1))
            .await?;
        Ok(sequence_number)
    }

    async fn get_logger_name(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "loggerName", "Ljava/lang/String;").await
    }

    async fn set_logger_name(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, name: ClassInstanceRef<String>) -> Result<()> {
        jvm.put_field(&mut this, "loggerName", "Ljava/lang/String;", name).await
    }

    async fn get_resource_bundle(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        jvm.get_field(&this, "resourceBundle", "Ljava/util/ResourceBundle;").await
    }

    async fn set_resource_bundle(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        bundle: ClassInstanceRef<Object>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "resourceBundle", "Ljava/util/ResourceBundle;", bundle).await
    }

    async fn get_resource_bundle_name(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "resourceBundleName", "Ljava/lang/String;").await
    }

    async fn set_resource_bundle_name(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "resourceBundleName", "Ljava/lang/String;", name).await
    }

    async fn get_level(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Level>> {
        jvm.get_field(&this, "level", "Ljava/util/logging/Level;").await
    }

    async fn set_level(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, level: ClassInstanceRef<Level>) -> Result<()> {
        if level.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "level").await);
        }

        jvm.put_field(&mut this, "level", "Ljava/util/logging/Level;", level).await
    }

    async fn get_sequence_number(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        jvm.get_field(&this, "sequenceNumber", "J").await
    }

    async fn set_sequence_number(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, sequence_number: i64) -> Result<()> {
        jvm.put_field(&mut this, "sequenceNumber", "J", sequence_number).await
    }

    async fn get_source_class_name(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "sourceClassName", "Ljava/lang/String;").await
    }

    async fn set_source_class_name(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        source_class_name: ClassInstanceRef<String>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "sourceClassName", "Ljava/lang/String;", source_class_name).await
    }

    async fn get_source_method_name(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "sourceMethodName", "Ljava/lang/String;").await
    }

    async fn set_source_method_name(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        source_method_name: ClassInstanceRef<String>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "sourceMethodName", "Ljava/lang/String;", source_method_name)
            .await
    }

    async fn get_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "message", "Ljava/lang/String;").await
    }

    async fn set_message(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        jvm.put_field(&mut this, "message", "Ljava/lang/String;", message).await
    }

    async fn get_thread_id(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "threadID", "I").await
    }

    async fn set_thread_id(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, thread_id: i32) -> Result<()> {
        jvm.put_field(&mut this, "threadID", "I", thread_id).await
    }

    async fn get_millis(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        jvm.get_field(&this, "millis", "J").await
    }

    async fn set_millis(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, millis: i64) -> Result<()> {
        jvm.put_field(&mut this, "millis", "J", millis).await
    }

    async fn get_thrown(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Throwable>> {
        jvm.get_field(&this, "thrown", "Ljava/lang/Throwable;").await
    }

    async fn set_thrown(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, thrown: ClassInstanceRef<Throwable>) -> Result<()> {
        jvm.put_field(&mut this, "thrown", "Ljava/lang/Throwable;", thrown).await
    }

    async fn get_parameters(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Array<Object>>> {
        jvm.get_field(&this, "parameters", "[Ljava/lang/Object;").await
    }

    async fn set_parameters(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        parameters: ClassInstanceRef<Array<Object>>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "parameters", "[Ljava/lang/Object;", parameters).await
    }
}
