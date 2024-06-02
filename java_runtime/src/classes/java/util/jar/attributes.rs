use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{classes::java::io::InputStream, RuntimeClassProto, RuntimeContext};

// class java.util.jar.Attributes
pub struct Attributes {}

impl Attributes {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new("<init>", "()V", Self::init, Default::default())],
            fields: vec![],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, is: ClassInstanceRef<InputStream>) -> Result<()> {
        tracing::debug!("java.util.jar.Manifest::<init>({:?}, {:?})", &this, &is);

        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }
}
