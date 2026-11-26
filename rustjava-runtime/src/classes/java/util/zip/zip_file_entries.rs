use alloc::{boxed::Box, vec::Vec};

use jvm::{Array, ClassInstance, ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{
        lang::{Object, String},
        util::zip::ZipFile,
    },
};

// class java.util.zip.ZipFile$Entries
#[java_class(name = "java/util/zip/ZipFile$Entries", extends = "java/lang/Object", implements("java/util/Enumeration"))]
pub struct ZipFileEntries {
    #[field(private)]
    zipFile: ClassInstanceRef<ZipFile>,
    #[field(private)]
    names: ClassInstanceRef<Array<String>>,
    #[field(private)]
    i: i32,
}

#[java_methods]
impl ZipFileEntries {
    #[method(constructor)]
    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        zip_file: ClassInstanceRef<ZipFile>,
        names: ClassInstanceRef<Array<String>>,
    ) -> Result<()> {
        tracing::debug!("java.util.zip.ZipFile$Entries::<init>({this:?}, {zip_file:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "java/util/zip/ZipFile$Entries", "zipFile", "Ljava/util/zip/ZipFile;", zip_file)
            .await?;
        jvm.put_field(&mut this, "java/util/zip/ZipFile$Entries", "names", "[Ljava/lang/String;", names)
            .await?;
        jvm.put_field(&mut this, "java/util/zip/ZipFile$Entries", "i", "I", 0).await?;

        Ok(())
    }

    #[method(public)]
    async fn has_more_elements(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.zip.ZipFile$Entries::hasMoreElements({this:?})");

        let i: i32 = jvm.get_field(&this, "java/util/zip/ZipFile$Entries", "i", "I").await?;
        let names: ClassInstanceRef<Array<String>> = jvm
            .get_field(&this, "java/util/zip/ZipFile$Entries", "names", "[Ljava/lang/String;")
            .await?;
        let names_length = jvm.array_length(&names).await?;

        Ok(i < names_length as i32)
    }

    #[method(public)]
    async fn next_element(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.zip.ZipFile$Entries::nextElement({this:?})");

        let i: i32 = jvm.get_field(&this, "java/util/zip/ZipFile$Entries", "i", "I").await?;
        let names: ClassInstanceRef<Array<String>> = jvm
            .get_field(&this, "java/util/zip/ZipFile$Entries", "names", "[Ljava/lang/String;")
            .await?;
        let name: Vec<ClassInstanceRef<String>> = jvm.load_array(&names, i as _, 1).await?;

        let zip_file: Box<dyn ClassInstance> = jvm
            .get_field(&this, "java/util/zip/ZipFile$Entries", "zipFile", "Ljava/util/zip/ZipFile;")
            .await?;
        let entry = jvm
            .invoke_virtual(
                &zip_file,
                "java/util/zip/ZipFile",
                "getEntry",
                "(Ljava/lang/String;)Ljava/util/zip/ZipEntry;",
                (name[0].clone(),),
            )
            .await?;

        jvm.put_field(&mut this, "java/util/zip/ZipFile$Entries", "i", "I", i + 1).await?;

        Ok(entry)
    }
}
