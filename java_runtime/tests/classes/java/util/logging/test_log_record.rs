use java_runtime::classes::java::{
    lang::{Object, String},
    util::logging::{Level, LogRecord},
};
use jvm::{Array, ClassInstanceRef, JavaError, Result, runtime::JavaLangString};
use test_utils::test_jvm;

#[tokio::test]
async fn log_record_initializes_event_metadata_and_round_trips_properties() -> Result<()> {
    let jvm = test_jvm().await?;
    let info: ClassInstanceRef<Level> = jvm
        .get_static_field("java/util/logging/Level", "INFO", "Ljava/util/logging/Level;")
        .await?;
    let message = JavaLangString::from_rust_string(&jvm, "message").await?;
    let first: ClassInstanceRef<LogRecord> = jvm
        .new_class(
            "java/util/logging/LogRecord",
            "(Ljava/util/logging/Level;Ljava/lang/String;)V",
            (info.clone(), message),
        )
        .await?
        .into();
    let second: ClassInstanceRef<LogRecord> = jvm
        .new_class(
            "java/util/logging/LogRecord",
            "(Ljava/util/logging/Level;Ljava/lang/String;)V",
            (info, JavaLangString::from_rust_string(&jvm, "next").await?),
        )
        .await?
        .into();

    let first_sequence: i64 = jvm.invoke_virtual(&first, "getSequenceNumber", "()J", ()).await?;
    let second_sequence: i64 = jvm.invoke_virtual(&second, "getSequenceNumber", "()J", ()).await?;
    assert_eq!(second_sequence, first_sequence + 1);
    assert!(jvm.invoke_virtual::<_, i64>(&first, "getMillis", "()J", ()).await? > 0);

    let logger_name = JavaLangString::from_rust_string(&jvm, "app").await?;
    let source_class = JavaLangString::from_rust_string(&jvm, "App").await?;
    let source_method = JavaLangString::from_rust_string(&jvm, "run").await?;
    let mut parameters: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 1).await?.into();
    let parameter = JavaLangString::from_rust_string(&jvm, "value").await?;
    jvm.store_array(&mut parameters, 0, [parameter]).await?;
    let _: () = jvm
        .invoke_virtual(&first, "setLoggerName", "(Ljava/lang/String;)V", (logger_name,))
        .await?;
    let _: () = jvm
        .invoke_virtual(&first, "setSourceClassName", "(Ljava/lang/String;)V", (source_class,))
        .await?;
    let _: () = jvm
        .invoke_virtual(&first, "setSourceMethodName", "(Ljava/lang/String;)V", (source_method,))
        .await?;
    let _: () = jvm
        .invoke_virtual(&first, "setParameters", "([Ljava/lang/Object;)V", (parameters,))
        .await?;

    let actual: ClassInstanceRef<String> = jvm.invoke_virtual(&first, "getLoggerName", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &actual).await?, "app");
    let actual: ClassInstanceRef<String> = jvm.invoke_virtual(&first, "getSourceClassName", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &actual).await?, "App");
    let actual: ClassInstanceRef<String> = jvm.invoke_virtual(&first, "getSourceMethodName", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &actual).await?, "run");
    let actual: ClassInstanceRef<Array<Object>> = jvm.invoke_virtual(&first, "getParameters", "()[Ljava/lang/Object;", ()).await?;
    assert_eq!(jvm.array_length(&actual).await?, 1);

    Ok(())
}

#[tokio::test]
async fn log_record_rejects_null_level() -> Result<()> {
    let jvm = test_jvm().await?;
    let level: ClassInstanceRef<Level> = None.into();
    let message: ClassInstanceRef<String> = None.into();
    let result: Result<Box<dyn jvm::ClassInstance>> = jvm
        .new_class(
            "java/util/logging/LogRecord",
            "(Ljava/util/logging/Level;Ljava/lang/String;)V",
            (level, message),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("null level must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));
    Ok(())
}
