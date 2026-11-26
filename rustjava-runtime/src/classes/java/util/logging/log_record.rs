use jvm::{Array, ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::lang::{Object, String, Throwable},
};

use super::Level;

// public class java.util.logging.LogRecord
#[java_class(
    name = "java/util/logging/LogRecord",
    extends = "java/lang/Object",
    implements("java/io/Serializable"),
    public
)]
pub struct LogRecord {
    #[field(private, static)]
    nextSequenceNumber: i64,
    #[field(private)]
    loggerName: ClassInstanceRef<String>,
    #[field(descriptor = "Ljava/util/ResourceBundle;", private)]
    resourceBundle: ClassInstanceRef<Object>,
    #[field(private)]
    resourceBundleName: ClassInstanceRef<String>,
    #[field(private)]
    level: ClassInstanceRef<Level>,
    #[field(private)]
    sequenceNumber: i64,
    #[field(private)]
    sourceClassName: ClassInstanceRef<String>,
    #[field(private)]
    sourceMethodName: ClassInstanceRef<String>,
    #[field(private)]
    message: ClassInstanceRef<String>,
    #[field(private)]
    threadID: i32,
    #[field(private)]
    millis: i64,
    #[field(private)]
    thrown: ClassInstanceRef<Throwable>,
    #[field(private)]
    parameters: ClassInstanceRef<Array<Object>>,
}

#[java_methods]
impl LogRecord {
    #[method(constructor, public)]
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
        let thread_id: i64 = jvm.get_field(&current_thread, "java/lang/Thread", "id", "J").await?;

