use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

// class java.util.TreeMap$EntrySet
pub struct TreeMapEntrySet;

impl TreeMapEntrySet {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/TreeMap$EntrySet",
            parent_class: Some("java/util/AbstractSet"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/util/SortedMap;)V", Self::init, Default::default()),
                JavaMethodProto::new("size", "()I", Self::size, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("contains", "(Ljava/lang/Object;)Z", Self::contains, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("remove", "(Ljava/lang/Object;)Z", Self::remove, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("clear", "()V", Self::clear, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("iterator", "()Ljava/util/Iterator;", Self::iterator, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![JavaFieldProto::new("map", "Ljava/util/SortedMap;", FieldAccessFlags::PRIVATE)],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, map: ClassInstanceRef<Object>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/util/AbstractSet", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "map", "Ljava/util/SortedMap;", map).await
    }

    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "map", "Ljava/util/SortedMap;").await?;
        jvm.invoke_virtual(&map, "size", "()I", ()).await
    }

    async fn contains(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, candidate: ClassInstanceRef<Object>) -> Result<bool> {
        if candidate.is_null() || !jvm.is_instance(candidate.as_ref(), "java/util/Map$Entry") {
            return Ok(false);
        }
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "map", "Ljava/util/SortedMap;").await?;
        let value: ClassInstanceRef<Object> = jvm.invoke_virtual(&candidate, "getValue", "()Ljava/lang/Object;", ()).await?;
        let key: ClassInstanceRef<Object> = jvm.invoke_virtual(&candidate, "getKey", "()Ljava/lang/Object;", ()).await?;
        if !jvm
            .invoke_virtual::<_, bool>(&map, "containsKey", "(Ljava/lang/Object;)Z", (key.clone(),))
            .await?
        {
            return Ok(false);
        }
        let stored: ClassInstanceRef<Object> = jvm.invoke_virtual(&map, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (key,)).await?;
        if stored.is_null() {
            Ok(value.is_null())
        } else {
            jvm.invoke_virtual(&stored, "equals", "(Ljava/lang/Object;)Z", (value,)).await
        }
    }

    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, candidate: ClassInstanceRef<Object>) -> Result<bool> {
        if candidate.is_null() || !jvm.is_instance(candidate.as_ref(), "java/util/Map$Entry") {
            return Ok(false);
        }
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "map", "Ljava/util/SortedMap;").await?;
        let value: ClassInstanceRef<Object> = jvm.invoke_virtual(&candidate, "getValue", "()Ljava/lang/Object;", ()).await?;
        let key: ClassInstanceRef<Object> = jvm.invoke_virtual(&candidate, "getKey", "()Ljava/lang/Object;", ()).await?;
        if !jvm
            .invoke_virtual::<_, bool>(&map, "containsKey", "(Ljava/lang/Object;)Z", (key.clone(),))
            .await?
        {
            return Ok(false);
        }
        let stored: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&map, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (key.clone(),))
            .await?;
        let equal = if stored.is_null() {
            value.is_null()
        } else {
            jvm.invoke_virtual::<_, bool>(&stored, "equals", "(Ljava/lang/Object;)Z", (value,))
                .await?
        };
        if !equal {
            return Ok(false);
        }
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&map, "remove", "(Ljava/lang/Object;)Ljava/lang/Object;", (key,))
            .await?;
        Ok(true)
    }

    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "map", "Ljava/util/SortedMap;").await?;
        jvm.invoke_virtual(&map, "clear", "()V", ()).await
    }

    async fn iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "map", "Ljava/util/SortedMap;").await?;
        jvm.invoke_virtual(&map, "entryIterator", "()Ljava/util/Iterator;", ()).await
    }
}
