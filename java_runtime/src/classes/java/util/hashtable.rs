use alloc::{vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

use super::HashtableEntry;

const DEFAULT_INITIAL_CAPACITY: i32 = 11;
const DEFAULT_LOAD_FACTOR: f32 = 0.75;

// class java.util.Hashtable
pub struct Hashtable;

impl Hashtable {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Hashtable",
            parent_class: Some("java/util/Dictionary"),
            interfaces: vec!["java/util/Map"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("size", "()I", Self::size, Default::default()),
                JavaMethodProto::new("isEmpty", "()Z", Self::is_empty, Default::default()),
                JavaMethodProto::new("containsKey", "(Ljava/lang/Object;)Z", Self::contains_key, Default::default()),
                JavaMethodProto::new("containsValue", "(Ljava/lang/Object;)Z", Self::contains_value, Default::default()),
                JavaMethodProto::new(
                    "put",
                    "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                    Self::put,
                    Default::default(),
                ),
                JavaMethodProto::new("get", "(Ljava/lang/Object;)Ljava/lang/Object;", Self::get, Default::default()),
                JavaMethodProto::new("remove", "(Ljava/lang/Object;)Ljava/lang/Object;", Self::remove, Default::default()),
                JavaMethodProto::new("clear", "()V", Self::clear, Default::default()),
                JavaMethodProto::new("keySet", "()Ljava/util/Set;", Self::key_set, Default::default()),
                JavaMethodProto::new("values", "()Ljava/util/Collection;", Self::values, Default::default()),
                JavaMethodProto::new("entrySet", "()Ljava/util/Set;", Self::entry_set, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("table", "[Ljava/util/Hashtable$Entry;", Default::default()),
                JavaFieldProto::new("count", "I", Default::default()),
                JavaFieldProto::new("threshold", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Hashtable::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/util/Dictionary", "<init>", "()V", ()).await?;

        let table = jvm
            .instantiate_array("Ljava/util/Hashtable$Entry;", DEFAULT_INITIAL_CAPACITY as _)
            .await?;
        jvm.put_field(&mut this, "table", "[Ljava/util/Hashtable$Entry;", table).await?;
        jvm.put_field(&mut this, "count", "I", 0).await?;
        jvm.put_field(
            &mut this,
            "threshold",
            "I",
            (DEFAULT_INITIAL_CAPACITY as f32 * DEFAULT_LOAD_FACTOR) as i32,
        )
        .await?;

        Ok(())
    }

    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.Hashtable::size({this:?})");

        jvm.get_field(&this, "count", "I").await
    }

    async fn is_empty(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.Hashtable::isEmpty({this:?})");

        let count: i32 = jvm.get_field(&this, "count", "I").await?;

        Ok(count == 0)
    }

    async fn contains_key(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Hashtable::containsKey({this:?}, {key:?})");

        let key_hash = Self::key_hash(jvm, &key).await?;
        let table = jvm.get_field(&this, "table", "[Ljava/util/Hashtable$Entry;").await?;
        let table_len = jvm.array_length(&table).await? as i32;
        let bucket_index = ((key_hash & 0x7FFFFFFF) % table_len) as usize;

        let mut entry: ClassInstanceRef<HashtableEntry> = jvm.load_array(&table, bucket_index, 1).await?.into_iter().next().unwrap();
        while !entry.is_null() {
            let entry_hash: i32 = jvm.get_field(&entry, "hash", "I").await?;
            if entry_hash == key_hash {
                let entry_key: ClassInstanceRef<Object> = jvm.get_field(&entry, "key", "Ljava/lang/Object;").await?;
                let equals: bool = jvm.invoke_virtual(&entry_key, "equals", "(Ljava/lang/Object;)Z", (key.clone(),)).await?;
                if equals {
                    return Ok(true);
                }
            }
            entry = jvm.get_field(&entry, "next", "Ljava/util/Hashtable$Entry;").await?;
        }

        Ok(false)
    }

    async fn contains_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Hashtable::containsValue({this:?}, {value:?})");

        if value.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Hashtable value is null").await);
        }

        let table: ClassInstanceRef<Array<HashtableEntry>> = jvm.get_field(&this, "table", "[Ljava/util/Hashtable$Entry;").await?;
        let table_len = jvm.array_length(&table).await?;
        for bucket_index in 0..table_len {
            let mut entry = Self::load_bucket(jvm, &table, bucket_index).await?;
            while !entry.is_null() {
                let entry_value: ClassInstanceRef<Object> = jvm.get_field(&entry, "value", "Ljava/lang/Object;").await?;
                if Self::object_equals(jvm, &value, &entry_value).await? {
                    return Ok(true);
                }

                entry = jvm.get_field(&entry, "next", "Ljava/util/Hashtable$Entry;").await?;
            }
        }

        Ok(false)
    }

    async fn get(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key: ClassInstanceRef<Object>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Hashtable::get({this:?}, {key:?})");

        let key_hash = Self::key_hash(jvm, &key).await?;
        let table = jvm.get_field(&this, "table", "[Ljava/util/Hashtable$Entry;").await?;
        let table_len = jvm.array_length(&table).await? as i32;
        let bucket_index = ((key_hash & 0x7FFFFFFF) % table_len) as usize;

        let mut entry: ClassInstanceRef<HashtableEntry> = jvm.load_array(&table, bucket_index, 1).await?.into_iter().next().unwrap();
        while !entry.is_null() {
            let entry_hash: i32 = jvm.get_field(&entry, "hash", "I").await?;
            if entry_hash == key_hash {
                let entry_key: ClassInstanceRef<Object> = jvm.get_field(&entry, "key", "Ljava/lang/Object;").await?;
                let equals: bool = jvm.invoke_virtual(&entry_key, "equals", "(Ljava/lang/Object;)Z", (key.clone(),)).await?;
                if equals {
                    return jvm.get_field(&entry, "value", "Ljava/lang/Object;").await;
                }
            }
            entry = jvm.get_field(&entry, "next", "Ljava/util/Hashtable$Entry;").await?;
        }

        Ok(None.into())
    }

    async fn remove(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        key: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Hashtable::remove({this:?}, {key:?})");

        let key_hash = Self::key_hash(jvm, &key).await?;
        let mut table = jvm.get_field(&this, "table", "[Ljava/util/Hashtable$Entry;").await?;
        let table_len = jvm.array_length(&table).await? as i32;
        let bucket_index = ((key_hash & 0x7FFFFFFF) % table_len) as usize;

        let mut prev: ClassInstanceRef<HashtableEntry> = None.into();
        let mut entry: ClassInstanceRef<HashtableEntry> = jvm.load_array(&table, bucket_index, 1).await?.into_iter().next().unwrap();

        while !entry.is_null() {
            let entry_hash: i32 = jvm.get_field(&entry, "hash", "I").await?;
            if entry_hash == key_hash {
                let entry_key: ClassInstanceRef<Object> = jvm.get_field(&entry, "key", "Ljava/lang/Object;").await?;
                let equals: bool = jvm.invoke_virtual(&entry_key, "equals", "(Ljava/lang/Object;)Z", (key.clone(),)).await?;
                if equals {
                    let next: ClassInstanceRef<HashtableEntry> = jvm.get_field(&entry, "next", "Ljava/util/Hashtable$Entry;").await?;
                    if prev.is_null() {
                        jvm.store_array(&mut table, bucket_index, core::iter::once(next)).await?;
                    } else {
                        jvm.put_field(&mut prev, "next", "Ljava/util/Hashtable$Entry;", next).await?;
                    }

                    let count: i32 = jvm.get_field(&this, "count", "I").await?;
                    jvm.put_field(&mut this, "count", "I", count - 1).await?;

                    return jvm.get_field(&entry, "value", "Ljava/lang/Object;").await;
                }
            }
            prev = entry;
            entry = jvm.get_field(&prev, "next", "Ljava/util/Hashtable$Entry;").await?;
        }

        Ok(None.into())
    }

    async fn put(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        key: ClassInstanceRef<Object>,
        value: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Hashtable::put({this:?}, {key:?}, {value:?})");

        if value.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Hashtable value is null").await);
        }

        let key_hash = Self::key_hash(jvm, &key).await?;
        let mut table = jvm.get_field(&this, "table", "[Ljava/util/Hashtable$Entry;").await?;
        let table_len = jvm.array_length(&table).await? as i32;
        let bucket_index = ((key_hash & 0x7FFFFFFF) % table_len) as usize;

        let mut entry: ClassInstanceRef<HashtableEntry> = jvm.load_array(&table, bucket_index, 1).await?.into_iter().next().unwrap();
        while !entry.is_null() {
            let entry_hash: i32 = jvm.get_field(&entry, "hash", "I").await?;
            if entry_hash == key_hash {
                let entry_key: ClassInstanceRef<Object> = jvm.get_field(&entry, "key", "Ljava/lang/Object;").await?;
                let equals: bool = jvm.invoke_virtual(&entry_key, "equals", "(Ljava/lang/Object;)Z", (key.clone(),)).await?;
                if equals {
                    let old_value: ClassInstanceRef<Object> = jvm.get_field(&entry, "value", "Ljava/lang/Object;").await?;
                    jvm.put_field(&mut entry, "value", "Ljava/lang/Object;", value).await?;
                    return Ok(old_value);
                }
            }
            entry = jvm.get_field(&entry, "next", "Ljava/util/Hashtable$Entry;").await?;
        }

        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        let threshold: i32 = jvm.get_field(&this, "threshold", "I").await?;

        if count >= threshold {
            Self::rehash(jvm, &mut this).await?;
            table = jvm.get_field(&this, "table", "[Ljava/util/Hashtable$Entry;").await?;
            let new_table_len = jvm.array_length(&table).await? as i32;
            let new_bucket_index = ((key_hash & 0x7FFFFFFF) % new_table_len) as usize;

            let existing: ClassInstanceRef<HashtableEntry> = jvm.load_array(&table, new_bucket_index, 1).await?.into_iter().next().unwrap();
            let new_entry = jvm
                .new_class(
                    "java/util/Hashtable$Entry",
                    "(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/Hashtable$Entry;)V",
                    (key_hash, key, value, existing),
                )
                .await?;
            jvm.store_array(&mut table, new_bucket_index, core::iter::once(new_entry)).await?;
        } else {
            let existing: ClassInstanceRef<HashtableEntry> = jvm.load_array(&table, bucket_index, 1).await?.into_iter().next().unwrap();
            let new_entry = jvm
                .new_class(
                    "java/util/Hashtable$Entry",
                    "(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/Hashtable$Entry;)V",
                    (key_hash, key, value, existing),
                )
                .await?;
            jvm.store_array(&mut table, bucket_index, core::iter::once(new_entry)).await?;
        }

        jvm.put_field(&mut this, "count", "I", count + 1).await?;

        Ok(None.into())
    }

    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Hashtable::clear({this:?})");

        let mut table: ClassInstanceRef<Array<HashtableEntry>> = jvm.get_field(&this, "table", "[Ljava/util/Hashtable$Entry;").await?;
        let table_len = jvm.array_length(&table).await?;
        if table_len > 0 {
            let nulls: Vec<ClassInstanceRef<HashtableEntry>> = (0..table_len).map(|_| None.into()).collect();
            jvm.store_array(&mut table, 0, nulls).await?;
        }
        jvm.put_field(&mut this, "count", "I", 0).await?;

        Ok(())
    }

    async fn key_set(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Hashtable::keySet({this:?})");

        let key_set = jvm.new_class("java/util/Hashtable$KeySet", "(Ljava/util/Hashtable;)V", (this,)).await?;

        Ok(key_set.into())
    }

    async fn values(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Hashtable::values({this:?})");

        let values = jvm.new_class("java/util/Hashtable$Values", "(Ljava/util/Hashtable;)V", (this,)).await?;

        Ok(values.into())
    }

    async fn entry_set(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Hashtable::entrySet({this:?})");

        let entry_set = jvm.new_class("java/util/Hashtable$EntrySet", "(Ljava/util/Hashtable;)V", (this,)).await?;

        Ok(entry_set.into())
    }

    pub(super) async fn keys_snapshot(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Array<Object>>> {
        Self::snapshot_entries(jvm, this, SnapshotKind::Keys).await
    }

    pub(super) async fn values_snapshot(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Array<Object>>> {
        Self::snapshot_entries(jvm, this, SnapshotKind::Values).await
    }

    pub(super) async fn entries_snapshot(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Array<Object>>> {
        Self::snapshot_entries(jvm, this, SnapshotKind::Entries).await
    }

    pub(super) async fn find_entry(
        jvm: &Jvm,
        this: &ClassInstanceRef<Self>,
        key: &ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<HashtableEntry>> {
        let key_hash = Self::key_hash(jvm, key).await?;
        let table: ClassInstanceRef<Array<HashtableEntry>> = jvm.get_field(this, "table", "[Ljava/util/Hashtable$Entry;").await?;
        let table_len = jvm.array_length(&table).await? as i32;
        let bucket_index = ((key_hash & 0x7FFFFFFF) % table_len) as usize;

        let mut entry = Self::load_bucket(jvm, &table, bucket_index).await?;
        while !entry.is_null() {
            let entry_hash: i32 = jvm.get_field(&entry, "hash", "I").await?;
            if entry_hash == key_hash {
                let entry_key: ClassInstanceRef<Object> = jvm.get_field(&entry, "key", "Ljava/lang/Object;").await?;
                let equals: bool = jvm.invoke_virtual(&entry_key, "equals", "(Ljava/lang/Object;)Z", (key.clone(),)).await?;
                if equals {
                    return Ok(entry);
                }
            }

            entry = jvm.get_field(&entry, "next", "Ljava/util/Hashtable$Entry;").await?;
        }

        Ok(None.into())
    }

    async fn rehash(jvm: &Jvm, this: &mut ClassInstanceRef<Self>) -> Result<()> {
        let old_table = jvm.get_field(this, "table", "[Ljava/util/Hashtable$Entry;").await?;
        let old_capacity = jvm.array_length(&old_table).await?;
        let new_capacity = old_capacity * 2 + 1;

        let mut new_table = jvm.instantiate_array("Ljava/util/Hashtable$Entry;", new_capacity).await?;

        for i in 0..old_capacity {
            let mut entry: ClassInstanceRef<HashtableEntry> = jvm.load_array(&old_table, i, 1).await?.into_iter().next().unwrap();
            while !entry.is_null() {
                let next: ClassInstanceRef<HashtableEntry> = jvm.get_field(&entry, "next", "Ljava/util/Hashtable$Entry;").await?;
                let entry_hash: i32 = jvm.get_field(&entry, "hash", "I").await?;
                let new_index = ((entry_hash & 0x7FFFFFFF) % new_capacity as i32) as usize;

                let existing: ClassInstanceRef<HashtableEntry> = jvm.load_array(&new_table, new_index, 1).await?.into_iter().next().unwrap();
                jvm.put_field(&mut entry, "next", "Ljava/util/Hashtable$Entry;", existing).await?;
                jvm.store_array(&mut new_table, new_index, core::iter::once(entry)).await?;

                entry = next;
            }
        }

        jvm.put_field(this, "table", "[Ljava/util/Hashtable$Entry;", new_table).await?;
        jvm.put_field(this, "threshold", "I", (new_capacity as f32 * DEFAULT_LOAD_FACTOR) as i32)
            .await?;

        Ok(())
    }

    async fn load_bucket(
        jvm: &Jvm,
        table: &ClassInstanceRef<Array<HashtableEntry>>,
        bucket_index: usize,
    ) -> Result<ClassInstanceRef<HashtableEntry>> {
        let mut entries = jvm.load_array(table, bucket_index, 1).await?;

        Ok(entries.pop().unwrap_or_else(|| None.into()))
    }

    async fn key_hash(jvm: &Jvm, key: &ClassInstanceRef<Object>) -> Result<i32> {
        if key.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Hashtable key is null").await);
        }

        jvm.invoke_virtual(key, "hashCode", "()I", ()).await
    }

    async fn object_equals(jvm: &Jvm, left: &ClassInstanceRef<Object>, right: &ClassInstanceRef<Object>) -> Result<bool> {
        if left.is_null() {
            return Ok(right.is_null());
        }

        if right.is_null() {
            return Ok(false);
        }

        jvm.invoke_virtual(left, "equals", "(Ljava/lang/Object;)Z", (right.clone(),)).await
    }

    async fn snapshot_entries(jvm: &Jvm, this: &ClassInstanceRef<Self>, kind: SnapshotKind) -> Result<ClassInstanceRef<Array<Object>>> {
        let table: ClassInstanceRef<Array<HashtableEntry>> = jvm.get_field(this, "table", "[Ljava/util/Hashtable$Entry;").await?;
        let table_len = jvm.array_length(&table).await?;
        let count: i32 = jvm.get_field(this, "count", "I").await?;
        let mut elements: Vec<ClassInstanceRef<Object>> = Vec::with_capacity(count.max(0) as usize);

        for bucket_index in 0..table_len {
            let mut entry = Self::load_bucket(jvm, &table, bucket_index).await?;
            while !entry.is_null() {
                let element = match kind {
                    SnapshotKind::Keys => jvm.get_field(&entry, "key", "Ljava/lang/Object;").await?,
                    SnapshotKind::Values => jvm.get_field(&entry, "value", "Ljava/lang/Object;").await?,
                    SnapshotKind::Entries => ClassInstanceRef::new(entry.clone().instance),
                };
                elements.push(element);

                entry = jvm.get_field(&entry, "next", "Ljava/util/Hashtable$Entry;").await?;
            }
        }

        let mut snapshot: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", elements.len()).await?.into();
        if !elements.is_empty() {
            jvm.store_array(&mut snapshot, 0, elements).await?;
        }

        Ok(snapshot)
    }
}

#[derive(Copy, Clone)]
enum SnapshotKind {
    Keys,
    Values,
    Entries,
}
