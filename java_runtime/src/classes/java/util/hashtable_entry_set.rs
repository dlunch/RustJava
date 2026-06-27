use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

use super::Hashtable;

// class java.util.Hashtable$EntrySet
pub struct HashtableEntrySet;

impl HashtableEntrySet {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Hashtable$EntrySet",
            parent_class: Some("java/util/AbstractSet"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/util/Hashtable;)V", Self::init, Default::default()),
                JavaMethodProto::new("size", "()I", Self::size, Default::default()),
                JavaMethodProto::new("isEmpty", "()Z", Self::is_empty, Default::default()),
                JavaMethodProto::new("contains", "(Ljava/lang/Object;)Z", Self::contains, Default::default()),
                JavaMethodProto::new("clear", "()V", Self::clear, Default::default()),
                JavaMethodProto::new("iterator", "()Ljava/util/Iterator;", Self::iterator, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("map", "Ljava/util/Hashtable;", Default::default())],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, map: ClassInstanceRef<Hashtable>) -> Result<()> {
        tracing::debug!("java.util.Hashtable$EntrySet::<init>({this:?}, {map:?})");

        let _: () = jvm.invoke_special(&this, "java/util/AbstractSet", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "map", "Ljava/util/Hashtable;", map).await?;

        Ok(())
    }

    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.Hashtable$EntrySet::size({this:?})");

        let map: ClassInstanceRef<Hashtable> = jvm.get_field(&this, "map", "Ljava/util/Hashtable;").await?;

        jvm.invoke_virtual(&map, "size", "()I", ()).await
    }

    async fn is_empty(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.Hashtable$EntrySet::isEmpty({this:?})");

        let map: ClassInstanceRef<Hashtable> = jvm.get_field(&this, "map", "Ljava/util/Hashtable;").await?;

        jvm.invoke_virtual(&map, "isEmpty", "()Z", ()).await
    }

    async fn contains(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, candidate: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Hashtable$EntrySet::contains({this:?}, {candidate:?})");

        if candidate.is_null() || !jvm.is_instance(&**candidate, "java/util/Map$Entry") {
            return Ok(false);
        }

        let candidate_key: ClassInstanceRef<Object> = jvm.invoke_virtual(&candidate, "getKey", "()Ljava/lang/Object;", ()).await?;
        if candidate_key.is_null() {
            return Ok(false);
        }

        let map: ClassInstanceRef<Hashtable> = jvm.get_field(&this, "map", "Ljava/util/Hashtable;").await?;
        let entry = Hashtable::find_entry(jvm, &map, &candidate_key).await?;
        if entry.is_null() {
            return Ok(false);
        }

        let candidate_value: ClassInstanceRef<Object> = jvm.invoke_virtual(&candidate, "getValue", "()Ljava/lang/Object;", ()).await?;
        let entry_value: ClassInstanceRef<Object> = jvm.get_field(&entry, "value", "Ljava/lang/Object;").await?;

        Self::object_equals(jvm, &candidate_value, &entry_value).await
    }

    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Hashtable$EntrySet::clear({this:?})");

        let map: ClassInstanceRef<Hashtable> = jvm.get_field(&this, "map", "Ljava/util/Hashtable;").await?;

        jvm.invoke_virtual(&map, "clear", "()V", ()).await
    }

    async fn iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Hashtable$EntrySet::iterator({this:?})");

        let map: ClassInstanceRef<Hashtable> = jvm.get_field(&this, "map", "Ljava/util/Hashtable;").await?;
        let snapshot = Hashtable::entries_snapshot(jvm, &map).await?;
        let iterator = jvm
            .new_class("java/util/Hashtable$Enumerator", "([Ljava/lang/Object;)V", (snapshot,))
            .await?;

        Ok(iterator.into())
    }

    async fn object_equals(jvm: &Jvm, left: &ClassInstanceRef<Object>, right: &ClassInstanceRef<Object>) -> Result<bool> {
        if left.is_null() {
            return Ok(right.is_null());
        }

        jvm.invoke_virtual(left, "equals", "(Ljava/lang/Object;)Z", (right.clone(),)).await
    }
}
