use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{classes::java::lang::String, RuntimeClassProto, RuntimeContext};

// class java.util.jar.JarEntry
pub struct JarEntry {}

impl JarEntry {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/util/zip/ZipEntry"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init, Default::default())],
            fields: vec![],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, name: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.util.zip.ZipEntry::<init>({:?}, {:?})", &this, &name,);

        let _: () = jvm
            .invoke_special(&this, "java/util/zip/ZipEntry", "<init>", "(Ljava/lang/String;)V", (name,))
            .await?;

        Ok(())
    }
}
