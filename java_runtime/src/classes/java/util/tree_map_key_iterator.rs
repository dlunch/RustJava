use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::{ClassAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

use super::{TreeMap, TreeMapEntry, TreeMapPrivateEntryIterator};

// class java.util.TreeMap$KeyIterator
pub struct TreeMapKeyIterator;

impl TreeMapKeyIterator {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/TreeMap$KeyIterator",
            parent_class: Some("java/util/TreeMap$PrivateEntryIterator"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/util/TreeMap;Ljava/util/TreeMap$Entry;Ljava/lang/Object;Z)V",
                    Self::init,
                    MethodAccessFlags::empty(),
                ),
                JavaMethodProto::new("next", "()Ljava/lang/Object;", Self::next, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::FINAL,
        }
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        map: ClassInstanceRef<TreeMap>,
        next: ClassInstanceRef<TreeMapEntry>,
        upper: ClassInstanceRef<Object>,
        to_end: bool,
    ) -> Result<()> {
        jvm.invoke_special(
            &this,
            "java/util/TreeMap$PrivateEntryIterator",
            "<init>",
            "(Ljava/util/TreeMap;Ljava/util/TreeMap$Entry;Ljava/lang/Object;Z)V",
            (map, next, upper, to_end),
        )
        .await
    }

    async fn next(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let entry = TreeMapPrivateEntryIterator::next_entry(jvm, this).await?;
        jvm.get_field(&entry, "key", "Ljava/lang/Object;").await
    }
}
