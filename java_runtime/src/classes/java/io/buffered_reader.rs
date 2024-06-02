use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{classes::java::io::Reader, RuntimeClassProto, RuntimeContext};

// class java.io.BufferedReader
pub struct BufferedReader {}

impl BufferedReader {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/io/Reader"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new("<init>", "(Ljava/io/Reader;)V", Self::init, Default::default())],
            fields: vec![],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, r#in: ClassInstanceRef<Reader>) -> Result<()> {
        tracing::debug!("java.io.BufferedReader::<init>({:?}, {:?})", &this, &r#in);

        jvm.invoke_special(&this, "java/io/Reader", "<init>", "()V", ()).await?;

        Ok(())
    }
}
