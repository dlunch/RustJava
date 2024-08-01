use alloc::{vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{
    classes::java::{
        lang::{Object, String},
        util::zip::ZipFile,
    },
    RuntimeClassProto, RuntimeContext,
};

// class java.util.zip.ZipFile$Entries
pub struct ZipFileEntries {}

impl ZipFileEntries {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/util/Enumeration"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/util/zip/ZipFile;[Ljava/lang/String;)V", Self::init, Default::default()),
                JavaMethodProto::new("hasMoreElements", "()Z", Self::has_more_elements, Default::default()),
                JavaMethodProto::new("nextElement", "()Ljava/lang/Object;", Self::next_element, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("zipFile", "Ljava/util/zip/ZipFile;", Default::default()),
                JavaFieldProto::new("names", "[Ljava/lang/String;", Default::default()),
                JavaFieldProto::new("i", "I", Default::default()),
            ],
        }
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        zip_file: ClassInstanceRef<ZipFile>,
        names: ClassInstanceRef<Array<String>>,
    ) -> Result<()> {
        tracing::debug!("java.util.zip.ZipFile$Entries::<init>({:?}, {:?})", &this, &zip_file,);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "zipFile", "Ljava/util/zip/ZipFile;", zip_file).await?;
        jvm.put_field(&mut this, "names", "[Ljava/lang/String;", names).await?;
        jvm.put_field(&mut this, "i", "I", 0).await?;

        Ok(())
    }

    async fn has_more_elements(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.zip.ZipFile$Entries::hasMoreElements({:?})", &this);

        let i: i32 = jvm.get_field(&this, "i", "I").await?;
        let names: ClassInstanceRef<Array<String>> = jvm.get_field(&this, "names", "[Ljava/lang/String;").await?;
        let names_length = jvm.array_length(&names).await?;

        Ok(i < names_length as i32)
    }

    async fn next_element(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.zip.ZipFile$Entries::nextElement({:?})", &this);

        let i: i32 = jvm.get_field(&this, "i", "I").await?;
        let names: ClassInstanceRef<Array<String>> = jvm.get_field(&this, "names", "[Ljava/lang/String;").await?;
        let name: Vec<ClassInstanceRef<String>> = jvm.load_array(&names, i as _, 1).await?;

        let zip_file = jvm.get_field(&this, "zipFile", "Ljava/util/zip/ZipFile;").await?;
        let entry = jvm
            .invoke_virtual(&zip_file, "getEntry", "(Ljava/lang/String;)Ljava/util/zip/ZipEntry;", (name[0].clone(),))
            .await?;

        jvm.put_field(&mut this, "i", "I", i + 1).await?;

        Ok(entry)
    }
}
