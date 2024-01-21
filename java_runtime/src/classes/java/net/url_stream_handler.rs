use alloc::vec;

use java_class_proto::{JavaMethodProto, JavaResult};
use jvm::{ClassInstanceRef, Jvm};

use crate::{RuntimeClassProto, RuntimeContext};

// class java.net.URLStreamHandler
pub struct URLStreamHandler {}

impl URLStreamHandler {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new_abstract("openConnection", "(Ljava/net/URL;)Ljava/net/URLConnection;", Default::default()),
            ],
            fields: vec![],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> JavaResult<()> {
        tracing::debug!("java.net.URLStreamHandler::<init>({:?})", &this);

        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }
}
