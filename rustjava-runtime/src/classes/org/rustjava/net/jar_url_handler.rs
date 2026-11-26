use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::net::{URL, URLConnection},
};

// class rustjava.net.JarURLHandler
#[java_class(name = "org/rustjava/net/JarURLHandler", extends = "java/net/URLStreamHandler")]
pub struct JarURLHandler;

#[java_methods]
impl JarURLHandler {
    #[method(constructor)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("org.rustjava.net.JarURLHandler::<init>({this:?})");

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
        tracing::debug!("org.rustjava.net.JarURLHandler::openConnection({this:?}, {url:?})");

        let connection = jvm.new_class("org/rustjava/net/JarURLConnection", "(Ljava/net/URL;)V", (url,)).await?;

        Ok(connection.into())
    }
}
