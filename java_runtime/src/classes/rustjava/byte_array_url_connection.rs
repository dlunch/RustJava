use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto, JavaResult};
use jvm::{Array, ClassInstanceRef, Jvm};

use crate::{classes::java::net::URL, RuntimeClassProto, RuntimeContext};

// class rustjava.ByteArrayURLConnection
pub struct ByteArrayURLConnection {}

impl ByteArrayURLConnection {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/net/URLConnection"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/net/URL;[B)V", Self::init, Default::default()),
                JavaMethodProto::new("getInputStream", "()Ljava/io/InputStream;", Self::get_input_stream, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("data", "[B", Default::default())],
        }
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        url: ClassInstanceRef<URL>,
        data: ClassInstanceRef<Array<i8>>,
    ) -> JavaResult<()> {
        tracing::debug!("rustjava.ByteArrayURLConnection::<init>({:?}, {:?})", &this, &url);

        jvm.invoke_special(&this, "java/net/URLConnection", "<init>", "(Ljava/net/URL;)V", (url,))
            .await?;

        jvm.put_field(&mut this, "data", "[B", data)?;

        Ok(())
    }

    async fn get_input_stream(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> JavaResult<ClassInstanceRef<()>> {
        tracing::debug!("rustjava.ByteArrayURLConnection::getInputStream({:?})", &this);

        let data: ClassInstanceRef<Array<i8>> = jvm.get_field(&this, "data", "[B")?;

        let input_stream = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (data,)).await?;

        Ok(input_stream.into())
    }
}
