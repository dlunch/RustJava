use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{classes::java::net::URL, RuntimeClassProto, RuntimeContext};

// class rustjava.net.FileURLConnection
pub struct FileURLConnection {}

impl FileURLConnection {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/net/URLConnection"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new("<init>", "(Ljava/net/URL;)V", Self::init, Default::default())],
            fields: vec![],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, url: ClassInstanceRef<URL>) -> Result<()> {
        tracing::debug!("rustjava.net.FileURLConnection::<init>({:?}, {:?})", &this, &url);

        jvm.invoke_special(&this, "java/net/URLConnection", "<init>", "(Ljava/net/URL;)V", (url,))
            .await?;

        Ok(())
    }
}
