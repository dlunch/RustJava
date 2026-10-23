use alloc::vec;

use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::JavaMethodProto;
use jvm_types::{ClassAccessFlags, MethodAccessFlags};

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
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::empty()),
                JavaMethodProto::new(
                    "openConnection",
                    "(Ljava/net/URL;)Ljava/net/URLConnection;",
                    Self::open_connection,
                    MethodAccessFlags::PROTECTED,
                ),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::empty(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("org.rustjava.net.FileURLHandler::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/net/URLStreamHandler", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn open_connection(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        url: ClassInstanceRef<URL>,
    ) -> Result<ClassInstanceRef<URLConnection>> {
        tracing::debug!("org.rustjava.net.FileURLHandler::openConnection({this:?}, {url:?})");

        let file: ClassInstanceRef<String> = jvm.invoke_virtual(&url, "java/net/URL", "getFile", "()Ljava/lang/String;", ()).await?;
        let file: ClassInstanceRef<String> = jvm
            .invoke_static("org/rustjava/net/URLPath", "decode", "(Ljava/lang/String;)Ljava/lang/String;", (file,))
            .await?;
        let file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (file,)).await?;

        let connection = jvm
            .new_class("org/rustjava/net/FileURLConnection", "(Ljava/net/URL;Ljava/io/File;)V", (url, file))
            .await?;

        Ok(connection.into())
    }
}
