use alloc::{borrow::ToOwned, string::ToString, vec};

use java_constants::ClassAccessFlags;
use url::Url;

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{lang::String, net::URL},
};

// abstract class java.net.URLStreamHandler
pub struct URLStreamHandler;

impl URLStreamHandler {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/net/URLStreamHandler",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new_abstract("openConnection", "(Ljava/net/URL;)Ljava/net/URLConnection;", Default::default()),
                JavaMethodProto::new("parseURL", "(Ljava/net/URL;Ljava/lang/String;II)V", Self::parse_url, Default::default()),
                JavaMethodProto::new(
                    "setURL",
                    "(Ljava/net/URL;Ljava/lang/String;Ljava/lang/String;ILjava/lang/String;Ljava/lang/String;)V",
                    Self::set_url,
                    Default::default(),
                ),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::ABSTRACT,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.net.URLStreamHandler::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    async fn set_url(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        url: ClassInstanceRef<URL>,
        protocol: ClassInstanceRef<String>,
        host: ClassInstanceRef<String>,
        port: i32,
        file: ClassInstanceRef<String>,
        r#ref: ClassInstanceRef<String>,
    ) -> Result<()> {
        tracing::debug!(
            "java.net.URLStreamHandler::setURL({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?})",
            &this,
            &url,
            &protocol,
            &host,
            &port,
            &file,
            &r#ref,
        );

        let _: () = jvm
            .invoke_virtual(
                &url,
                "set",
                "(Ljava/lang/String;Ljava/lang/String;ILjava/lang/String;Ljava/lang/String;)V",
                (protocol, host, port, file, r#ref),
            )
            .await?;

        Ok(())
    }

    async fn parse_url(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        url: ClassInstanceRef<URL>,
        spec: ClassInstanceRef<String>,
        start: i32,
        limit: i32,
    ) -> Result<()> {
        tracing::debug!(
            "java.net.URLStreamHandler::parseURL({:?}, {:?}, {:?}, {:?}, {:?})",
            &this,
            &url,
            &spec,
            &start,
            &limit
        );

        let spec_str = JavaLangString::to_rust_string(jvm, &spec).await?;

        let parsed_url = Url::parse(&spec_str);
        if let Err(x) = parsed_url {
            return Err(jvm.exception("java/net/MalformedURLException", &x.to_string()).await);
        }

        let parsed_url = parsed_url.unwrap();

        let protocol = parsed_url.scheme();
        let path = parsed_url.path().to_owned() + &parsed_url.query().map(|x| "?".to_owned() + x).unwrap_or("".into());
        // TODO handle more elegantly..
        let file = if protocol == "file" { path.trim_start_matches('/') } else { &path };

        let protocol = JavaLangString::from_rust_string(jvm, parsed_url.scheme()).await?;
        let host = JavaLangString::from_rust_string(jvm, parsed_url.host_str().unwrap_or("")).await?;
        let port = parsed_url.port().map(|x| x as i32).unwrap_or(-1);
        let file = JavaLangString::from_rust_string(jvm, file).await?;

        let _: () = jvm
            .invoke_virtual(
                &this,
                "setURL",
                "(Ljava/net/URL;Ljava/lang/String;Ljava/lang/String;ILjava/lang/String;Ljava/lang/String;)V",
                (url, protocol, host, port, file, None),
            )
            .await?;

        Ok(())
    }
}
