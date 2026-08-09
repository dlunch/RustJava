use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

use super::{LinkedHashMap, LinkedHashMapEntry};

// abstract class java.util.LinkedHashMap$LinkedHashIterator
pub struct LinkedHashMapLinkedHashIterator;

impl LinkedHashMapLinkedHashIterator {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/LinkedHashMap$LinkedHashIterator",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/util/Iterator"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/util/LinkedHashMap;)V", Self::init, Default::default()),
                JavaMethodProto::new("hasNext", "()Z", Self::has_next, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("remove", "()V", Self::remove, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("nextEntry", "()Ljava/util/LinkedHashMap$Entry;", Self::next_entry, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("map", "Ljava/util/LinkedHashMap;", Default::default()),
                JavaFieldProto::new("nextEntry", "Ljava/util/LinkedHashMap$Entry;", Default::default()),
                JavaFieldProto::new("lastReturned", "Ljava/util/LinkedHashMap$Entry;", Default::default()),
                JavaFieldProto::new("expectedModCount", "I", Default::default()),
            ],
            access_flags: ClassAccessFlags::ABSTRACT,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, map: ClassInstanceRef<LinkedHashMap>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        let header: ClassInstanceRef<LinkedHashMapEntry> = jvm.get_field(&map, "header", "Ljava/util/LinkedHashMap$Entry;").await?;
        let next: ClassInstanceRef<LinkedHashMapEntry> = jvm.get_field(&header, "after", "Ljava/util/LinkedHashMap$Entry;").await?;
        let mod_count: i32 = jvm.get_field(&map, "modCount", "I").await?;
        let last_returned: ClassInstanceRef<LinkedHashMapEntry> = None.into();
        jvm.put_field(&mut this, "map", "Ljava/util/LinkedHashMap;", map).await?;
        jvm.put_field(&mut this, "nextEntry", "Ljava/util/LinkedHashMap$Entry;", next).await?;
        jvm.put_field(&mut this, "lastReturned", "Ljava/util/LinkedHashMap$Entry;", last_returned)
            .await?;
        jvm.put_field(&mut this, "expectedModCount", "I", mod_count).await
    }

    async fn has_next(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let map: ClassInstanceRef<LinkedHashMap> = jvm.get_field(&this, "map", "Ljava/util/LinkedHashMap;").await?;
        let header: ClassInstanceRef<LinkedHashMapEntry> = jvm.get_field(&map, "header", "Ljava/util/LinkedHashMap$Entry;").await?;
        let next: ClassInstanceRef<LinkedHashMapEntry> = jvm.get_field(&this, "nextEntry", "Ljava/util/LinkedHashMap$Entry;").await?;

        Ok(next.identity() != header.identity())
    }

    async fn next_entry(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<LinkedHashMapEntry>> {
        let map: ClassInstanceRef<LinkedHashMap> = jvm.get_field(&this, "map", "Ljava/util/LinkedHashMap;").await?;
        let expected_mod_count: i32 = jvm.get_field(&this, "expectedModCount", "I").await?;
        let mod_count: i32 = jvm.get_field(&map, "modCount", "I").await?;
        if expected_mod_count != mod_count {
            return Err(jvm
                .exception("java/util/ConcurrentModificationException", "LinkedHashMap modified during iteration")
                .await);
        }

        let header: ClassInstanceRef<LinkedHashMapEntry> = jvm.get_field(&map, "header", "Ljava/util/LinkedHashMap$Entry;").await?;
        let next: ClassInstanceRef<LinkedHashMapEntry> = jvm.get_field(&this, "nextEntry", "Ljava/util/LinkedHashMap$Entry;").await?;
        if next.identity() == header.identity() {
            return Err(jvm
                .exception("java/util/NoSuchElementException", "LinkedHashMap iterator exhausted")
                .await);
        }
        let after: ClassInstanceRef<LinkedHashMapEntry> = jvm.get_field(&next, "after", "Ljava/util/LinkedHashMap$Entry;").await?;
        jvm.put_field(&mut this, "lastReturned", "Ljava/util/LinkedHashMap$Entry;", next.clone())
            .await?;
        jvm.put_field(&mut this, "nextEntry", "Ljava/util/LinkedHashMap$Entry;", after).await?;

        Ok(next)
    }

    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let last_returned: ClassInstanceRef<LinkedHashMapEntry> = jvm.get_field(&this, "lastReturned", "Ljava/util/LinkedHashMap$Entry;").await?;
        if last_returned.is_null() {
            return Err(jvm.exception("java/lang/IllegalStateException", "Iterator.remove").await);
        }
        let map: ClassInstanceRef<LinkedHashMap> = jvm.get_field(&this, "map", "Ljava/util/LinkedHashMap;").await?;
        let expected_mod_count: i32 = jvm.get_field(&this, "expectedModCount", "I").await?;
        let mod_count: i32 = jvm.get_field(&map, "modCount", "I").await?;
        if expected_mod_count != mod_count {
            return Err(jvm
                .exception("java/util/ConcurrentModificationException", "LinkedHashMap modified during iteration")
                .await);
        }

        let key: ClassInstanceRef<Object> = jvm.get_field(&last_returned, "key", "Ljava/lang/Object;").await?;
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&map, "remove", "(Ljava/lang/Object;)Ljava/lang/Object;", (key,))
            .await?;
        let last_returned: ClassInstanceRef<LinkedHashMapEntry> = None.into();
        jvm.put_field(&mut this, "lastReturned", "Ljava/util/LinkedHashMap$Entry;", last_returned)
            .await?;
        let mod_count: i32 = jvm.get_field(&map, "modCount", "I").await?;
        jvm.put_field(&mut this, "expectedModCount", "I", mod_count).await
    }
}
