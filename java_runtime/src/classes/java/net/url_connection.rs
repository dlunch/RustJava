use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto, JavaResult};
use jvm::{ClassInstanceRef, Jvm};

use crate::{classes::java::net::URL, RuntimeClassProto, RuntimeContext};

// class java.net.URLConnection
pub struct URLConnection {}

impl URLConnection {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new("<init>", "(Ljava/net/URL;)V", Self::init, Default::default())],
            fields: vec![JavaFieldProto::new("url", "Ljava/net/URL;", Default::default())],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, url: ClassInstanceRef<URL>) -> JavaResult<()> {
        tracing::debug!("java.net.URL::<init>({:?}, {:?})", &this, &url,);

        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }
}
