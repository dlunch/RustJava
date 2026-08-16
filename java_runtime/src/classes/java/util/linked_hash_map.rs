use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

use super::{HashMap, HashMapEntry, LinkedHashMapEntry};

const DEFAULT_INITIAL_CAPACITY: i32 = 16;
const DEFAULT_LOAD_FACTOR: f32 = 0.75;

// public class java.util.LinkedHashMap
pub struct LinkedHashMap;

impl LinkedHashMap {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/LinkedHashMap",
            parent_class: Some("java/util/HashMap"),
            interfaces: vec!["java/util/Map"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init_default, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(I)V", Self::init_with_capacity, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(IF)V", Self::init_with_capacity_and_load_factor, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "<init>",
                    "(IFZ)V",
                    Self::init_with_capacity_load_factor_and_order,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("<init>", "(Ljava/util/Map;)V", Self::init_from_map, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("containsValue", "(Ljava/lang/Object;)Z", Self::contains_value, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("get", "(Ljava/lang/Object;)Ljava/lang/Object;", Self::get, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("clear", "()V", Self::clear, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "removeEldestEntry",
                    "(Ljava/util/Map$Entry;)Z",
                    Self::remove_eldest_entry,
                    MethodAccessFlags::PROTECTED,
                ),
                JavaMethodProto::new("initializeMap", "()V", Self::initialize_map, MethodAccessFlags::empty()),
                JavaMethodProto::new(
                    "storeNewEntry",
                    "(ILjava/lang/Object;Ljava/lang/Object;I)V",
                    Self::store_new_entry,
                    MethodAccessFlags::empty(),
                ),
                JavaMethodProto::new(
                    "insertNewEntry",
                    "(ILjava/lang/Object;Ljava/lang/Object;I)V",
                    Self::insert_new_entry,
                    MethodAccessFlags::empty(),
                ),
                JavaMethodProto::new("keyIterator", "()Ljava/util/Iterator;", Self::key_iterator, MethodAccessFlags::empty()),
                JavaMethodProto::new(
                    "valueIterator",
                    "()Ljava/util/Iterator;",
                    Self::value_iterator,
                    MethodAccessFlags::empty(),
                ),
                JavaMethodProto::new(
                    "entryIterator",
                    "()Ljava/util/Iterator;",
                    Self::entry_iterator,
                    MethodAccessFlags::empty(),
                ),
            ],
            fields: vec![
                JavaFieldProto::new(
                    "header",
                    "Ljava/util/LinkedHashMap$Entry;",
                    FieldAccessFlags::PRIVATE | FieldAccessFlags::TRANSIENT,
                ),
                JavaFieldProto::new("accessOrder", "Z", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init_default(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(
            &this,
            "java/util/LinkedHashMap",
            "<init>",
            "(IFZ)V",
            (DEFAULT_INITIAL_CAPACITY, DEFAULT_LOAD_FACTOR, false),
        )
        .await
    }

    async fn init_with_capacity(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, capacity: i32) -> Result<()> {
        jvm.invoke_special(
            &this,
            "java/util/LinkedHashMap",
            "<init>",
            "(IFZ)V",
            (capacity, DEFAULT_LOAD_FACTOR, false),
        )
        .await
    }

    async fn init_with_capacity_and_load_factor(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        capacity: i32,
        load_factor: f32,
    ) -> Result<()> {
        jvm.invoke_special(&this, "java/util/LinkedHashMap", "<init>", "(IFZ)V", (capacity, load_factor, false))
            .await
    }

    async fn init_with_capacity_load_factor_and_order(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        capacity: i32,
        load_factor: f32,
        access_order: bool,
    ) -> Result<()> {
        let _: () = jvm
            .invoke_special(&this, "java/util/HashMap", "<init>", "(IF)V", (capacity, load_factor))
            .await?;
        jvm.put_field(&mut this, "accessOrder", "Z", access_order).await
    }

    async fn init_from_map(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, map: ClassInstanceRef<Object>) -> Result<()> {
        if map.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "map").await);
        }
        let size: i32 = jvm.invoke_virtual(&map, &map.class_definition().name(), "size", "()I", ()).await?;
        let capacity = size.saturating_mul(2).max(DEFAULT_INITIAL_CAPACITY);
        let _: () = jvm
            .invoke_special(
                &this,
                "java/util/LinkedHashMap",
                "<init>",
                "(IFZ)V",
                (capacity, DEFAULT_LOAD_FACTOR, false),
            )
            .await?;

        let entry_set: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&map, &map.class_definition().name(), "entrySet", "()Ljava/util/Set;", ())
            .await?;
        let entries: ClassInstanceRef<Array<Object>> = jvm
            .invoke_virtual(&entry_set, &entry_set.class_definition().name(), "toArray", "()[Ljava/lang/Object;", ())
            .await?;
        let count = jvm.array_length(&entries).await?;
        let mut hash_map: ClassInstanceRef<HashMap> = ClassInstanceRef::new(this.instance);
        for entry in jvm.load_array::<ClassInstanceRef<Object>>(&entries, 0, count).await? {
            let key: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&entry, &entry.class_definition().name(), "getKey", "()Ljava/lang/Object;", ())
                .await?;
            let value: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&entry, &entry.class_definition().name(), "getValue", "()Ljava/lang/Object;", ())
                .await?;
            HashMap::put_for_create(jvm, &mut hash_map, key, value).await?;
        }

