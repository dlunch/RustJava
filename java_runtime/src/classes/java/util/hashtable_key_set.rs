use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

use super::Hashtable;

// class java.util.Hashtable$KeySet
pub struct HashtableKeySet;

impl HashtableKeySet {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Hashtable$KeySet",
            parent_class: Some("java/util/AbstractSet"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/util/Hashtable;)V", Self::init, Default::default()),
                JavaMethodProto::new("size", "()I", Self::size, Default::default()),
                JavaMethodProto::new("isEmpty", "()Z", Self::is_empty, Default::default()),
                JavaMethodProto::new("contains", "(Ljava/lang/Object;)Z", Self::contains, Default::default()),
                JavaMethodProto::new("remove", "(Ljava/lang/Object;)Z", Self::remove, Default::default()),
                JavaMethodProto::new("clear", "()V", Self::clear, Default::default()),
                JavaMethodProto::new("iterator", "()Ljava/util/Iterator;", Self::iterator, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("map", "Ljava/util/Hashtable;", Default::default())],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, map: ClassInstanceRef<Hashtable>) -> Result<()> {
        tracing::debug!("java.util.Hashtable$KeySet::<init>({this:?}, {map:?})");

        let _: () = jvm.invoke_special(&this, "java/util/AbstractSet", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "map", "Ljava/util/Hashtable;", map).await?;

        Ok(())
    }

    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.Hashtable$KeySet::size({this:?})");

        let map: ClassInstanceRef<Hashtable> = jvm.get_field(&this, "map", "Ljava/util/Hashtable;").await?;

        jvm.invoke_virtual(&map, "size", "()I", ()).await
    }

    async fn is_empty(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.Hashtable$KeySet::isEmpty({this:?})");

        let map: ClassInstanceRef<Hashtable> = jvm.get_field(&this, "map", "Ljava/util/Hashtable;").await?;

        jvm.invoke_virtual(&map, "isEmpty", "()Z", ()).await
    }

    async fn contains(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Hashtable$KeySet::contains({this:?}, {key:?})");

        let map: ClassInstanceRef<Hashtable> = jvm.get_field(&this, "map", "Ljava/util/Hashtable;").await?;

        jvm.invoke_virtual(&map, "containsKey", "(Ljava/lang/Object;)Z", (key,)).await
    }

    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Hashtable$KeySet::remove({this:?}, {key:?})");

        let map: ClassInstanceRef<Hashtable> = jvm.get_field(&this, "map", "Ljava/util/Hashtable;").await?;
        let contains: bool = jvm.invoke_virtual(&map, "containsKey", "(Ljava/lang/Object;)Z", (key.clone(),)).await?;
        if !contains {
            return Ok(false);
        }

        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&map, "remove", "(Ljava/lang/Object;)Ljava/lang/Object;", (key,))
            .await?;

        Ok(true)
    }

    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Hashtable$KeySet::clear({this:?})");

        let map: ClassInstanceRef<Hashtable> = jvm.get_field(&this, "map", "Ljava/util/Hashtable;").await?;

        jvm.invoke_virtual(&map, "clear", "()V", ()).await
    }

    async fn iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Hashtable$KeySet::iterator({this:?})");

        let map: ClassInstanceRef<Hashtable> = jvm.get_field(&this, "map", "Ljava/util/Hashtable;").await?;
        let snapshot = Hashtable::keys_snapshot(jvm, &map).await?;
        let iterator = jvm
            .new_class("java/util/Hashtable$Enumerator", "([Ljava/lang/Object;)V", (snapshot,))
            .await?;

        Ok(iterator.into())
    }
}
