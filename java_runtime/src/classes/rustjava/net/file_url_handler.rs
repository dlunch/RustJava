use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

// class rustjava.net.FileURLHandler
pub struct FileURLHandler {}

impl FileURLHandler {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/net/URLStreamHandler"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new("<init>", "()V", Self::init, Default::default())],
            fields: vec![],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("rustjava.net.FileURLHandler::<init>({:?})", &this);

        jvm.invoke_special(&this, "java/net/URLStreamHandler", "<init>", "()V", ()).await?;

        Ok(())
    }
}
