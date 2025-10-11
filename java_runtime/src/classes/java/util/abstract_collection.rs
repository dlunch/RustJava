use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::ClassAccessFlags;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

// abstract class java.util.AbstractCollection
pub struct AbstractCollection;

impl AbstractCollection {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/AbstractCollection",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new("<init>", "()V", Self::init, Default::default())],
            fields: vec![],
            access_flags: ClassAccessFlags::ABSTRACT,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.AbstractCollection::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }
}
