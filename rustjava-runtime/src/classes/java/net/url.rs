use alloc::{boxed::Box, format};

use jvm::{ClassInstance, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{
        io::InputStream,
        lang::String,
        net::{URLConnection, URLStreamHandler},
    },
};

// class java.net.URL
#[java_class(name = "java/net/URL", implements("java/io/Serializable"), public, final)]
pub struct URL {
    #[field(private)]
    protocol: ClassInstanceRef<String>,
    #[field(private)]
    host: ClassInstanceRef<String>,
    #[field(private)]
    port: i32,
    #[field(private)]
    file: ClassInstanceRef<String>,
    #[field(transient)]
    handler: ClassInstanceRef<URLStreamHandler>,
}

#[java_methods]
impl URL {
    #[method(constructor, public)]
    async fn init_with_spec(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, spec: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.net.URL::<init>({this:?}, {spec:?})");

        let _: () = jvm
            .invoke_special(&this, "java/net/URL", "<init>", "(Ljava/net/URL;Ljava/lang/String;)V", (None, spec))
            .await?;

        Ok(())
    }

    #[method(constructor, public)]
    async fn init_with_context_spec(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        context: ClassInstanceRef<URL>,
        spec: ClassInstanceRef<String>,
    ) -> Result<()> {
        tracing::debug!("java.net.URL::<init>({this:?}, {context:?}, {spec:?})");

        let _: () = jvm
            .invoke_special(
                &this,
                "java/net/URL",
                "<init>",
                "(Ljava/net/URL;Ljava/lang/String;Ljava/net/URLStreamHandler;)V",
                (context, spec, None),
            )
            .await?;

        Ok(())
    }

    #[method(constructor, public)]
    async fn init_with_context_spec_handler(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        context: ClassInstanceRef<URL>,
        spec: ClassInstanceRef<String>,
        handler: ClassInstanceRef<URLStreamHandler>,
    ) -> Result<()> {
        tracing::debug!("java.net.URL::<init>({this:?}, {context:?}, {spec:?}, {handler:?})");

        if spec.is_null() {
            return Err(jvm.exception("java/net/MalformedURLException", "spec is null").await);
        }

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        let spec_str = JavaLangString::to_rust_string(jvm, &spec).await?;

        let protocol = spec_str.split(':').next().unwrap_or_default();

        let handler = Self::get_handler(jvm, protocol).await?;

        jvm.put_field(&mut this, "java/net/URL", "handler", "Ljava/net/URLStreamHandler;", handler.clone())
            .await?;

        let _: () = jvm
            .invoke_virtual(
                &handler,
                "java/net/URLStreamHandler",
                "parseURL",
                "(Ljava/net/URL;Ljava/lang/String;II)V",
                (this, spec, 0, spec_str.len() as i32),
            )
            .await?;

        Ok(())
    }

