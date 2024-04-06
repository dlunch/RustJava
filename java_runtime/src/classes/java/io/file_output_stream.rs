use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{classes::java::io::File, RuntimeClassProto, RuntimeContext};

// class java.io.FileOutputStream
pub struct FileOutputStream {}

impl FileOutputStream {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/io/OutputStream"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new("<init>", "(Ljava/io/File;)V", Self::init, Default::default())],
            fields: vec![],
        }
    }

    async fn init(_jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, file: ClassInstanceRef<File>) -> Result<()> {
        tracing::debug!("java.io.FileOutputStream::<init>({:?}, {:?})", &this, &file);

        Ok(())
    }
}
