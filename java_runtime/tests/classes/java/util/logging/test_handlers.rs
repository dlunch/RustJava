use alloc::{boxed::Box, collections::btree_map::BTreeMap, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use java_runtime::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::{ByteArrayOutputStream, OutputStream},
        lang::{String, Throwable},
        util::logging::{Filter, Formatter, Level, LogRecord, SimpleFormatter, StreamHandler},
    },
};
use jvm::{Array, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};
use jvm_rust::ClassDefinitionImpl;
use test_utils::{TestRuntime, create_test_jvm};

struct ConfigurableFilter;

impl ConfigurableFilter {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "ConfigurableLoggingFilter",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/util/logging/Filter"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Z)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "isLoggable",
                    "(Ljava/util/logging/LogRecord;)Z",
                    Self::is_loggable,
                    MethodAccessFlags::PUBLIC,
                ),
            ],
            fields: vec![JavaFieldProto::new("allowed", "Z", FieldAccessFlags::PRIVATE)],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, allowed: bool) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "allowed", "Z", allowed).await
    }

    async fn is_loggable(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, _: ClassInstanceRef<LogRecord>) -> Result<bool> {
        jvm.get_field(&this, "allowed", "Z").await
    }
}

struct FailingOutputStream;

impl FailingOutputStream {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "FailingLoggingOutputStream",
            parent_class: Some("java/io/OutputStream"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("write", "(I)V", Self::write, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/io/OutputStream", "<init>", "()V", ()).await
    }

    async fn write(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: i32) -> Result<()> {
        Err(jvm.exception("java/io/IOException", "write failed").await)
    }
}

async fn logging_jvm() -> Result<Jvm> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;
    jvm.register_class(
        Box::new(ClassDefinitionImpl::from_class_proto(
            ConfigurableFilter::as_proto(),
            Box::new(runtime.clone()) as Box<_>,
        )),
        None,
    )
    .await?;
    jvm.register_class(
        Box::new(ClassDefinitionImpl::from_class_proto(
            FailingOutputStream::as_proto(),
            Box::new(runtime) as Box<_>,
        )),
        None,
    )
    .await?;
    Ok(jvm)
}

#[tokio::test]
async fn formatter_substitutes_message_parameters() -> Result<()> {
    let jvm = logging_jvm().await?;
    let info: ClassInstanceRef<Level> = jvm
        .get_static_field("java/util/logging/Level", "INFO", "Ljava/util/logging/Level;")
        .await?;
    let message = JavaLangString::from_rust_string(&jvm, "hello {0} from {10}").await?;
    let record: ClassInstanceRef<LogRecord> = jvm
        .new_class(
            "java/util/logging/LogRecord",
            "(Ljava/util/logging/Level;Ljava/lang/String;)V",
            (info, message),
        )
        .await?
        .into();
    let mut parameters: ClassInstanceRef<Array<()>> = jvm.instantiate_array("Ljava/lang/Object;", 11).await?.into();
    jvm.store_array(&mut parameters, 0, [JavaLangString::from_rust_string(&jvm, "world").await?])
        .await?;
    jvm.store_array(&mut parameters, 10, [JavaLangString::from_rust_string(&jvm, "parameter ten").await?])
        .await?;
    let _: () = jvm
        .invoke_virtual(
            &record,
            "java/util/logging/LogRecord",
            "setParameters",
            "([Ljava/lang/Object;)V",
            (parameters,),
        )
        .await?;

    let formatter: ClassInstanceRef<SimpleFormatter> = jvm.new_class("java/util/logging/SimpleFormatter", "()V", ()).await?.into();
    let formatted: ClassInstanceRef<String> = jvm
        .invoke_virtual(
            &formatter,
            "java/util/logging/SimpleFormatter",
            "formatMessage",
            "(Ljava/util/logging/LogRecord;)Ljava/lang/String;",
            (record,),
        )
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &formatted).await?, "hello world from parameter ten");
    Ok(())
}

#[tokio::test]
async fn simple_formatter_includes_throwable_stack_trace() -> Result<()> {
    let jvm = logging_jvm().await?;
    let info: ClassInstanceRef<Level> = jvm
        .get_static_field("java/util/logging/Level", "INFO", "Ljava/util/logging/Level;")
        .await?;
    let record: ClassInstanceRef<LogRecord> = jvm
        .new_class(
            "java/util/logging/LogRecord",
            "(Ljava/util/logging/Level;Ljava/lang/String;)V",
            (info, JavaLangString::from_rust_string(&jvm, "failed").await?),
        )
        .await?
        .into();
    let mut thrown: ClassInstanceRef<Throwable> = jvm
        .new_class(
            "java/lang/RuntimeException",
            "(Ljava/lang/String;)V",
            (JavaLangString::from_rust_string(&jvm, "boom").await?,),
        )
        .await?
        .into();
    let mut stack_trace: ClassInstanceRef<Array<String>> = jvm.instantiate_array("Ljava/lang/String;", 1).await?.into();
    jvm.store_array(
        &mut stack_trace,
        0,
        [JavaLangString::from_rust_string(&jvm, "example.Test.run(Test.java:7)").await?],
    )
    .await?;
    jvm.put_field(&mut thrown, "stackTrace", "[Ljava/lang/String;", stack_trace).await?;
    let _: () = jvm
        .invoke_virtual(&record, "java/util/logging/LogRecord", "setThrown", "(Ljava/lang/Throwable;)V", (thrown,))
        .await?;

    let formatter: ClassInstanceRef<SimpleFormatter> = jvm.new_class("java/util/logging/SimpleFormatter", "()V", ()).await?.into();
    let formatted: ClassInstanceRef<String> = jvm
        .invoke_virtual(
            &formatter,
            "java/util/logging/SimpleFormatter",
            "format",
            "(Ljava/util/logging/LogRecord;)Ljava/lang/String;",
            (record,),
        )
        .await?;
    let formatted = JavaLangString::to_rust_string(&jvm, &formatted).await?;
    assert!(formatted.contains("java.lang.RuntimeException: boom"));
    assert!(formatted.contains("\tat example.Test.run(Test.java:7)"));
    Ok(())
}

