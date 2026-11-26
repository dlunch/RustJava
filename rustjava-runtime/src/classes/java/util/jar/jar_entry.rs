use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::util::zip::ZipEntry};

// class java.util.jar.JarEntry
#[java_class(name = "java/util/jar/JarEntry", extends = "java/util/zip/ZipEntry", public)]
pub struct JarEntry;

#[java_methods]
impl JarEntry {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, entry: ClassInstanceRef<ZipEntry>) -> Result<()> {
        tracing::debug!("java.util.jar.JarEntry::<init>({this:?}, {entry:?})");

        let _: () = jvm
            .invoke_special(&this, "java/util/zip/ZipEntry", "<init>", "(Ljava/util/zip/ZipEntry;)V", (entry,))
            .await?;

        Ok(())
    }
}
