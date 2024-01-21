use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto, JavaResult};
use jvm::{ClassInstanceRef, Jvm};

use crate::{
    classes::java::{
        io::InputStream,
        lang::String,
        net::{URLConnection, URLStreamHandler},
    },
    RuntimeClassProto, RuntimeContext,
};

// class java.net.URL
pub struct URL {}

impl URL {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/lang/String;Ljava/lang/String;ILjava/lang/String;Ljava/net/URLStreamHandler;)V",
                    Self::init,
                    Default::default(),
                ),
                JavaMethodProto::new("openStream", "()Ljava/io/InputStream;", Self::open_stream, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("protocol", "Ljava/lang/String;", Default::default()),
                JavaFieldProto::new("host", "Ljava/lang/String;", Default::default()),
                JavaFieldProto::new("port", "I", Default::default()),
                JavaFieldProto::new("file", "Ljava/lang/String;", Default::default()),
                JavaFieldProto::new("handler", "Ljava/net/URLStreamHandler;", Default::default()),
            ],
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        protocol: ClassInstanceRef<String>,
        host: ClassInstanceRef<String>,
        port: i32,
        file: ClassInstanceRef<String>,
        handler: ClassInstanceRef<URLStreamHandler>,
    ) -> JavaResult<()> {
        tracing::debug!(
            "java.net.URL::<init>({:?}, {:?}, {:?}, {:?}, {:?}, {:?})",
            &this,
            &protocol,
            &host,
            &port,
            &file,
            &handler
        );

        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "protocol", "Ljava/lang/String;", protocol)?;
        jvm.put_field(&mut this, "host", "Ljava/lang/String;", host)?;
        jvm.put_field(&mut this, "port", "I", port)?;
        jvm.put_field(&mut this, "file", "Ljava/lang/String;", file)?;
        jvm.put_field(&mut this, "handler", "Ljava/net/URLStreamHandler;", handler)?;

        Ok(())
    }

    async fn open_stream(jvm: &Jvm, _runtime: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> JavaResult<ClassInstanceRef<InputStream>> {
        tracing::debug!("java.net.URL::openStream({:?})", &this);

        let handler: ClassInstanceRef<URLStreamHandler> = jvm.get_field(&this, "handler", "Ljava/net/URLStreamHandler;")?;
        let connection: ClassInstanceRef<URLConnection> = jvm
            .invoke_virtual(&handler, "openConnection", "(Ljava/net/URL;)Ljava/net/URLConnection;", (this,))
            .await?;

        let stream = jvm.invoke_virtual(&connection, "getInputStream", "()Ljava/io/InputStream;", ()).await?;

        Ok(stream)
    }
}