use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{classes::java::io::File, RuntimeClassProto, RuntimeContext};

// class java.util.jar.JarFile
pub struct JarFile {}

impl JarFile {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/util/zip/ZipFile"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new("<init>", "(Ljava/io/File;)V", Self::init, Default::default())],
            fields: vec![],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, file: ClassInstanceRef<File>) -> Result<()> {
        tracing::debug!("java.util.jar.JarFile::<init>({:?}, {:?})", &this, &file,);

        jvm.invoke_special(&this, "java/util/zip/ZipFile", "<init>", "(Ljava/io/File;)V", (file,))
            .await?;

        Ok(())
    }
}