#[tokio::test]
async fn stream_handler_reports_output_failures_without_propagating_them() -> Result<()> {
    let jvm = logging_jvm().await?;
    let output: ClassInstanceRef<OutputStream> = jvm.new_class("FailingLoggingOutputStream", "()V", ()).await?.into();
    let formatter: ClassInstanceRef<Formatter> = jvm.new_class("java/util/logging/SimpleFormatter", "()V", ()).await?.into();
    let handler: ClassInstanceRef<StreamHandler> = jvm
        .new_class(
            "java/util/logging/StreamHandler",
            "(Ljava/io/OutputStream;Ljava/util/logging/Formatter;)V",
            (output, formatter),
        )
        .await?
        .into();
    let info: ClassInstanceRef<Level> = jvm
        .get_static_field("java/util/logging/Level", "INFO", "Ljava/util/logging/Level;")
        .await?;
    let record: ClassInstanceRef<LogRecord> = jvm
        .new_class(
            "java/util/logging/LogRecord",
            "(Ljava/util/logging/Level;Ljava/lang/String;)V",
            (info, JavaLangString::from_rust_string(&jvm, "message").await?),
        )
        .await?
        .into();

    let _: () = jvm
        .invoke_virtual(
            &handler,
            "java/util/logging/StreamHandler",
            "publish",
            "(Ljava/util/logging/LogRecord;)V",
            (record,),
        )
        .await?;
    Ok(())
}

#[tokio::test]
async fn stream_handler_applies_level_and_custom_filter_before_writing() -> Result<()> {
    let jvm = logging_jvm().await?;
    let output: ClassInstanceRef<ByteArrayOutputStream> = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?.into();
    let formatter: ClassInstanceRef<Formatter> = jvm.new_class("java/util/logging/SimpleFormatter", "()V", ()).await?.into();
    let handler: ClassInstanceRef<StreamHandler> = jvm
        .new_class(
            "java/util/logging/StreamHandler",
            "(Ljava/io/OutputStream;Ljava/util/logging/Formatter;)V",
            (ClassInstanceRef::<OutputStream>::new(output.instance.clone()), formatter),
        )
        .await?
        .into();
    let warning: ClassInstanceRef<Level> = jvm
        .get_static_field("java/util/logging/Level", "WARNING", "Ljava/util/logging/Level;")
        .await?;
    let _: () = jvm
        .invoke_virtual(
            &handler,
            "java/util/logging/StreamHandler",
            "setLevel",
            "(Ljava/util/logging/Level;)V",
            (warning.clone(),),
        )
        .await?;

    let denied: ClassInstanceRef<Filter> = jvm.new_class("ConfigurableLoggingFilter", "(Z)V", (false,)).await?.into();
    let _: () = jvm
        .invoke_virtual(
            &handler,
            "java/util/logging/StreamHandler",
            "setFilter",
            "(Ljava/util/logging/Filter;)V",
            (denied,),
        )
        .await?;
    let denied_record: ClassInstanceRef<LogRecord> = jvm
        .new_class(
            "java/util/logging/LogRecord",
            "(Ljava/util/logging/Level;Ljava/lang/String;)V",
            (warning.clone(), JavaLangString::from_rust_string(&jvm, "denied").await?),
        )
        .await?
        .into();
    let _: () = jvm
        .invoke_virtual(
            &handler,
            "java/util/logging/StreamHandler",
            "publish",
            "(Ljava/util/logging/LogRecord;)V",
            (denied_record,),
        )
        .await?;

    let allowed: ClassInstanceRef<Filter> = jvm.new_class("ConfigurableLoggingFilter", "(Z)V", (true,)).await?.into();
    let _: () = jvm
        .invoke_virtual(
            &handler,
            "java/util/logging/StreamHandler",
            "setFilter",
            "(Ljava/util/logging/Filter;)V",
            (allowed,),
        )
        .await?;
    let allowed_record: ClassInstanceRef<LogRecord> = jvm
        .new_class(
            "java/util/logging/LogRecord",
            "(Ljava/util/logging/Level;Ljava/lang/String;)V",
            (warning, JavaLangString::from_rust_string(&jvm, "allowed").await?),
        )
        .await?
        .into();
    let _: () = jvm
        .invoke_virtual(
            &handler,
            "java/util/logging/StreamHandler",
            "publish",
            "(Ljava/util/logging/LogRecord;)V",
            (allowed_record,),
        )
        .await?;
    let _: () = jvm
        .invoke_virtual(&handler, "java/util/logging/StreamHandler", "flush", "()V", ())
        .await?;

    let text: ClassInstanceRef<String> = jvm
        .invoke_virtual(&output, "java/io/ByteArrayOutputStream", "toString", "()Ljava/lang/String;", ())
        .await?;
    let text = JavaLangString::to_rust_string(&jvm, &text).await?;
    assert!(!text.contains("denied"));
    assert!(text.contains("allowed"));
    Ok(())
}
