use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{
        lang::String,
        net::{URL, URLConnection},
    },
};

// class rustjava.net.FileURLHandler
#[java_class(name = "org/rustjava/net/FileURLHandler", extends = "java/net/URLStreamHandler")]
pub struct FileURLHandler;

#[java_methods]
impl FileURLHandler {
    #[method(constructor)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("org.rustjava.net.FileURLHandler::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/net/URLStreamHandler", "<init>", "()V", ()).await?;

        Ok(())
    }

    #[method(protected)]
    async fn open_connection(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        url: ClassInstanceRef<URL>,
    ) -> Result<ClassInstanceRef<URLConnection>> {
        tracing::debug!("org.rustjava.net.FileURLHandler::openConnection({this:?}, {url:?})");

        if url.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "url is null").await);
        }

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
