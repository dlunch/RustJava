use alloc::boxed::Box;

use jvm::{ClassInstance, ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{
        lang::Object,
        util::zip::{ZipEntry, ZipFileEntries},
    },
};

// class java.util.jar.JarFile$Entries
#[java_class(name = "java/util/jar/JarFile$Entries", extends = "java/lang/Object", implements("java/util/Enumeration"))]
pub struct JarFileEntries {
    #[field(private)]
    entries: ClassInstanceRef<ZipFileEntries>,
}

#[java_methods]
impl JarFileEntries {
    #[method(constructor)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, entries: ClassInstanceRef<ZipFileEntries>) -> Result<()> {
        tracing::debug!("java.util.jar.JarFile$Entries::<init>({this:?}, {entries:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        jvm.put_field(
            &mut this,
            "java/util/jar/JarFile$Entries",
            "entries",
            "Ljava/util/zip/ZipFile$Entries;",
            entries,
        )
        .await?;

        Ok(())
    }

    #[method(public)]
    async fn has_more_elements(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.jar.JarFile$Entries::hasMoreElements({this:?})");

        let entries: Box<dyn ClassInstance> = jvm
            .get_field(&this, "java/util/jar/JarFile$Entries", "entries", "Ljava/util/zip/ZipFile$Entries;")
            .await?;

        jvm.invoke_virtual(&entries, "java/util/zip/ZipFile$Entries", "hasMoreElements", "()Z", ())
            .await
    }

    #[method(public)]
    async fn next_element(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.jar.JarFile$Entries::nextElement({this:?})");

        let entries: Box<dyn ClassInstance> = jvm
            .get_field(&this, "java/util/jar/JarFile$Entries", "entries", "Ljava/util/zip/ZipFile$Entries;")
            .await?;

        let element: ClassInstanceRef<ZipEntry> = jvm
            .invoke_virtual(&entries, "java/util/zip/ZipFile$Entries", "nextElement", "()Ljava/lang/Object;", ())
            .await?;

        let entry = jvm.new_class("java/util/jar/JarEntry", "(Ljava/util/zip/ZipEntry;)V", (element,)).await?;

        Ok(entry.into())
    }
}