        jvm.put_field(&mut this, "java/util/logging/LogRecord", "level", "Ljava/util/logging/Level;", level)
            .await?;
        jvm.put_field(&mut this, "java/util/logging/LogRecord", "message", "Ljava/lang/String;", message)
            .await?;
        jvm.put_field(&mut this, "java/util/logging/LogRecord", "sequenceNumber", "J", sequence_number)
            .await?;
        jvm.put_field(&mut this, "java/util/logging/LogRecord", "millis", "J", context.now() as i64)
            .await?;
        jvm.put_field(&mut this, "java/util/logging/LogRecord", "threadID", "I", thread_id as i32)
            .await
    }

    #[method(private, static, synchronized)]
    async fn allocate_sequence_number(jvm: &Jvm, _: &mut RuntimeContext) -> Result<i64> {
        let sequence_number: i64 = jvm.get_static_field("java/util/logging/LogRecord", "nextSequenceNumber", "J").await?;
        jvm.put_static_field("java/util/logging/LogRecord", "nextSequenceNumber", "J", sequence_number.wrapping_add(1))
            .await?;
        Ok(sequence_number)
    }

    #[method(public)]
    async fn get_logger_name(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "java/util/logging/LogRecord", "loggerName", "Ljava/lang/String;")
            .await
    }

    #[method(public)]
    async fn set_logger_name(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, name: ClassInstanceRef<String>) -> Result<()> {
        jvm.put_field(&mut this, "java/util/logging/LogRecord", "loggerName", "Ljava/lang/String;", name)
            .await
    }

    #[method(descriptor = "()Ljava/util/ResourceBundle;", public)]
    async fn get_resource_bundle(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        jvm.get_field(&this, "java/util/logging/LogRecord", "resourceBundle", "Ljava/util/ResourceBundle;")
            .await
    }

    #[method(descriptor = "(Ljava/util/ResourceBundle;)V", public)]
    async fn set_resource_bundle(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        bundle: ClassInstanceRef<Object>,
    ) -> Result<()> {
        jvm.put_field(
            &mut this,
            "java/util/logging/LogRecord",
            "resourceBundle",
            "Ljava/util/ResourceBundle;",
            bundle,
        )
        .await
    }

    #[method(public)]
    async fn get_resource_bundle_name(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "java/util/logging/LogRecord", "resourceBundleName", "Ljava/lang/String;")
            .await
    }

    #[method(public)]
    async fn set_resource_bundle_name(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "java/util/logging/LogRecord", "resourceBundleName", "Ljava/lang/String;", name)
            .await
    }

    #[method(public)]
    async fn get_level(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Level>> {
        jvm.get_field(&this, "java/util/logging/LogRecord", "level", "Ljava/util/logging/Level;")
            .await
    }

    #[method(public)]
    async fn set_level(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, level: ClassInstanceRef<Level>) -> Result<()> {
        if level.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "level").await);
        }

        jvm.put_field(&mut this, "java/util/logging/LogRecord", "level", "Ljava/util/logging/Level;", level)
            .await
    }

    #[method(public)]
    async fn get_sequence_number(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        jvm.get_field(&this, "java/util/logging/LogRecord", "sequenceNumber", "J").await
    }

    #[method(public)]
    async fn set_sequence_number(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, sequence_number: i64) -> Result<()> {
        jvm.put_field(&mut this, "java/util/logging/LogRecord", "sequenceNumber", "J", sequence_number)
            .await
    }

    #[method(public)]
    async fn get_source_class_name(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "java/util/logging/LogRecord", "sourceClassName", "Ljava/lang/String;")
            .await
    }

    #[method(public)]
    async fn set_source_class_name(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        source_class_name: ClassInstanceRef<String>,
    ) -> Result<()> {
        jvm.put_field(
            &mut this,
            "java/util/logging/LogRecord",
            "sourceClassName",
            "Ljava/lang/String;",
            source_class_name,
        )
        .await
    }

    #[method(public)]
    async fn get_source_method_name(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "java/util/logging/LogRecord", "sourceMethodName", "Ljava/lang/String;")
            .await
    }

    #[method(public)]
    async fn set_source_method_name(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        source_method_name: ClassInstanceRef<String>,
    ) -> Result<()> {
        jvm.put_field(
            &mut this,
            "java/util/logging/LogRecord",
            "sourceMethodName",
            "Ljava/lang/String;",
            source_method_name,
        )
        .await
    }

    #[method(public)]
    async fn get_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "java/util/logging/LogRecord", "message", "Ljava/lang/String;").await
    }

    #[method(public)]
    async fn set_message(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        jvm.put_field(&mut this, "java/util/logging/LogRecord", "message", "Ljava/lang/String;", message)
            .await
    }

    #[method(name = "getThreadID", public)]
    async fn get_thread_id(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "java/util/logging/LogRecord", "threadID", "I").await
    }

    #[method(name = "setThreadID", public)]
    async fn set_thread_id(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, thread_id: i32) -> Result<()> {
        jvm.put_field(&mut this, "java/util/logging/LogRecord", "threadID", "I", thread_id).await
    }

    #[method(public)]
    async fn get_millis(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        jvm.get_field(&this, "java/util/logging/LogRecord", "millis", "J").await
    }

    #[method(public)]
    async fn set_millis(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, millis: i64) -> Result<()> {
        jvm.put_field(&mut this, "java/util/logging/LogRecord", "millis", "J", millis).await
    }

    #[method(public)]
    async fn get_thrown(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Throwable>> {
        jvm.get_field(&this, "java/util/logging/LogRecord", "thrown", "Ljava/lang/Throwable;")
            .await
    }

    #[method(public)]
    async fn set_thrown(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, thrown: ClassInstanceRef<Throwable>) -> Result<()> {
        jvm.put_field(&mut this, "java/util/logging/LogRecord", "thrown", "Ljava/lang/Throwable;", thrown)
            .await
    }

    #[method(public)]
    async fn get_parameters(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Array<Object>>> {
        jvm.get_field(&this, "java/util/logging/LogRecord", "parameters", "[Ljava/lang/Object;")
            .await
    }

    #[method(public)]
    async fn set_parameters(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        parameters: ClassInstanceRef<Array<Object>>,
    ) -> Result<()> {
        jvm.put_field(&mut this, "java/util/logging/LogRecord", "parameters", "[Ljava/lang/Object;", parameters)
            .await
    }
}
