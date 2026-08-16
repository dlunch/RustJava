use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

use super::TreeMapEntry;

// public class java.util.TreeMap
pub struct TreeMap;

impl TreeMap {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/TreeMap",
            parent_class: Some("java/util/AbstractMap"),
            interfaces: vec!["java/util/SortedMap", "java/lang/Cloneable", "java/io/Serializable"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(Ljava/util/Comparator;)V", Self::init_comparator, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(Ljava/util/Map;)V", Self::init_map, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(Ljava/util/SortedMap;)V", Self::init_sorted_map, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("size", "()I", Self::size, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("containsKey", "(Ljava/lang/Object;)Z", Self::contains_key, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("containsValue", "(Ljava/lang/Object;)Z", Self::contains_value, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("get", "(Ljava/lang/Object;)Ljava/lang/Object;", Self::get, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "put",
                    "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                    Self::put,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("putAll", "(Ljava/util/Map;)V", Self::put_all, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "remove",
                    "(Ljava/lang/Object;)Ljava/lang/Object;",
                    Self::remove,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("clear", "()V", Self::clear, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("comparator", "()Ljava/util/Comparator;", Self::comparator, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("firstKey", "()Ljava/lang/Object;", Self::first_key, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("lastKey", "()Ljava/lang/Object;", Self::last_key, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "subMap",
                    "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;",
                    Self::sub_map,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "headMap",
                    "(Ljava/lang/Object;)Ljava/util/SortedMap;",
                    Self::head_map,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "tailMap",
                    "(Ljava/lang/Object;)Ljava/util/SortedMap;",
                    Self::tail_map,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("keySet", "()Ljava/util/Set;", Self::key_set, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("values", "()Ljava/util/Collection;", Self::values, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("entrySet", "()Ljava/util/Set;", Self::entry_set, MethodAccessFlags::PUBLIC),
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
                    "root",
                    "Ljava/util/TreeMap$Entry;",
                    FieldAccessFlags::PRIVATE | FieldAccessFlags::TRANSIENT,
                ),
                JavaFieldProto::new("size", "I", FieldAccessFlags::PRIVATE | FieldAccessFlags::TRANSIENT),
                JavaFieldProto::new("comparator", "Ljava/util/Comparator;", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/util/AbstractMap", "<init>", "()V", ()).await?;
        Ok(())
    }

    async fn init_comparator(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        comparator: ClassInstanceRef<Object>,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/util/AbstractMap", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "comparator", "Ljava/util/Comparator;", comparator).await
    }

    async fn init_map(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, map: ClassInstanceRef<Object>) -> Result<()> {
        if map.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "map").await);
        }
        let _: () = jvm.invoke_special(&this, "java/util/TreeMap", "<init>", "()V", ()).await?;
        Self::copy_from_map(jvm, this, map).await
    }

    async fn init_sorted_map(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, map: ClassInstanceRef<Object>) -> Result<()> {
        if map.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "map").await);
        }
        let comparator: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&map, &map.class_definition().name(), "comparator", "()Ljava/util/Comparator;", ())
            .await?;
        let _: () = jvm
            .invoke_special(&this, "java/util/TreeMap", "<init>", "(Ljava/util/Comparator;)V", (comparator,))
            .await?;
        Self::copy_from_map(jvm, this, map).await
    }

    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "size", "I").await
    }

    async fn contains_key(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key: ClassInstanceRef<Object>) -> Result<bool> {
        Ok(!Self::find_entry(jvm, &this, &key).await?.is_null())
    }

    async fn contains_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Object>) -> Result<bool> {
        let mut entry = Self::first_entry(jvm, &this).await?;
        while !entry.is_null() {
            let current: ClassInstanceRef<Object> = jvm.get_field(&entry, "value", "Ljava/lang/Object;").await?;
            let equal = if value.is_null() {
                current.is_null()
            } else {
                jvm.invoke_virtual(&value, "java/lang/Object", "equals", "(Ljava/lang/Object;)Z", (current,))
                    .await?
            };
            if equal {
                return Ok(true);
            }
            entry = Self::successor(jvm, entry).await?;
        }
        Ok(false)
    }

    async fn get(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key: ClassInstanceRef<Object>) -> Result<ClassInstanceRef<Object>> {
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
        let root: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&this, "root", "Ljava/util/TreeMap$Entry;").await?;
        if root.is_null() {
            let _ = Self::compare(jvm, &this, &key, &key).await?;
            let entry: ClassInstanceRef<TreeMapEntry> = jvm
                .new_class(
                    "java/util/TreeMap$Entry",
                    "(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/TreeMap$Entry;)V",
                    (key, value, ClassInstanceRef::<TreeMapEntry>::from(None)),
                )
                .await?
                .into();
            jvm.put_field(&mut this, "root", "Ljava/util/TreeMap$Entry;", entry).await?;
            jvm.put_field(&mut this, "size", "I", 1).await?;
            return Ok(None.into());
        }

        let mut parent = root;
        let comparison;
        loop {
            let stored_key: ClassInstanceRef<Object> = jvm.get_field(&parent, "key", "Ljava/lang/Object;").await?;
            let current_comparison = Self::compare(jvm, &this, &key, &stored_key).await?;
            if current_comparison == 0 {
                let old_value: ClassInstanceRef<Object> = jvm.get_field(&parent, "value", "Ljava/lang/Object;").await?;
                let mut parent = parent;
                jvm.put_field(&mut parent, "value", "Ljava/lang/Object;", value).await?;
                return Ok(old_value);
            }
            let child: ClassInstanceRef<TreeMapEntry> = if current_comparison < 0 {
                jvm.get_field(&parent, "left", "Ljava/util/TreeMap$Entry;").await?
            } else {
                jvm.get_field(&parent, "right", "Ljava/util/TreeMap$Entry;").await?
            };
            if child.is_null() {
                comparison = current_comparison;
                break;
            }
            parent = child;
        }

        let mut entry: ClassInstanceRef<TreeMapEntry> = jvm
            .new_class(
                "java/util/TreeMap$Entry",
                "(Ljava/lang/Object;Ljava/lang/Object;Ljava/util/TreeMap$Entry;)V",
                (key, value, parent.clone()),
            )
            .await?
            .into();
        jvm.put_field(&mut entry, "color", "Z", false).await?;
        let mut parent = parent;
        if comparison < 0 {
            jvm.put_field(&mut parent, "left", "Ljava/util/TreeMap$Entry;", entry.clone()).await?;
        } else {
            jvm.put_field(&mut parent, "right", "Ljava/util/TreeMap$Entry;", entry.clone()).await?;
        }
        let size: i32 = jvm.get_field(&this, "size", "I").await?;
        jvm.put_field(&mut this, "size", "I", size + 1).await?;
        Self::fix_after_insertion(jvm, &mut this, entry).await?;
        Ok(None.into())
    }

    async fn put_all(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, map: ClassInstanceRef<Object>) -> Result<()> {
        if map.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "map").await);
        }
        if this.identity() == map.identity() {
            return Ok(());
        }
        Self::copy_from_map(jvm, this, map).await
    }

    async fn remove(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        key: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        let entry = Self::find_entry(jvm, &this, &key).await?;
        if entry.is_null() {
            return Ok(None.into());
        }
        let old_value: ClassInstanceRef<Object> = jvm.get_field(&entry, "value", "Ljava/lang/Object;").await?;
        Self::delete_entry(jvm, &mut this, entry).await?;
        Ok(old_value)
    }

    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.put_field(
            &mut this,
            "root",
            "Ljava/util/TreeMap$Entry;",
            ClassInstanceRef::<TreeMapEntry>::from(None),
        )
        .await?;
        jvm.put_field(&mut this, "size", "I", 0).await
    }

    async fn comparator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        jvm.get_field(&this, "comparator", "Ljava/util/Comparator;").await
    }

    async fn first_key(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let entry = Self::first_entry(jvm, &this).await?;
        if entry.is_null() {
            return Err(jvm.exception("java/util/NoSuchElementException", "empty TreeMap").await);
        }
        jvm.get_field(&entry, "key", "Ljava/lang/Object;").await
    }

    async fn last_key(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let entry = Self::last_entry(jvm, &this).await?;
        if entry.is_null() {
            return Err(jvm.exception("java/util/NoSuchElementException", "empty TreeMap").await);
        }
        jvm.get_field(&entry, "key", "Ljava/lang/Object;").await
    }

    async fn sub_map(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        from_key: ClassInstanceRef<Object>,
        to_key: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        if Self::compare(jvm, &this, &from_key, &to_key).await? > 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "fromKey > toKey").await);
        }
        Ok(jvm
            .new_class(
                "java/util/TreeMap$SubMap",
                "(Ljava/util/TreeMap;ZLjava/lang/Object;ZLjava/lang/Object;)V",
                (this, false, from_key, false, to_key),
            )
            .await?
            .into())
    }

    async fn head_map(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        to_key: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        let _ = Self::compare(jvm, &this, &to_key, &to_key).await?;
        Ok(jvm
            .new_class(
                "java/util/TreeMap$SubMap",
                "(Ljava/util/TreeMap;ZLjava/lang/Object;ZLjava/lang/Object;)V",
                (this, true, ClassInstanceRef::<Object>::from(None), false, to_key),
            )
            .await?
            .into())
    }

    async fn tail_map(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        from_key: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        let _ = Self::compare(jvm, &this, &from_key, &from_key).await?;
        Ok(jvm
            .new_class(
                "java/util/TreeMap$SubMap",
                "(Ljava/util/TreeMap;ZLjava/lang/Object;ZLjava/lang/Object;)V",
                (this, false, from_key, true, ClassInstanceRef::<Object>::from(None)),
            )
            .await?
            .into())
    }

    async fn key_set(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        Ok(jvm
            .new_class("java/util/TreeMap$KeySet", "(Ljava/util/SortedMap;)V", (this,))
            .await?
            .into())
    }

    async fn values(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        Ok(jvm
            .new_class("java/util/TreeMap$Values", "(Ljava/util/SortedMap;)V", (this,))
            .await?
            .into())
    }

    async fn entry_set(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        Ok(jvm
            .new_class("java/util/TreeMap$EntrySet", "(Ljava/util/SortedMap;)V", (this,))
            .await?
            .into())
    }

    async fn key_iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let first = Self::first_entry(jvm, &this).await?;
        Ok(jvm
            .new_class(
                "java/util/TreeMap$KeyIterator",
                "(Ljava/util/TreeMap;Ljava/util/TreeMap$Entry;Ljava/lang/Object;Z)V",
                (this, first, ClassInstanceRef::<Object>::from(None), true),
            )
            .await?
            .into())
    }

    async fn value_iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let first = Self::first_entry(jvm, &this).await?;
        Ok(jvm
            .new_class(
                "java/util/TreeMap$ValueIterator",
                "(Ljava/util/TreeMap;Ljava/util/TreeMap$Entry;Ljava/lang/Object;Z)V",
                (this, first, ClassInstanceRef::<Object>::from(None), true),
            )
            .await?
            .into())
    }

    async fn entry_iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let first = Self::first_entry(jvm, &this).await?;
        Ok(jvm
            .new_class(
                "java/util/TreeMap$EntryIterator",
                "(Ljava/util/TreeMap;Ljava/util/TreeMap$Entry;Ljava/lang/Object;Z)V",
                (this, first, ClassInstanceRef::<Object>::from(None), true),
            )
            .await?
            .into())
    }

    async fn copy_from_map(jvm: &Jvm, this: ClassInstanceRef<Self>, map: ClassInstanceRef<Object>) -> Result<()> {
        let entry_set: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&map, &map.class_definition().name(), "entrySet", "()Ljava/util/Set;", ())
            .await?;
        let iterator: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&entry_set, &entry_set.class_definition().name(), "iterator", "()Ljava/util/Iterator;", ())
            .await?;
        while jvm
            .invoke_virtual::<_, bool>(&iterator, &iterator.class_definition().name(), "hasNext", "()Z", ())
            .await?
        {
            let entry: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&iterator, &iterator.class_definition().name(), "next", "()Ljava/lang/Object;", ())
                .await?;
            let key: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&entry, &entry.class_definition().name(), "getKey", "()Ljava/lang/Object;", ())
                .await?;
            let value: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&entry, &entry.class_definition().name(), "getValue", "()Ljava/lang/Object;", ())
                .await?;
            let _: ClassInstanceRef<Object> = jvm
                .invoke_virtual(
                    &this,
                    "java/util/TreeMap",
                    "put",
                    "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                    (key, value),
                )
                .await?;
        }
        Ok(())
    }

    pub(super) async fn compare(
        jvm: &Jvm,
        this: &ClassInstanceRef<Self>,
        left: &ClassInstanceRef<Object>,
        right: &ClassInstanceRef<Object>,
    ) -> Result<i32> {
        let comparator: ClassInstanceRef<Object> = jvm.get_field(this, "comparator", "Ljava/util/Comparator;").await?;
        if !comparator.is_null() {
            return jvm
                .invoke_virtual(
                    &comparator,
                    &comparator.class_definition().name(),
                    "compare",
                    "(Ljava/lang/Object;Ljava/lang/Object;)I",
                    (left.clone(), right.clone()),
                )
                .await;
        }
        if left.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "null key").await);
        }
        if !jvm.is_instance(left.as_ref(), "java/lang/Comparable") {
            return Err(jvm.exception("java/lang/ClassCastException", "key is not Comparable").await);
        }
        jvm.invoke_virtual(
            left,
            &left.class_definition().name(),
            "compareTo",
            "(Ljava/lang/Object;)I",
            (right.clone(),),
        )
        .await
    }

    pub(super) async fn find_entry(
        jvm: &Jvm,
        this: &ClassInstanceRef<Self>,
        key: &ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<TreeMapEntry>> {
        let comparator: ClassInstanceRef<Object> = jvm.get_field(this, "comparator", "Ljava/util/Comparator;").await?;
        if comparator.is_null() {
            if key.is_null() {
                return Err(jvm.exception("java/lang/NullPointerException", "null key").await);
            }
            if !jvm.is_instance(key.as_ref(), "java/lang/Comparable") {
                return Err(jvm.exception("java/lang/ClassCastException", "key is not Comparable").await);
            }
        }

        let mut entry: ClassInstanceRef<TreeMapEntry> = jvm.get_field(this, "root", "Ljava/util/TreeMap$Entry;").await?;
        while !entry.is_null() {
            let stored_key: ClassInstanceRef<Object> = jvm.get_field(&entry, "key", "Ljava/lang/Object;").await?;
            let comparison: i32 = if comparator.is_null() {
                jvm.invoke_virtual(key, &key.class_definition().name(), "compareTo", "(Ljava/lang/Object;)I", (stored_key,))
                    .await?
            } else {
                jvm.invoke_virtual(
                    &comparator,
                    &comparator.class_definition().name(),
                    "compare",
                    "(Ljava/lang/Object;Ljava/lang/Object;)I",
                    (key.clone(), stored_key),
                )
                .await?
            };
            if comparison == 0 {
                return Ok(entry);
            }
            entry = if comparison < 0 {
                jvm.get_field(&entry, "left", "Ljava/util/TreeMap$Entry;").await?
            } else {
                jvm.get_field(&entry, "right", "Ljava/util/TreeMap$Entry;").await?
            };
        }
        Ok(None.into())
    }

    pub(super) async fn first_entry(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<ClassInstanceRef<TreeMapEntry>> {
        let mut entry: ClassInstanceRef<TreeMapEntry> = jvm.get_field(this, "root", "Ljava/util/TreeMap$Entry;").await?;
        if entry.is_null() {
            return Ok(entry);
        }
        loop {
            let left: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&entry, "left", "Ljava/util/TreeMap$Entry;").await?;
            if left.is_null() {
                return Ok(entry);
            }
            entry = left;
        }
    }

    pub(super) async fn last_entry(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<ClassInstanceRef<TreeMapEntry>> {
        let mut entry: ClassInstanceRef<TreeMapEntry> = jvm.get_field(this, "root", "Ljava/util/TreeMap$Entry;").await?;
        if entry.is_null() {
            return Ok(entry);
        }
        loop {
            let right: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&entry, "right", "Ljava/util/TreeMap$Entry;").await?;
            if right.is_null() {
                return Ok(entry);
            }
            entry = right;
        }
    }

    pub(super) async fn ceiling_entry(
        jvm: &Jvm,
        this: &ClassInstanceRef<Self>,
        key: &ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<TreeMapEntry>> {
        let mut entry: ClassInstanceRef<TreeMapEntry> = jvm.get_field(this, "root", "Ljava/util/TreeMap$Entry;").await?;
        let mut candidate: ClassInstanceRef<TreeMapEntry> = None.into();
        while !entry.is_null() {
            let stored_key: ClassInstanceRef<Object> = jvm.get_field(&entry, "key", "Ljava/lang/Object;").await?;
            let comparison = Self::compare(jvm, this, key, &stored_key).await?;
            if comparison == 0 {
                return Ok(entry);
            }
            if comparison < 0 {
                candidate = entry.clone();
                entry = jvm.get_field(&entry, "left", "Ljava/util/TreeMap$Entry;").await?;
            } else {
                entry = jvm.get_field(&entry, "right", "Ljava/util/TreeMap$Entry;").await?;
            }
        }
        Ok(candidate)
    }

    pub(super) async fn lower_entry(
        jvm: &Jvm,
        this: &ClassInstanceRef<Self>,
        key: &ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<TreeMapEntry>> {
        let mut entry: ClassInstanceRef<TreeMapEntry> = jvm.get_field(this, "root", "Ljava/util/TreeMap$Entry;").await?;
        let mut candidate: ClassInstanceRef<TreeMapEntry> = None.into();
        while !entry.is_null() {
            let stored_key: ClassInstanceRef<Object> = jvm.get_field(&entry, "key", "Ljava/lang/Object;").await?;
            if Self::compare(jvm, this, key, &stored_key).await? <= 0 {
                entry = jvm.get_field(&entry, "left", "Ljava/util/TreeMap$Entry;").await?;
            } else {
                candidate = entry.clone();
                entry = jvm.get_field(&entry, "right", "Ljava/util/TreeMap$Entry;").await?;
            }
        }
        Ok(candidate)
    }

    pub(super) async fn successor(jvm: &Jvm, entry: ClassInstanceRef<TreeMapEntry>) -> Result<ClassInstanceRef<TreeMapEntry>> {
        if entry.is_null() {
            return Ok(None.into());
        }
        let right: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&entry, "right", "Ljava/util/TreeMap$Entry;").await?;
        if !right.is_null() {
            let mut candidate = right;
            loop {
                let left: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&candidate, "left", "Ljava/util/TreeMap$Entry;").await?;
                if left.is_null() {
                    return Ok(candidate);
                }
                candidate = left;
            }
        }

        let mut child = entry;
        let mut parent: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&child, "parent", "Ljava/util/TreeMap$Entry;").await?;
        while !parent.is_null() {
            let parent_right: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&parent, "right", "Ljava/util/TreeMap$Entry;").await?;
            if parent_right.is_null() || child.identity() != parent_right.identity() {
                break;
            }
            child = parent;
            parent = jvm.get_field(&child, "parent", "Ljava/util/TreeMap$Entry;").await?;
        }
        Ok(parent)
    }

    async fn rotate_left(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, mut pivot: ClassInstanceRef<TreeMapEntry>) -> Result<()> {
        let mut right: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&pivot, "right", "Ljava/util/TreeMap$Entry;").await?;
        let right_left: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&right, "left", "Ljava/util/TreeMap$Entry;").await?;
        jvm.put_field(&mut pivot, "right", "Ljava/util/TreeMap$Entry;", right_left.clone())
            .await?;
        if !right_left.is_null() {
            let mut right_left = right_left;
            jvm.put_field(&mut right_left, "parent", "Ljava/util/TreeMap$Entry;", pivot.clone())
                .await?;
        }
        let parent: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&pivot, "parent", "Ljava/util/TreeMap$Entry;").await?;
        jvm.put_field(&mut right, "parent", "Ljava/util/TreeMap$Entry;", parent.clone()).await?;
        if parent.is_null() {
            jvm.put_field(this, "root", "Ljava/util/TreeMap$Entry;", right.clone()).await?;
        } else {
            let parent_left: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&parent, "left", "Ljava/util/TreeMap$Entry;").await?;
            let mut parent = parent;
            if !parent_left.is_null() && parent_left.identity() == pivot.identity() {
                jvm.put_field(&mut parent, "left", "Ljava/util/TreeMap$Entry;", right.clone()).await?;
            } else {
                jvm.put_field(&mut parent, "right", "Ljava/util/TreeMap$Entry;", right.clone()).await?;
            }
        }
        jvm.put_field(&mut right, "left", "Ljava/util/TreeMap$Entry;", pivot.clone()).await?;
        jvm.put_field(&mut pivot, "parent", "Ljava/util/TreeMap$Entry;", right).await
    }

    async fn rotate_right(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, mut pivot: ClassInstanceRef<TreeMapEntry>) -> Result<()> {
        let mut left: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&pivot, "left", "Ljava/util/TreeMap$Entry;").await?;
        let left_right: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&left, "right", "Ljava/util/TreeMap$Entry;").await?;
        jvm.put_field(&mut pivot, "left", "Ljava/util/TreeMap$Entry;", left_right.clone()).await?;
        if !left_right.is_null() {
            let mut left_right = left_right;
            jvm.put_field(&mut left_right, "parent", "Ljava/util/TreeMap$Entry;", pivot.clone())
                .await?;
        }
        let parent: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&pivot, "parent", "Ljava/util/TreeMap$Entry;").await?;
        jvm.put_field(&mut left, "parent", "Ljava/util/TreeMap$Entry;", parent.clone()).await?;
        if parent.is_null() {
            jvm.put_field(this, "root", "Ljava/util/TreeMap$Entry;", left.clone()).await?;
        } else {
            let parent_right: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&parent, "right", "Ljava/util/TreeMap$Entry;").await?;
            let mut parent = parent;
            if !parent_right.is_null() && parent_right.identity() == pivot.identity() {
                jvm.put_field(&mut parent, "right", "Ljava/util/TreeMap$Entry;", left.clone()).await?;
            } else {
                jvm.put_field(&mut parent, "left", "Ljava/util/TreeMap$Entry;", left.clone()).await?;
            }
        }
        jvm.put_field(&mut left, "right", "Ljava/util/TreeMap$Entry;", pivot.clone()).await?;
        jvm.put_field(&mut pivot, "parent", "Ljava/util/TreeMap$Entry;", left).await
    }

    async fn fix_after_insertion(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, mut entry: ClassInstanceRef<TreeMapEntry>) -> Result<()> {
        jvm.put_field(&mut entry, "color", "Z", false).await?;
        loop {
            let root: ClassInstanceRef<TreeMapEntry> = jvm.get_field(this, "root", "Ljava/util/TreeMap$Entry;").await?;
            if entry.identity() == root.identity() {
                break;
            }
            let mut parent: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&entry, "parent", "Ljava/util/TreeMap$Entry;").await?;
            if parent.is_null() || jvm.get_field::<bool>(&parent, "color", "Z").await? {
                break;
            }
            let mut grand: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&parent, "parent", "Ljava/util/TreeMap$Entry;").await?;
            let grand_left: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&grand, "left", "Ljava/util/TreeMap$Entry;").await?;
            if !grand_left.is_null() && grand_left.identity() == parent.identity() {
                let mut uncle: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&grand, "right", "Ljava/util/TreeMap$Entry;").await?;
                let uncle_is_red = !uncle.is_null() && !jvm.get_field::<bool>(&uncle, "color", "Z").await?;
                if uncle_is_red {
                    jvm.put_field(&mut parent, "color", "Z", true).await?;
                    jvm.put_field(&mut uncle, "color", "Z", true).await?;
                    jvm.put_field(&mut grand, "color", "Z", false).await?;
                    entry = grand;
                    continue;
                }
                let parent_right: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&parent, "right", "Ljava/util/TreeMap$Entry;").await?;
                if !parent_right.is_null() && parent_right.identity() == entry.identity() {
                    entry = parent.clone();
                    Self::rotate_left(jvm, this, entry.clone()).await?;
                    parent = jvm.get_field(&entry, "parent", "Ljava/util/TreeMap$Entry;").await?;
                    grand = jvm.get_field(&parent, "parent", "Ljava/util/TreeMap$Entry;").await?;
                }
                jvm.put_field(&mut parent, "color", "Z", true).await?;
                jvm.put_field(&mut grand, "color", "Z", false).await?;
                Self::rotate_right(jvm, this, grand).await?;
            } else {
                let mut uncle: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&grand, "left", "Ljava/util/TreeMap$Entry;").await?;
                let uncle_is_red = !uncle.is_null() && !jvm.get_field::<bool>(&uncle, "color", "Z").await?;
                if uncle_is_red {
                    jvm.put_field(&mut parent, "color", "Z", true).await?;
                    jvm.put_field(&mut uncle, "color", "Z", true).await?;
                    jvm.put_field(&mut grand, "color", "Z", false).await?;
                    entry = grand;
                    continue;
                }
                let parent_left: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&parent, "left", "Ljava/util/TreeMap$Entry;").await?;
                if !parent_left.is_null() && parent_left.identity() == entry.identity() {
                    entry = parent.clone();
                    Self::rotate_right(jvm, this, entry.clone()).await?;
                    parent = jvm.get_field(&entry, "parent", "Ljava/util/TreeMap$Entry;").await?;
                    grand = jvm.get_field(&parent, "parent", "Ljava/util/TreeMap$Entry;").await?;
                }
                jvm.put_field(&mut parent, "color", "Z", true).await?;
                jvm.put_field(&mut grand, "color", "Z", false).await?;
                Self::rotate_left(jvm, this, grand).await?;
            }
        }
        let mut root: ClassInstanceRef<TreeMapEntry> = jvm.get_field(this, "root", "Ljava/util/TreeMap$Entry;").await?;
        jvm.put_field(&mut root, "color", "Z", true).await?;
        Ok(())
    }

    pub(super) async fn delete_entry(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, entry: ClassInstanceRef<TreeMapEntry>) -> Result<()> {
        let size: i32 = jvm.get_field(this, "size", "I").await?;
        jvm.put_field(this, "size", "I", size - 1).await?;

        let mut target = entry;
        let left: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&target, "left", "Ljava/util/TreeMap$Entry;").await?;
        let right: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&target, "right", "Ljava/util/TreeMap$Entry;").await?;
        if !left.is_null() && !right.is_null() {
            let successor = Self::successor(jvm, target.clone()).await?;
            let key: ClassInstanceRef<Object> = jvm.get_field(&successor, "key", "Ljava/lang/Object;").await?;
            let value: ClassInstanceRef<Object> = jvm.get_field(&successor, "value", "Ljava/lang/Object;").await?;
            jvm.put_field(&mut target, "key", "Ljava/lang/Object;", key).await?;
            jvm.put_field(&mut target, "value", "Ljava/lang/Object;", value).await?;
            target = successor;
        }

        let target_left: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&target, "left", "Ljava/util/TreeMap$Entry;").await?;
        let target_right: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&target, "right", "Ljava/util/TreeMap$Entry;").await?;
        let mut replacement = if !target_left.is_null() { target_left } else { target_right };
        if !replacement.is_null() {
            let parent: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&target, "parent", "Ljava/util/TreeMap$Entry;").await?;
            jvm.put_field(&mut replacement, "parent", "Ljava/util/TreeMap$Entry;", parent.clone())
                .await?;
            if parent.is_null() {
                jvm.put_field(this, "root", "Ljava/util/TreeMap$Entry;", replacement.clone()).await?;
            } else {
                let parent_left: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&parent, "left", "Ljava/util/TreeMap$Entry;").await?;
                let mut parent = parent;
                if !parent_left.is_null() && parent_left.identity() == target.identity() {
                    jvm.put_field(&mut parent, "left", "Ljava/util/TreeMap$Entry;", replacement.clone())
                        .await?;
                } else {
                    jvm.put_field(&mut parent, "right", "Ljava/util/TreeMap$Entry;", replacement.clone())
                        .await?;
                }
            }
            jvm.put_field(
                &mut target,
                "left",
                "Ljava/util/TreeMap$Entry;",
                ClassInstanceRef::<TreeMapEntry>::from(None),
            )
            .await?;
            jvm.put_field(
                &mut target,
                "right",
                "Ljava/util/TreeMap$Entry;",
                ClassInstanceRef::<TreeMapEntry>::from(None),
            )
            .await?;
            jvm.put_field(
                &mut target,
                "parent",
                "Ljava/util/TreeMap$Entry;",
                ClassInstanceRef::<TreeMapEntry>::from(None),
            )
            .await?;
            if jvm.get_field::<bool>(&target, "color", "Z").await? {
                Self::fix_after_deletion(jvm, this, replacement).await?;
            }
        } else {
            let parent: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&target, "parent", "Ljava/util/TreeMap$Entry;").await?;
            if parent.is_null() {
                jvm.put_field(this, "root", "Ljava/util/TreeMap$Entry;", ClassInstanceRef::<TreeMapEntry>::from(None))
                    .await?;
            } else {
                if jvm.get_field::<bool>(&target, "color", "Z").await? {
                    Self::fix_after_deletion(jvm, this, target.clone()).await?;
                }
                let parent: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&target, "parent", "Ljava/util/TreeMap$Entry;").await?;
                if !parent.is_null() {
                    let parent_left: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&parent, "left", "Ljava/util/TreeMap$Entry;").await?;
                    let parent_right: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&parent, "right", "Ljava/util/TreeMap$Entry;").await?;
                    let mut parent = parent;
                    if !parent_left.is_null() && parent_left.identity() == target.identity() {
                        jvm.put_field(
                            &mut parent,
                            "left",
                            "Ljava/util/TreeMap$Entry;",
                            ClassInstanceRef::<TreeMapEntry>::from(None),
                        )
                        .await?;
                    } else if !parent_right.is_null() && parent_right.identity() == target.identity() {
                        jvm.put_field(
                            &mut parent,
                            "right",
                            "Ljava/util/TreeMap$Entry;",
                            ClassInstanceRef::<TreeMapEntry>::from(None),
                        )
                        .await?;
                    }
                    jvm.put_field(
                        &mut target,
                        "parent",
                        "Ljava/util/TreeMap$Entry;",
                        ClassInstanceRef::<TreeMapEntry>::from(None),
                    )
                    .await?;
                }
            }
        }
        Ok(())
    }

    async fn fix_after_deletion(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, mut entry: ClassInstanceRef<TreeMapEntry>) -> Result<()> {
        loop {
            let root: ClassInstanceRef<TreeMapEntry> = jvm.get_field(this, "root", "Ljava/util/TreeMap$Entry;").await?;
            if entry.identity() == root.identity() || !jvm.get_field::<bool>(&entry, "color", "Z").await? {
                break;
            }
            let parent: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&entry, "parent", "Ljava/util/TreeMap$Entry;").await?;
            let parent_left: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&parent, "left", "Ljava/util/TreeMap$Entry;").await?;
            if !parent_left.is_null() && parent_left.identity() == entry.identity() {
                let mut sibling: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&parent, "right", "Ljava/util/TreeMap$Entry;").await?;
                if !sibling.is_null() && !jvm.get_field::<bool>(&sibling, "color", "Z").await? {
                    jvm.put_field(&mut sibling, "color", "Z", true).await?;
                    let mut parent = parent.clone();
                    jvm.put_field(&mut parent, "color", "Z", false).await?;
                    Self::rotate_left(jvm, this, parent.clone()).await?;
                    sibling = jvm.get_field(&parent, "right", "Ljava/util/TreeMap$Entry;").await?;
                }
                let sibling_left: ClassInstanceRef<TreeMapEntry> = if sibling.is_null() {
                    None.into()
                } else {
                    jvm.get_field(&sibling, "left", "Ljava/util/TreeMap$Entry;").await?
                };
                let sibling_right: ClassInstanceRef<TreeMapEntry> = if sibling.is_null() {
                    None.into()
                } else {
                    jvm.get_field(&sibling, "right", "Ljava/util/TreeMap$Entry;").await?
                };
                let left_black = sibling_left.is_null() || jvm.get_field::<bool>(&sibling_left, "color", "Z").await?;
                let right_black = sibling_right.is_null() || jvm.get_field::<bool>(&sibling_right, "color", "Z").await?;
                if left_black && right_black {
                    if !sibling.is_null() {
                        jvm.put_field(&mut sibling, "color", "Z", false).await?;
                    }
                    entry = parent;
                } else {
                    if right_black {
                        if !sibling_left.is_null() {
                            let mut sibling_left = sibling_left;
                            jvm.put_field(&mut sibling_left, "color", "Z", true).await?;
                        }
                        if !sibling.is_null() {
                            jvm.put_field(&mut sibling, "color", "Z", false).await?;
                            Self::rotate_right(jvm, this, sibling.clone()).await?;
                        }
                        sibling = jvm.get_field(&parent, "right", "Ljava/util/TreeMap$Entry;").await?;
                    }
                    if !sibling.is_null() {
                        let parent_color: bool = jvm.get_field(&parent, "color", "Z").await?;
                        jvm.put_field(&mut sibling, "color", "Z", parent_color).await?;
                    }
                    let mut parent = parent;
                    jvm.put_field(&mut parent, "color", "Z", true).await?;
                    if !sibling.is_null() {
                        let mut sibling_right: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&sibling, "right", "Ljava/util/TreeMap$Entry;").await?;
                        if !sibling_right.is_null() {
                            jvm.put_field(&mut sibling_right, "color", "Z", true).await?;
                        }
                    }
                    Self::rotate_left(jvm, this, parent).await?;
                    entry = jvm.get_field(this, "root", "Ljava/util/TreeMap$Entry;").await?;
                }
            } else {
                let mut sibling: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&parent, "left", "Ljava/util/TreeMap$Entry;").await?;
                if !sibling.is_null() && !jvm.get_field::<bool>(&sibling, "color", "Z").await? {
                    jvm.put_field(&mut sibling, "color", "Z", true).await?;
                    let mut parent = parent.clone();
                    jvm.put_field(&mut parent, "color", "Z", false).await?;
                    Self::rotate_right(jvm, this, parent.clone()).await?;
                    sibling = jvm.get_field(&parent, "left", "Ljava/util/TreeMap$Entry;").await?;
                }
                let sibling_left: ClassInstanceRef<TreeMapEntry> = if sibling.is_null() {
                    None.into()
                } else {
                    jvm.get_field(&sibling, "left", "Ljava/util/TreeMap$Entry;").await?
                };
                let sibling_right: ClassInstanceRef<TreeMapEntry> = if sibling.is_null() {
                    None.into()
                } else {
                    jvm.get_field(&sibling, "right", "Ljava/util/TreeMap$Entry;").await?
                };
                let left_black = sibling_left.is_null() || jvm.get_field::<bool>(&sibling_left, "color", "Z").await?;
                let right_black = sibling_right.is_null() || jvm.get_field::<bool>(&sibling_right, "color", "Z").await?;
                if left_black && right_black {
                    if !sibling.is_null() {
                        jvm.put_field(&mut sibling, "color", "Z", false).await?;
                    }
                    entry = parent;
                } else {
                    if left_black {
                        if !sibling_right.is_null() {
                            let mut sibling_right = sibling_right;
                            jvm.put_field(&mut sibling_right, "color", "Z", true).await?;
                        }
                        if !sibling.is_null() {
                            jvm.put_field(&mut sibling, "color", "Z", false).await?;
                            Self::rotate_left(jvm, this, sibling.clone()).await?;
                        }
                        sibling = jvm.get_field(&parent, "left", "Ljava/util/TreeMap$Entry;").await?;
                    }
                    if !sibling.is_null() {
                        let parent_color: bool = jvm.get_field(&parent, "color", "Z").await?;
                        jvm.put_field(&mut sibling, "color", "Z", parent_color).await?;
                    }
                    let mut parent = parent;
                    jvm.put_field(&mut parent, "color", "Z", true).await?;
                    if !sibling.is_null() {
                        let mut sibling_left: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&sibling, "left", "Ljava/util/TreeMap$Entry;").await?;
                        if !sibling_left.is_null() {
                            jvm.put_field(&mut sibling_left, "color", "Z", true).await?;
                        }
                    }
                    Self::rotate_right(jvm, this, parent).await?;
                    entry = jvm.get_field(this, "root", "Ljava/util/TreeMap$Entry;").await?;
                }
            }
        }
        if !entry.is_null() {
            jvm.put_field(&mut entry, "color", "Z", true).await?;
        }
        Ok(())
    }
}
