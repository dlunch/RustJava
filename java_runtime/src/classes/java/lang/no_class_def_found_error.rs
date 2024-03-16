use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{classes::java::lang::String, RuntimeClassProto, RuntimeContext};

// class java.lang.NoClassDefFoundError
pub struct NoClassDefFoundError {}

impl NoClassDefFoundError {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/LinkageError"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init, Default::default())],
            fields: vec![],
        }
    }

    async fn init(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.NoClassDefFoundError::<init>({:?}, {:?})", &this, &message);

        Ok(())
    }
}
