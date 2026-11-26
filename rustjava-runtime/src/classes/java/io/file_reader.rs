use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{
        io::{File, FileDescriptor},
        lang::String,
    },
};

// class java.io.FileReader
#[java_class(name = "java/io/FileReader", extends = "java/io/InputStreamReader", public)]
pub struct FileReader;

#[java_methods]
impl FileReader {
    #[method(constructor, public)]
    async fn init_with_path(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, path: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.io.FileReader::<init>({this:?}, {path:?})");

        if path.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "path is null").await);
        }
        let file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (path,)).await?;
        jvm.invoke_special(&this, "java/io/FileReader", "<init>", "(Ljava/io/File;)V", (file,))
            .await
    }

    #[method(constructor, public)]
    async fn init_with_file(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, file: ClassInstanceRef<File>) -> Result<()> {
        tracing::debug!("java.io.FileReader::<init>({this:?}, {file:?})");

        if file.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "file is null").await);
        }
        let input = jvm.new_class("java/io/FileInputStream", "(Ljava/io/File;)V", (file,)).await?;
        jvm.invoke_special(&this, "java/io/InputStreamReader", "<init>", "(Ljava/io/InputStream;)V", (input,))
            .await
    }

    #[method(constructor, public)]
    async fn init_with_file_descriptor(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        descriptor: ClassInstanceRef<FileDescriptor>,
    ) -> Result<()> {
        tracing::debug!("java.io.FileReader::<init>({this:?}, {descriptor:?})");

        if descriptor.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "file descriptor is null").await);
        }
        let input = jvm
            .new_class("java/io/FileInputStream", "(Ljava/io/FileDescriptor;)V", (descriptor,))
            .await?;
        jvm.invoke_special(&this, "java/io/InputStreamReader", "<init>", "(Ljava/io/InputStream;)V", (input,))
            .await
    }
}
