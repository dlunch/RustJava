use alloc::vec;

use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm_types::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

use super::{HashMap, HashMapEntry, LinkedHashMap};

// class java.util.LinkedHashMap$Entry
pub struct LinkedHashMapEntry;

impl LinkedHashMapEntry {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/LinkedHashMap$Entry",
            parent_class: Some("java/util/HashMap$Entry"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new(
                    "<init>",
                    "(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Entry;)V",
                    Self::init,
                    MethodAccessFlags::empty(),
                ),
                JavaMethodProto::new("onAccess", "(Ljava/util/HashMap;)V", Self::on_access, MethodAccessFlags::empty()),
                JavaMethodProto::new("onRemoval", "(Ljava/util/HashMap;)V", Self::on_removal, MethodAccessFlags::empty()),
            ],
            fields: vec![
                JavaFieldProto::new("before", "Ljava/util/LinkedHashMap$Entry;", FieldAccessFlags::empty()),
                JavaFieldProto::new("after", "Ljava/util/LinkedHashMap$Entry;", FieldAccessFlags::empty()),
            ],
            access_flags: ClassAccessFlags::empty(),
        }
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        hash: i32,
        key: ClassInstanceRef<Object>,
        value: ClassInstanceRef<Object>,
        next: ClassInstanceRef<HashMapEntry>,
    ) -> Result<()> {
        jvm.invoke_special(
            &this,
            "java/util/HashMap$Entry",
            "<init>",
            "(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Entry;)V",
            (hash, key, value, next),
        )
        .await
    }

    async fn on_access(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, map: ClassInstanceRef<HashMap>) -> Result<()> {
        let mut map: ClassInstanceRef<LinkedHashMap> = ClassInstanceRef::new(map.instance);
        if !jvm.get_field::<bool>(&map, "java/util/LinkedHashMap", "accessOrder", "Z").await? {
            return Ok(());
        }

        let mut header: ClassInstanceRef<Self> = jvm
            .get_field(&map, "java/util/LinkedHashMap", "header", "Ljava/util/LinkedHashMap$Entry;")
            .await?;
        let mut tail: ClassInstanceRef<Self> = jvm
            .get_field(&header, "java/util/LinkedHashMap$Entry", "before", "Ljava/util/LinkedHashMap$Entry;")
            .await?;
        if this.identity() == tail.identity() {
            return Ok(());
        }

        let mut before: ClassInstanceRef<Self> = jvm
            .get_field(&this, "java/util/LinkedHashMap$Entry", "before", "Ljava/util/LinkedHashMap$Entry;")
            .await?;
        let mut after: ClassInstanceRef<Self> = jvm
            .get_field(&this, "java/util/LinkedHashMap$Entry", "after", "Ljava/util/LinkedHashMap$Entry;")
            .await?;
        jvm.put_field(
            &mut before,
            "java/util/LinkedHashMap$Entry",
            "after",
            "Ljava/util/LinkedHashMap$Entry;",
            after.clone(),
        )
        .await?;
        jvm.put_field(
            &mut after,
            "java/util/LinkedHashMap$Entry",
            "before",
            "Ljava/util/LinkedHashMap$Entry;",
            before,
        )
        .await?;

        jvm.put_field(
            &mut this,
            "java/util/LinkedHashMap$Entry",
            "before",
            "Ljava/util/LinkedHashMap$Entry;",
            tail.clone(),
        )
        .await?;
        jvm.put_field(
            &mut this,
            "java/util/LinkedHashMap$Entry",
            "after",
            "Ljava/util/LinkedHashMap$Entry;",
            header.clone(),
        )
        .await?;
        jvm.put_field(
            &mut tail,
            "java/util/LinkedHashMap$Entry",
            "after",
            "Ljava/util/LinkedHashMap$Entry;",
            this.clone(),
        )
        .await?;
        jvm.put_field(
            &mut header,
            "java/util/LinkedHashMap$Entry",
            "before",
            "Ljava/util/LinkedHashMap$Entry;",
            this,
        )
        .await?;

        let mod_count: i32 = jvm.get_field(&map, "java/util/LinkedHashMap", "modCount", "I").await?;
        jvm.put_field(&mut map, "java/util/LinkedHashMap", "modCount", "I", mod_count.wrapping_add(1))
            .await
    }

    async fn on_removal(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, _: ClassInstanceRef<HashMap>) -> Result<()> {
        let mut before: ClassInstanceRef<Self> = jvm
            .get_field(&this, "java/util/LinkedHashMap$Entry", "before", "Ljava/util/LinkedHashMap$Entry;")
            .await?;
        let mut after: ClassInstanceRef<Self> = jvm
            .get_field(&this, "java/util/LinkedHashMap$Entry", "after", "Ljava/util/LinkedHashMap$Entry;")
            .await?;
        jvm.put_field(
            &mut before,
            "java/util/LinkedHashMap$Entry",
            "after",
            "Ljava/util/LinkedHashMap$Entry;",
            after.clone(),
        )
        .await?;
        jvm.put_field(
            &mut after,
            "java/util/LinkedHashMap$Entry",
            "before",
            "Ljava/util/LinkedHashMap$Entry;",
            before,
        )
        .await
    }
}
