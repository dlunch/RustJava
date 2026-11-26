use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::Object};

use super::{TreeMap, TreeMapEntry};

// class java.util.TreeMap$SubMap
#[java_class(name = "java/util/TreeMap$SubMap", extends = "java/util/AbstractMap", implements("java/util/SortedMap"))]
pub struct TreeMapSubMap {
    #[field(private)]
    m: ClassInstanceRef<TreeMap>,
    #[field(private)]
    fromStart: bool,
    #[field(private)]
    fromKey: ClassInstanceRef<Object>,
    #[field(private)]
    toEnd: bool,
    #[field(private)]
    toKey: ClassInstanceRef<Object>,
}

#[java_methods]
impl TreeMapSubMap {
    #[allow(clippy::too_many_arguments)]
    #[method(constructor)]
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
        jvm.put_field(&mut this, "java/util/TreeMap$SubMap", "m", "Ljava/util/TreeMap;", map)
            .await?;
        jvm.put_field(&mut this, "java/util/TreeMap$SubMap", "fromStart", "Z", from_start).await?;
        jvm.put_field(&mut this, "java/util/TreeMap$SubMap", "fromKey", "Ljava/lang/Object;", from_key)
            .await?;
        jvm.put_field(&mut this, "java/util/TreeMap$SubMap", "toEnd", "Z", to_end).await?;
        jvm.put_field(&mut this, "java/util/TreeMap$SubMap", "toKey", "Ljava/lang/Object;", to_key)
            .await
    }

    #[method(public)]
    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let iterator: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&this, "java/util/TreeMap$SubMap", "entryIterator", "()Ljava/util/Iterator;", ())
            .await?;
        let mut size = 0;
        while jvm
            .invoke_virtual::<_, bool>(&iterator, &iterator.class_definition().name(), "hasNext", "()Z", ())
            .await?
        {
            let _: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&iterator, &iterator.class_definition().name(), "next", "()Ljava/lang/Object;", ())
                .await?;
            size += 1;
        }
        Ok(size)
    }

    #[method(public)]
    async fn contains_key(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key: ClassInstanceRef<Object>) -> Result<bool> {
        if !Self::in_range(jvm, &this, &key, false).await? {
            return Ok(false);
        }
        let map: ClassInstanceRef<TreeMap> = jvm.get_field(&this, "java/util/TreeMap$SubMap", "m", "Ljava/util/TreeMap;").await?;
        Ok(!TreeMap::find_entry(jvm, &map, &key).await?.is_null())
    }

    #[method(public)]
    async fn contains_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Object>) -> Result<bool> {
        let iterator: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&this, "java/util/TreeMap$SubMap", "valueIterator", "()Ljava/util/Iterator;", ())
            .await?;
        while jvm
            .invoke_virtual::<_, bool>(&iterator, &iterator.class_definition().name(), "hasNext", "()Z", ())
            .await?
        {
            let current: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&iterator, &iterator.class_definition().name(), "next", "()Ljava/lang/Object;", ())
                .await?;
            let equal = if value.is_null() {
                current.is_null()
            } else {
                jvm.invoke_virtual(&value, "java/lang/Object", "equals", "(Ljava/lang/Object;)Z", (current,))
                    .await?
            };
            if equal {
                return Ok(true);
            }
        }
        Ok(false)
    }

    #[method(public)]
    async fn get(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key: ClassInstanceRef<Object>) -> Result<ClassInstanceRef<Object>> {
        if !Self::in_range(jvm, &this, &key, false).await? {
            return Ok(None.into());
        }
        let map: ClassInstanceRef<TreeMap> = jvm.get_field(&this, "java/util/TreeMap$SubMap", "m", "Ljava/util/TreeMap;").await?;
        let entry = TreeMap::find_entry(jvm, &map, &key).await?;
        if entry.is_null() {
            return Ok(None.into());
        }
        jvm.get_field(&entry, "java/util/TreeMap$Entry", "value", "Ljava/lang/Object;").await
    }

    #[method(public)]
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
        let map: ClassInstanceRef<TreeMap> = jvm.get_field(&this, "java/util/TreeMap$SubMap", "m", "Ljava/util/TreeMap;").await?;
        jvm.invoke_virtual(
            &map,
            "java/util/TreeMap",
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (key, value),
        )
        .await
    }

    #[method(public)]
    async fn remove(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        key: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        if !Self::in_range(jvm, &this, &key, false).await? {
            return Ok(None.into());
        }
        let map: ClassInstanceRef<TreeMap> = jvm.get_field(&this, "java/util/TreeMap$SubMap", "m", "Ljava/util/TreeMap;").await?;
        jvm.invoke_virtual(&map, "java/util/TreeMap", "remove", "(Ljava/lang/Object;)Ljava/lang/Object;", (key,))
            .await
    }

    #[method(public)]
    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let iterator: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&this, "java/util/TreeMap$SubMap", "entryIterator", "()Ljava/util/Iterator;", ())
            .await?;
        while jvm
            .invoke_virtual::<_, bool>(&iterator, &iterator.class_definition().name(), "hasNext", "()Z", ())
            .await?
        {
            let _: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&iterator, &iterator.class_definition().name(), "next", "()Ljava/lang/Object;", ())
                .await?;
            let _: () = jvm
                .invoke_virtual(&iterator, &iterator.class_definition().name(), "remove", "()V", ())
                .await?;
        }
        Ok(())
    }

    #[method(descriptor = "()Ljava/util/Comparator;", public)]
    async fn comparator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<TreeMap> = jvm.get_field(&this, "java/util/TreeMap$SubMap", "m", "Ljava/util/TreeMap;").await?;
        jvm.invoke_virtual(&map, "java/util/TreeMap", "comparator", "()Ljava/util/Comparator;", ())
            .await
    }

    #[method(public)]
    async fn first_key(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let entry = Self::first_entry(jvm, &this).await?;
        if entry.is_null() {
            return Err(jvm.exception("java/util/NoSuchElementException", "empty subMap").await);
        }
        jvm.get_field(&entry, "java/util/TreeMap$Entry", "key", "Ljava/lang/Object;").await
    }

    #[method(public)]
    async fn last_key(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let entry = Self::last_entry(jvm, &this).await?;
        if entry.is_null() {
            return Err(jvm.exception("java/util/NoSuchElementException", "empty subMap").await);
        }
        jvm.get_field(&entry, "java/util/TreeMap$Entry", "key", "Ljava/lang/Object;").await
    }

    #[method(descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;", public)]
    async fn sub_map(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        from_key: ClassInstanceRef<Object>,
        to_key: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<TreeMap> = jvm.get_field(&this, "java/util/TreeMap$SubMap", "m", "Ljava/util/TreeMap;").await?;
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

    #[method(descriptor = "(Ljava/lang/Object;)Ljava/util/SortedMap;", public)]
    async fn head_map(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        to_key: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        if !Self::in_range(jvm, &this, &to_key, true).await? {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "endpoint outside range").await);
        }
        let map: ClassInstanceRef<TreeMap> = jvm.get_field(&this, "java/util/TreeMap$SubMap", "m", "Ljava/util/TreeMap;").await?;
        let from_start: bool = jvm.get_field(&this, "java/util/TreeMap$SubMap", "fromStart", "Z").await?;
        let from_key: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/TreeMap$SubMap", "fromKey", "Ljava/lang/Object;").await?;
        Ok(jvm
            .new_class(
                "java/util/TreeMap$SubMap",
                "(Ljava/util/TreeMap;ZLjava/lang/Object;ZLjava/lang/Object;)V",
                (map, from_start, from_key, false, to_key),
            )
            .await?
            .into())
    }

    #[method(descriptor = "(Ljava/lang/Object;)Ljava/util/SortedMap;", public)]
    async fn tail_map(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        from_key: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        if !Self::in_range(jvm, &this, &from_key, false).await? {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "endpoint outside range").await);
        }
        let map: ClassInstanceRef<TreeMap> = jvm.get_field(&this, "java/util/TreeMap$SubMap", "m", "Ljava/util/TreeMap;").await?;
        let to_end: bool = jvm.get_field(&this, "java/util/TreeMap$SubMap", "toEnd", "Z").await?;
        let to_key: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/TreeMap$SubMap", "toKey", "Ljava/lang/Object;").await?;
        Ok(jvm
            .new_class(
                "java/util/TreeMap$SubMap",
                "(Ljava/util/TreeMap;ZLjava/lang/Object;ZLjava/lang/Object;)V",
                (map, false, from_key, to_end, to_key),
            )
            .await?
            .into())
    }

    #[method(descriptor = "()Ljava/util/Set;", public)]
    async fn key_set(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        Ok(jvm
            .new_class("java/util/TreeMap$KeySet", "(Ljava/util/SortedMap;)V", (this,))
            .await?
            .into())
    }

    #[method(descriptor = "()Ljava/util/Collection;", public)]
    async fn values(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        Ok(jvm
            .new_class("java/util/TreeMap$Values", "(Ljava/util/SortedMap;)V", (this,))
            .await?
            .into())
    }

    #[method(descriptor = "()Ljava/util/Set;", public)]
    async fn entry_set(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        Ok(jvm
            .new_class("java/util/TreeMap$EntrySet", "(Ljava/util/SortedMap;)V", (this,))
            .await?
            .into())
    }

    #[method(descriptor = "()Ljava/util/Iterator;")]
    async fn key_iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<TreeMap> = jvm.get_field(&this, "java/util/TreeMap$SubMap", "m", "Ljava/util/TreeMap;").await?;
        let first = Self::first_entry(jvm, &this).await?;
        let to_end: bool = jvm.get_field(&this, "java/util/TreeMap$SubMap", "toEnd", "Z").await?;
        let to_key: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/TreeMap$SubMap", "toKey", "Ljava/lang/Object;").await?;
        Ok(jvm
            .new_class(
                "java/util/TreeMap$KeyIterator",
                "(Ljava/util/TreeMap;Ljava/util/TreeMap$Entry;Ljava/lang/Object;Z)V",
                (map, first, to_key, to_end),
            )
            .await?
            .into())
    }

    #[method(descriptor = "()Ljava/util/Iterator;")]
    async fn value_iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<TreeMap> = jvm.get_field(&this, "java/util/TreeMap$SubMap", "m", "Ljava/util/TreeMap;").await?;
        let first = Self::first_entry(jvm, &this).await?;
        let to_end: bool = jvm.get_field(&this, "java/util/TreeMap$SubMap", "toEnd", "Z").await?;
        let to_key: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/TreeMap$SubMap", "toKey", "Ljava/lang/Object;").await?;
        Ok(jvm
            .new_class(
                "java/util/TreeMap$ValueIterator",
                "(Ljava/util/TreeMap;Ljava/util/TreeMap$Entry;Ljava/lang/Object;Z)V",
                (map, first, to_key, to_end),
            )
            .await?
            .into())
    }

    #[method(descriptor = "()Ljava/util/Iterator;")]
    async fn entry_iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<TreeMap> = jvm.get_field(&this, "java/util/TreeMap$SubMap", "m", "Ljava/util/TreeMap;").await?;
        let first = Self::first_entry(jvm, &this).await?;
        let to_end: bool = jvm.get_field(&this, "java/util/TreeMap$SubMap", "toEnd", "Z").await?;
        let to_key: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/TreeMap$SubMap", "toKey", "Ljava/lang/Object;").await?;
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
        let map: ClassInstanceRef<TreeMap> = jvm.get_field(this, "java/util/TreeMap$SubMap", "m", "Ljava/util/TreeMap;").await?;
        if !jvm.get_field::<bool>(this, "java/util/TreeMap$SubMap", "fromStart", "Z").await? {
            let from_key: ClassInstanceRef<Object> = jvm.get_field(this, "java/util/TreeMap$SubMap", "fromKey", "Ljava/lang/Object;").await?;
            if TreeMap::compare(jvm, &map, key, &from_key).await? < 0 {
                return Ok(false);
            }
        }
        if !jvm.get_field::<bool>(this, "java/util/TreeMap$SubMap", "toEnd", "Z").await? {
            let to_key: ClassInstanceRef<Object> = jvm.get_field(this, "java/util/TreeMap$SubMap", "toKey", "Ljava/lang/Object;").await?;
            let comparison = TreeMap::compare(jvm, &map, key, &to_key).await?;
            if comparison > 0 || (!allow_equal_upper && comparison == 0) {
                return Ok(false);
            }
        }
        Ok(true)
    }

    async fn first_entry(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<ClassInstanceRef<TreeMapEntry>> {
        let map: ClassInstanceRef<TreeMap> = jvm.get_field(this, "java/util/TreeMap$SubMap", "m", "Ljava/util/TreeMap;").await?;
        let entry = if jvm.get_field::<bool>(this, "java/util/TreeMap$SubMap", "fromStart", "Z").await? {
            TreeMap::first_entry(jvm, &map).await?
        } else {
            let from_key: ClassInstanceRef<Object> = jvm.get_field(this, "java/util/TreeMap$SubMap", "fromKey", "Ljava/lang/Object;").await?;
            TreeMap::ceiling_entry(jvm, &map, &from_key).await?
        };
        if entry.is_null() || jvm.get_field::<bool>(this, "java/util/TreeMap$SubMap", "toEnd", "Z").await? {
            return Ok(entry);
        }
        let key: ClassInstanceRef<Object> = jvm.get_field(&entry, "java/util/TreeMap$Entry", "key", "Ljava/lang/Object;").await?;
        let to_key: ClassInstanceRef<Object> = jvm.get_field(this, "java/util/TreeMap$SubMap", "toKey", "Ljava/lang/Object;").await?;
        if TreeMap::compare(jvm, &map, &key, &to_key).await? >= 0 {
            Ok(None.into())
        } else {
            Ok(entry)
        }
    }

    async fn last_entry(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<ClassInstanceRef<TreeMapEntry>> {
        let map: ClassInstanceRef<TreeMap> = jvm.get_field(this, "java/util/TreeMap$SubMap", "m", "Ljava/util/TreeMap;").await?;
        let entry = if jvm.get_field::<bool>(this, "java/util/TreeMap$SubMap", "toEnd", "Z").await? {
            TreeMap::last_entry(jvm, &map).await?
        } else {
            let to_key: ClassInstanceRef<Object> = jvm.get_field(this, "java/util/TreeMap$SubMap", "toKey", "Ljava/lang/Object;").await?;
            TreeMap::lower_entry(jvm, &map, &to_key).await?
        };
        if entry.is_null() || jvm.get_field::<bool>(this, "java/util/TreeMap$SubMap", "fromStart", "Z").await? {
            return Ok(entry);
        }
        let key: ClassInstanceRef<Object> = jvm.get_field(&entry, "java/util/TreeMap$Entry", "key", "Ljava/lang/Object;").await?;
        let from_key: ClassInstanceRef<Object> = jvm.get_field(this, "java/util/TreeMap$SubMap", "fromKey", "Ljava/lang/Object;").await?;
        if TreeMap::compare(jvm, &map, &key, &from_key).await? < 0 {
            Ok(None.into())
        } else {
            Ok(entry)
        }
    }
}
