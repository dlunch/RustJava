use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

use super::HashMap;

const DEFAULT_INITIAL_CAPACITY: i32 = 16;

// class java.util.HashSet
pub struct HashSet;

impl HashSet {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/HashSet",
            parent_class: Some("java/util/AbstractSet"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(I)V", Self::init_with_capacity, Default::default()),
                JavaMethodProto::new("add", "(Ljava/lang/Object;)Z", Self::add, Default::default()),
                JavaMethodProto::new("remove", "(Ljava/lang/Object;)Z", Self::remove, Default::default()),
                JavaMethodProto::new("contains", "(Ljava/lang/Object;)Z", Self::contains, Default::default()),
                JavaMethodProto::new("size", "()I", Self::size, Default::default()),
                JavaMethodProto::new("isEmpty", "()Z", Self::is_empty, Default::default()),
                JavaMethodProto::new("clear", "()V", Self::clear, Default::default()),
                JavaMethodProto::new("iterator", "()Ljava/util/Iterator;", Self::iterator, Default::default()),
                JavaMethodProto::new("toArray", "()[Ljava/lang/Object;", Self::to_array, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("map", "Ljava/util/HashMap;", Default::default()),
                JavaFieldProto::new("present", "Ljava/lang/Object;", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.HashSet::<init>({this:?})");

        let _: () = jvm
            .invoke_special(&this, "java/util/HashSet", "<init>", "(I)V", (DEFAULT_INITIAL_CAPACITY,))
            .await?;

        Ok(())
    }

    async fn init_with_capacity(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, capacity: i32) -> Result<()> {
        tracing::debug!("java.util.HashSet::<init>({this:?}, {capacity:?})");

        let _: () = jvm.invoke_special(&this, "java/util/AbstractSet", "<init>", "()V", ()).await?;

        let map: ClassInstanceRef<HashMap> = jvm.new_class("java/util/HashMap", "(I)V", (capacity,)).await?.into();
        let present: ClassInstanceRef<Object> = jvm.new_class("java/lang/Object", "()V", ()).await?.into();

        jvm.put_field(&mut this, "map", "Ljava/util/HashMap;", map).await?;
        jvm.put_field(&mut this, "present", "Ljava/lang/Object;", present).await?;

        Ok(())
    }

    async fn add(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.HashSet::add({this:?}, {element:?})");

        let map: ClassInstanceRef<HashMap> = jvm.get_field(&this, "map", "Ljava/util/HashMap;").await?;
        let present: ClassInstanceRef<Object> = jvm.get_field(&this, "present", "Ljava/lang/Object;").await?;
        let old: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &map,
                "put",
                "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                (element, present),
            )
            .await?;

        Ok(old.is_null())
    }

    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.HashSet::remove({this:?}, {element:?})");

        let map: ClassInstanceRef<HashMap> = jvm.get_field(&this, "map", "Ljava/util/HashMap;").await?;
        let old: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&map, "remove", "(Ljava/lang/Object;)Ljava/lang/Object;", (element,))
            .await?;

        Ok(!old.is_null())
    }

    async fn contains(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.HashSet::contains({this:?}, {element:?})");

        let map: ClassInstanceRef<HashMap> = jvm.get_field(&this, "map", "Ljava/util/HashMap;").await?;

        jvm.invoke_virtual(&map, "containsKey", "(Ljava/lang/Object;)Z", (element,)).await
    }

    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.HashSet::size({this:?})");

        let map: ClassInstanceRef<HashMap> = jvm.get_field(&this, "map", "Ljava/util/HashMap;").await?;

        jvm.invoke_virtual(&map, "size", "()I", ()).await
    }

    async fn is_empty(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.HashSet::isEmpty({this:?})");

        let map: ClassInstanceRef<HashMap> = jvm.get_field(&this, "map", "Ljava/util/HashMap;").await?;

        jvm.invoke_virtual(&map, "isEmpty", "()Z", ()).await
    }

    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.HashSet::clear({this:?})");

        let map: ClassInstanceRef<HashMap> = jvm.get_field(&this, "map", "Ljava/util/HashMap;").await?;

        jvm.invoke_virtual(&map, "clear", "()V", ()).await
    }

    async fn iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.HashSet::iterator({this:?})");

        let key_set = Self::key_set(jvm, &this).await?;

        jvm.invoke_virtual(&key_set, "iterator", "()Ljava/util/Iterator;", ()).await
    }

    async fn to_array(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Array<Object>>> {
        tracing::debug!("java.util.HashSet::toArray({this:?})");

        let key_set = Self::key_set(jvm, &this).await?;

        jvm.invoke_virtual(&key_set, "toArray", "()[Ljava/lang/Object;", ()).await
    }

    async fn key_set(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<HashMap> = jvm.get_field(this, "map", "Ljava/util/HashMap;").await?;

        jvm.invoke_virtual(&map, "keySet", "()Ljava/util/Set;", ()).await
    }
}
