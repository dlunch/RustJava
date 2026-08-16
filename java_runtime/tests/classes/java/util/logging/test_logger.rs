use java_runtime::classes::java::{
    io::{ByteArrayOutputStream, OutputStream},
    lang::String,
    util::logging::{Formatter, Handler, Level, LogManager, Logger},
};
use jvm::{Array, ClassInstanceRef, JavaError, Result, runtime::JavaLangString};
use test_utils::test_jvm;

#[tokio::test]
async fn named_loggers_are_reused_and_reparented_when_ancestor_is_added_late() -> Result<()> {
    let jvm = test_jvm().await?;
    let child_name = JavaLangString::from_rust_string(&jvm, "app.service.worker").await?;
    let child: ClassInstanceRef<Logger> = jvm
        .invoke_static(
            "java/util/logging/Logger",
            "getLogger",
            "(Ljava/lang/String;)Ljava/util/logging/Logger;",
            (child_name.clone(),),
        )
        .await?;
    let same: ClassInstanceRef<Logger> = jvm
        .invoke_static(
            "java/util/logging/Logger",
            "getLogger",
            "(Ljava/lang/String;)Ljava/util/logging/Logger;",
            (child_name,),
        )
        .await?;
    assert_eq!(child.identity(), same.identity());

    let parent_name = JavaLangString::from_rust_string(&jvm, "app.service").await?;
    let parent: ClassInstanceRef<Logger> = jvm
        .invoke_static(
            "java/util/logging/Logger",
            "getLogger",
            "(Ljava/lang/String;)Ljava/util/logging/Logger;",
            (parent_name,),
        )
        .await?;
    let actual: ClassInstanceRef<Logger> = jvm
        .invoke_virtual(&child, "java/util/logging/Logger", "getParent", "()Ljava/util/logging/Logger;", ())
        .await?;
    assert_eq!(actual.identity(), parent.identity());
    Ok(())
}

#[tokio::test]
async fn existing_logger_preserves_and_validates_resource_bundle_name() -> Result<()> {
    let jvm = test_jvm().await?;
    let name = JavaLangString::from_rust_string(&jvm, "bundled.logger").await?;
    let logger: ClassInstanceRef<Logger> = jvm
        .invoke_static(
            "java/util/logging/Logger",
            "getLogger",
            "(Ljava/lang/String;)Ljava/util/logging/Logger;",
            (name.clone(),),
        )
        .await?;
    let bundle = JavaLangString::from_rust_string(&jvm, "messages").await?;
    let bundled: ClassInstanceRef<Logger> = jvm
        .invoke_static(
            "java/util/logging/Logger",
            "getLogger",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/util/logging/Logger;",
            (name.clone(), bundle.clone()),
        )
        .await?;
    assert_eq!(logger.identity(), bundled.identity());
    let actual: ClassInstanceRef<String> = jvm
        .invoke_virtual(&logger, "java/util/logging/Logger", "getResourceBundleName", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &actual).await?, "messages");

    let same: ClassInstanceRef<Logger> = jvm
        .invoke_static(
            "java/util/logging/Logger",
            "getLogger",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/util/logging/Logger;",
            (name.clone(), bundle),
        )
        .await?;
    assert_eq!(logger.identity(), same.identity());

    let result: Result<ClassInstanceRef<Logger>> = jvm
        .invoke_static(
            "java/util/logging/Logger",
            "getLogger",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/util/logging/Logger;",
            (name, JavaLangString::from_rust_string(&jvm, "other").await?),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("a different resource bundle name must throw IllegalArgumentException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalArgumentException"));
    Ok(())
}

