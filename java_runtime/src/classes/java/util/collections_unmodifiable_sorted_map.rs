use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

// class java.util.Collections$UnmodifiableSortedMap
pub struct CollectionsUnmodifiableSortedMap;

impl CollectionsUnmodifiableSortedMap {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Collections$UnmodifiableSortedMap",
            parent_class: Some("java/util/Collections$UnmodifiableMap"),
            interfaces: vec!["java/util/SortedMap", "java/io/Serializable"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/util/SortedMap;)V", Self::init, Default::default()),
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
            ],
            fields: vec![JavaFieldProto::new(
                "sm",
                "Ljava/util/SortedMap;",
                FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL,
            )],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, map: ClassInstanceRef<Object>) -> Result<()> {
        let _: () = jvm
            .invoke_special(
                &this,
                "java/util/Collections$UnmodifiableMap",
                "<init>",
                "(Ljava/util/Map;)V",
                (map.clone(),),
            )
            .await?;
        jvm.put_field(&mut this, "sm", "Ljava/util/SortedMap;", map).await
    }

    async fn comparator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "sm", "Ljava/util/SortedMap;").await?;
        jvm.invoke_virtual(&map, "comparator", "()Ljava/util/Comparator;", ()).await
    }

    async fn first_key(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "sm", "Ljava/util/SortedMap;").await?;
        jvm.invoke_virtual(&map, "firstKey", "()Ljava/lang/Object;", ()).await
    }

    async fn last_key(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "sm", "Ljava/util/SortedMap;").await?;
        jvm.invoke_virtual(&map, "lastKey", "()Ljava/lang/Object;", ()).await
    }

    async fn sub_map(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        from: ClassInstanceRef<Object>,
        to: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "sm", "Ljava/util/SortedMap;").await?;
        let range: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&map, "subMap", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;", (from, to))
            .await?;
        Ok(jvm
            .new_class("java/util/Collections$UnmodifiableSortedMap", "(Ljava/util/SortedMap;)V", (range,))
            .await?
            .into())
    }

    async fn head_map(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        to: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "sm", "Ljava/util/SortedMap;").await?;
        let range: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&map, "headMap", "(Ljava/lang/Object;)Ljava/util/SortedMap;", (to,))
            .await?;
        Ok(jvm
            .new_class("java/util/Collections$UnmodifiableSortedMap", "(Ljava/util/SortedMap;)V", (range,))
            .await?
            .into())
    }

    async fn tail_map(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        from: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "sm", "Ljava/util/SortedMap;").await?;
        let range: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&map, "tailMap", "(Ljava/lang/Object;)Ljava/util/SortedMap;", (from,))
            .await?;
        Ok(jvm
            .new_class("java/util/Collections$UnmodifiableSortedMap", "(Ljava/util/SortedMap;)V", (range,))
            .await?
            .into())
    }
}
