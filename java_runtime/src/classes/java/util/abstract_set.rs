use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::ClassAccessFlags;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

// abstract class java.util.AbstractSet
pub struct AbstractSet;

impl AbstractSet {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/AbstractSet",
            parent_class: Some("java/util/AbstractCollection"),
            interfaces: vec!["java/util/Set"],
            methods: vec![JavaMethodProto::new("<init>", "()V", Self::init, Default::default())],
            fields: vec![],
            access_flags: ClassAccessFlags::ABSTRACT,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.AbstractSet::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/util/AbstractCollection", "<init>", "()V", ()).await?;

        Ok(())
    }
}
