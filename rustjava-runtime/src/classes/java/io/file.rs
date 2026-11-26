use alloc::boxed::Box;

use jvm::{ClassInstance, ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{FileType, RuntimeContext, classes::java::lang::String};

// class java.io.File
#[java_class(name = "java/io/File", extends = "java/lang/Object", implements("java/io/Serializable"), public)]
pub struct File {
    #[field(public, static, final)]
    separatorChar: JavaChar,
    #[field(public, static, final)]
    separator: ClassInstanceRef<String>,
    #[field(public, static, final)]
    pathSeparatorChar: JavaChar,
    #[field(public, static, final)]
    pathSeparator: ClassInstanceRef<String>,
    #[field(private, final)]
    path: ClassInstanceRef<String>,
}

#[java_methods]
impl File {
    #[method(static_initializer)]
    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        tracing::debug!("java.io.File::<clinit>()");

        let separator_char = if cfg!(windows) { '\\' } else { '/' };
        let separator = if cfg!(windows) { "\\" } else { "/" };
        let path_separator_char = if cfg!(windows) { ';' } else { ':' };
        let path_separator = if cfg!(windows) { ";" } else { ":" };

        jvm.put_static_field("java/io/File", "separatorChar", "C", separator_char as JavaChar)
            .await?;
        jvm.put_static_field(
            "java/io/File",
            "separator",
            "Ljava/lang/String;",
            JavaLangString::from_rust_string(jvm, separator).await?,
        )
        .await?;
        jvm.put_static_field("java/io/File", "pathSeparatorChar", "C", path_separator_char as JavaChar)
            .await?;
        jvm.put_static_field(
            "java/io/File",
            "pathSeparator",
            "Ljava/lang/String;",
            JavaLangString::from_rust_string(jvm, path_separator).await?,
        )
        .await?;

        Ok(())
    }

    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, pathname: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.io.File::<init>({this:?}, {pathname:?})");
        if pathname.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "argument is null").await);
        }

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "java/io/File", "path", "Ljava/lang/String;", pathname).await?;

        Ok(())
    }

    #[method(public)]
    async fn get_path(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.io.File::getPath({this:?})");

        jvm.get_field(&this, "java/io/File", "path", "Ljava/lang/String;").await
    }

    #[method(public)]
    async fn exists(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.io.File::exists({this:?})");

        let path: Box<dyn ClassInstance> = jvm.invoke_virtual(&this, "java/io/File", "getPath", "()Ljava/lang/String;", ()).await?;
        let path = JavaLangString::to_rust_string(jvm, &path).await?;

        Ok(context.metadata(&path).await.is_ok())
    }

    #[method(public)]
    async fn is_directory(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.io.File::isDirectory({this:?})");

        let path: Box<dyn ClassInstance> = jvm.invoke_virtual(&this, "java/io/File", "getPath", "()Ljava/lang/String;", ()).await?;
        let path = JavaLangString::to_rust_string(jvm, &path).await?;

        Ok(context.metadata(&path).await.is_ok_and(|x| x.r#type == FileType::Directory))
    }

    #[method(public)]
    async fn is_file(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.io.File::isFile({this:?})");

        let path: Box<dyn ClassInstance> = jvm.invoke_virtual(&this, "java/io/File", "getPath", "()Ljava/lang/String;", ()).await?;
        let path = JavaLangString::to_rust_string(jvm, &path).await?;

        Ok(context.metadata(&path).await.is_ok_and(|x| x.r#type == FileType::File))
    }

    #[method(public)]
    async fn delete(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.io.File::delete({this:?})");

        let path: Box<dyn ClassInstance> = jvm.invoke_virtual(&this, "java/io/File", "getPath", "()Ljava/lang/String;", ()).await?;
        let path = JavaLangString::to_rust_string(jvm, &path).await?;

        Ok(context.unlink(&path).await.is_ok())
    }

    #[method(public)]
    async fn length(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        tracing::debug!("java.io.File::length({this:?})");

        let path: Box<dyn ClassInstance> = jvm.invoke_virtual(&this, "java/io/File", "getPath", "()Ljava/lang/String;", ()).await?;
        let path = JavaLangString::to_rust_string(jvm, &path).await?;

        // File.length() is 0 when the file does not exist
        Ok(context.metadata(&path).await.map_or(0, |x| x.size as _))
    }
}