#[tokio::test]
async fn logger_inherits_levels_and_publishes_convenience_methods() -> Result<()> {
    let jvm = test_jvm().await?;
    let logger: ClassInstanceRef<Logger> = jvm
        .invoke_static(
            "java/util/logging/Logger",
            "getLogger",
            "(Ljava/lang/String;)Ljava/util/logging/Logger;",
            (JavaLangString::from_rust_string(&jvm, "output.test").await?,),
        )
        .await?;
    let _: () = jvm
        .invoke_virtual(&logger, "java/util/logging/Logger", "setUseParentHandlers", "(Z)V", (false,))
        .await?;

    let output: ClassInstanceRef<ByteArrayOutputStream> = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?.into();
    let formatter: ClassInstanceRef<Formatter> = jvm.new_class("java/util/logging/SimpleFormatter", "()V", ()).await?.into();
    let handler: ClassInstanceRef<Handler> = jvm
        .new_class(
            "java/util/logging/StreamHandler",
            "(Ljava/io/OutputStream;Ljava/util/logging/Formatter;)V",
            (ClassInstanceRef::<OutputStream>::new(output.instance.clone()), formatter),
        )
        .await?
        .into();
    let _: () = jvm
        .invoke_virtual(
            &logger,
            "java/util/logging/Logger",
            "addHandler",
            "(Ljava/util/logging/Handler;)V",
            (handler.clone(),),
        )
        .await?;

    let warning: ClassInstanceRef<Level> = jvm
        .get_static_field("java/util/logging/Level", "WARNING", "Ljava/util/logging/Level;")
        .await?;
    let _: () = jvm
        .invoke_virtual(
            &logger,
            "java/util/logging/Logger",
            "setLevel",
            "(Ljava/util/logging/Level;)V",
            (warning,),
        )
        .await?;
    let _: () = jvm
        .invoke_virtual(
            &logger,
            "java/util/logging/Logger",
            "info",
            "(Ljava/lang/String;)V",
            (JavaLangString::from_rust_string(&jvm, "hidden").await?,),
        )
        .await?;
    let _: () = jvm
        .invoke_virtual(
            &logger,
            "java/util/logging/Logger",
            "warning",
            "(Ljava/lang/String;)V",
            (JavaLangString::from_rust_string(&jvm, "visible").await?,),
        )
        .await?;
    let _: () = jvm.invoke_virtual(&handler, "java/util/logging/Handler", "flush", "()V", ()).await?;

    let text: ClassInstanceRef<String> = jvm
        .invoke_virtual(&output, "java/io/ByteArrayOutputStream", "toString", "()Ljava/lang/String;", ())
        .await?;
    let text = JavaLangString::to_rust_string(&jvm, &text).await?;
    assert!(!text.contains("hidden"));
    assert!(text.contains("visible"));
    Ok(())
}

#[tokio::test]
async fn logger_publishes_to_parent_handlers_until_propagation_is_disabled() -> Result<()> {
    let jvm = test_jvm().await?;
    let parent: ClassInstanceRef<Logger> = jvm
        .invoke_static(
            "java/util/logging/Logger",
            "getLogger",
            "(Ljava/lang/String;)Ljava/util/logging/Logger;",
            (JavaLangString::from_rust_string(&jvm, "propagation").await?,),
        )
        .await?;
    let child: ClassInstanceRef<Logger> = jvm
        .invoke_static(
            "java/util/logging/Logger",
            "getLogger",
            "(Ljava/lang/String;)Ljava/util/logging/Logger;",
            (JavaLangString::from_rust_string(&jvm, "propagation.child").await?,),
        )
        .await?;
    let _: () = jvm
        .invoke_virtual(&parent, "java/util/logging/Logger", "setUseParentHandlers", "(Z)V", (false,))
        .await?;

    let output: ClassInstanceRef<ByteArrayOutputStream> = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?.into();
    let formatter: ClassInstanceRef<Formatter> = jvm.new_class("java/util/logging/SimpleFormatter", "()V", ()).await?.into();
    let handler: ClassInstanceRef<Handler> = jvm
        .new_class(
            "java/util/logging/StreamHandler",
            "(Ljava/io/OutputStream;Ljava/util/logging/Formatter;)V",
            (ClassInstanceRef::<OutputStream>::new(output.instance.clone()), formatter),
        )
        .await?
        .into();
    let _: () = jvm
        .invoke_virtual(
            &parent,
            "java/util/logging/Logger",
            "addHandler",
            "(Ljava/util/logging/Handler;)V",
            (handler.clone(),),
        )
        .await?;

    let _: () = jvm
        .invoke_virtual(
            &child,
            "java/util/logging/Logger",
            "info",
            "(Ljava/lang/String;)V",
            (JavaLangString::from_rust_string(&jvm, "from child").await?,),
        )
        .await?;
    let _: () = jvm.invoke_virtual(&handler, "java/util/logging/Handler", "flush", "()V", ()).await?;
    let text: ClassInstanceRef<String> = jvm
        .invoke_virtual(&output, "java/io/ByteArrayOutputStream", "toString", "()Ljava/lang/String;", ())
        .await?;
    assert!(JavaLangString::to_rust_string(&jvm, &text).await?.contains("from child"));

    let _: () = jvm
        .invoke_virtual(&child, "java/util/logging/Logger", "setUseParentHandlers", "(Z)V", (false,))
        .await?;
    let _: () = jvm
        .invoke_virtual(
            &child,
            "java/util/logging/Logger",
            "info",
            "(Ljava/lang/String;)V",
            (JavaLangString::from_rust_string(&jvm, "not propagated").await?,),
        )
        .await?;
    let _: () = jvm.invoke_virtual(&handler, "java/util/logging/Handler", "flush", "()V", ()).await?;
    let text: ClassInstanceRef<String> = jvm
        .invoke_virtual(&output, "java/io/ByteArrayOutputStream", "toString", "()Ljava/lang/String;", ())
        .await?;
    assert!(!JavaLangString::to_rust_string(&jvm, &text).await?.contains("not propagated"));
    Ok(())
}

