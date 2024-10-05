use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{runtime::JavaLangString, ClassInstanceRef, Jvm, Result};

use crate::{classes::java::lang::String, FileType, RuntimeClassProto, RuntimeContext};

// class java.io.File
pub struct File;

impl File {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/File",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init, Default::default()),
                JavaMethodProto::new("getPath", "()Ljava/lang/String;", Self::get_path, Default::default()),
                JavaMethodProto::new("exists", "()Z", Self::exists, Default::default()),
                JavaMethodProto::new("isDirectory", "()Z", Self::is_directory, Default::default()),
                JavaMethodProto::new("isFile", "()Z", Self::is_file, Default::default()),
                JavaMethodProto::new("delete", "()Z", Self::delete, Default::default()),
                JavaMethodProto::new("length", "()J", Self::length, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("path", "Ljava/lang/String;", Default::default())],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, pathname: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.io.File::<init>({:?}, {:?})", &this, &pathname);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "path", "Ljava/lang/String;", pathname).await?;

        Ok(())
    }

    async fn get_path(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.io.File::getPath({:?})", &this);

        jvm.get_field(&this, "path", "Ljava/lang/String;").await
    }

    async fn exists(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.io.File::exists({:?})", &this);

        let path = jvm.invoke_virtual(&this, "getPath", "()Ljava/lang/String;", ()).await?;
        let path = JavaLangString::to_rust_string(jvm, &path).await?;

        Ok(context.metadata(&path).await.is_ok())
    }

    async fn is_directory(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.io.File::isDirectory({:?})", &this);

        let path = jvm.invoke_virtual(&this, "getPath", "()Ljava/lang/String;", ()).await?;
        let path = JavaLangString::to_rust_string(jvm, &path).await?;

        let stat = context.metadata(&path).await.unwrap();

        Ok(stat.r#type == FileType::Directory)
    }

    async fn is_file(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.io.File::isFile({:?})", &this);

        let path = jvm.invoke_virtual(&this, "getPath", "()Ljava/lang/String;", ()).await?;
        let path = JavaLangString::to_rust_string(jvm, &path).await?;

        let stat = context.metadata(&path).await.unwrap();

        Ok(stat.r#type == FileType::File)
    }

    async fn delete(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.io.File::delete({:?})", &this);

        let path = jvm.invoke_virtual(&this, "getPath", "()Ljava/lang/String;", ()).await?;
        let path = JavaLangString::to_rust_string(jvm, &path).await?;

        Ok(context.unlink(&path).await.is_ok())
    }

    async fn length(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        tracing::debug!("java.io.File::length({:?})", &this);

        let path = jvm.invoke_virtual(&this, "getPath", "()Ljava/lang/String;", ()).await?;
        let path = JavaLangString::to_rust_string(jvm, &path).await?;

        let stat = context.metadata(&path).await.unwrap();

        Ok(stat.size as _)
    }
}
