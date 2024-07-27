use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{
    classes::java::{
        io::{File, InputStream},
        net::URL,
    },
    RuntimeClassProto, RuntimeContext,
};

// class rustjava.net.FileURLConnection
pub struct FileURLConnection {}

impl FileURLConnection {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/net/URLConnection"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/net/URL;Ljava/io/File;)V", Self::init, Default::default()),
                JavaMethodProto::new("getInputStream", "()Ljava/io/InputStream;", Self::get_input_stream, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("file", "Ljava/io/File;", Default::default())],
        }
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        url: ClassInstanceRef<URL>,
        file: ClassInstanceRef<File>,
    ) -> Result<()> {
        tracing::debug!("rustjava.net.FileURLConnection::<init>({:?}, {:?}, {:?})", &this, &url, &file);

        let _: () = jvm
            .invoke_special(&this, "java/net/URLConnection", "<init>", "(Ljava/net/URL;)V", (url,))
            .await?;

        jvm.put_field(&mut this, "file", "Ljava/io/File;", file).await?;

        Ok(())
    }

    async fn get_input_stream(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<InputStream>> {
        tracing::debug!("rustjava.net.FileURLConnection::getInputStream({:?})", &this);

        let file: ClassInstanceRef<File> = jvm.get_field(&this, "file", "Ljava/io/File;").await?;
        let file_input_stream = jvm.new_class("java/io/FileInputStream", "(Ljava/io/File;)V", (file,)).await?;

        Ok(file_input_stream.into())
    }
}

#[cfg(test)]
mod test {
    use bytemuck::cast_vec;

    use jvm::{runtime::JavaLangString, Result};

    use crate::test::test_jvm_filesystem;

    #[tokio::test]
    async fn test_file_url() -> Result<()> {
        let filesystem = [("test.txt".into(), b"test file content".to_vec())].into_iter().collect();
        let jvm = test_jvm_filesystem(filesystem).await?;

        let url_spec = JavaLangString::from_rust_string(&jvm, "file:test.txt").await?;
        let url = jvm.new_class("java/net/URL", "(Ljava/lang/String;)V", (url_spec,)).await?;

        let stream = jvm.invoke_virtual(&url, "openStream", "()Ljava/io/InputStream;", ()).await?;

        let buf = jvm.instantiate_array("B", 17).await?;
        let len: i32 = jvm.invoke_virtual(&stream, "read", "([B)I", (buf.clone(),)).await?;

        let data = jvm.load_byte_array(&buf, 0, len as _).await?;

        assert_eq!(cast_vec::<i8, u8>(data), b"test file content");

        Ok(())
    }
}
