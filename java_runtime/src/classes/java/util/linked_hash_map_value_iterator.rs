use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::MethodAccessFlags;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

use super::{LinkedHashMap, LinkedHashMapEntry};

// class java.util.LinkedHashMap$ValueIterator
pub struct LinkedHashMapValueIterator;

impl LinkedHashMapValueIterator {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/LinkedHashMap$ValueIterator",
            parent_class: Some("java/util/LinkedHashMap$LinkedHashIterator"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/util/LinkedHashMap;)V", Self::init, Default::default()),
                JavaMethodProto::new("next", "()Ljava/lang/Object;", Self::next, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, map: ClassInstanceRef<LinkedHashMap>) -> Result<()> {
        jvm.invoke_special(
            &this,
            "java/util/LinkedHashMap$LinkedHashIterator",
            "<init>",
            "(Ljava/util/LinkedHashMap;)V",
            (map,),
        )
        .await
    }

    async fn next(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let entry: ClassInstanceRef<LinkedHashMapEntry> = jvm
            .invoke_special(
                &this,
                "java/util/LinkedHashMap$LinkedHashIterator",
                "nextEntry",
                "()Ljava/util/LinkedHashMap$Entry;",
                (),
            )
            .await?;

        jvm.get_field(&entry, "value", "Ljava/lang/Object;").await
    }
}
