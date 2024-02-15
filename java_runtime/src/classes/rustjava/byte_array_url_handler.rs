use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{Array, ClassInstanceRef, Jvm, JvmResult};

use crate::{
    classes::{java::net::URL, rustjava::ByteArrayURLConnection},
    RuntimeClassProto, RuntimeContext,
};

// class rustjava.ByteArrayURLHandler
pub struct ByteArrayURLHandler {}

impl ByteArrayURLHandler {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/net/URLStreamHandler"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "([B)V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "openConnection",
                    "(Ljava/net/URL;)Ljava/net/URLConnection;",
                    Self::open_connection,
                    Default::default(),
                ),
            ],
            fields: vec![JavaFieldProto::new("data", "[B", Default::default())],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, data: ClassInstanceRef<Array<i8>>) -> JvmResult<()> {
        tracing::debug!("rustjava.ByteArrayURLHandler::<init>({:?})", &this);

        jvm.invoke_special(&this, "java/net/URLStreamHandler", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "data", "[B", data)?;

        Ok(())
    }

    async fn open_connection(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        url: ClassInstanceRef<URL>,
    ) -> JvmResult<ClassInstanceRef<ByteArrayURLConnection>> {
        tracing::debug!("rustjava.ByteArrayURLHandler::openConnection({:?}, {:?})", &this, &url);

        let data: ClassInstanceRef<Array<i8>> = jvm.get_field(&this, "data", "[B")?;
        let url_connection = jvm
            .new_class("rustjava/ByteArrayURLConnection", "(Ljava/net/URL;[B)V", (url, data))
            .await?;

        Ok(url_connection.into())
    }
}
