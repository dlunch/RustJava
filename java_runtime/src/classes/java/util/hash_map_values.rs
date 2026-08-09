use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::MethodAccessFlags;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

use super::HashMap;

// class java.util.HashMap$Values
pub struct HashMapValues;

impl HashMapValues {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/HashMap$Values",
            parent_class: Some("java/util/AbstractCollection"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/util/HashMap;)V", Self::init, Default::default()),
                JavaMethodProto::new("size", "()I", Self::size, Default::default()),
                JavaMethodProto::new("isEmpty", "()Z", Self::is_empty, Default::default()),
                JavaMethodProto::new("contains", "(Ljava/lang/Object;)Z", Self::contains, Default::default()),
                JavaMethodProto::new("remove", "(Ljava/lang/Object;)Z", Self::remove, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("clear", "()V", Self::clear, Default::default()),
                JavaMethodProto::new("iterator", "()Ljava/util/Iterator;", Self::iterator, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("map", "Ljava/util/HashMap;", Default::default())],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, map: ClassInstanceRef<HashMap>) -> Result<()> {
        tracing::debug!("java.util.HashMap$Values::<init>({this:?}, {map:?})");

        let _: () = jvm.invoke_special(&this, "java/util/AbstractCollection", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "map", "Ljava/util/HashMap;", map).await?;

        Ok(())
    }

    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.HashMap$Values::size({this:?})");

        let map: ClassInstanceRef<HashMap> = jvm.get_field(&this, "map", "Ljava/util/HashMap;").await?;

        jvm.invoke_virtual(&map, "size", "()I", ()).await
    }

    async fn is_empty(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.HashMap$Values::isEmpty({this:?})");

        let map: ClassInstanceRef<HashMap> = jvm.get_field(&this, "map", "Ljava/util/HashMap;").await?;

        jvm.invoke_virtual(&map, "isEmpty", "()Z", ()).await
    }

    async fn contains(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.HashMap$Values::contains({this:?}, {value:?})");

        let map: ClassInstanceRef<HashMap> = jvm.get_field(&this, "map", "Ljava/util/HashMap;").await?;

        jvm.invoke_virtual(&map, "containsValue", "(Ljava/lang/Object;)Z", (value,)).await
    }

    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.HashMap$Values::remove({this:?}, {value:?})");

        let map: ClassInstanceRef<HashMap> = jvm.get_field(&this, "map", "Ljava/util/HashMap;").await?;
        let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&map, "entryIterator", "()Ljava/util/Iterator;", ()).await?;
        while jvm.invoke_virtual::<_, bool>(&iterator, "hasNext", "()Z", ()).await? {
            let entry: ClassInstanceRef<Object> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await?;
            let entry_value: ClassInstanceRef<Object> = jvm.invoke_virtual(&entry, "getValue", "()Ljava/lang/Object;", ()).await?;
            let equal = if value.is_null() {
                entry_value.is_null()
            } else if entry_value.is_null() {
                false
            } else {
                jvm.invoke_virtual(&value, "equals", "(Ljava/lang/Object;)Z", (entry_value,)).await?
            };
            if equal {
                let key: ClassInstanceRef<Object> = jvm.invoke_virtual(&entry, "getKey", "()Ljava/lang/Object;", ()).await?;
                let _: ClassInstanceRef<Object> = jvm
                    .invoke_virtual(&map, "remove", "(Ljava/lang/Object;)Ljava/lang/Object;", (key,))
                    .await?;
                return Ok(true);
            }
        }

        Ok(false)
    }

    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.HashMap$Values::clear({this:?})");

        let map: ClassInstanceRef<HashMap> = jvm.get_field(&this, "map", "Ljava/util/HashMap;").await?;

        jvm.invoke_virtual(&map, "clear", "()V", ()).await
    }

    async fn iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.HashMap$Values::iterator({this:?})");

        let map: ClassInstanceRef<HashMap> = jvm.get_field(&this, "map", "Ljava/util/HashMap;").await?;

        jvm.invoke_virtual(&map, "valueIterator", "()Ljava/util/Iterator;", ()).await
    }
}
