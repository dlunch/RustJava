use alloc::{vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        lang::{Object, String, Throwable},
        util::{
            Vector,
            logging::{Filter, Handler, Level, LogManager, LogRecord},
        },
    },
};

// public class java.util.logging.Logger
pub struct Logger;

impl Logger {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/logging/Logger",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/lang/String;Ljava/lang/String;)V",
                    Self::init,
                    MethodAccessFlags::PROTECTED,
                ),
                JavaMethodProto::new(
                    "getLogger",
                    "(Ljava/lang/String;)Ljava/util/logging/Logger;",
                    Self::get_logger,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "getLogger",
                    "(Ljava/lang/String;Ljava/lang/String;)Ljava/util/logging/Logger;",
                    Self::get_logger_with_resource_bundle,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "configureResourceBundle",
                    "(Ljava/lang/String;)V",
                    Self::configure_resource_bundle,
                    MethodAccessFlags::PRIVATE | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "getAnonymousLogger",
                    "()Ljava/util/logging/Logger;",
                    Self::get_anonymous_logger,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "getAnonymousLogger",
                    "(Ljava/lang/String;)Ljava/util/logging/Logger;",
                    Self::get_anonymous_logger_with_resource_bundle,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "getGlobal",
                    "()Ljava/util/logging/Logger;",
                    Self::get_global,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC | MethodAccessFlags::FINAL,
                ),
                JavaMethodProto::new("getName", "()Ljava/lang/String;", Self::get_name, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getFilter", "()Ljava/util/logging/Filter;", Self::get_filter, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "setFilter",
                    "(Ljava/util/logging/Filter;)V",
                    Self::set_filter,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new("getLevel", "()Ljava/util/logging/Level;", Self::get_level, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "setLevel",
                    "(Ljava/util/logging/Level;)V",
                    Self::set_level,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new("getParent", "()Ljava/util/logging/Logger;", Self::get_parent, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "setParent",
                    "(Ljava/util/logging/Logger;)V",
                    Self::set_parent,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new("getUseParentHandlers", "()Z", Self::get_use_parent_handlers, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "setUseParentHandlers",
                    "(Z)V",
                    Self::set_use_parent_handlers,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "addHandler",
                    "(Ljava/util/logging/Handler;)V",
                    Self::add_handler,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "removeHandler",
                    "(Ljava/util/logging/Handler;)V",
                    Self::remove_handler,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "getHandlers",
                    "()[Ljava/util/logging/Handler;",
                    Self::get_handlers,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new("isLoggable", "(Ljava/util/logging/Level;)Z", Self::is_loggable, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("log", "(Ljava/util/logging/LogRecord;)V", Self::log_record, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "log",
                    "(Ljava/util/logging/Level;Ljava/lang/String;)V",
                    Self::log,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "log",
                    "(Ljava/util/logging/Level;Ljava/lang/String;Ljava/lang/Object;)V",
                    Self::log_with_parameter,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "log",
                    "(Ljava/util/logging/Level;Ljava/lang/String;[Ljava/lang/Object;)V",
                    Self::log_with_parameters,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "log",
                    "(Ljava/util/logging/Level;Ljava/lang/String;Ljava/lang/Throwable;)V",
                    Self::log_with_throwable,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "logp",
                    "(Ljava/util/logging/Level;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
                    Self::logp,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "logp",
                    "(Ljava/util/logging/Level;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/Object;)V",
                    Self::logp_with_parameter,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "logp",
                    "(Ljava/util/logging/Level;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;[Ljava/lang/Object;)V",
                    Self::logp_with_parameters,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "logp",
                    "(Ljava/util/logging/Level;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/Throwable;)V",
                    Self::logp_with_throwable,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "logrb",
                    "(Ljava/util/logging/Level;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
                    Self::logrb,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "logrb",
                    "(Ljava/util/logging/Level;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/Object;)V",
                    Self::logrb_with_parameter,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "logrb",
                    "(Ljava/util/logging/Level;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;[Ljava/lang/Object;)V",
                    Self::logrb_with_parameters,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "logrb",
                    "(Ljava/util/logging/Level;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/Throwable;)V",
                    Self::logrb_with_throwable,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("severe", "(Ljava/lang/String;)V", Self::severe, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("warning", "(Ljava/lang/String;)V", Self::warning, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("info", "(Ljava/lang/String;)V", Self::info, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("config", "(Ljava/lang/String;)V", Self::config, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("fine", "(Ljava/lang/String;)V", Self::fine, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("finer", "(Ljava/lang/String;)V", Self::finer, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("finest", "(Ljava/lang/String;)V", Self::finest, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "entering",
                    "(Ljava/lang/String;Ljava/lang/String;)V",
                    Self::entering,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "entering",
                    "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/Object;)V",
                    Self::entering_with_parameter,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "entering",
                    "(Ljava/lang/String;Ljava/lang/String;[Ljava/lang/Object;)V",
                    Self::entering_with_parameters,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "exiting",
                    "(Ljava/lang/String;Ljava/lang/String;)V",
                    Self::exiting,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "exiting",
                    "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/Object;)V",
                    Self::exiting_with_result,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "throwing",
                    "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/Throwable;)V",
                    Self::throwing,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "getResourceBundleName",
                    "()Ljava/lang/String;",
                    Self::get_resource_bundle_name,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "getResourceBundle",
                    "()Ljava/util/ResourceBundle;",
                    Self::get_resource_bundle,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("reset", "(Z)[Ljava/util/logging/Handler;", Self::reset, MethodAccessFlags::SYNCHRONIZED),
            ],
            fields: vec![
                JavaFieldProto::new(
                    "global",
                    "Ljava/util/logging/Logger;",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "GLOBAL_LOGGER_NAME",
                    "Ljava/lang/String;",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new("name", "Ljava/lang/String;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("level", "Ljava/util/logging/Level;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("filter", "Ljava/util/logging/Filter;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("handlers", "Ljava/util/Vector;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("parent", "Ljava/util/logging/Logger;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("useParentHandlers", "Z", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("resourceBundleName", "Ljava/lang/String;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("resourceBundle", "Ljava/util/ResourceBundle;", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        tracing::debug!("java.util.logging.Logger::<clinit>()");

        let name = JavaLangString::from_rust_string(jvm, "global").await?;
        jvm.put_static_field("java/util/logging/Logger", "GLOBAL_LOGGER_NAME", "Ljava/lang/String;", name.clone())
            .await?;
        let resource_bundle_name: ClassInstanceRef<String> = None.into();
        let global = jvm
            .new_class(
                "java/util/logging/Logger",
                "(Ljava/lang/String;Ljava/lang/String;)V",
                (name.clone(), resource_bundle_name),
            )
            .await?;
        jvm.put_static_field("java/util/logging/Logger", "global", "Ljava/util/logging/Logger;", global)
            .await
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
        resource_bundle_name: ClassInstanceRef<String>,
    ) -> Result<()> {
        tracing::debug!("java.util.logging.Logger::<init>({this:?}, {name:?}, {resource_bundle_name:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        let handlers = jvm.new_class("java/util/Vector", "()V", ()).await?;
        jvm.put_field(&mut this, "name", "Ljava/lang/String;", name).await?;
        jvm.put_field(&mut this, "handlers", "Ljava/util/Vector;", handlers).await?;
        jvm.put_field(&mut this, "useParentHandlers", "Z", true).await?;
        jvm.put_field(&mut this, "resourceBundleName", "Ljava/lang/String;", resource_bundle_name)
            .await
    }

    async fn get_logger(jvm: &Jvm, _: &mut RuntimeContext, name: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.util.logging.Logger::getLogger({name:?})");

        let resource_bundle_name: ClassInstanceRef<String> = None.into();
        jvm.invoke_static(
            "java/util/logging/Logger",
            "getLogger",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/util/logging/Logger;",
            (name, resource_bundle_name),
        )
        .await
    }

    async fn get_logger_with_resource_bundle(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        name: ClassInstanceRef<String>,
        resource_bundle_name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.util.logging.Logger::getLogger({name:?}, {resource_bundle_name:?})");

        if name.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "name").await);
        }

        let manager: ClassInstanceRef<LogManager> = jvm
            .invoke_static("java/util/logging/LogManager", "getLogManager", "()Ljava/util/logging/LogManager;", ())
            .await?;
        let mut logger: ClassInstanceRef<Self> = jvm
            .invoke_virtual(
                &manager,
                "java/util/logging/LogManager",
                "getLogger",
                "(Ljava/lang/String;)Ljava/util/logging/Logger;",
                (name.clone(),),
            )
            .await?;
        if logger.is_null() {
            let candidate: ClassInstanceRef<Self> = jvm
                .new_class(
                    "java/util/logging/Logger",
                    "(Ljava/lang/String;Ljava/lang/String;)V",
                    (name.clone(), resource_bundle_name.clone()),
                )
                .await?
                .into();
            let added: bool = jvm
                .invoke_virtual(
                    &manager,
                    "java/util/logging/LogManager",
                    "addLogger",
                    "(Ljava/util/logging/Logger;)Z",
                    (candidate.clone(),),
                )
                .await?;
            logger = if added {
                candidate
            } else {
                jvm.invoke_virtual(
                    &manager,
                    "java/util/logging/LogManager",
                    "getLogger",
                    "(Ljava/lang/String;)Ljava/util/logging/Logger;",
                    (name,),
                )
                .await?
            };
        }

        let _: () = jvm
            .invoke_special(
                &logger,
                "java/util/logging/Logger",
                "configureResourceBundle",
                "(Ljava/lang/String;)V",
                (resource_bundle_name,),
            )
            .await?;
        Ok(logger)
    }

    async fn configure_resource_bundle(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        resource_bundle_name: ClassInstanceRef<String>,
    ) -> Result<()> {
        if resource_bundle_name.is_null() {
            return Ok(());
        }

        let current: ClassInstanceRef<String> = jvm.get_field(&this, "resourceBundleName", "Ljava/lang/String;").await?;
        if current.is_null() {
            return jvm
                .put_field(&mut this, "resourceBundleName", "Ljava/lang/String;", resource_bundle_name)
                .await;
        }
        if JavaLangString::to_rust_string(jvm, &current).await? != JavaLangString::to_rust_string(jvm, &resource_bundle_name).await? {
            return Err(jvm
                .exception("java/lang/IllegalArgumentException", "logger already uses a different resource bundle")
                .await);
        }
        Ok(())
    }

    async fn get_anonymous_logger(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.util.logging.Logger::getAnonymousLogger()");

        let resource_bundle_name: ClassInstanceRef<String> = None.into();
        jvm.invoke_static(
            "java/util/logging/Logger",
            "getAnonymousLogger",
            "(Ljava/lang/String;)Ljava/util/logging/Logger;",
            (resource_bundle_name,),
        )
        .await
    }

    async fn get_anonymous_logger_with_resource_bundle(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        resource_bundle_name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.util.logging.Logger::getAnonymousLogger({resource_bundle_name:?})");

        let name: ClassInstanceRef<String> = None.into();
        let mut logger: ClassInstanceRef<Self> = jvm
            .new_class(
                "java/util/logging/Logger",
                "(Ljava/lang/String;Ljava/lang/String;)V",
                (name, resource_bundle_name),
            )
            .await?
            .into();
        let manager: ClassInstanceRef<LogManager> = jvm
            .invoke_static("java/util/logging/LogManager", "getLogManager", "()Ljava/util/logging/LogManager;", ())
            .await?;
        let root_name = JavaLangString::from_rust_string(jvm, "").await?;
        let root: ClassInstanceRef<Self> = jvm
            .invoke_virtual(
                &manager,
                "java/util/logging/LogManager",
                "getLogger",
                "(Ljava/lang/String;)Ljava/util/logging/Logger;",
                (root_name,),
            )
            .await?;
        jvm.put_field(&mut logger, "parent", "Ljava/util/logging/Logger;", root).await?;
        Ok(logger)
    }

    async fn get_global(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.util.logging.Logger::getGlobal()");
        jvm.get_static_field("java/util/logging/Logger", "global", "Ljava/util/logging/Logger;")
            .await
    }

    async fn get_name(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.logging.Logger::getName({this:?})");
        jvm.get_field(&this, "name", "Ljava/lang/String;").await
    }

    async fn get_filter(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Filter>> {
        tracing::debug!("java.util.logging.Logger::getFilter({this:?})");
        jvm.get_field(&this, "filter", "Ljava/util/logging/Filter;").await
    }

    async fn set_filter(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, filter: ClassInstanceRef<Filter>) -> Result<()> {
        tracing::debug!("java.util.logging.Logger::setFilter({this:?}, {filter:?})");
        jvm.put_field(&mut this, "filter", "Ljava/util/logging/Filter;", filter).await
    }

    async fn get_level(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Level>> {
        tracing::debug!("java.util.logging.Logger::getLevel({this:?})");
        jvm.get_field(&this, "level", "Ljava/util/logging/Level;").await
    }

    async fn set_level(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, level: ClassInstanceRef<Level>) -> Result<()> {
        tracing::debug!("java.util.logging.Logger::setLevel({this:?}, {level:?})");
        jvm.put_field(&mut this, "level", "Ljava/util/logging/Level;", level).await
    }

    async fn get_parent(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.util.logging.Logger::getParent({this:?})");
        jvm.get_field(&this, "parent", "Ljava/util/logging/Logger;").await
    }

    async fn set_parent(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, parent: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.logging.Logger::setParent({this:?}, {parent:?})");
        if parent.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "parent").await);
        }
        jvm.put_field(&mut this, "parent", "Ljava/util/logging/Logger;", parent).await
    }

    async fn get_use_parent_handlers(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.logging.Logger::getUseParentHandlers({this:?})");
        jvm.get_field(&this, "useParentHandlers", "Z").await
    }

    async fn set_use_parent_handlers(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: bool) -> Result<()> {
        tracing::debug!("java.util.logging.Logger::setUseParentHandlers({this:?}, {value})");
        jvm.put_field(&mut this, "useParentHandlers", "Z", value).await
    }

    async fn add_handler(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, handler: ClassInstanceRef<Handler>) -> Result<()> {
        tracing::debug!("java.util.logging.Logger::addHandler({this:?}, {handler:?})");
        if handler.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "handler").await);
        }
        let handlers: ClassInstanceRef<Vector> = jvm.get_field(&this, "handlers", "Ljava/util/Vector;").await?;
        jvm.invoke_virtual(&handlers, "java/util/Vector", "addElement", "(Ljava/lang/Object;)V", (handler,))
            .await
    }

    async fn remove_handler(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, handler: ClassInstanceRef<Handler>) -> Result<()> {
        tracing::debug!("java.util.logging.Logger::removeHandler({this:?}, {handler:?})");
        if handler.is_null() {
            return Ok(());
        }
        let handlers: ClassInstanceRef<Vector> = jvm.get_field(&this, "handlers", "Ljava/util/Vector;").await?;
        let _: bool = jvm
            .invoke_virtual(&handlers, "java/util/Vector", "removeElement", "(Ljava/lang/Object;)Z", (handler,))
            .await?;
        Ok(())
    }

    async fn get_handlers(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Array<Handler>>> {
        tracing::debug!("java.util.logging.Logger::getHandlers({this:?})");
        let handlers: ClassInstanceRef<Vector> = jvm.get_field(&this, "handlers", "Ljava/util/Vector;").await?;
        let size: i32 = jvm.invoke_virtual(&handlers, "java/util/Vector", "size", "()I", ()).await?;
        let mut result: ClassInstanceRef<Array<Handler>> = jvm.instantiate_array("Ljava/util/logging/Handler;", size as usize).await?.into();
        for index in 0..size {
            let handler: ClassInstanceRef<Handler> = jvm
                .invoke_virtual(&handlers, "java/util/Vector", "elementAt", "(I)Ljava/lang/Object;", (index,))
                .await?;
            jvm.store_array(&mut result, index as usize, [handler]).await?;
        }
        Ok(result)
    }

    async fn is_loggable(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, level: ClassInstanceRef<Level>) -> Result<bool> {
        tracing::debug!("java.util.logging.Logger::isLoggable({this:?}, {level:?})");
        if level.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "level").await);
        }

        let requested: i32 = jvm.invoke_virtual(&level, "java/util/logging/Level", "intValue", "()I", ()).await?;
        let mut logger = this;
        let effective = loop {
            let configured: ClassInstanceRef<Level> = jvm.get_field(&logger, "level", "Ljava/util/logging/Level;").await?;
            if !configured.is_null() {
                break jvm.invoke_virtual(&configured, "java/util/logging/Level", "intValue", "()I", ()).await?;
            }
            let parent: ClassInstanceRef<Self> = jvm.get_field(&logger, "parent", "Ljava/util/logging/Logger;").await?;
            if parent.is_null() {
                let info: ClassInstanceRef<Level> = jvm
                    .get_static_field("java/util/logging/Level", "INFO", "Ljava/util/logging/Level;")
                    .await?;
                break jvm.invoke_virtual(&info, "java/util/logging/Level", "intValue", "()I", ()).await?;
            }
            logger = parent;
        };

        Ok(effective != i32::MAX && requested >= effective)
    }

    async fn log_record(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, mut record: ClassInstanceRef<LogRecord>) -> Result<()> {
        tracing::debug!("java.util.logging.Logger::log({this:?}, {record:?})");
        if record.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "record").await);
        }
        let level: ClassInstanceRef<Level> = jvm
            .invoke_virtual(&record, "java/util/logging/LogRecord", "getLevel", "()Ljava/util/logging/Level;", ())
            .await?;
        if !jvm
            .invoke_virtual::<_, bool>(&this, "java/util/logging/Logger", "isLoggable", "(Ljava/util/logging/Level;)Z", (level,))
            .await?
        {
            return Ok(());
        }

        let filter: ClassInstanceRef<Filter> = jvm.get_field(&this, "filter", "Ljava/util/logging/Filter;").await?;
        if !filter.is_null()
            && !jvm
                .invoke_virtual::<_, bool>(
                    &filter,
                    &filter.class_definition().name(),
                    "isLoggable",
                    "(Ljava/util/logging/LogRecord;)Z",
                    (record.clone(),),
                )
                .await?
        {
            return Ok(());
        }

        let logger_name: ClassInstanceRef<String> = jvm
            .invoke_virtual(&record, "java/util/logging/LogRecord", "getLoggerName", "()Ljava/lang/String;", ())
            .await?;
        if logger_name.is_null() {
            let name: ClassInstanceRef<String> = jvm.get_field(&this, "name", "Ljava/lang/String;").await?;
            jvm.put_field(&mut record, "loggerName", "Ljava/lang/String;", name).await?;
        }

        let mut logger = this;
        loop {
            let handlers: ClassInstanceRef<Array<Handler>> = jvm
                .invoke_virtual(&logger, "java/util/logging/Logger", "getHandlers", "()[Ljava/util/logging/Handler;", ())
                .await?;
            let length = jvm.array_length(&handlers).await?;
            let handlers: Vec<ClassInstanceRef<Handler>> = jvm.load_array(&handlers, 0, length).await?;
            for handler in handlers {
                let _: () = jvm
                    .invoke_virtual(
                        &handler,
                        "java/util/logging/Handler",
                        "publish",
                        "(Ljava/util/logging/LogRecord;)V",
                        (record.clone(),),
                    )
                    .await?;
            }

            let use_parent: bool = jvm.get_field(&logger, "useParentHandlers", "Z").await?;
            if !use_parent {
                break;
            }
            let parent: ClassInstanceRef<Self> = jvm.get_field(&logger, "parent", "Ljava/util/logging/Logger;").await?;
            if parent.is_null() {
                break;
            }
            logger = parent;
        }
        Ok(())
    }

    async fn log(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        level: ClassInstanceRef<Level>,
        message: ClassInstanceRef<String>,
    ) -> Result<()> {
        let parameters: ClassInstanceRef<Array<Object>> = None.into();
        let thrown: ClassInstanceRef<Throwable> = None.into();
        let source_class: ClassInstanceRef<String> = None.into();
        let source_method: ClassInstanceRef<String> = None.into();
        let resource_bundle_name: ClassInstanceRef<String> = None.into();
        Self::log_values(
            jvm,
            this,
            level,
            message,
            parameters,
            thrown,
            source_class,
            source_method,
            resource_bundle_name,
        )
        .await
    }

    async fn log_with_parameter(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        level: ClassInstanceRef<Level>,
        message: ClassInstanceRef<String>,
        parameter: ClassInstanceRef<Object>,
    ) -> Result<()> {
        let mut parameters: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 1).await?.into();
        jvm.store_array(&mut parameters, 0, [parameter]).await?;
        let thrown: ClassInstanceRef<Throwable> = None.into();
        let source_class: ClassInstanceRef<String> = None.into();
        let source_method: ClassInstanceRef<String> = None.into();
        let resource_bundle_name: ClassInstanceRef<String> = None.into();
        Self::log_values(
            jvm,
            this,
            level,
            message,
            parameters,
            thrown,
            source_class,
            source_method,
            resource_bundle_name,
        )
        .await
    }

    async fn log_with_parameters(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        level: ClassInstanceRef<Level>,
        message: ClassInstanceRef<String>,
        parameters: ClassInstanceRef<Array<Object>>,
    ) -> Result<()> {
        let thrown: ClassInstanceRef<Throwable> = None.into();
        let source_class: ClassInstanceRef<String> = None.into();
        let source_method: ClassInstanceRef<String> = None.into();
        let resource_bundle_name: ClassInstanceRef<String> = None.into();
        Self::log_values(
            jvm,
            this,
            level,
            message,
            parameters,
            thrown,
            source_class,
            source_method,
            resource_bundle_name,
        )
        .await
    }

    async fn log_with_throwable(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        level: ClassInstanceRef<Level>,
        message: ClassInstanceRef<String>,
        thrown: ClassInstanceRef<Throwable>,
    ) -> Result<()> {
        let parameters: ClassInstanceRef<Array<Object>> = None.into();
        let source_class: ClassInstanceRef<String> = None.into();
        let source_method: ClassInstanceRef<String> = None.into();
        let resource_bundle_name: ClassInstanceRef<String> = None.into();
        Self::log_values(
            jvm,
            this,
            level,
            message,
            parameters,
            thrown,
            source_class,
            source_method,
            resource_bundle_name,
        )
        .await
    }

    async fn logp(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        level: ClassInstanceRef<Level>,
        source_class: ClassInstanceRef<String>,
        source_method: ClassInstanceRef<String>,
        message: ClassInstanceRef<String>,
    ) -> Result<()> {
        let parameters: ClassInstanceRef<Array<Object>> = None.into();
        let thrown: ClassInstanceRef<Throwable> = None.into();
        let resource_bundle_name: ClassInstanceRef<String> = None.into();
        Self::log_values(
            jvm,
            this,
            level,
            message,
            parameters,
            thrown,
            source_class,
            source_method,
            resource_bundle_name,
        )
        .await
    }

    #[allow(clippy::too_many_arguments)]
    async fn logp_with_parameter(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        level: ClassInstanceRef<Level>,
        source_class: ClassInstanceRef<String>,
        source_method: ClassInstanceRef<String>,
        message: ClassInstanceRef<String>,
        parameter: ClassInstanceRef<Object>,
    ) -> Result<()> {
        let mut parameters: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 1).await?.into();
        jvm.store_array(&mut parameters, 0, [parameter]).await?;
        let thrown: ClassInstanceRef<Throwable> = None.into();
        let resource_bundle_name: ClassInstanceRef<String> = None.into();
        Self::log_values(
            jvm,
            this,
            level,
            message,
            parameters,
            thrown,
            source_class,
            source_method,
            resource_bundle_name,
        )
        .await
    }

    #[allow(clippy::too_many_arguments)]
    async fn logp_with_parameters(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        level: ClassInstanceRef<Level>,
        source_class: ClassInstanceRef<String>,
        source_method: ClassInstanceRef<String>,
        message: ClassInstanceRef<String>,
        parameters: ClassInstanceRef<Array<Object>>,
    ) -> Result<()> {
        let thrown: ClassInstanceRef<Throwable> = None.into();
        let resource_bundle_name: ClassInstanceRef<String> = None.into();
        Self::log_values(
            jvm,
            this,
            level,
            message,
            parameters,
            thrown,
            source_class,
            source_method,
            resource_bundle_name,
        )
        .await
    }

    #[allow(clippy::too_many_arguments)]
    async fn logp_with_throwable(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        level: ClassInstanceRef<Level>,
        source_class: ClassInstanceRef<String>,
        source_method: ClassInstanceRef<String>,
        message: ClassInstanceRef<String>,
        thrown: ClassInstanceRef<Throwable>,
    ) -> Result<()> {
        let parameters: ClassInstanceRef<Array<Object>> = None.into();
        let resource_bundle_name: ClassInstanceRef<String> = None.into();
        Self::log_values(
            jvm,
            this,
            level,
            message,
            parameters,
            thrown,
            source_class,
            source_method,
            resource_bundle_name,
        )
        .await
    }

    #[allow(clippy::too_many_arguments)]
    async fn logrb(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        level: ClassInstanceRef<Level>,
        source_class: ClassInstanceRef<String>,
        source_method: ClassInstanceRef<String>,
        resource_bundle_name: ClassInstanceRef<String>,
        message: ClassInstanceRef<String>,
    ) -> Result<()> {
        let parameters: ClassInstanceRef<Array<Object>> = None.into();
        let thrown: ClassInstanceRef<Throwable> = None.into();
        Self::log_values(
            jvm,
            this,
            level,
            message,
            parameters,
            thrown,
            source_class,
            source_method,
            resource_bundle_name,
        )
        .await
    }

    #[allow(clippy::too_many_arguments)]
    async fn logrb_with_parameter(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        level: ClassInstanceRef<Level>,
        source_class: ClassInstanceRef<String>,
        source_method: ClassInstanceRef<String>,
        resource_bundle_name: ClassInstanceRef<String>,
        message: ClassInstanceRef<String>,
        parameter: ClassInstanceRef<Object>,
    ) -> Result<()> {
        let mut parameters: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 1).await?.into();
        jvm.store_array(&mut parameters, 0, [parameter]).await?;
        let thrown: ClassInstanceRef<Throwable> = None.into();
        Self::log_values(
            jvm,
            this,
            level,
            message,
            parameters,
            thrown,
            source_class,
            source_method,
            resource_bundle_name,
        )
        .await
    }

    #[allow(clippy::too_many_arguments)]
    async fn logrb_with_parameters(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        level: ClassInstanceRef<Level>,
        source_class: ClassInstanceRef<String>,
        source_method: ClassInstanceRef<String>,
        resource_bundle_name: ClassInstanceRef<String>,
        message: ClassInstanceRef<String>,
        parameters: ClassInstanceRef<Array<Object>>,
    ) -> Result<()> {
        let thrown: ClassInstanceRef<Throwable> = None.into();
        Self::log_values(
            jvm,
            this,
            level,
            message,
            parameters,
            thrown,
            source_class,
            source_method,
            resource_bundle_name,
        )
        .await
    }

    #[allow(clippy::too_many_arguments)]
    async fn logrb_with_throwable(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        level: ClassInstanceRef<Level>,
        source_class: ClassInstanceRef<String>,
        source_method: ClassInstanceRef<String>,
        resource_bundle_name: ClassInstanceRef<String>,
        message: ClassInstanceRef<String>,
        thrown: ClassInstanceRef<Throwable>,
    ) -> Result<()> {
        let parameters: ClassInstanceRef<Array<Object>> = None.into();
        Self::log_values(
            jvm,
            this,
            level,
            message,
            parameters,
            thrown,
            source_class,
            source_method,
            resource_bundle_name,
        )
        .await
    }

    async fn severe(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        Self::log_at(jvm, this, "SEVERE", message).await
    }

    async fn warning(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        Self::log_at(jvm, this, "WARNING", message).await
    }

    async fn info(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        Self::log_at(jvm, this, "INFO", message).await
    }

    async fn config(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        Self::log_at(jvm, this, "CONFIG", message).await
    }

    async fn fine(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        Self::log_at(jvm, this, "FINE", message).await
    }

    async fn finer(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        Self::log_at(jvm, this, "FINER", message).await
    }

    async fn finest(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        Self::log_at(jvm, this, "FINEST", message).await
    }

    async fn entering(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        source_class: ClassInstanceRef<String>,
        source_method: ClassInstanceRef<String>,
    ) -> Result<()> {
        let level = jvm
            .get_static_field("java/util/logging/Level", "FINER", "Ljava/util/logging/Level;")
            .await?;
        let message: ClassInstanceRef<String> = JavaLangString::from_rust_string(jvm, "ENTRY").await?.into();
        let parameters: ClassInstanceRef<Array<Object>> = None.into();
        let thrown: ClassInstanceRef<Throwable> = None.into();
        let resource_bundle_name: ClassInstanceRef<String> = None.into();
        Self::log_values(
            jvm,
            this,
            level,
            message,
            parameters,
            thrown,
            source_class,
            source_method,
            resource_bundle_name,
        )
        .await
    }

    async fn entering_with_parameter(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        source_class: ClassInstanceRef<String>,
        source_method: ClassInstanceRef<String>,
        parameter: ClassInstanceRef<Object>,
    ) -> Result<()> {
        let level = jvm
            .get_static_field("java/util/logging/Level", "FINER", "Ljava/util/logging/Level;")
            .await?;
        let message: ClassInstanceRef<String> = JavaLangString::from_rust_string(jvm, "ENTRY {0}").await?.into();
        let mut parameters: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 1).await?.into();
        jvm.store_array(&mut parameters, 0, [parameter]).await?;
        let thrown: ClassInstanceRef<Throwable> = None.into();
        let resource_bundle_name: ClassInstanceRef<String> = None.into();
        Self::log_values(
            jvm,
            this,
            level,
            message,
            parameters,
            thrown,
            source_class,
            source_method,
            resource_bundle_name,
        )
        .await
    }

    async fn entering_with_parameters(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        source_class: ClassInstanceRef<String>,
        source_method: ClassInstanceRef<String>,
        parameters: ClassInstanceRef<Array<Object>>,
    ) -> Result<()> {
        let level = jvm
            .get_static_field("java/util/logging/Level", "FINER", "Ljava/util/logging/Level;")
            .await?;
        let message: ClassInstanceRef<String> = JavaLangString::from_rust_string(jvm, "ENTRY").await?.into();
        let thrown: ClassInstanceRef<Throwable> = None.into();
        let resource_bundle_name: ClassInstanceRef<String> = None.into();
        Self::log_values(
            jvm,
            this,
            level,
            message,
            parameters,
            thrown,
            source_class,
            source_method,
            resource_bundle_name,
        )
        .await
    }

    async fn exiting(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        source_class: ClassInstanceRef<String>,
        source_method: ClassInstanceRef<String>,
    ) -> Result<()> {
        let level = jvm
            .get_static_field("java/util/logging/Level", "FINER", "Ljava/util/logging/Level;")
            .await?;
        let message: ClassInstanceRef<String> = JavaLangString::from_rust_string(jvm, "RETURN").await?.into();
        let parameters: ClassInstanceRef<Array<Object>> = None.into();
        let thrown: ClassInstanceRef<Throwable> = None.into();
        let resource_bundle_name: ClassInstanceRef<String> = None.into();
        Self::log_values(
            jvm,
            this,
            level,
            message,
            parameters,
            thrown,
            source_class,
            source_method,
            resource_bundle_name,
        )
        .await
    }

    async fn exiting_with_result(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        source_class: ClassInstanceRef<String>,
        source_method: ClassInstanceRef<String>,
        result: ClassInstanceRef<Object>,
    ) -> Result<()> {
        let level = jvm
            .get_static_field("java/util/logging/Level", "FINER", "Ljava/util/logging/Level;")
            .await?;
        let message: ClassInstanceRef<String> = JavaLangString::from_rust_string(jvm, "RETURN {0}").await?.into();
        let mut parameters: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 1).await?.into();
        jvm.store_array(&mut parameters, 0, [result]).await?;
        let thrown: ClassInstanceRef<Throwable> = None.into();
        let resource_bundle_name: ClassInstanceRef<String> = None.into();
        Self::log_values(
            jvm,
            this,
            level,
            message,
            parameters,
            thrown,
            source_class,
            source_method,
            resource_bundle_name,
        )
        .await
    }

    async fn throwing(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        source_class: ClassInstanceRef<String>,
        source_method: ClassInstanceRef<String>,
        thrown: ClassInstanceRef<Throwable>,
    ) -> Result<()> {
        let level = jvm
            .get_static_field("java/util/logging/Level", "FINER", "Ljava/util/logging/Level;")
            .await?;
        let message: ClassInstanceRef<String> = JavaLangString::from_rust_string(jvm, "THROW").await?.into();
        let parameters: ClassInstanceRef<Array<Object>> = None.into();
        let resource_bundle_name: ClassInstanceRef<String> = None.into();
        Self::log_values(
            jvm,
            this,
            level,
            message,
            parameters,
            thrown,
            source_class,
            source_method,
            resource_bundle_name,
        )
        .await
    }

    async fn get_resource_bundle_name(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.logging.Logger::getResourceBundleName({this:?})");
        jvm.get_field(&this, "resourceBundleName", "Ljava/lang/String;").await
    }

    async fn get_resource_bundle(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.logging.Logger::getResourceBundle({this:?})");
        jvm.get_field(&this, "resourceBundle", "Ljava/util/ResourceBundle;").await
    }

    async fn reset(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, root: bool) -> Result<ClassInstanceRef<Array<Handler>>> {
        let handlers: ClassInstanceRef<Vector> = jvm.get_field(&this, "handlers", "Ljava/util/Vector;").await?;
        let size: i32 = jvm.invoke_virtual(&handlers, "java/util/Vector", "size", "()I", ()).await?;
        let mut removed: ClassInstanceRef<Array<Handler>> = jvm.instantiate_array("Ljava/util/logging/Handler;", size as usize).await?.into();
        for index in 0..size {
            let handler: ClassInstanceRef<Handler> = jvm
                .invoke_virtual(&handlers, "java/util/Vector", "elementAt", "(I)Ljava/lang/Object;", (index,))
                .await?;
            jvm.store_array(&mut removed, index as usize, [handler]).await?;
        }
        let _: () = jvm.invoke_virtual(&handlers, "java/util/Vector", "removeAllElements", "()V", ()).await?;

        let level: ClassInstanceRef<Level> = if root {
            jvm.get_static_field("java/util/logging/Level", "INFO", "Ljava/util/logging/Level;")
                .await?
        } else {
            None.into()
        };
        jvm.put_field(&mut this, "level", "Ljava/util/logging/Level;", level).await?;
        Ok(removed)
    }

    async fn log_at(jvm: &Jvm, this: ClassInstanceRef<Self>, field: &str, message: ClassInstanceRef<String>) -> Result<()> {
        let level: ClassInstanceRef<Level> = jvm
            .get_static_field("java/util/logging/Level", field, "Ljava/util/logging/Level;")
            .await?;
        jvm.invoke_virtual(
            &this,
            "java/util/logging/Logger",
            "log",
            "(Ljava/util/logging/Level;Ljava/lang/String;)V",
            (level, message),
        )
        .await
    }

    #[allow(clippy::too_many_arguments)]
    async fn log_values(
        jvm: &Jvm,
        this: ClassInstanceRef<Self>,
        level: ClassInstanceRef<Level>,
        message: ClassInstanceRef<String>,
        parameters: ClassInstanceRef<Array<Object>>,
        thrown: ClassInstanceRef<Throwable>,
        source_class: ClassInstanceRef<String>,
        source_method: ClassInstanceRef<String>,
        resource_bundle_name: ClassInstanceRef<String>,
    ) -> Result<()> {
        if !jvm
            .invoke_virtual::<_, bool>(
                &this,
                "java/util/logging/Logger",
                "isLoggable",
                "(Ljava/util/logging/Level;)Z",
                (level.clone(),),
            )
            .await?
        {
            return Ok(());
        }

        let mut record: ClassInstanceRef<LogRecord> = jvm
            .new_class(
                "java/util/logging/LogRecord",
                "(Ljava/util/logging/Level;Ljava/lang/String;)V",
                (level, message),
            )
            .await?
            .into();
        if !parameters.is_null() {
            jvm.put_field(&mut record, "parameters", "[Ljava/lang/Object;", parameters).await?;
        }
        if !thrown.is_null() {
            jvm.put_field(&mut record, "thrown", "Ljava/lang/Throwable;", thrown).await?;
        }
        if !source_class.is_null() {
            jvm.put_field(&mut record, "sourceClassName", "Ljava/lang/String;", source_class).await?;
        }
        if !source_method.is_null() {
            jvm.put_field(&mut record, "sourceMethodName", "Ljava/lang/String;", source_method)
                .await?;
        }
        if !resource_bundle_name.is_null() {
            jvm.put_field(&mut record, "resourceBundleName", "Ljava/lang/String;", resource_bundle_name)
                .await?;
        }
        jvm.invoke_virtual(&this, "java/util/logging/Logger", "log", "(Ljava/util/logging/LogRecord;)V", (record,))
            .await
    }
}
