use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{io::InputStream, net::URL},
};

// class java.net.URLConnection
#[java_class(name = "java/net/URLConnection", extends = "java/lang/Object", public, abstract)]
pub struct URLConnection {
    #[field(protected)]
    url: ClassInstanceRef<URL>,
}

#[java_methods]
impl URLConnection {
    #[method(constructor, protected)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, url: ClassInstanceRef<URL>) -> Result<()> {
        tracing::debug!("java.net.URLConnection::<init>({this:?}, {url:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    #[method(public)]
    async fn get_input_stream(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<InputStream>> {
        tracing::debug!("java.net.URLConnection::getInputStream({this:?})");

        Err(jvm.exception("java/net/UnknownServiceException", "unsupported").await)
    }
}
