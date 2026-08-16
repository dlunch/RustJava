use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::{ClassAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::lang::{String, Throwable},
};

// class java.lang.Exception
pub struct Exception;

impl Exception {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Exception",
            parent_class: Some("java/lang/Throwable"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_with_message, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(Ljava/lang/Throwable;)V", Self::init_with_cause, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/lang/String;Ljava/lang/Throwable;)V",
                    Self::init_with_message_and_cause,
                    MethodAccessFlags::PUBLIC,
                ),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.Exception::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Throwable", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn init_with_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.Exception::<init>({this:?}, {message:?})");

        let _: () = jvm
            .invoke_special(&this, "java/lang/Throwable", "<init>", "(Ljava/lang/String;)V", (message,))
            .await?;

        Ok(())
    }

    async fn init_with_cause(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, cause: ClassInstanceRef<Throwable>) -> Result<()> {
        tracing::debug!("java.lang.Exception::<init>({this:?}, {cause:?})");

        let _: () = jvm
            .invoke_special(&this, "java/lang/Throwable", "<init>", "(Ljava/lang/Throwable;)V", (cause,))
            .await?;

        Ok(())
    }

    async fn init_with_message_and_cause(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        message: ClassInstanceRef<String>,
        cause: ClassInstanceRef<Throwable>,
    ) -> Result<()> {
        tracing::debug!("java.lang.Exception::<init>({this:?}, {message:?}, {cause:?})");

        let _: () = jvm
            .invoke_special(
                &this,
                "java/lang/Throwable",
                "<init>",
                "(Ljava/lang/String;Ljava/lang/Throwable;)V",
                (message, cause),
            )
            .await?;

        Ok(())
    }
}
