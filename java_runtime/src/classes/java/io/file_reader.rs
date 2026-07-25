use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::{ClassAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::{File, FileDescriptor},
        lang::String,
    },
};

// class java.io.FileReader
pub struct FileReader;

impl FileReader {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/FileReader",
            parent_class: Some("java/io/InputStreamReader"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_with_path, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(Ljava/io/File;)V", Self::init_with_file, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/io/FileDescriptor;)V",
                    Self::init_with_file_descriptor,
                    MethodAccessFlags::PUBLIC,
                ),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init_with_path(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, path: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.io.FileReader::<init>({this:?}, {path:?})");

        if path.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "path is null").await);
        }
        let file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (path,)).await?;
        jvm.invoke_special(&this, "java/io/FileReader", "<init>", "(Ljava/io/File;)V", (file,))
            .await
    }

    async fn init_with_file(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, file: ClassInstanceRef<File>) -> Result<()> {
        tracing::debug!("java.io.FileReader::<init>({this:?}, {file:?})");

        if file.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "file is null").await);
        }
        let input = jvm.new_class("java/io/FileInputStream", "(Ljava/io/File;)V", (file,)).await?;
        jvm.invoke_special(&this, "java/io/InputStreamReader", "<init>", "(Ljava/io/InputStream;)V", (input,))
            .await
    }

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
