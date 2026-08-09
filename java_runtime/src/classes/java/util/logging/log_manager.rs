use alloc::{string::String as RustString, vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        lang::{Object, String},
        util::{
            Hashtable,
            logging::{ConsoleHandler, Handler, Level, Logger},
        },
    },
};

// public class java.util.logging.LogManager
pub struct LogManager;

impl LogManager {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/logging/LogManager",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PROTECTED),
                JavaMethodProto::new(
                    "getLogManager",
                    "()Ljava/util/logging/LogManager;",
                    Self::get_log_manager,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("addLogger", "(Ljava/util/logging/Logger;)Z", Self::add_logger, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "registerLogger",
                    "(Ljava/lang/String;Ljava/util/logging/Logger;)Z",
                    Self::register_logger,
                    MethodAccessFlags::PRIVATE | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "getLogger",
                    "(Ljava/lang/String;)Ljava/util/logging/Logger;",
                    Self::get_logger,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "getLoggerNames",
                    "()Ljava/util/Enumeration;",
                    Self::get_logger_names,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new("reset", "()V", Self::reset, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "snapshotLoggers",
                    "()[Ljava/util/logging/Logger;",
                    Self::snapshot_loggers,
                    MethodAccessFlags::PRIVATE | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new("checkAccess", "()V", Self::check_access, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new(
                    "manager",
                    "Ljava/util/logging/LogManager;",
                    FieldAccessFlags::PRIVATE | FieldAccessFlags::STATIC,
                ),
                JavaFieldProto::new("loggers", "Ljava/util/Hashtable;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("root", "Ljava/util/logging/Logger;", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        tracing::debug!("java.util.logging.LogManager::<clinit>()");
        let manager = jvm.new_class("java/util/logging/LogManager", "()V", ()).await?;
        jvm.put_static_field("java/util/logging/LogManager", "manager", "Ljava/util/logging/LogManager;", manager)
            .await
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.logging.LogManager::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        let loggers: ClassInstanceRef<Hashtable> = jvm.new_class("java/util/Hashtable", "()V", ()).await?.into();
        jvm.put_field(&mut this, "loggers", "Ljava/util/Hashtable;", loggers.clone()).await?;

        let root_name = JavaLangString::from_rust_string(jvm, "").await?;
        let resource_bundle_name: ClassInstanceRef<String> = None.into();
        let mut root: ClassInstanceRef<Logger> = jvm
            .new_class(
                "java/util/logging/Logger",
                "(Ljava/lang/String;Ljava/lang/String;)V",
                (root_name.clone(), resource_bundle_name),
            )
            .await?
            .into();
        let info: ClassInstanceRef<Level> = jvm
            .get_static_field("java/util/logging/Level", "INFO", "Ljava/util/logging/Level;")
            .await?;
        jvm.put_field(&mut root, "level", "Ljava/util/logging/Level;", info).await?;
        jvm.put_field(&mut root, "useParentHandlers", "Z", false).await?;
        let console: ClassInstanceRef<ConsoleHandler> = jvm.new_class("java/util/logging/ConsoleHandler", "()V", ()).await?.into();
        let _: () = jvm
            .invoke_virtual(&root, "addHandler", "(Ljava/util/logging/Handler;)V", (console,))
            .await?;
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &loggers,
                "put",
                "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                (root_name, root.clone()),
            )
            .await?;
        jvm.put_field(&mut this, "root", "Ljava/util/logging/Logger;", root.clone()).await?;

        let global_name: ClassInstanceRef<String> = jvm
            .get_static_field("java/util/logging/Logger", "GLOBAL_LOGGER_NAME", "Ljava/lang/String;")
            .await?;
        let mut global: ClassInstanceRef<Logger> = jvm
            .get_static_field("java/util/logging/Logger", "global", "Ljava/util/logging/Logger;")
            .await?;
        jvm.put_field(&mut global, "parent", "Ljava/util/logging/Logger;", root).await?;
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &loggers,
                "put",
                "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                (global_name, global),
            )
            .await?;
        Ok(())
    }

    async fn get_log_manager(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.util.logging.LogManager::getLogManager()");
        jvm.get_static_field("java/util/logging/LogManager", "manager", "Ljava/util/logging/LogManager;")
            .await
    }

    async fn add_logger(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, logger: ClassInstanceRef<Logger>) -> Result<bool> {
        tracing::debug!("java.util.logging.LogManager::addLogger({this:?}, {logger:?})");
        if logger.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "logger").await);
        }
        let name: ClassInstanceRef<String> = jvm.invoke_virtual(&logger, "getName", "()Ljava/lang/String;", ()).await?;
        if name.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "logger name").await);
        }

        jvm.invoke_special(
            &this,
            "java/util/logging/LogManager",
            "registerLogger",
            "(Ljava/lang/String;Ljava/util/logging/Logger;)Z",
            (name, logger),
        )
        .await
    }

    async fn register_logger(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
        logger: ClassInstanceRef<Logger>,
    ) -> Result<bool> {
        let loggers: ClassInstanceRef<Hashtable> = jvm.get_field(&this, "loggers", "Ljava/util/Hashtable;").await?;
        let existing: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&loggers, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (name.clone(),))
            .await?;
        if !existing.is_null() {
            return Ok(false);
        }
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &loggers,
                "put",
                "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                (name, logger),
            )
            .await?;
        Self::reconnect_parents(jvm, &this, &loggers).await?;
        Ok(true)
    }

    async fn get_logger(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Logger>> {
        tracing::debug!("java.util.logging.LogManager::getLogger({this:?}, {name:?})");
        if name.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "name").await);
        }
        let loggers: ClassInstanceRef<Hashtable> = jvm.get_field(&this, "loggers", "Ljava/util/Hashtable;").await?;
        jvm.invoke_virtual(&loggers, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (name,))
            .await
    }

    async fn get_logger_names(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.logging.LogManager::getLoggerNames({this:?})");
        let loggers: ClassInstanceRef<Hashtable> = jvm.get_field(&this, "loggers", "Ljava/util/Hashtable;").await?;
        jvm.invoke_virtual(&loggers, "keys", "()Ljava/util/Enumeration;", ()).await
    }

    async fn reset(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.logging.LogManager::reset({this:?})");
        let loggers: ClassInstanceRef<Array<Logger>> = jvm
            .invoke_special(
                &this,
                "java/util/logging/LogManager",
                "snapshotLoggers",
                "()[Ljava/util/logging/Logger;",
                (),
            )
            .await?;
        let root: ClassInstanceRef<Logger> = jvm.get_field(&this, "root", "Ljava/util/logging/Logger;").await?;
        let length = jvm.array_length(&loggers).await?;
        let loggers: Vec<ClassInstanceRef<Logger>> = jvm.load_array(&loggers, 0, length).await?;
        for logger in loggers {
            let handlers: ClassInstanceRef<Array<Handler>> = jvm
                .invoke_special(
                    &logger,
                    "java/util/logging/Logger",
                    "reset",
                    "(Z)[Ljava/util/logging/Handler;",
                    (logger.identity() == root.identity(),),
                )
                .await?;
            let length = jvm.array_length(&handlers).await?;
            let handlers: Vec<ClassInstanceRef<Handler>> = jvm.load_array(&handlers, 0, length).await?;
            for handler in handlers {
                let _: () = jvm.invoke_virtual(&handler, "close", "()V", ()).await?;
            }
        }
        Ok(())
    }

    async fn snapshot_loggers(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Array<Logger>>> {
        let loggers: ClassInstanceRef<Hashtable> = jvm.get_field(&this, "loggers", "Ljava/util/Hashtable;").await?;
        let values: ClassInstanceRef<Object> = jvm.invoke_virtual(&loggers, "elements", "()Ljava/util/Enumeration;", ()).await?;
        let mut snapshot: Vec<ClassInstanceRef<Logger>> = Vec::new();
        while jvm.invoke_virtual::<_, bool>(&values, "hasMoreElements", "()Z", ()).await? {
            snapshot.push(jvm.invoke_virtual(&values, "nextElement", "()Ljava/lang/Object;", ()).await?);
        }

        let mut result: ClassInstanceRef<Array<Logger>> = jvm.instantiate_array("Ljava/util/logging/Logger;", snapshot.len()).await?.into();
        for (index, logger) in snapshot.into_iter().enumerate() {
            jvm.store_array(&mut result, index, [logger]).await?;
        }
        Ok(result)
    }

    async fn check_access(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.logging.LogManager::checkAccess({this:?})");
        Ok(())
    }

    async fn reconnect_parents(jvm: &Jvm, manager: &ClassInstanceRef<Self>, loggers: &ClassInstanceRef<Hashtable>) -> Result<()> {
        let names: ClassInstanceRef<Object> = jvm.invoke_virtual(loggers, "keys", "()Ljava/util/Enumeration;", ()).await?;
        let mut keys = Vec::new();
        while jvm.invoke_virtual::<_, bool>(&names, "hasMoreElements", "()Z", ()).await? {
            keys.push(jvm.invoke_virtual(&names, "nextElement", "()Ljava/lang/Object;", ()).await?);
        }

        let root: ClassInstanceRef<Logger> = jvm.get_field(manager, "root", "Ljava/util/logging/Logger;").await?;
        for key in keys {
            let name: RustString = JavaLangString::to_rust_string(jvm, &key).await?;
            if name.is_empty() {
                continue;
            }
            let mut logger: ClassInstanceRef<Logger> = jvm
                .invoke_virtual(loggers, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (key,))
                .await?;
            let mut parent = root.clone();
            let mut prefix = name.as_str();
            while let Some(index) = prefix.rfind('.') {
                prefix = &prefix[..index];
                let candidate_name = JavaLangString::from_rust_string(jvm, prefix).await?;
                let candidate: ClassInstanceRef<Logger> = jvm
                    .invoke_virtual(loggers, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (candidate_name,))
                    .await?;
                if !candidate.is_null() {
                    parent = candidate;
                    break;
                }
            }
            jvm.put_field(&mut logger, "parent", "Ljava/util/logging/Logger;", parent).await?;
        }
        Ok(())
    }
}
