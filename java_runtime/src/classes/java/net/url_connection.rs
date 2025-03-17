use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{io::InputStream, net::URL},
};

// class java.net.URLConnection
pub struct URLConnection;

impl URLConnection {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/net/URLConnection",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/net/URL;)V", Self::init, Default::default()),
                JavaMethodProto::new("getInputStream", "()Ljava/io/InputStream;", Self::get_input_stream, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("url", "Ljava/net/URL;", Default::default())],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, url: ClassInstanceRef<URL>) -> Result<()> {
        tracing::debug!("java.net.URL::<init>({:?}, {:?})", &this, &url,);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn get_input_stream(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<InputStream>> {
        tracing::debug!("java.net.URL::getInputStream({:?})", &this);

        Err(jvm.exception("java/io/UnknownServiceException", "unsupported").await)
    }
}
