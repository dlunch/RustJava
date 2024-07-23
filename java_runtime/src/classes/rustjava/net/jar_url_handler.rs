use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{
    classes::java::net::{URLConnection, URL},
    RuntimeClassProto, RuntimeContext,
};

// class rustjava.net.JarURLHandler
pub struct JarURLHandler {}

impl JarURLHandler {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/net/URLStreamHandler"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "openConnection",
                    "(Ljava/net/URL;)Ljava/net/URLConnection;",
                    Self::open_connection,
                    Default::default(),
                ),
            ],
            fields: vec![],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("rustjava.net.JarURLHandler::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/net/URLStreamHandler", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn open_connection(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        url: ClassInstanceRef<URL>,
    ) -> Result<ClassInstanceRef<URLConnection>> {
        tracing::debug!("rustjava.net.JarURLHandler::openConnection({:?}, {:?})", &this, &url);

        let connection = jvm.new_class("rustjava/net/JarURLConnection", "(Ljava/net/URL;)V", (url,)).await?;

        Ok(connection.into())
    }
}
