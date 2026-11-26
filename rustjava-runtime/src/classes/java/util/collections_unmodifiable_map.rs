use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{
        lang::Object,
        util::{Collection, Map, Set},
    },
};

// class java.util.Collections$UnmodifiableMap
#[java_class(
    name = "java/util/Collections$UnmodifiableMap",
    extends = "java/lang/Object",
    implements("java/util/Map", "java/io/Serializable")
)]
pub struct CollectionsUnmodifiableMap {
    #[field(private, final)]
    m: ClassInstanceRef<Map>,
    #[field(private, transient)]
    keySet: ClassInstanceRef<Set>,
    #[field(private, transient)]
    entrySet: ClassInstanceRef<Set>,
    #[field(private, transient)]
    values: ClassInstanceRef<Collection>,
}

#[java_methods]
impl CollectionsUnmodifiableMap {
    #[method(constructor, descriptor = "(Ljava/util/Map;)V")]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, map: ClassInstanceRef<Object>) -> Result<()> {
        if map.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "map").await);
        }
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/util/Collections$UnmodifiableMap", "m", "Ljava/util/Map;", map)
            .await
    }

    #[method(public)]
    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let map: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableMap", "m", "Ljava/util/Map;")
            .await?;
        jvm.invoke_virtual(&map, &map.class_definition().name(), "size", "()I", ()).await
    }

    #[method(public)]
    async fn is_empty(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let map: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableMap", "m", "Ljava/util/Map;")
            .await?;
        jvm.invoke_virtual(&map, &map.class_definition().name(), "isEmpty", "()Z", ()).await
    }

    #[method(public)]
    async fn contains_key(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key: ClassInstanceRef<Object>) -> Result<bool> {
        let map: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableMap", "m", "Ljava/util/Map;")
            .await?;
        jvm.invoke_virtual(&map, &map.class_definition().name(), "containsKey", "(Ljava/lang/Object;)Z", (key,))
            .await
    }

    #[method(public)]
    async fn contains_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Object>) -> Result<bool> {
        let map: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableMap", "m", "Ljava/util/Map;")
            .await?;
        jvm.invoke_virtual(&map, &map.class_definition().name(), "containsValue", "(Ljava/lang/Object;)Z", (value,))
            .await
    }

    #[method(public)]
    async fn get(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key: ClassInstanceRef<Object>) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableMap", "m", "Ljava/util/Map;")
            .await?;
        jvm.invoke_virtual(
            &map,
            &map.class_definition().name(),
            "get",
            "(Ljava/lang/Object;)Ljava/lang/Object;",
            (key,),
        )
        .await
    }

    #[method(descriptor = "()Ljava/util/Set;", public)]
    async fn key_set(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let cached: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableMap", "keySet", "Ljava/util/Set;")
            .await?;
        if !cached.is_null() {
            return Ok(cached);
        }
        let map: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableMap", "m", "Ljava/util/Map;")
            .await?;
        let keys: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&map, &map.class_definition().name(), "keySet", "()Ljava/util/Set;", ())
            .await?;
        let wrapped: ClassInstanceRef<Object> = jvm
            .new_class("java/util/Collections$UnmodifiableSet", "(Ljava/util/Set;)V", (keys,))
            .await?
            .into();
        jvm.put_field(
            &mut this,
            "java/util/Collections$UnmodifiableMap",
            "keySet",
            "Ljava/util/Set;",
            wrapped.clone(),
        )
        .await?;
        Ok(wrapped)
    }

    #[method(descriptor = "()Ljava/util/Collection;", public)]
    async fn values(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let cached: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableMap", "values", "Ljava/util/Collection;")
            .await?;
        if !cached.is_null() {
            return Ok(cached);
        }
        let map: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableMap", "m", "Ljava/util/Map;")
            .await?;
        let values: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&map, &map.class_definition().name(), "values", "()Ljava/util/Collection;", ())
            .await?;
        let wrapped: ClassInstanceRef<Object> = jvm
            .new_class("java/util/Collections$UnmodifiableCollection", "(Ljava/util/Collection;)V", (values,))
            .await?
            .into();
        jvm.put_field(
            &mut this,
            "java/util/Collections$UnmodifiableMap",
            "values",
            "Ljava/util/Collection;",
            wrapped.clone(),
        )
        .await?;
        Ok(wrapped)
    }

    #[method(descriptor = "()Ljava/util/Set;", public)]
    async fn entry_set(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let cached: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableMap", "entrySet", "Ljava/util/Set;")
            .await?;
        if !cached.is_null() {
            return Ok(cached);
        }
        let map: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableMap", "m", "Ljava/util/Map;")
            .await?;
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
        jvm.put_field(
            &mut this,
            "java/util/Collections$UnmodifiableMap",
            "entrySet",
            "Ljava/util/Set;",
            wrapped.clone(),
        )
        .await?;
        Ok(wrapped)
    }

    #[method(public)]
    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if !other.is_null() && this.identity() == other.identity() {
            return Ok(true);
        }
        let map: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableMap", "m", "Ljava/util/Map;")
            .await?;
        jvm.invoke_virtual(&map, "java/lang/Object", "equals", "(Ljava/lang/Object;)Z", (other,))
            .await
    }

    #[method(public)]
    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let map: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableMap", "m", "Ljava/util/Map;")
            .await?;
        jvm.invoke_virtual(&map, "java/lang/Object", "hashCode", "()I", ()).await
    }

    #[method(descriptor = "()Ljava/lang/String;", public)]
    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableMap", "m", "Ljava/util/Map;")
            .await?;
        jvm.invoke_virtual(&map, "java/lang/Object", "toString", "()Ljava/lang/String;", ()).await
    }

    #[method(public)]
    async fn put(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _: ClassInstanceRef<Self>,
        _: ClassInstanceRef<Object>,
        _: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "unmodifiable map").await)
    }

    #[method(public)]
    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<ClassInstanceRef<Object>> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "unmodifiable map").await)
    }

    #[method(descriptor = "(Ljava/util/Map;)V", public)]
    async fn put_all(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<()> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "unmodifiable map").await)
    }

    #[method(public)]
    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<()> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "unmodifiable map").await)
    }
}