        Ok(())
    }

    async fn initialize_map(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let null: ClassInstanceRef<Object> = None.into();
        let next: ClassInstanceRef<HashMapEntry> = None.into();
        let mut header: ClassInstanceRef<LinkedHashMapEntry> = jvm
            .new_class(
                "java/util/LinkedHashMap$Entry",
                "(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Entry;)V",
                (-1, null.clone(), null, next),
            )
            .await?
            .into();
        let header_ref = header.clone();
        jvm.put_field(&mut header, "before", "Ljava/util/LinkedHashMap$Entry;", header_ref)
            .await?;
        let header_ref = header.clone();
        jvm.put_field(&mut header, "after", "Ljava/util/LinkedHashMap$Entry;", header_ref).await?;
        jvm.put_field(&mut this, "header", "Ljava/util/LinkedHashMap$Entry;", header).await
    }

    async fn contains_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Object>) -> Result<bool> {
        let header: ClassInstanceRef<LinkedHashMapEntry> = jvm.get_field(&this, "header", "Ljava/util/LinkedHashMap$Entry;").await?;
        let mut entry: ClassInstanceRef<LinkedHashMapEntry> = jvm.get_field(&header, "after", "Ljava/util/LinkedHashMap$Entry;").await?;
        while entry.identity() != header.identity() {
            let entry_value: ClassInstanceRef<Object> = jvm.get_field(&entry, "value", "Ljava/lang/Object;").await?;
            let equal = if value.is_null() {
                entry_value.is_null()
            } else if entry_value.is_null() {
                false
            } else {
                jvm.invoke_virtual(&value, "java/lang/Object", "equals", "(Ljava/lang/Object;)Z", (entry_value,))
                    .await?
            };
            if equal {
                return Ok(true);
            }
            entry = jvm.get_field(&entry, "after", "Ljava/util/LinkedHashMap$Entry;").await?;
        }

        Ok(false)
    }

    async fn get(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key: ClassInstanceRef<Object>) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<HashMap> = ClassInstanceRef::new(this.instance.clone());
        let entry = HashMap::find_entry(jvm, &map, &key).await?;
        if entry.is_null() {
            return Ok(None.into());
        }
        let _: () = jvm
            .invoke_virtual(&entry, "java/util/HashMap$Entry", "onAccess", "(Ljava/util/HashMap;)V", (map,))
            .await?;

        jvm.get_field(&entry, "value", "Ljava/lang/Object;").await
    }

    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/util/HashMap", "clear", "()V", ()).await?;
        let mut header: ClassInstanceRef<LinkedHashMapEntry> = jvm.get_field(&this, "header", "Ljava/util/LinkedHashMap$Entry;").await?;
        let header_ref = header.clone();
        jvm.put_field(&mut header, "before", "Ljava/util/LinkedHashMap$Entry;", header_ref)
            .await?;
        let header_ref = header.clone();
        jvm.put_field(&mut header, "after", "Ljava/util/LinkedHashMap$Entry;", header_ref).await
    }

    async fn store_new_entry(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        hash: i32,
        key: ClassInstanceRef<Object>,
        value: ClassInstanceRef<Object>,
        bucket_index: i32,
    ) -> Result<()> {
        let mut table: ClassInstanceRef<Array<HashMapEntry>> = jvm.get_field(&this, "table", "[Ljava/util/HashMap$Entry;").await?;
        let existing = jvm
            .load_array::<ClassInstanceRef<HashMapEntry>>(&table, bucket_index as usize, 1)
            .await?
            .remove(0);
        let mut entry: ClassInstanceRef<LinkedHashMapEntry> = jvm
            .new_class(
                "java/util/LinkedHashMap$Entry",
                "(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Entry;)V",
                (hash, key, value, existing),
            )
            .await?
            .into();
        let bucket_entry: ClassInstanceRef<HashMapEntry> = ClassInstanceRef::new(entry.instance.clone());
        jvm.store_array(&mut table, bucket_index as usize, core::iter::once(bucket_entry)).await?;

        let mut header: ClassInstanceRef<LinkedHashMapEntry> = jvm.get_field(&this, "header", "Ljava/util/LinkedHashMap$Entry;").await?;
        let mut tail: ClassInstanceRef<LinkedHashMapEntry> = jvm.get_field(&header, "before", "Ljava/util/LinkedHashMap$Entry;").await?;
        jvm.put_field(&mut entry, "before", "Ljava/util/LinkedHashMap$Entry;", tail.clone())
            .await?;
        jvm.put_field(&mut entry, "after", "Ljava/util/LinkedHashMap$Entry;", header.clone())
            .await?;
        jvm.put_field(&mut tail, "after", "Ljava/util/LinkedHashMap$Entry;", entry.clone())
            .await?;
        jvm.put_field(&mut header, "before", "Ljava/util/LinkedHashMap$Entry;", entry).await?;

        let size: i32 = jvm.get_field(&this, "size", "I").await?;
        jvm.put_field(&mut this, "size", "I", size + 1).await
    }

    async fn insert_new_entry(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        hash: i32,
        key: ClassInstanceRef<Object>,
        value: ClassInstanceRef<Object>,
        bucket_index: i32,
    ) -> Result<()> {
        let size: i32 = jvm.get_field(&this, "size", "I").await?;
        let mod_count: i32 = jvm.get_field(&this, "modCount", "I").await?;
        jvm.put_field(&mut this, "modCount", "I", mod_count.wrapping_add(1)).await?;
        let _: () = jvm
            .invoke_virtual(
                &this,
                "java/util/HashMap",
                "storeNewEntry",
                "(ILjava/lang/Object;Ljava/lang/Object;I)V",
                (hash, key, value, bucket_index),
            )
            .await?;

        let header: ClassInstanceRef<LinkedHashMapEntry> = jvm.get_field(&this, "header", "Ljava/util/LinkedHashMap$Entry;").await?;
        let eldest: ClassInstanceRef<LinkedHashMapEntry> = jvm.get_field(&header, "after", "Ljava/util/LinkedHashMap$Entry;").await?;
        let eldest_entry: ClassInstanceRef<Object> = ClassInstanceRef::new(eldest.instance.clone());
        if jvm
            .invoke_virtual::<_, bool>(
                &this,
                "java/util/LinkedHashMap",
                "removeEldestEntry",
                "(Ljava/util/Map$Entry;)Z",
                (eldest_entry,),
            )
            .await?
        {
            let key: ClassInstanceRef<Object> = jvm.get_field(&eldest, "key", "Ljava/lang/Object;").await?;
            let _: ClassInstanceRef<Object> = jvm
                .invoke_virtual(
                    &this,
                    "java/util/LinkedHashMap",
                    "remove",
                    "(Ljava/lang/Object;)Ljava/lang/Object;",
                    (key,),
                )
                .await?;
        } else {
            let threshold: i32 = jvm.get_field(&this, "threshold", "I").await?;
            if size >= threshold {
                let mut map: ClassInstanceRef<HashMap> = ClassInstanceRef::new(this.instance.clone());
                HashMap::rehash(jvm, &mut map).await?;
            }
        }

        Ok(())
    }

    async fn remove_eldest_entry(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<bool> {
        Ok(false)
    }

    async fn key_iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        Ok(jvm
            .new_class("java/util/LinkedHashMap$KeyIterator", "(Ljava/util/LinkedHashMap;)V", (this,))
            .await?
            .into())
    }

    async fn value_iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        Ok(jvm
            .new_class("java/util/LinkedHashMap$ValueIterator", "(Ljava/util/LinkedHashMap;)V", (this,))
            .await?
            .into())
    }

    async fn entry_iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        Ok(jvm
            .new_class("java/util/LinkedHashMap$EntryIterator", "(Ljava/util/LinkedHashMap;)V", (this,))
            .await?
            .into())
    }
}
