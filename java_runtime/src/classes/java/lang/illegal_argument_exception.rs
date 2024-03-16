use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{classes::java::lang::String, RuntimeClassProto, RuntimeContext};

// class java.lang.IllegalArgumentException
pub struct IllegalArgumentException {}

impl IllegalArgumentException {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/RuntimeException"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_with_message, Default::default()),
            ],
            fields: vec![],
        }
    }

    async fn init(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.IllegalArgumentException::<init>({:?})", &this);

        Ok(())
    }

    async fn init_with_message(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.IllegalArgumentException::<init>({:?}, {:?})", &this, &message);

        Ok(())
    }
}
