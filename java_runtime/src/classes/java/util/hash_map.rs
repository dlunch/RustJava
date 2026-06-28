use alloc::{format, vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

use super::HashMapEntry;

const DEFAULT_INITIAL_CAPACITY: i32 = 16;
const DEFAULT_LOAD_FACTOR: f32 = 0.75;

// class java.util.HashMap
pub struct HashMap;

impl HashMap {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/HashMap",
            parent_class: Some("java/util/AbstractMap"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(I)V", Self::init_with_capacity, Default::default()),
                JavaMethodProto::new("size", "()I", Self::size, Default::default()),
                JavaMethodProto::new("isEmpty", "()Z", Self::is_empty, Default::default()),
                JavaMethodProto::new("containsKey", "(Ljava/lang/Object;)Z", Self::contains_key, Default::default()),
                JavaMethodProto::new("containsValue", "(Ljava/lang/Object;)Z", Self::contains_value, Default::default()),
                JavaMethodProto::new("get", "(Ljava/lang/Object;)Ljava/lang/Object;", Self::get, Default::default()),
                JavaMethodProto::new(
                    "put",
                    "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                    Self::put,
                    Default::default(),
                ),
                JavaMethodProto::new("remove", "(Ljava/lang/Object;)Ljava/lang/Object;", Self::remove, Default::default()),
                JavaMethodProto::new("clear", "()V", Self::clear, Default::default()),
                JavaMethodProto::new("keySet", "()Ljava/util/Set;", Self::key_set, Default::default()),
                JavaMethodProto::new("values", "()Ljava/util/Collection;", Self::values, Default::default()),
                JavaMethodProto::new("entrySet", "()Ljava/util/Set;", Self::entry_set, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("table", "[Ljava/util/HashMap$Entry;", Default::default()),
                JavaFieldProto::new("size", "I", Default::default()),
                JavaFieldProto::new("threshold", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.HashMap::<init>({this:?})");

        let _: () = jvm
            .invoke_special(&this, "java/util/HashMap", "<init>", "(I)V", (DEFAULT_INITIAL_CAPACITY,))
            .await?;

        Ok(())
    }

    async fn key_set(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.HashMap::keySet({this:?})");

        let key_set = jvm.new_class("java/util/HashMap$KeySet", "(Ljava/util/HashMap;)V", (this,)).await?;

        Ok(key_set.into())
    }

    async fn values(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.HashMap::values({this:?})");

        let values = jvm.new_class("java/util/HashMap$Values", "(Ljava/util/HashMap;)V", (this,)).await?;

        Ok(values.into())
    }

    async fn entry_set(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.HashMap::entrySet({this:?})");

        let entry_set = jvm.new_class("java/util/HashMap$EntrySet", "(Ljava/util/HashMap;)V", (this,)).await?;

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

    async fn init_with_capacity(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, capacity: i32) -> Result<()> {
        tracing::debug!("java.util.HashMap::<init>({this:?}, {capacity:?})");

        if capacity < 0 {
            return Err(jvm
                .exception("java/lang/IllegalArgumentException", &format!("Illegal Capacity: {capacity}"))
                .await);
        }

        let _: () = jvm.invoke_special(&this, "java/util/AbstractMap", "<init>", "()V", ()).await?;

        let table: ClassInstanceRef<Array<HashMapEntry>> = jvm.instantiate_array("Ljava/util/HashMap$Entry;", capacity as usize).await?.into();
        jvm.put_field(&mut this, "table", "[Ljava/util/HashMap$Entry;", table).await?;
        jvm.put_field(&mut this, "size", "I", 0).await?;
        jvm.put_field(&mut this, "threshold", "I", Self::threshold_for_capacity(capacity)).await?;

        Ok(())
    }

    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.HashMap::size({this:?})");

        jvm.get_field(&this, "size", "I").await
    }

    async fn is_empty(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.HashMap::isEmpty({this:?})");

        let size: i32 = jvm.get_field(&this, "size", "I").await?;

        Ok(size == 0)
    }

    async fn contains_key(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.HashMap::containsKey({this:?}, {key:?})");

        let entry = Self::find_entry(jvm, &this, &key).await?;

        Ok(!entry.is_null())
    }

    async fn contains_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.HashMap::containsValue({this:?}, {value:?})");

        let table: ClassInstanceRef<Array<HashMapEntry>> = jvm.get_field(&this, "table", "[Ljava/util/HashMap$Entry;").await?;
        let table_len = jvm.array_length(&table).await?;
        for bucket_index in 0..table_len {
            let mut entry = Self::load_bucket(jvm, &table, bucket_index).await?;
            while !entry.is_null() {
                let entry_value: ClassInstanceRef<Object> = jvm.get_field(&entry, "value", "Ljava/lang/Object;").await?;
                if Self::object_equals(jvm, &value, &entry_value).await? {
                    return Ok(true);
                }

                entry = jvm.get_field(&entry, "next", "Ljava/util/HashMap$Entry;").await?;
            }
        }

        Ok(false)
    }

    async fn get(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key: ClassInstanceRef<Object>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.HashMap::get({this:?}, {key:?})");

        let entry = Self::find_entry(jvm, &this, &key).await?;
        if entry.is_null() {
            return Ok(None.into());
        }

        jvm.get_field(&entry, "value", "Ljava/lang/Object;").await
    }

    async fn put(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        key: ClassInstanceRef<Object>,
        value: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.HashMap::put({this:?}, {key:?}, {value:?})");

        let key_hash = Self::object_hash_or_zero(jvm, &key).await?;
        Self::ensure_table_for_insert(jvm, &mut this).await?;

        let mut table: ClassInstanceRef<Array<HashMapEntry>> = jvm.get_field(&this, "table", "[Ljava/util/HashMap$Entry;").await?;
        let table_len = jvm.array_length(&table).await? as i32;
        let Some(bucket_index) = Self::bucket_index(key_hash, table_len) else {
            return Err(jvm.exception("java/lang/RuntimeException", "HashMap table is empty").await);
        };

        let mut entry = Self::load_bucket(jvm, &table, bucket_index).await?;
        while !entry.is_null() {
            let entry_hash: i32 = jvm.get_field(&entry, "hash", "I").await?;
            if entry_hash == key_hash {
                let entry_key: ClassInstanceRef<Object> = jvm.get_field(&entry, "key", "Ljava/lang/Object;").await?;
                if Self::keys_equal(jvm, &key, &entry_key).await? {
                    let old_value: ClassInstanceRef<Object> = jvm.get_field(&entry, "value", "Ljava/lang/Object;").await?;
                    jvm.put_field(&mut entry, "value", "Ljava/lang/Object;", value).await?;
                    return Ok(old_value);
                }
            }

            entry = jvm.get_field(&entry, "next", "Ljava/util/HashMap$Entry;").await?;
        }

        let size: i32 = jvm.get_field(&this, "size", "I").await?;
        let threshold: i32 = jvm.get_field(&this, "threshold", "I").await?;
        let bucket_index = if size >= threshold {
            Self::rehash(jvm, &mut this).await?;
            table = jvm.get_field(&this, "table", "[Ljava/util/HashMap$Entry;").await?;
            let table_len = jvm.array_length(&table).await? as i32;
            let Some(bucket_index) = Self::bucket_index(key_hash, table_len) else {
                return Err(jvm.exception("java/lang/RuntimeException", "HashMap table is empty").await);
            };
            bucket_index
        } else {
            bucket_index
        };

        let existing = Self::load_bucket(jvm, &table, bucket_index).await?;
        let new_entry: ClassInstanceRef<HashMapEntry> = jvm
            .new_class(
                "java/util/HashMap$Entry",
                "(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Entry;)V",
                (key_hash, key, value, existing),
            )
            .await?
            .into();
        jvm.store_array(&mut table, bucket_index, core::iter::once(new_entry)).await?;
        jvm.put_field(&mut this, "size", "I", size + 1).await?;

        Ok(None.into())
    }

    async fn remove(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        key: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.HashMap::remove({this:?}, {key:?})");

        let key_hash = Self::object_hash_or_zero(jvm, &key).await?;
        let mut table: ClassInstanceRef<Array<HashMapEntry>> = jvm.get_field(&this, "table", "[Ljava/util/HashMap$Entry;").await?;
        let table_len = jvm.array_length(&table).await? as i32;
        let Some(bucket_index) = Self::bucket_index(key_hash, table_len) else {
            return Ok(None.into());
        };

        let mut previous: ClassInstanceRef<HashMapEntry> = None.into();
        let mut entry = Self::load_bucket(jvm, &table, bucket_index).await?;
        while !entry.is_null() {
            let entry_hash: i32 = jvm.get_field(&entry, "hash", "I").await?;
            if entry_hash == key_hash {
                let entry_key: ClassInstanceRef<Object> = jvm.get_field(&entry, "key", "Ljava/lang/Object;").await?;
                if Self::keys_equal(jvm, &key, &entry_key).await? {
                    let next: ClassInstanceRef<HashMapEntry> = jvm.get_field(&entry, "next", "Ljava/util/HashMap$Entry;").await?;
                    if previous.is_null() {
                        jvm.store_array(&mut table, bucket_index, core::iter::once(next.clone())).await?;
                    } else {
                        jvm.put_field(&mut previous, "next", "Ljava/util/HashMap$Entry;", next.clone()).await?;
                    }

                    let old_value: ClassInstanceRef<Object> = jvm.get_field(&entry, "value", "Ljava/lang/Object;").await?;
                    let null_entry: ClassInstanceRef<HashMapEntry> = None.into();
                    jvm.put_field(&mut entry, "next", "Ljava/util/HashMap$Entry;", null_entry).await?;

                    let size: i32 = jvm.get_field(&this, "size", "I").await?;
                    jvm.put_field(&mut this, "size", "I", size - 1).await?;

                    return Ok(old_value);
                }
            }

            previous = entry;
            entry = jvm.get_field(&previous, "next", "Ljava/util/HashMap$Entry;").await?;
        }

        Ok(None.into())
    }

    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.HashMap::clear({this:?})");

        let mut table: ClassInstanceRef<Array<HashMapEntry>> = jvm.get_field(&this, "table", "[Ljava/util/HashMap$Entry;").await?;
        let table_len = jvm.array_length(&table).await?;
        if table_len > 0 {
            let nulls: Vec<ClassInstanceRef<HashMapEntry>> = (0..table_len).map(|_| None.into()).collect();
            jvm.store_array(&mut table, 0, nulls).await?;
        }
        jvm.put_field(&mut this, "size", "I", 0).await?;

        Ok(())
    }

    pub(super) async fn find_entry(
        jvm: &Jvm,
        this: &ClassInstanceRef<Self>,
        key: &ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<HashMapEntry>> {
        let key_hash = Self::object_hash_or_zero(jvm, key).await?;
        let table: ClassInstanceRef<Array<HashMapEntry>> = jvm.get_field(this, "table", "[Ljava/util/HashMap$Entry;").await?;
        let table_len = jvm.array_length(&table).await? as i32;
        let Some(bucket_index) = Self::bucket_index(key_hash, table_len) else {
            return Ok(None.into());
        };

        let mut entry = Self::load_bucket(jvm, &table, bucket_index).await?;
        while !entry.is_null() {
            let entry_hash: i32 = jvm.get_field(&entry, "hash", "I").await?;
            if entry_hash == key_hash {
                let entry_key: ClassInstanceRef<Object> = jvm.get_field(&entry, "key", "Ljava/lang/Object;").await?;
                if Self::keys_equal(jvm, key, &entry_key).await? {
                    return Ok(entry);
                }
            }

            entry = jvm.get_field(&entry, "next", "Ljava/util/HashMap$Entry;").await?;
        }

        Ok(None.into())
    }

    async fn ensure_table_for_insert(jvm: &Jvm, this: &mut ClassInstanceRef<Self>) -> Result<()> {
        let table: ClassInstanceRef<Array<HashMapEntry>> = jvm.get_field(this, "table", "[Ljava/util/HashMap$Entry;").await?;
        if jvm.array_length(&table).await? > 0 {
            return Ok(());
        }

        let new_capacity = 1;
        let new_table: ClassInstanceRef<Array<HashMapEntry>> = jvm.instantiate_array("Ljava/util/HashMap$Entry;", new_capacity).await?.into();
        jvm.put_field(this, "table", "[Ljava/util/HashMap$Entry;", new_table).await?;
        jvm.put_field(this, "threshold", "I", Self::threshold_for_capacity(new_capacity as i32))
            .await?;

        Ok(())
    }

    async fn rehash(jvm: &Jvm, this: &mut ClassInstanceRef<Self>) -> Result<()> {
        let old_table: ClassInstanceRef<Array<HashMapEntry>> = jvm.get_field(this, "table", "[Ljava/util/HashMap$Entry;").await?;
        let old_capacity = jvm.array_length(&old_table).await?;
        let new_capacity = if old_capacity == 0 { 1 } else { old_capacity * 2 + 1 };

        let mut new_table: ClassInstanceRef<Array<HashMapEntry>> = jvm.instantiate_array("Ljava/util/HashMap$Entry;", new_capacity).await?.into();
        for bucket_index in 0..old_capacity {
            let mut entry = Self::load_bucket(jvm, &old_table, bucket_index).await?;
            while !entry.is_null() {
                let next: ClassInstanceRef<HashMapEntry> = jvm.get_field(&entry, "next", "Ljava/util/HashMap$Entry;").await?;
                let entry_hash: i32 = jvm.get_field(&entry, "hash", "I").await?;
                let Some(new_index) = Self::bucket_index(entry_hash, new_capacity as i32) else {
                    return Err(jvm.exception("java/lang/RuntimeException", "HashMap table is empty").await);
                };

                let existing = Self::load_bucket(jvm, &new_table, new_index).await?;
                jvm.put_field(&mut entry, "next", "Ljava/util/HashMap$Entry;", existing).await?;
                jvm.store_array(&mut new_table, new_index, core::iter::once(entry)).await?;

                entry = next;
            }
        }

        jvm.put_field(this, "table", "[Ljava/util/HashMap$Entry;", new_table).await?;
        jvm.put_field(this, "threshold", "I", Self::threshold_for_capacity(new_capacity as i32))
            .await?;

        Ok(())
    }

    async fn load_bucket(jvm: &Jvm, table: &ClassInstanceRef<Array<HashMapEntry>>, bucket_index: usize) -> Result<ClassInstanceRef<HashMapEntry>> {
        let mut entries = jvm.load_array(table, bucket_index, 1).await?;

        Ok(entries.pop().unwrap_or_else(|| None.into()))
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

    async fn keys_equal(jvm: &Jvm, query_key: &ClassInstanceRef<Object>, stored_key: &ClassInstanceRef<Object>) -> Result<bool> {
        if query_key.is_null() {
            return Ok(stored_key.is_null());
        }

        if stored_key.is_null() {
            return Ok(false);
        }

        jvm.invoke_virtual(query_key, "equals", "(Ljava/lang/Object;)Z", (stored_key.clone(),))
            .await
    }

    async fn object_hash_or_zero(jvm: &Jvm, value: &ClassInstanceRef<Object>) -> Result<i32> {
        if value.is_null() {
            return Ok(0);
        }

        jvm.invoke_virtual(value, "hashCode", "()I", ()).await
    }

    fn bucket_index(hash: i32, table_len: i32) -> Option<usize> {
        if table_len <= 0 {
            return None;
        }

        Some(((hash & 0x7FFFFFFF) % table_len) as usize)
    }

    async fn snapshot_entries(jvm: &Jvm, this: &ClassInstanceRef<Self>, kind: SnapshotKind) -> Result<ClassInstanceRef<Array<Object>>> {
        let table: ClassInstanceRef<Array<HashMapEntry>> = jvm.get_field(this, "table", "[Ljava/util/HashMap$Entry;").await?;
        let table_len = jvm.array_length(&table).await?;
        let size: i32 = jvm.get_field(this, "size", "I").await?;
        let mut elements: Vec<ClassInstanceRef<Object>> = Vec::with_capacity(size.max(0) as usize);

        for bucket_index in 0..table_len {
            let mut entry = Self::load_bucket(jvm, &table, bucket_index).await?;
            while !entry.is_null() {
                let element = match kind {
                    SnapshotKind::Keys => jvm.get_field(&entry, "key", "Ljava/lang/Object;").await?,
                    SnapshotKind::Values => jvm.get_field(&entry, "value", "Ljava/lang/Object;").await?,
                    SnapshotKind::Entries => ClassInstanceRef::new(entry.clone().instance),
                };
                elements.push(element);

                entry = jvm.get_field(&entry, "next", "Ljava/util/HashMap$Entry;").await?;
            }
        }

        let mut snapshot: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", elements.len()).await?.into();
        if !elements.is_empty() {
            jvm.store_array(&mut snapshot, 0, elements).await?;
        }

        Ok(snapshot)
    }

    fn threshold_for_capacity(capacity: i32) -> i32 {
        if capacity <= 0 {
            0
        } else {
            (capacity as f32 * DEFAULT_LOAD_FACTOR) as i32
        }
    }
}

#[derive(Copy, Clone)]
enum SnapshotKind {
    Keys,
    Values,
    Entries,
}
