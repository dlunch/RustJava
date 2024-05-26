use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{classes::java::lang::String, RuntimeClassProto, RuntimeContext};

// class java.io.File
pub struct File {}

impl File {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init, Default::default())],
            fields: vec![JavaFieldProto::new("path", "Ljava/lang/String;", Default::default())],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, pathname: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.io.File::<init>({:?}, {:?})", &this, &pathname);

        jvm.put_field(&mut this, "path", "Ljava/lang/String;", pathname).await?;

        Ok(())
    }
}
