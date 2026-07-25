use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

use super::{TreeMap, TreeMapEntry};

// class java.util.TreeMap$SubMap
pub struct TreeMapSubMap;

impl TreeMapSubMap {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/TreeMap$SubMap",
            parent_class: Some("java/util/AbstractMap"),
            interfaces: vec!["java/util/SortedMap"],
            methods: vec![
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/util/TreeMap;ZLjava/lang/Object;ZLjava/lang/Object;)V",
                    Self::init,
                    Default::default(),
                ),
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
                JavaMethodProto::new("keyIterator", "()Ljava/util/Iterator;", Self::key_iterator, Default::default()),
                JavaMethodProto::new("valueIterator", "()Ljava/util/Iterator;", Self::value_iterator, Default::default()),
                JavaMethodProto::new("entryIterator", "()Ljava/util/Iterator;", Self::entry_iterator, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("m", "Ljava/util/TreeMap;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("fromStart", "Z", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("fromKey", "Ljava/lang/Object;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("toEnd", "Z", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("toKey", "Ljava/lang/Object;", FieldAccessFlags::PRIVATE),
            ],
            access_flags: Default::default(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        map: ClassInstanceRef<TreeMap>,
        from_start: bool,
        from_key: ClassInstanceRef<Object>,
        to_end: bool,
        to_key: ClassInstanceRef<Object>,
    ) -> Result<()> {
        if map.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "map").await);
        }
        if !from_start {
            let _ = TreeMap::compare(jvm, &map, &from_key, &from_key).await?;
        }
        if !to_end {
            let _ = TreeMap::compare(jvm, &map, &to_key, &to_key).await?;
        }
        if !from_start && !to_end && TreeMap::compare(jvm, &map, &from_key, &to_key).await? > 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "fromKey > toKey").await);
        }
        let _: () = jvm.invoke_special(&this, "java/util/AbstractMap", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "m", "Ljava/util/TreeMap;", map).await?;
        jvm.put_field(&mut this, "fromStart", "Z", from_start).await?;
        jvm.put_field(&mut this, "fromKey", "Ljava/lang/Object;", from_key).await?;
        jvm.put_field(&mut this, "toEnd", "Z", to_end).await?;
        jvm.put_field(&mut this, "toKey", "Ljava/lang/Object;", to_key).await
    }

    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&this, "entryIterator", "()Ljava/util/Iterator;", ()).await?;
        let mut size = 0;
        while jvm.invoke_virtual::<_, bool>(&iterator, "hasNext", "()Z", ()).await? {
            let _: ClassInstanceRef<Object> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await?;
            size += 1;
        }
        Ok(size)
    }

    async fn contains_key(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key: ClassInstanceRef<Object>) -> Result<bool> {
        if !Self::in_range(jvm, &this, &key, false).await? {
            return Ok(false);
        }
        let map: ClassInstanceRef<TreeMap> = jvm.get_field(&this, "m", "Ljava/util/TreeMap;").await?;
        Ok(!TreeMap::find_entry(jvm, &map, &key).await?.is_null())
    }

    async fn contains_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Object>) -> Result<bool> {
        let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&this, "valueIterator", "()Ljava/util/Iterator;", ()).await?;
        while jvm.invoke_virtual::<_, bool>(&iterator, "hasNext", "()Z", ()).await? {
            let current: ClassInstanceRef<Object> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await?;
            let equal = if value.is_null() {
                current.is_null()
            } else {
                jvm.invoke_virtual(&value, "equals", "(Ljava/lang/Object;)Z", (current,)).await?
            };
            if equal {
                return Ok(true);
            }
        }
        Ok(false)
    }

    async fn get(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key: ClassInstanceRef<Object>) -> Result<ClassInstanceRef<Object>> {
        if !Self::in_range(jvm, &this, &key, false).await? {
            return Ok(None.into());
        }
        let map: ClassInstanceRef<TreeMap> = jvm.get_field(&this, "m", "Ljava/util/TreeMap;").await?;
        let entry = TreeMap::find_entry(jvm, &map, &key).await?;
        if entry.is_null() {
            return Ok(None.into());
        }
        jvm.get_field(&entry, "value", "Ljava/lang/Object;").await
    }

    async fn put(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        key: ClassInstanceRef<Object>,
        value: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        if !Self::in_range(jvm, &this, &key, false).await? {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "key outside range").await);
        }
        let map: ClassInstanceRef<TreeMap> = jvm.get_field(&this, "m", "Ljava/util/TreeMap;").await?;
        jvm.invoke_virtual(&map, "put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", (key, value))
            .await
    }

    async fn remove(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        key: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        if !Self::in_range(jvm, &this, &key, false).await? {
            return Ok(None.into());
        }
        let map: ClassInstanceRef<TreeMap> = jvm.get_field(&this, "m", "Ljava/util/TreeMap;").await?;
        jvm.invoke_virtual(&map, "remove", "(Ljava/lang/Object;)Ljava/lang/Object;", (key,)).await
    }

    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&this, "entryIterator", "()Ljava/util/Iterator;", ()).await?;
        while jvm.invoke_virtual::<_, bool>(&iterator, "hasNext", "()Z", ()).await? {
            let _: ClassInstanceRef<Object> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await?;
            let _: () = jvm.invoke_virtual(&iterator, "remove", "()V", ()).await?;
        }
        Ok(())
    }

    async fn comparator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<TreeMap> = jvm.get_field(&this, "m", "Ljava/util/TreeMap;").await?;
        jvm.invoke_virtual(&map, "comparator", "()Ljava/util/Comparator;", ()).await
    }

    async fn first_key(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let entry = Self::first_entry(jvm, &this).await?;
        if entry.is_null() {
            return Err(jvm.exception("java/util/NoSuchElementException", "empty subMap").await);
        }
        jvm.get_field(&entry, "key", "Ljava/lang/Object;").await
    }

    async fn last_key(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let entry = Self::last_entry(jvm, &this).await?;
        if entry.is_null() {
            return Err(jvm.exception("java/util/NoSuchElementException", "empty subMap").await);
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
        let map: ClassInstanceRef<TreeMap> = jvm.get_field(&this, "m", "Ljava/util/TreeMap;").await?;
        if !Self::in_range(jvm, &this, &from_key, false).await? {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "fromKey outside range").await);
        }
        if !Self::in_range(jvm, &this, &to_key, true).await? {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "toKey outside range").await);
        }
        if TreeMap::compare(jvm, &map, &from_key, &to_key).await? > 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "fromKey > toKey").await);
        }
        Ok(jvm
            .new_class(
                "java/util/TreeMap$SubMap",
                "(Ljava/util/TreeMap;ZLjava/lang/Object;ZLjava/lang/Object;)V",
                (map, false, from_key, false, to_key),
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
        if !Self::in_range(jvm, &this, &to_key, true).await? {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "endpoint outside range").await);
        }
        let map: ClassInstanceRef<TreeMap> = jvm.get_field(&this, "m", "Ljava/util/TreeMap;").await?;
        let from_start: bool = jvm.get_field(&this, "fromStart", "Z").await?;
        let from_key: ClassInstanceRef<Object> = jvm.get_field(&this, "fromKey", "Ljava/lang/Object;").await?;
        Ok(jvm
            .new_class(
                "java/util/TreeMap$SubMap",
                "(Ljava/util/TreeMap;ZLjava/lang/Object;ZLjava/lang/Object;)V",
                (map, from_start, from_key, false, to_key),
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
        if !Self::in_range(jvm, &this, &from_key, false).await? {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "endpoint outside range").await);
        }
        let map: ClassInstanceRef<TreeMap> = jvm.get_field(&this, "m", "Ljava/util/TreeMap;").await?;
        let to_end: bool = jvm.get_field(&this, "toEnd", "Z").await?;
        let to_key: ClassInstanceRef<Object> = jvm.get_field(&this, "toKey", "Ljava/lang/Object;").await?;
        Ok(jvm
            .new_class(
                "java/util/TreeMap$SubMap",
                "(Ljava/util/TreeMap;ZLjava/lang/Object;ZLjava/lang/Object;)V",
                (map, false, from_key, to_end, to_key),
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
        let map: ClassInstanceRef<TreeMap> = jvm.get_field(&this, "m", "Ljava/util/TreeMap;").await?;
        let first = Self::first_entry(jvm, &this).await?;
        let to_end: bool = jvm.get_field(&this, "toEnd", "Z").await?;
        let to_key: ClassInstanceRef<Object> = jvm.get_field(&this, "toKey", "Ljava/lang/Object;").await?;
        Ok(jvm
            .new_class(
                "java/util/TreeMap$KeyIterator",
                "(Ljava/util/TreeMap;Ljava/util/TreeMap$Entry;Ljava/lang/Object;Z)V",
                (map, first, to_key, to_end),
            )
            .await?
            .into())
    }

    async fn value_iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<TreeMap> = jvm.get_field(&this, "m", "Ljava/util/TreeMap;").await?;
        let first = Self::first_entry(jvm, &this).await?;
        let to_end: bool = jvm.get_field(&this, "toEnd", "Z").await?;
        let to_key: ClassInstanceRef<Object> = jvm.get_field(&this, "toKey", "Ljava/lang/Object;").await?;
        Ok(jvm
            .new_class(
                "java/util/TreeMap$ValueIterator",
                "(Ljava/util/TreeMap;Ljava/util/TreeMap$Entry;Ljava/lang/Object;Z)V",
                (map, first, to_key, to_end),
            )
            .await?
            .into())
    }

    async fn entry_iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<TreeMap> = jvm.get_field(&this, "m", "Ljava/util/TreeMap;").await?;
        let first = Self::first_entry(jvm, &this).await?;
        let to_end: bool = jvm.get_field(&this, "toEnd", "Z").await?;
        let to_key: ClassInstanceRef<Object> = jvm.get_field(&this, "toKey", "Ljava/lang/Object;").await?;
        Ok(jvm
            .new_class(
                "java/util/TreeMap$EntryIterator",
                "(Ljava/util/TreeMap;Ljava/util/TreeMap$Entry;Ljava/lang/Object;Z)V",
                (map, first, to_key, to_end),
            )
            .await?
            .into())
    }

    async fn in_range(jvm: &Jvm, this: &ClassInstanceRef<Self>, key: &ClassInstanceRef<Object>, allow_equal_upper: bool) -> Result<bool> {
        let map: ClassInstanceRef<TreeMap> = jvm.get_field(this, "m", "Ljava/util/TreeMap;").await?;
        if !jvm.get_field::<bool>(this, "fromStart", "Z").await? {
            let from_key: ClassInstanceRef<Object> = jvm.get_field(this, "fromKey", "Ljava/lang/Object;").await?;
            if TreeMap::compare(jvm, &map, key, &from_key).await? < 0 {
                return Ok(false);
            }
        }
        if !jvm.get_field::<bool>(this, "toEnd", "Z").await? {
            let to_key: ClassInstanceRef<Object> = jvm.get_field(this, "toKey", "Ljava/lang/Object;").await?;
            let comparison = TreeMap::compare(jvm, &map, key, &to_key).await?;
            if comparison > 0 || (!allow_equal_upper && comparison == 0) {
                return Ok(false);
            }
        }
        Ok(true)
    }

    async fn first_entry(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<ClassInstanceRef<TreeMapEntry>> {
        let map: ClassInstanceRef<TreeMap> = jvm.get_field(this, "m", "Ljava/util/TreeMap;").await?;
        let entry = if jvm.get_field::<bool>(this, "fromStart", "Z").await? {
            TreeMap::first_entry(jvm, &map).await?
        } else {
            let from_key: ClassInstanceRef<Object> = jvm.get_field(this, "fromKey", "Ljava/lang/Object;").await?;
            TreeMap::ceiling_entry(jvm, &map, &from_key).await?
        };
        if entry.is_null() || jvm.get_field::<bool>(this, "toEnd", "Z").await? {
            return Ok(entry);
        }
        let key: ClassInstanceRef<Object> = jvm.get_field(&entry, "key", "Ljava/lang/Object;").await?;
        let to_key: ClassInstanceRef<Object> = jvm.get_field(this, "toKey", "Ljava/lang/Object;").await?;
        if TreeMap::compare(jvm, &map, &key, &to_key).await? >= 0 {
            Ok(None.into())
        } else {
            Ok(entry)
        }
    }

    async fn last_entry(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<ClassInstanceRef<TreeMapEntry>> {
        let map: ClassInstanceRef<TreeMap> = jvm.get_field(this, "m", "Ljava/util/TreeMap;").await?;
        let entry = if jvm.get_field::<bool>(this, "toEnd", "Z").await? {
            TreeMap::last_entry(jvm, &map).await?
        } else {
            let to_key: ClassInstanceRef<Object> = jvm.get_field(this, "toKey", "Ljava/lang/Object;").await?;
            TreeMap::lower_entry(jvm, &map, &to_key).await?
        };
        if entry.is_null() || jvm.get_field::<bool>(this, "fromStart", "Z").await? {
            return Ok(entry);
        }
        let key: ClassInstanceRef<Object> = jvm.get_field(&entry, "key", "Ljava/lang/Object;").await?;
        let from_key: ClassInstanceRef<Object> = jvm.get_field(this, "fromKey", "Ljava/lang/Object;").await?;
        if TreeMap::compare(jvm, &map, &key, &from_key).await? < 0 {
            Ok(None.into())
        } else {
            Ok(entry)
        }
    }
}
