use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

// class java.util.zip.ZipEntry
pub struct ZipEntry;

impl ZipEntry {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/zip/ZipEntry",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/lang/Cloneable"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/util/zip/ZipEntry;)V",
                    Self::init_with_zip_entry,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("getName", "()Ljava/lang/String;", Self::get_name, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("setSize", "(J)V", Self::set_size, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getSize", "()J", Self::get_size, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("name", "Ljava/lang/String;", FieldAccessFlags::empty()),
                JavaFieldProto::new("size", "J", FieldAccessFlags::empty()),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, name: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.util.zip.ZipEntry::<init>({this:?}, {name:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "name", "Ljava/lang/String;", name).await?;

        Ok(())
    }

    async fn init_with_zip_entry(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        zip_entry: ClassInstanceRef<Self>,
    ) -> Result<()> {
        tracing::debug!("java.util.zip.ZipEntry::<init>({this:?}, {zip_entry:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        let name: ClassInstanceRef<String> = jvm.get_field(&zip_entry, "name", "Ljava/lang/String;").await?;
        let size: i64 = jvm.get_field(&zip_entry, "size", "J").await?;

        jvm.put_field(&mut this, "name", "Ljava/lang/String;", name).await?;
        jvm.put_field(&mut this, "size", "J", size).await?;

        Ok(())
    }

    async fn get_name(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.zip.ZipEntry::getName({this:?})");

        jvm.get_field(&this, "name", "Ljava/lang/String;").await
    }

    async fn set_size(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, size: i64) -> Result<()> {
        tracing::debug!("java.util.zip.ZipEntry::setSize({this:?}, {size:?})");

        jvm.put_field(&mut this, "size", "J", size).await?;

        Ok(())
    }

    async fn get_size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        tracing::debug!("java.util.zip.ZipEntry::getSize({this:?})");

        jvm.get_field(&this, "size", "J").await
    }
}
