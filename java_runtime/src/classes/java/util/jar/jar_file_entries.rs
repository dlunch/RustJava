use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{
    classes::java::{
        lang::Object,
        util::zip::{ZipEntry, ZipFileEntries},
    },
    RuntimeClassProto, RuntimeContext,
};

// class java.util.jar.JarFile$Entries
pub struct JarFileEntries {}

impl JarFileEntries {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/jar/JarFile$Entries",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/util/Enumeration"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/util/zip/ZipFile$Entries;)V", Self::init, Default::default()),
                JavaMethodProto::new("hasMoreElements", "()Z", Self::has_more_elements, Default::default()),
                JavaMethodProto::new("nextElement", "()Ljava/lang/Object;", Self::next_element, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("entries", "Ljava/util/zip/ZipFile$Entries;", Default::default())],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, entries: ClassInstanceRef<ZipFileEntries>) -> Result<()> {
        tracing::debug!("java.util.jar.JarFile$Entries::<init>({:?}, {:?})", &this, &entries,);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "entries", "Ljava/util/zip/ZipFile$Entries;", entries).await?;

        Ok(())
    }

    async fn has_more_elements(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.jar.JarFile$Entries::hasMoreElements({:?})", &this);

        let entries = jvm.get_field(&this, "entries", "Ljava/util/zip/ZipFile$Entries;").await?;

        jvm.invoke_virtual(&entries, "hasMoreElements", "()Z", ()).await
    }

    async fn next_element(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.jar.JarFile$Entries::nextElement({:?})", &this);

        let entries = jvm.get_field(&this, "entries", "Ljava/util/zip/ZipFile$Entries;").await?;

        let element: ClassInstanceRef<ZipEntry> = jvm.invoke_virtual(&entries, "nextElement", "()Ljava/lang/Object;", ()).await?;

        let entry = jvm.new_class("java/util/jar/JarEntry", "(Ljava/util/zip/ZipEntry;)V", (element,)).await?;

        Ok(entry.into())
    }
}