#[tokio::test]
async fn log_manager_reset_removes_handlers_and_restores_levels() -> Result<()> {
    let jvm = test_jvm().await?;
    let manager: ClassInstanceRef<LogManager> = jvm
        .invoke_static("java/util/logging/LogManager", "getLogManager", "()Ljava/util/logging/LogManager;", ())
        .await?;
    let logger: ClassInstanceRef<Logger> = jvm
        .invoke_static(
            "java/util/logging/Logger",
            "getLogger",
            "(Ljava/lang/String;)Ljava/util/logging/Logger;",
            (JavaLangString::from_rust_string(&jvm, "reset.test").await?,),
        )
        .await?;
    let output: ClassInstanceRef<ByteArrayOutputStream> = jvm.new_class("java/io/ByteArrayOutputStream", "()V", ()).await?.into();
    let formatter: ClassInstanceRef<Formatter> = jvm.new_class("java/util/logging/SimpleFormatter", "()V", ()).await?.into();
    let handler: ClassInstanceRef<Handler> = jvm
        .new_class(
            "java/util/logging/StreamHandler",
            "(Ljava/io/OutputStream;Ljava/util/logging/Formatter;)V",
            (ClassInstanceRef::<OutputStream>::new(output.instance), formatter),
        )
        .await?
        .into();
    let severe: ClassInstanceRef<Level> = jvm
        .get_static_field("java/util/logging/Level", "SEVERE", "Ljava/util/logging/Level;")
        .await?;
    let _: () = jvm
        .invoke_virtual(
            &logger,
            "java/util/logging/Logger",
            "addHandler",
            "(Ljava/util/logging/Handler;)V",
            (handler,),
        )
        .await?;
    let _: () = jvm
        .invoke_virtual(&logger, "java/util/logging/Logger", "setLevel", "(Ljava/util/logging/Level;)V", (severe,))
        .await?;

    let _: () = jvm.invoke_virtual(&manager, "java/util/logging/LogManager", "reset", "()V", ()).await?;

    let handlers: ClassInstanceRef<Array<Handler>> = jvm
        .invoke_virtual(&logger, "java/util/logging/Logger", "getHandlers", "()[Ljava/util/logging/Handler;", ())
        .await?;
    assert_eq!(jvm.array_length(&handlers).await?, 0);
    let level: ClassInstanceRef<Level> = jvm
        .invoke_virtual(&logger, "java/util/logging/Logger", "getLevel", "()Ljava/util/logging/Level;", ())
        .await?;
    assert!(level.is_null());

    let root: ClassInstanceRef<Logger> = jvm
        .invoke_virtual(
            &manager,
            "java/util/logging/LogManager",
            "getLogger",
            "(Ljava/lang/String;)Ljava/util/logging/Logger;",
            (JavaLangString::from_rust_string(&jvm, "").await?,),
        )
        .await?;
    let root_level: ClassInstanceRef<Level> = jvm
        .invoke_virtual(&root, "java/util/logging/Logger", "getLevel", "()Ljava/util/logging/Level;", ())
        .await?;
    let value: i32 = jvm.invoke_virtual(&root_level, "java/util/logging/Level", "intValue", "()I", ()).await?;
    assert_eq!(value, 800);
    Ok(())
}

#[tokio::test]
async fn global_logger_and_manager_initialize_in_either_entry_order() -> Result<()> {
    let jvm = test_jvm().await?;
    let global: ClassInstanceRef<Logger> = jvm
        .get_static_field("java/util/logging/Logger", "global", "Ljava/util/logging/Logger;")
        .await?;
    let manager: ClassInstanceRef<LogManager> = jvm
        .invoke_static("java/util/logging/LogManager", "getLogManager", "()Ljava/util/logging/LogManager;", ())
        .await?;
    let name = JavaLangString::from_rust_string(&jvm, "global").await?;
    let registered: ClassInstanceRef<Logger> = jvm
        .invoke_virtual(
            &manager,
            "java/util/logging/LogManager",
            "getLogger",
            "(Ljava/lang/String;)Ljava/util/logging/Logger;",
            (name,),
        )
        .await?;
    assert_eq!(global.identity(), registered.identity());

    let jvm = test_jvm().await?;
    let manager: ClassInstanceRef<LogManager> = jvm
        .invoke_static("java/util/logging/LogManager", "getLogManager", "()Ljava/util/logging/LogManager;", ())
        .await?;
    let global: ClassInstanceRef<Logger> = jvm
        .get_static_field("java/util/logging/Logger", "global", "Ljava/util/logging/Logger;")
        .await?;
    let name = JavaLangString::from_rust_string(&jvm, "global").await?;
    let registered: ClassInstanceRef<Logger> = jvm
        .invoke_virtual(
            &manager,
            "java/util/logging/LogManager",
            "getLogger",
            "(Ljava/lang/String;)Ljava/util/logging/Logger;",
            (name,),
        )
        .await?;
    assert_eq!(global.identity(), registered.identity());
    Ok(())
}
