use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{
    classes::java::{io::InputStream, net::URL},
    RuntimeClassProto, RuntimeContext,
};

// class rustjava.net.JarURLConnection
pub struct JarURLConnection {}

impl JarURLConnection {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/net/JarURLConnection"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/net/URL;)V", Self::init, Default::default()),
                JavaMethodProto::new("getJarFile", "()Ljava/util/jar/JarFile;", Self::get_jar_file, Default::default()),
                JavaMethodProto::new("getInputStream", "()Ljava/io/InputStream;", Self::get_input_stream, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("file", "Ljava/io/File;", Default::default())],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, url: ClassInstanceRef<URL>) -> Result<()> {
        tracing::debug!("rustjava.net.JarURLConnection::<init>({:?}, {:?})", &this, &url);

        jvm.invoke_special(&this, "java/net/JarURLConnection", "<init>", "(Ljava/net/URL;)V", (url.clone(),))
            .await?;

        Ok(())
    }

    async fn get_jar_file(
        _jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<crate::classes::java::util::jar::JarFile>> {
        tracing::debug!("rustjava.net.JarURLConnection::getJarFile({:?})", &this);

        Ok(None.into())
    }

    async fn get_input_stream(_jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<InputStream>> {
        tracing::debug!("rustjava.net.JarURLConnection::getInputStream({:?})", &this);

        Ok(None.into())
    }
}
