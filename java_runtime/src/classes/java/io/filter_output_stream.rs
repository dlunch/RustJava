use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::FieldAccessFlags;
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
            fields: vec![JavaFieldProto::new("out", "Ljava/io/OutputStream;", FieldAccessFlags::PROTECTED)],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, out: ClassInstanceRef<OutputStream>) -> Result<()> {
        tracing::debug!("java.io.FilterOutputStream::<init>({:?}, {:?})", &this, &out);

        jvm.invoke_special(&this, "java/io/OutputStream", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "out", "Ljava/io/OutputStream;", out).await?;

        Ok(())
    }
}
