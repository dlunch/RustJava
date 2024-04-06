use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{classes::java::io::OutputStream, RuntimeClassProto, RuntimeContext};

// class java.io.FilterOutputStream
pub struct FilterOutputStream {}

impl FilterOutputStream {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/io/OutputStream"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new(
                "<init>",
                "(Ljava/io/OutputStream;)V",
                Self::init,
                Default::default(),
            )],
            fields: vec![],
        }
    }

    async fn init(_jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, out: ClassInstanceRef<OutputStream>) -> Result<()> {
        tracing::debug!("java.io.FilterOutputStream::<init>({:?}, {:?})", &this, &out);

        Ok(())
    }
}
