use alloc::{format, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{runtime::JavaLangString, ClassInstanceRef, Jvm, Result};

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
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_with_spec, Default::default()),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/net/URL;Ljava/lang/String;)V",
                    Self::init_with_context_spec,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/net/URL;Ljava/lang/String;Ljava/net/URLStreamHandler;)V",
                    Self::init_with_context_spec_handler,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/lang/String;Ljava/lang/String;ILjava/lang/String;Ljava/net/URLStreamHandler;)V",
                    Self::init_with_protocol_host_port_file_handler,
                    Default::default(),
                ),
                JavaMethodProto::new("openConnection", "()Ljava/net/URLConnection;", Self::open_connection, Default::default()),
                JavaMethodProto::new("openStream", "()Ljava/io/InputStream;", Self::open_stream, Default::default()),
                JavaMethodProto::new(
                    "set",
                    "(Ljava/lang/String;Ljava/lang/String;ILjava/lang/String;Ljava/lang/String;)V",
                    Self::set,
                    Default::default(),
                ),
                JavaMethodProto::new("getPort", "()I", Self::get_port, Default::default()),
                JavaMethodProto::new("getProtocol", "()Ljava/lang/String;", Self::get_protocol, Default::default()),
                JavaMethodProto::new("getHost", "()Ljava/lang/String;", Self::get_host, Default::default()),
                JavaMethodProto::new("getFile", "()Ljava/lang/String;", Self::get_file, Default::default()),
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

    async fn init_with_spec(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, spec: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.net.URL::<init>({:?}, {:?})", &this, &spec);

        jvm.invoke_special(&this, "java/net/URL", "<init>", "(Ljava/net/URL;Ljava/lang/String;)V", (None, spec))
            .await?;

        Ok(())
    }

    async fn init_with_context_spec(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        context: ClassInstanceRef<URL>,
        spec: ClassInstanceRef<String>,
    ) -> Result<()> {
        tracing::debug!("java.net.URL::<init>({:?}, {:?}, {:?})", &this, &context, &spec);

        jvm.invoke_special(
            &this,
            "java/net/URL",
            "<init>",
            "(Ljava/net/URL;Ljava/lang/String;Ljava/net/URLStreamHandler;)V",
            (context, spec, None),
        )
        .await?;

        Ok(())
    }

    async fn init_with_context_spec_handler(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        context: ClassInstanceRef<URL>,
        spec: ClassInstanceRef<String>,
        handler: ClassInstanceRef<URLStreamHandler>,
    ) -> Result<()> {
        tracing::debug!("java.net.URL::<init>({:?}, {:?}, {:?}, {:?})", &this, &context, &spec, &handler);

        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        let spec_str = JavaLangString::to_rust_string(jvm, &spec).await?;

        let protocol = spec_str.split(':').next().unwrap_or_default();

        let handler = if protocol == "file" {
            jvm.new_class("rustjava/net/FileURLHandler", "()V", ()).await?
        } else {
            return Err(jvm
                .exception("java/net/MalformedURLException", &format!("unknown protocol: {}", protocol))
                .await);
        };

        jvm.put_field(&mut this, "handler", "Ljava/net/URLStreamHandler;", handler.clone())
            .await?;

        jvm.invoke_virtual(
            &handler,
            "parseURL",
            "(Ljava/net/URL;Ljava/lang/String;II)V",
            (this, spec, 0, spec_str.len() as i32),
        )
        .await?;

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    async fn init_with_protocol_host_port_file_handler(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        protocol: ClassInstanceRef<String>,
        host: ClassInstanceRef<String>,
        port: i32,
        file: ClassInstanceRef<String>,
        handler: ClassInstanceRef<URLStreamHandler>,
    ) -> Result<()> {
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

        jvm.put_field(&mut this, "handler", "Ljava/net/URLStreamHandler;", handler).await?;

        jvm.invoke_virtual(
            &this,
            "set",
            "(Ljava/lang/String;Ljava/lang/String;ILjava/lang/String;Ljava/lang/String;)V",
            (protocol, host, port, file, None),
        )
        .await?;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    async fn set(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        protocol: ClassInstanceRef<String>,
        host: ClassInstanceRef<String>,
        port: i32,
        file: ClassInstanceRef<String>,
        r#ref: ClassInstanceRef<String>,
    ) -> Result<()> {
        tracing::debug!(
            "java.net.URL::set({:?}, {:?}, {:?}, {:?}, {:?}, {:?})",
            &this,
            &protocol,
            &host,
            &port,
            &file,
            &r#ref
        );

        jvm.put_field(&mut this, "protocol", "Ljava/lang/String;", protocol).await?;
        jvm.put_field(&mut this, "host", "Ljava/lang/String;", host).await?;
        jvm.put_field(&mut this, "port", "I", port).await?;
        jvm.put_field(&mut this, "file", "Ljava/lang/String;", file).await?;

        Ok(())
    }

    async fn open_connection(jvm: &Jvm, _runtime: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<URLConnection>> {
        tracing::debug!("java.net.URL::openConnection({:?})", &this);

        let handler = jvm.get_field(&this, "handler", "Ljava/net/URLStreamHandler;").await?;
        let connection = jvm
            .invoke_virtual(&handler, "openConnection", "(Ljava/net/URL;)Ljava/net/URLConnection;", (this,))
            .await?;

        Ok(connection)
    }

    async fn open_stream(jvm: &Jvm, _runtime: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<InputStream>> {
        tracing::debug!("java.net.URL::openStream({:?})", &this);

        let connection = jvm.invoke_virtual(&this, "openConnection", "()Ljava/net/URLConnection;", ()).await?;

        let stream = jvm.invoke_virtual(&connection, "getInputStream", "()Ljava/io/InputStream;", ()).await?;

        Ok(stream)
    }

    async fn get_port(jvm: &Jvm, _runtime: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.net.URL::getPort({:?})", &this);

        let port = jvm.get_field(&this, "port", "I").await?;

        Ok(port)
    }

    async fn get_protocol(jvm: &Jvm, _runtime: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.net.URL::getProtocol({:?})", &this);

        let protocol = jvm.get_field(&this, "protocol", "Ljava/lang/String;").await?;

        Ok(protocol)
    }

    async fn get_host(jvm: &Jvm, _runtime: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.net.URL::getHost({:?})", &this);

        let host = jvm.get_field(&this, "host", "Ljava/lang/String;").await?;

        Ok(host)
    }

    async fn get_file(jvm: &Jvm, _runtime: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.net.URL::getFile({:?})", &this);

        let file = jvm.get_field(&this, "file", "Ljava/lang/String;").await?;

        Ok(file)
    }
}

#[cfg(test)]
mod test {
    use jvm::{runtime::JavaLangString, Result};

    use crate::test::test_jvm;

    #[futures_test::test]
    async fn test_file_url() -> Result<()> {
        let jvm = test_jvm().await?;

        let url_spec = JavaLangString::from_rust_string(&jvm, "file:test.txt").await?;
        let url = jvm.new_class("java/net/URL", "(Ljava/lang/String;)V", (url_spec,)).await?;

        let protocol = jvm.invoke_virtual(&url, "getProtocol", "()Ljava/lang/String;", ()).await?;
        let host = jvm.invoke_virtual(&url, "getHost", "()Ljava/lang/String;", ()).await?;
        let port: i32 = jvm.invoke_virtual(&url, "getPort", "()I", ()).await?;
        let file = jvm.invoke_virtual(&url, "getFile", "()Ljava/lang/String;", ()).await?;

        assert_eq!(JavaLangString::to_rust_string(&jvm, &protocol).await?, "file");
        assert_eq!(port, -1);
        assert_eq!(JavaLangString::to_rust_string(&jvm, &host).await?, "");
        assert_eq!(JavaLangString::to_rust_string(&jvm, &file).await?, "test.txt");

        Ok(())
    }
}