    #[method(constructor, public)]
    async fn init_with_protocol_host_file(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        protocol: ClassInstanceRef<String>,
        host: ClassInstanceRef<String>,
        file: ClassInstanceRef<String>,
    ) -> Result<()> {
        tracing::debug!("java.net.URL::<init>({this:?}, {protocol:?}, {host:?}, {file:?})");

        let _: () = jvm
            .invoke_special(
                &this,
                "java/net/URL",
                "<init>",
                "(Ljava/lang/String;Ljava/lang/String;ILjava/lang/String;Ljava/net/URLStreamHandler;)V",
                (protocol, host, -1, file, None),
            )
            .await?;

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    #[method(constructor, public)]
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
        tracing::debug!("java.net.URL::<init>({this:?}, {protocol:?}, {host:?}, {port:?}, {file:?}, {handler:?})");

        if protocol.is_null() || file.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "protocol or file is null").await);
        }

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "java/net/URL", "handler", "Ljava/net/URLStreamHandler;", handler.clone())
            .await?;

        let _: () = jvm
            .invoke_virtual(
                &this,
                "java/net/URL",
                "set",
                "(Ljava/lang/String;Ljava/lang/String;ILjava/lang/String;Ljava/lang/String;)V",
                (protocol.clone(), host, port, file, None),
            )
            .await?;

        if handler.is_null() {
            let protocol = JavaLangString::to_rust_string(jvm, &protocol).await?;
            let handler = Self::get_handler(jvm, &protocol).await?;

            jvm.put_field(&mut this, "java/net/URL", "handler", "Ljava/net/URLStreamHandler;", handler)
                .await?;
        }

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    #[method(protected)]
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
        tracing::debug!("java.net.URL::set({this:?}, {protocol:?}, {host:?}, {port:?}, {file:?}, {:?})", &r#ref);

        jvm.put_field(&mut this, "java/net/URL", "protocol", "Ljava/lang/String;", protocol)
            .await?;
        jvm.put_field(&mut this, "java/net/URL", "host", "Ljava/lang/String;", host).await?;
        jvm.put_field(&mut this, "java/net/URL", "port", "I", port).await?;
        jvm.put_field(&mut this, "java/net/URL", "file", "Ljava/lang/String;", file).await?;

        Ok(())
    }

    #[method(public)]
    async fn open_connection(jvm: &Jvm, _runtime: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<URLConnection>> {
        tracing::debug!("java.net.URL::openConnection({this:?})");

        let handler: Box<dyn ClassInstance> = jvm.get_field(&this, "java/net/URL", "handler", "Ljava/net/URLStreamHandler;").await?;
        let connection = jvm
            .invoke_virtual(
                &handler,
                "java/net/URLStreamHandler",
                "openConnection",
                "(Ljava/net/URL;)Ljava/net/URLConnection;",
                (this,),
            )
            .await?;

        Ok(connection)
    }

    #[method(public, final)]
    async fn open_stream(jvm: &Jvm, _runtime: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<InputStream>> {
        tracing::debug!("java.net.URL::openStream({this:?})");

        let connection: Box<dyn ClassInstance> = jvm
            .invoke_virtual(&this, "java/net/URL", "openConnection", "()Ljava/net/URLConnection;", ())
            .await?;

        let stream = jvm
            .invoke_virtual(&connection, "java/net/URLConnection", "getInputStream", "()Ljava/io/InputStream;", ())
            .await?;

        Ok(stream)
    }

    #[method(public)]
    async fn get_port(jvm: &Jvm, _runtime: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.net.URL::getPort({this:?})");

        let port = jvm.get_field(&this, "java/net/URL", "port", "I").await?;

        Ok(port)
    }

    #[method(public)]
    async fn get_protocol(jvm: &Jvm, _runtime: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.net.URL::getProtocol({this:?})");

        let protocol = jvm.get_field(&this, "java/net/URL", "protocol", "Ljava/lang/String;").await?;

        Ok(protocol)
    }

    #[method(public)]
    async fn get_host(jvm: &Jvm, _runtime: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.net.URL::getHost({this:?})");

        let host = jvm.get_field(&this, "java/net/URL", "host", "Ljava/lang/String;").await?;

        Ok(host)
    }

    #[method(public)]
    async fn get_file(jvm: &Jvm, _runtime: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.net.URL::getFile({this:?})");

        let file = jvm.get_field(&this, "java/net/URL", "file", "Ljava/lang/String;").await?;

        Ok(file)
    }

    async fn get_handler(jvm: &Jvm, protocol: &str) -> Result<ClassInstanceRef<URLStreamHandler>> {
        if protocol == "file" {
            Ok(jvm.new_class("org/rustjava/net/FileURLHandler", "()V", ()).await?.into())
        } else if protocol == "jar" {
            Ok(jvm.new_class("org/rustjava/net/JarURLHandler", "()V", ()).await?.into())
        } else {
            Err(jvm
                .exception("java/net/MalformedURLException", &format!("unknown protocol: {protocol}"))
                .await)
        }
    }
}
