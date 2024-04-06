use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{classes::java::io::File, RuntimeClassProto, RuntimeContext};

// class java.io.FileInputStream
pub struct FileInputStream {}

impl FileInputStream {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/io/InputStream"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new("<init>", "(Ljava/io/File;)V", Self::init, Default::default())],
            fields: vec![],
        }
    }

    async fn init(_jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, file: ClassInstanceRef<File>) -> Result<()> {
        tracing::debug!("java.io.FileInputStream::<init>({:?}, {:?})", &this, &file);

        Ok(())
    }
}
