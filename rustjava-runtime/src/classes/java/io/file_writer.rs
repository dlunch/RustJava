use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{
        io::{File, FileDescriptor},
        lang::String,
    },
};

// class java.io.FileWriter
#[java_class(name = "java/io/FileWriter", extends = "java/io/OutputStreamWriter", public)]
pub struct FileWriter;

#[java_methods]
impl FileWriter {
    #[method(constructor, public)]
    async fn init_with_path(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, path: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.io.FileWriter::<init>({this:?}, {path:?})");
        jvm.invoke_special(&this, "java/io/FileWriter", "<init>", "(Ljava/lang/String;Z)V", (path, false))
            .await
    }

    #[method(constructor, public)]
    async fn init_with_path_append(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        path: ClassInstanceRef<String>,
        append: bool,
    ) -> Result<()> {
        tracing::debug!("java.io.FileWriter::<init>({this:?}, {path:?}, {append})");

        if path.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "path is null").await);
        }
        let file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (path,)).await?;
        jvm.invoke_special(&this, "java/io/FileWriter", "<init>", "(Ljava/io/File;Z)V", (file, append))
            .await
    }

    #[method(constructor, public)]
    async fn init_with_file(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, file: ClassInstanceRef<File>) -> Result<()> {
        tracing::debug!("java.io.FileWriter::<init>({this:?}, {file:?})");
        jvm.invoke_special(&this, "java/io/FileWriter", "<init>", "(Ljava/io/File;Z)V", (file, false))
            .await
    }

    #[method(constructor, public)]
    async fn init_with_file_append(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        file: ClassInstanceRef<File>,
        append: bool,
    ) -> Result<()> {
        tracing::debug!("java.io.FileWriter::<init>({this:?}, {file:?}, {append})");

        if file.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "file is null").await);
        }
        let output = jvm.new_class("java/io/FileOutputStream", "(Ljava/io/File;Z)V", (file, append)).await?;

        jvm.invoke_special(&this, "java/io/OutputStreamWriter", "<init>", "(Ljava/io/OutputStream;)V", (output,))
            .await
    }

    #[method(constructor, public)]
    async fn init_with_file_descriptor(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        descriptor: ClassInstanceRef<FileDescriptor>,
    ) -> Result<()> {
        tracing::debug!("java.io.FileWriter::<init>({this:?}, {descriptor:?})");

        if descriptor.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "file descriptor is null").await);
        }
        let output = jvm
            .new_class("java/io/FileOutputStream", "(Ljava/io/FileDescriptor;)V", (descriptor,))
            .await?;
        jvm.invoke_special(&this, "java/io/OutputStreamWriter", "<init>", "(Ljava/io/OutputStream;)V", (output,))
            .await
    }
}
