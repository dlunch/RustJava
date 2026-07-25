use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

// class java.util.TreeMap$Values
pub struct TreeMapValues;

impl TreeMapValues {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/TreeMap$Values",
            parent_class: Some("java/util/AbstractCollection"),
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
        let _: () = jvm.invoke_special(&this, "java/util/AbstractCollection", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "map", "Ljava/util/SortedMap;", map).await
    }

    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "map", "Ljava/util/SortedMap;").await?;
        jvm.invoke_virtual(&map, "size", "()I", ()).await
    }

    async fn contains(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Object>) -> Result<bool> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "map", "Ljava/util/SortedMap;").await?;
        jvm.invoke_virtual(&map, "containsValue", "(Ljava/lang/Object;)Z", (value,)).await
    }

    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Object>) -> Result<bool> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "map", "Ljava/util/SortedMap;").await?;
        let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&map, "valueIterator", "()Ljava/util/Iterator;", ()).await?;
        while jvm.invoke_virtual::<_, bool>(&iterator, "hasNext", "()Z", ()).await? {
            let current: ClassInstanceRef<Object> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await?;
            let equal = if current.is_null() {
                value.is_null()
            } else {
                jvm.invoke_virtual(&current, "equals", "(Ljava/lang/Object;)Z", (value.clone(),)).await?
            };
            if equal {
                let _: () = jvm.invoke_virtual(&iterator, "remove", "()V", ()).await?;
                return Ok(true);
            }
        }
        Ok(false)
    }

    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "map", "Ljava/util/SortedMap;").await?;
        jvm.invoke_virtual(&map, "clear", "()V", ()).await
    }

    async fn iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "map", "Ljava/util/SortedMap;").await?;
        jvm.invoke_virtual(&map, "valueIterator", "()Ljava/util/Iterator;", ()).await
    }
}
