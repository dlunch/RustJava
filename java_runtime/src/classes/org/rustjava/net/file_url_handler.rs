use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        lang::String,
        net::{URL, URLConnection},
    },
};

// class rustjava.net.FileURLHandler
pub struct FileURLHandler;

impl FileURLHandler {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "org/rustjava/net/FileURLHandler",
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
        tracing::debug!("org.rustjava.net.FileURLHandler::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/net/URLStreamHandler", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn open_connection(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        url: ClassInstanceRef<URL>,
    ) -> Result<ClassInstanceRef<URLConnection>> {
        tracing::debug!("org.rustjava.net.FileURLHandler::openConnection({:?}, {:?})", &this, &url);

        let file: ClassInstanceRef<String> = jvm.invoke_virtual(&url, "getFile", "()Ljava/lang/String;", ()).await?;
        let file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (file,)).await?;

        let connection = jvm
            .new_class("org/rustjava/net/FileURLConnection", "(Ljava/net/URL;Ljava/io/File;)V", (url, file))
            .await?;

        Ok(connection.into())
    }
}
