use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

use super::Hashtable;

// class java.util.Hashtable$Values
pub struct HashtableValues;

impl HashtableValues {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Hashtable$Values",
            parent_class: Some("java/util/AbstractCollection"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/util/Hashtable;)V", Self::init, MethodAccessFlags::empty()),
                JavaMethodProto::new("size", "()I", Self::size, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("isEmpty", "()Z", Self::is_empty, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("contains", "(Ljava/lang/Object;)Z", Self::contains, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("remove", "(Ljava/lang/Object;)Z", Self::remove, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("clear", "()V", Self::clear, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("iterator", "()Ljava/util/Iterator;", Self::iterator, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![JavaFieldProto::new("map", "Ljava/util/Hashtable;", FieldAccessFlags::PRIVATE)],
            access_flags: ClassAccessFlags::empty(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, map: ClassInstanceRef<Hashtable>) -> Result<()> {
        tracing::debug!("java.util.Hashtable$Values::<init>({this:?}, {map:?})");

        let _: () = jvm.invoke_special(&this, "java/util/AbstractCollection", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "map", "Ljava/util/Hashtable;", map).await?;

        Ok(())
    }

    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.Hashtable$Values::size({this:?})");

        let map: ClassInstanceRef<Hashtable> = jvm.get_field(&this, "map", "Ljava/util/Hashtable;").await?;

        jvm.invoke_virtual(&map, "java/util/Hashtable", "size", "()I", ()).await
    }

    async fn is_empty(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.Hashtable$Values::isEmpty({this:?})");

        let map: ClassInstanceRef<Hashtable> = jvm.get_field(&this, "map", "Ljava/util/Hashtable;").await?;

        jvm.invoke_virtual(&map, "java/util/Hashtable", "isEmpty", "()Z", ()).await
    }

    async fn contains(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Hashtable$Values::contains({this:?}, {value:?})");

        let map: ClassInstanceRef<Hashtable> = jvm.get_field(&this, "map", "Ljava/util/Hashtable;").await?;

        jvm.invoke_virtual(&map, "java/util/Hashtable", "containsValue", "(Ljava/lang/Object;)Z", (value,))
            .await
    }

    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Hashtable$Values::remove({this:?}, {value:?})");

        if value.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Hashtable value is null").await);
        }

        let map: ClassInstanceRef<Hashtable> = jvm.get_field(&this, "map", "Ljava/util/Hashtable;").await?;
        let entries = Hashtable::entries_snapshot(jvm, &map).await?;
        let count = jvm.array_length(&entries).await?;
        for entry in jvm.load_array::<ClassInstanceRef<Object>>(&entries, 0, count).await? {
            let entry_value: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&entry, &entry.class_definition().name(), "getValue", "()Ljava/lang/Object;", ())
                .await?;
            let equal: bool = jvm
                .invoke_virtual(&value, "java/lang/Object", "equals", "(Ljava/lang/Object;)Z", (entry_value,))
                .await?;
            if equal {
                let key: ClassInstanceRef<Object> = jvm
                    .invoke_virtual(&entry, &entry.class_definition().name(), "getKey", "()Ljava/lang/Object;", ())
                    .await?;
                let _: ClassInstanceRef<Object> = jvm
                    .invoke_virtual(&map, "java/util/Hashtable", "remove", "(Ljava/lang/Object;)Ljava/lang/Object;", (key,))
                    .await?;
                return Ok(true);
            }
        }

        Ok(false)
    }

    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Hashtable$Values::clear({this:?})");

        let map: ClassInstanceRef<Hashtable> = jvm.get_field(&this, "map", "Ljava/util/Hashtable;").await?;

        jvm.invoke_virtual(&map, "java/util/Hashtable", "clear", "()V", ()).await
    }

    async fn iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Hashtable$Values::iterator({this:?})");

        let map: ClassInstanceRef<Hashtable> = jvm.get_field(&this, "map", "Ljava/util/Hashtable;").await?;
        let snapshot = Hashtable::values_snapshot(jvm, &map).await?;
        let iterator = jvm
            .new_class("java/util/Hashtable$Enumerator", "([Ljava/lang/Object;)V", (snapshot,))
            .await?;

        Ok(iterator.into())
    }
}
