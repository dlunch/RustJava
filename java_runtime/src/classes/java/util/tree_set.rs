use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

// public class java.util.TreeSet
pub struct TreeSet;

impl TreeSet {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/TreeSet",
            parent_class: Some("java/util/AbstractSet"),
            interfaces: vec!["java/util/SortedSet", "java/lang/Cloneable", "java/io/Serializable"],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(Ljava/util/Comparator;)V", Self::init_comparator, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(Ljava/util/Collection;)V", Self::init_collection, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(Ljava/util/SortedSet;)V", Self::init_sorted_set, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(Ljava/util/SortedMap;)V", Self::init_sorted_map, MethodAccessFlags::PRIVATE),
                JavaMethodProto::new("size", "()I", Self::size, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("contains", "(Ljava/lang/Object;)Z", Self::contains, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("add", "(Ljava/lang/Object;)Z", Self::add, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("remove", "(Ljava/lang/Object;)Z", Self::remove, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("clear", "()V", Self::clear, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("iterator", "()Ljava/util/Iterator;", Self::iterator, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("comparator", "()Ljava/util/Comparator;", Self::comparator, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("first", "()Ljava/lang/Object;", Self::first, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("last", "()Ljava/lang/Object;", Self::last, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "subSet",
                    "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedSet;",
                    Self::sub_set,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "headSet",
                    "(Ljava/lang/Object;)Ljava/util/SortedSet;",
                    Self::head_set,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "tailSet",
                    "(Ljava/lang/Object;)Ljava/util/SortedSet;",
                    Self::tail_set,
                    MethodAccessFlags::PUBLIC,
                ),
            ],
            fields: vec![
                JavaFieldProto::new("m", "Ljava/util/SortedMap;", FieldAccessFlags::PRIVATE | FieldAccessFlags::TRANSIENT),
                JavaFieldProto::new(
                    "PRESENT",
                    "Ljava/lang/Object;",
                    FieldAccessFlags::PRIVATE | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let present: ClassInstanceRef<Object> = jvm.new_class("java/lang/Object", "()V", ()).await?.into();
        jvm.put_static_field("java/util/TreeSet", "PRESENT", "Ljava/lang/Object;", present).await
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/util/AbstractSet", "<init>", "()V", ()).await?;
        let map: ClassInstanceRef<Object> = jvm.new_class("java/util/TreeMap", "()V", ()).await?.into();
        let mut this = this;
        jvm.put_field(&mut this, "m", "Ljava/util/SortedMap;", map).await
    }

    async fn init_comparator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, comparator: ClassInstanceRef<Object>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/util/AbstractSet", "<init>", "()V", ()).await?;
        let map: ClassInstanceRef<Object> = jvm
            .new_class("java/util/TreeMap", "(Ljava/util/Comparator;)V", (comparator,))
            .await?
            .into();
        let mut this = this;
        jvm.put_field(&mut this, "m", "Ljava/util/SortedMap;", map).await
    }

    async fn init_collection(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, collection: ClassInstanceRef<Object>) -> Result<()> {
        if collection.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "collection").await);
        }
        let _: () = jvm.invoke_special(&this, "java/util/TreeSet", "<init>", "()V", ()).await?;
        let _: bool = jvm
            .invoke_virtual(&this, "java/util/TreeSet", "addAll", "(Ljava/util/Collection;)Z", (collection,))
            .await?;
        Ok(())
    }

    async fn init_sorted_set(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, set: ClassInstanceRef<Object>) -> Result<()> {
        if set.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "set").await);
        }
        let comparator: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&set, &set.class_definition().name(), "comparator", "()Ljava/util/Comparator;", ())
            .await?;
        let _: () = jvm
            .invoke_special(&this, "java/util/TreeSet", "<init>", "(Ljava/util/Comparator;)V", (comparator,))
            .await?;
        let _: bool = jvm
            .invoke_virtual(&this, "java/util/TreeSet", "addAll", "(Ljava/util/Collection;)Z", (set,))
            .await?;
        Ok(())
    }

    async fn init_sorted_map(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, map: ClassInstanceRef<Object>) -> Result<()> {
        if map.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "map").await);
        }
        let _: () = jvm.invoke_special(&this, "java/util/AbstractSet", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "m", "Ljava/util/SortedMap;", map).await
    }

    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "m", "Ljava/util/SortedMap;").await?;
        jvm.invoke_virtual(&map, &map.class_definition().name(), "size", "()I", ()).await
    }

    async fn contains(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "m", "Ljava/util/SortedMap;").await?;
        jvm.invoke_virtual(&map, &map.class_definition().name(), "containsKey", "(Ljava/lang/Object;)Z", (element,))
            .await
    }

    async fn add(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "m", "Ljava/util/SortedMap;").await?;
        let present: ClassInstanceRef<Object> = jvm.get_static_field("java/util/TreeSet", "PRESENT", "Ljava/lang/Object;").await?;
        let old: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &map,
                &map.class_definition().name(),
                "put",
                "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                (element, present),
            )
            .await?;
        Ok(old.is_null())
    }

    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "m", "Ljava/util/SortedMap;").await?;
        if !jvm
            .invoke_virtual::<_, bool>(
                &map,
                &map.class_definition().name(),
                "containsKey",
                "(Ljava/lang/Object;)Z",
                (element.clone(),),
            )
            .await?
        {
            return Ok(false);
        }
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &map,
                &map.class_definition().name(),
                "remove",
                "(Ljava/lang/Object;)Ljava/lang/Object;",
                (element,),
            )
            .await?;
        Ok(true)
    }

    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "m", "Ljava/util/SortedMap;").await?;
        jvm.invoke_virtual(&map, &map.class_definition().name(), "clear", "()V", ()).await
    }

    async fn iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "m", "Ljava/util/SortedMap;").await?;
        let keys: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&map, &map.class_definition().name(), "keySet", "()Ljava/util/Set;", ())
            .await?;
        jvm.invoke_virtual(&keys, &keys.class_definition().name(), "iterator", "()Ljava/util/Iterator;", ())
            .await
    }

    async fn comparator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "m", "Ljava/util/SortedMap;").await?;
        jvm.invoke_virtual(&map, &map.class_definition().name(), "comparator", "()Ljava/util/Comparator;", ())
            .await
    }

    async fn first(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "m", "Ljava/util/SortedMap;").await?;
        jvm.invoke_virtual(&map, &map.class_definition().name(), "firstKey", "()Ljava/lang/Object;", ())
            .await
    }

    async fn last(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "m", "Ljava/util/SortedMap;").await?;
        jvm.invoke_virtual(&map, &map.class_definition().name(), "lastKey", "()Ljava/lang/Object;", ())
            .await
    }

    async fn sub_set(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        from: ClassInstanceRef<Object>,
        to: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "m", "Ljava/util/SortedMap;").await?;
        let range: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &map,
                &map.class_definition().name(),
                "subMap",
                "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;",
                (from, to),
            )
            .await?;
        Ok(jvm.new_class("java/util/TreeSet", "(Ljava/util/SortedMap;)V", (range,)).await?.into())
    }

    async fn head_set(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        to: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "m", "Ljava/util/SortedMap;").await?;
        let range: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &map,
                &map.class_definition().name(),
                "headMap",
                "(Ljava/lang/Object;)Ljava/util/SortedMap;",
                (to,),
            )
            .await?;
        Ok(jvm.new_class("java/util/TreeSet", "(Ljava/util/SortedMap;)V", (range,)).await?.into())
    }

    async fn tail_set(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        from: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "m", "Ljava/util/SortedMap;").await?;
        let range: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &map,
                &map.class_definition().name(),
                "tailMap",
                "(Ljava/lang/Object;)Ljava/util/SortedMap;",
                (from,),
            )
            .await?;
        Ok(jvm.new_class("java/util/TreeSet", "(Ljava/util/SortedMap;)V", (range,)).await?.into())
    }
}
