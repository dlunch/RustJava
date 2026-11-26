use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{
        io::{File, InputStream},
        net::URL,
    },
};

// class rustjava.net.FileURLConnection
#[java_class(name = "org/rustjava/net/FileURLConnection", extends = "java/net/URLConnection")]
pub struct FileURLConnection {
    #[field(private)]
    file: ClassInstanceRef<File>,
}

#[java_methods]
impl FileURLConnection {
    #[method(constructor)]
    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        url: ClassInstanceRef<URL>,
        file: ClassInstanceRef<File>,
    ) -> Result<()> {
        tracing::debug!("org.rustjava.net.FileURLConnection::<init>({this:?}, {url:?}, {file:?})");

        let _: () = jvm
            .invoke_special(&this, "java/net/URLConnection", "<init>", "(Ljava/net/URL;)V", (url,))
            .await?;

        jvm.put_field(&mut this, "org/rustjava/net/FileURLConnection", "file", "Ljava/io/File;", file)
            .await?;

        Ok(())
    }

    #[method(public)]
    async fn get_input_stream(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<InputStream>> {
        tracing::debug!("org.rustjava.net.FileURLConnection::getInputStream({this:?})");

        let file: ClassInstanceRef<File> = jvm
            .get_field(&this, "org/rustjava/net/FileURLConnection", "file", "Ljava/io/File;")
            .await?;
        let file_input_stream = jvm.new_class("java/io/FileInputStream", "(Ljava/io/File;)V", (file,)).await?;

        Ok(file_input_stream.into())
    }
}
