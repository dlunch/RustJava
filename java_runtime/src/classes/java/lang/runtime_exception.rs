use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::lang::{String, Throwable},
};

// class java.lang.RuntimeException
pub struct RuntimeException;

impl RuntimeException {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/RuntimeException",
            parent_class: Some("java/lang/Exception"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_with_message, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/Throwable;)V", Self::init_with_cause, Default::default()),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/lang/String;Ljava/lang/Throwable;)V",
                    Self::init_with_message_and_cause,
                    Default::default(),
                ),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.RuntimeException::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/lang/Exception", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn init_with_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.RuntimeException::<init>({:?}, {:?})", &this, &message);

        let _: () = jvm
            .invoke_special(&this, "java/lang/Exception", "<init>", "(Ljava/lang/String;)V", (message,))
            .await?;

        Ok(())
    }

    async fn init_with_cause(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, cause: ClassInstanceRef<Throwable>) -> Result<()> {
        tracing::debug!("java.lang.RuntimeException::<init>({:?}, {:?})", &this, &cause);

        let _: () = jvm
            .invoke_special(&this, "java/lang/Exception", "<init>", "(Ljava/lang/Throwable;)V", (cause,))
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
        tracing::debug!("java.lang.RuntimeException::<init>({:?}, {:?}, {:?})", &this, &message, &cause);

        let _: () = jvm
            .invoke_special(
                &this,
                "java/lang/Exception",
                "<init>",
                "(Ljava/lang/String;Ljava/lang/Throwable;)V",
                (message, cause),
            )
            .await?;

        Ok(())
    }
}
