use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::String};

// class java.util.zip.ZipEntry
#[java_class(name = "java/util/zip/ZipEntry", implements("java/lang/Cloneable"), public)]
pub struct ZipEntry {
    name: ClassInstanceRef<String>,
    size: i64,
}

#[java_methods]
impl ZipEntry {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, name: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.util.zip.ZipEntry::<init>({this:?}, {name:?})");

        if name.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "name is null").await);
        }

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "java/util/zip/ZipEntry", "name", "Ljava/lang/String;", name)
            .await?;

        Ok(())
    }

    #[method(constructor, public)]
    async fn init_with_zip_entry(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        zip_entry: ClassInstanceRef<Self>,
    ) -> Result<()> {
        tracing::debug!("java.util.zip.ZipEntry::<init>({this:?}, {zip_entry:?})");

        if zip_entry.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "entry is null").await);
        }

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        let name: ClassInstanceRef<String> = jvm.get_field(&zip_entry, "java/util/zip/ZipEntry", "name", "Ljava/lang/String;").await?;
        let size: i64 = jvm.get_field(&zip_entry, "java/util/zip/ZipEntry", "size", "J").await?;

        jvm.put_field(&mut this, "java/util/zip/ZipEntry", "name", "Ljava/lang/String;", name)
            .await?;
        jvm.put_field(&mut this, "java/util/zip/ZipEntry", "size", "J", size).await?;

        Ok(())
    }

    #[method(public)]
    async fn get_name(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.zip.ZipEntry::getName({this:?})");

        jvm.get_field(&this, "java/util/zip/ZipEntry", "name", "Ljava/lang/String;").await
    }

    #[method(public)]
    async fn set_size(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, size: i64) -> Result<()> {
        tracing::debug!("java.util.zip.ZipEntry::setSize({this:?}, {size:?})");

        jvm.put_field(&mut this, "java/util/zip/ZipEntry", "size", "J", size).await?;

        Ok(())
    }

    #[method(public)]
    async fn get_size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        tracing::debug!("java.util.zip.ZipEntry::getSize({this:?})");

        jvm.get_field(&this, "java/util/zip/ZipEntry", "size", "J").await
    }
}
