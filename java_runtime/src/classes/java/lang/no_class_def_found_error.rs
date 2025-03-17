use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

// class java.lang.NoClassDefFoundError
pub struct NoClassDefFoundError;

impl NoClassDefFoundError {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/NoClassDefFoundError",
            parent_class: Some("java/lang/LinkageError"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_with_message, Default::default()),
            ],
            fields: vec![],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.NoClassDefFoundError::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/lang/LinkageError", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn init_with_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.NoClassDefFoundError::<init>({:?}, {:?})", &this, &message);

        let _: () = jvm
            .invoke_special(&this, "java/lang/LinkageError", "<init>", "(Ljava/lang/String;)V", (message,))
            .await?;

        Ok(())
    }
}
