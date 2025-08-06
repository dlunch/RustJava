use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

// class java.util.AbstractList
pub struct AbstractList;

impl AbstractList {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/AbstractList",
            parent_class: Some("java/util/AbstractCollection"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new("<init>", "()V", Self::init, Default::default())],
            fields: vec![],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.AbstractList::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/util/AbstractCollection", "<init>", "()V", ()).await?;

        Ok(())
    }
}
