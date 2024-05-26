use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{classes::java::net::URL, RuntimeClassProto, RuntimeContext};

// class rustjava.net.FileURLConnection
pub struct FileURLConnection {}

impl FileURLConnection {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/net/URLConnection"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new("<init>", "(Ljava/net/URL;)V", Self::init, Default::default())],
            fields: vec![],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, url: ClassInstanceRef<URL>) -> Result<()> {
        tracing::debug!("rustjava.net.FileURLConnection::<init>({:?}, {:?})", &this, &url);

        jvm.invoke_special(&this, "java/net/URLConnection", "<init>", "(Ljava/net/URL;)V", (url,))
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use bytemuck::cast_vec;

    use jvm::{runtime::JavaLangString, Result};

    use crate::test::test_jvm_filesystem;

    // #[futures_test::test]
    async fn test_file_url() -> Result<()> {
        let filesystem = [("test.txt".into(), b"test file content".to_vec())].into_iter().collect();
        let jvm = test_jvm_filesystem(filesystem).await?;

        let url_spec = JavaLangString::from_rust_string(&jvm, "file:test.txt").await?;
        let url = jvm.new_class("java/net/URL", "(Ljava/lang/String;)V", (url_spec,)).await?;

        let stream = jvm.invoke_virtual(&url, "openStream", "()Ljava/io/InputStream;", ()).await?;

        let buf = jvm.instantiate_array("B", 16).await?;
        let len: i32 = jvm.invoke_virtual(&stream, "read", "([B)I", (buf.clone(),)).await?;

        let data = jvm.load_byte_array(&buf, 0, len as _).await?;

        assert_eq!(cast_vec::<i8, u8>(data), b"test file content");

        Ok(())
    }
}
