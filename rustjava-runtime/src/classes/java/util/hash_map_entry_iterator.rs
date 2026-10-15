use alloc::vec;

use jvm::{Array, ClassInstanceRef, Jvm, Result};
use jvm_class_proto::JavaMethodProto;
use jvm_types::{ClassAccessFlags, MethodAccessFlags};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

use super::HashMap;

// class java.util.HashMap$EntryIterator
pub struct HashMapEntryIterator;

impl HashMapEntryIterator {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/HashMap$EntryIterator",
            parent_class: Some("java/util/HashMap$HashIterator"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new(
                "<init>",
                "(Ljava/util/HashMap;[Ljava/lang/Object;)V",
                Self::init,
                MethodAccessFlags::empty(),
            )],
            fields: vec![],
            access_flags: ClassAccessFlags::FINAL,
        }
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        map: ClassInstanceRef<HashMap>,
        elements: ClassInstanceRef<Array<Object>>,
    ) -> Result<()> {
        tracing::debug!("java.util.HashMap$EntryIterator::<init>({this:?}, {elements:?})");

        jvm.invoke_special(
            &this,
            "java/util/HashMap$HashIterator",
            "<init>",
            "(Ljava/util/HashMap;[Ljava/lang/Object;)V",
            (map, elements),
        )
        .await
    }
}
