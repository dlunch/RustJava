use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

// class java.util.Collections$UnmodifiableMap
pub struct CollectionsUnmodifiableMap;

impl CollectionsUnmodifiableMap {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Collections$UnmodifiableMap",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/util/Map", "java/io/Serializable"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/util/Map;)V", Self::init, MethodAccessFlags::empty()),
                JavaMethodProto::new("size", "()I", Self::size, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("isEmpty", "()Z", Self::is_empty, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("containsKey", "(Ljava/lang/Object;)Z", Self::contains_key, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("containsValue", "(Ljava/lang/Object;)Z", Self::contains_value, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("get", "(Ljava/lang/Object;)Ljava/lang/Object;", Self::get, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("keySet", "()Ljava/util/Set;", Self::key_set, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("values", "()Ljava/util/Collection;", Self::values, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("entrySet", "()Ljava/util/Set;", Self::entry_set, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, MethodAccessFlags::PUBLIC),
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
                JavaMethodProto::new("putAll", "(Ljava/util/Map;)V", Self::put_all, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("clear", "()V", Self::clear, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("m", "Ljava/util/Map;", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL),
                JavaFieldProto::new("keySet", "Ljava/util/Set;", FieldAccessFlags::PRIVATE | FieldAccessFlags::TRANSIENT),
                JavaFieldProto::new("entrySet", "Ljava/util/Set;", FieldAccessFlags::PRIVATE | FieldAccessFlags::TRANSIENT),
                JavaFieldProto::new(
                    "values",
                    "Ljava/util/Collection;",
                    FieldAccessFlags::PRIVATE | FieldAccessFlags::TRANSIENT,
                ),
            ],
            access_flags: ClassAccessFlags::empty(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, map: ClassInstanceRef<Object>) -> Result<()> {
        if map.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "map").await);
        }
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "m", "Ljava/util/Map;", map).await
    }

    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "m", "Ljava/util/Map;").await?;
        jvm.invoke_virtual(&map, &map.class_definition().name(), "size", "()I", ()).await
    }

    async fn is_empty(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "m", "Ljava/util/Map;").await?;
        jvm.invoke_virtual(&map, &map.class_definition().name(), "isEmpty", "()Z", ()).await
    }

    async fn contains_key(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key: ClassInstanceRef<Object>) -> Result<bool> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "m", "Ljava/util/Map;").await?;
        jvm.invoke_virtual(&map, &map.class_definition().name(), "containsKey", "(Ljava/lang/Object;)Z", (key,))
            .await
    }

    async fn contains_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Object>) -> Result<bool> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "m", "Ljava/util/Map;").await?;
        jvm.invoke_virtual(&map, &map.class_definition().name(), "containsValue", "(Ljava/lang/Object;)Z", (value,))
            .await
    }

    async fn get(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key: ClassInstanceRef<Object>) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "m", "Ljava/util/Map;").await?;
        jvm.invoke_virtual(
            &map,
            &map.class_definition().name(),
            "get",
            "(Ljava/lang/Object;)Ljava/lang/Object;",
            (key,),
        )
        .await
    }

    async fn key_set(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let cached: ClassInstanceRef<Object> = jvm.get_field(&this, "keySet", "Ljava/util/Set;").await?;
        if !cached.is_null() {
            return Ok(cached);
        }
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "m", "Ljava/util/Map;").await?;
        let keys: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&map, &map.class_definition().name(), "keySet", "()Ljava/util/Set;", ())
            .await?;
        let wrapped: ClassInstanceRef<Object> = jvm
            .new_class("java/util/Collections$UnmodifiableSet", "(Ljava/util/Set;)V", (keys,))
            .await?
            .into();
        jvm.put_field(&mut this, "keySet", "Ljava/util/Set;", wrapped.clone()).await?;
        Ok(wrapped)
    }

    async fn values(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let cached: ClassInstanceRef<Object> = jvm.get_field(&this, "values", "Ljava/util/Collection;").await?;
        if !cached.is_null() {
            return Ok(cached);
        }
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "m", "Ljava/util/Map;").await?;
        let values: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&map, &map.class_definition().name(), "values", "()Ljava/util/Collection;", ())
            .await?;
        let wrapped: ClassInstanceRef<Object> = jvm
            .new_class("java/util/Collections$UnmodifiableCollection", "(Ljava/util/Collection;)V", (values,))
            .await?
            .into();
        jvm.put_field(&mut this, "values", "Ljava/util/Collection;", wrapped.clone()).await?;
        Ok(wrapped)
    }

    async fn entry_set(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let cached: ClassInstanceRef<Object> = jvm.get_field(&this, "entrySet", "Ljava/util/Set;").await?;
        if !cached.is_null() {
            return Ok(cached);
        }
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "m", "Ljava/util/Map;").await?;
        let entries: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&map, &map.class_definition().name(), "entrySet", "()Ljava/util/Set;", ())
            .await?;
        let wrapped: ClassInstanceRef<Object> = jvm
            .new_class(
                "java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet",
                "(Ljava/util/Set;)V",
                (entries,),
            )
            .await?
            .into();
        jvm.put_field(&mut this, "entrySet", "Ljava/util/Set;", wrapped.clone()).await?;
        Ok(wrapped)
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if !other.is_null() && this.identity() == other.identity() {
            return Ok(true);
        }
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "m", "Ljava/util/Map;").await?;
        jvm.invoke_virtual(&map, "java/lang/Object", "equals", "(Ljava/lang/Object;)Z", (other,))
            .await
    }

    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "m", "Ljava/util/Map;").await?;
        jvm.invoke_virtual(&map, "java/lang/Object", "hashCode", "()I", ()).await
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "m", "Ljava/util/Map;").await?;
        jvm.invoke_virtual(&map, "java/lang/Object", "toString", "()Ljava/lang/String;", ()).await
    }

    async fn put(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _: ClassInstanceRef<Self>,
        _: ClassInstanceRef<Object>,
        _: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "unmodifiable map").await)
    }

    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<ClassInstanceRef<Object>> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "unmodifiable map").await)
    }

    async fn put_all(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<()> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "unmodifiable map").await)
    }

    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<()> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "unmodifiable map").await)
    }
}
