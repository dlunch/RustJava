use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{classes::java::lang::String, RuntimeClassProto, RuntimeContext};

// class java.lang.NoSuchFieldError
pub struct NoSuchFieldError;

impl NoSuchFieldError {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/NoSuchFieldError",
            parent_class: Some("java/lang/IncompatibleClassChangeError"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_with_message, Default::default()),
            ],
            fields: vec![],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.NoSuchFieldError::<init>({:?})", &this);

        let _: () = jvm
            .invoke_special(&this, "java/lang/IncompatibleClassChangeError", "<init>", "()V", ())
            .await?;

        Ok(())
    }

    async fn init_with_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.NoSuchFieldError::<init>({:?}, {:?})", &this, &message);

        let _: () = jvm
            .invoke_special(
                &this,
                "java/lang/IncompatibleClassChangeError",
                "<init>",
                "(Ljava/lang/String;)V",
                (message,),
            )
            .await?;

        Ok(())
    }
}
