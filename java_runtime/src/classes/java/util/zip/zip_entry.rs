use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{classes::java::lang::String, RuntimeClassProto, RuntimeContext};

// class java.util.zip.ZipEntry
pub struct ZipEntry {}

impl ZipEntry {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init, Default::default()),
                JavaMethodProto::new("getName", "()Ljava/lang/String;", Self::get_name, Default::default()),
                JavaMethodProto::new("setSize", "(J)V", Self::set_size, Default::default()),
                JavaMethodProto::new("getSize", "()J", Self::get_size, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("name", "Ljava/lang/String;", Default::default()),
                JavaFieldProto::new("size", "J", Default::default()),
            ],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, name: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.util.zip.ZipEntry::<init>({:?}, {:?})", &this, &name,);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "name", "Ljava/lang/String;", name).await?;

        Ok(())
    }

    async fn get_name(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.zip.ZipEntry::getName({:?})", &this);

        jvm.get_field(&this, "name", "Ljava/lang/String;").await
    }

    async fn set_size(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, size: i64) -> Result<()> {
        tracing::debug!("java.util.zip.ZipEntry::setSize({:?}, {:?})", &this, &size);

        jvm.put_field(&mut this, "size", "J", size).await?;

        Ok(())
    }

    async fn get_size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        tracing::debug!("java.util.zip.ZipEntry::getSize({:?})", &this);

        jvm.get_field(&this, "size", "J").await
    }
}
