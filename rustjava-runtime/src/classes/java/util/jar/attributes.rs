use alloc::vec;

use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm_types::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        lang::{Object, String},
        util::{Collection, Set, jar::AttributesName},
    },
};

// class java.util.jar.Attributes
pub struct Attributes;

impl Attributes {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/jar/Attributes",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/lang/Cloneable", "java/util/Map"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "putValue",
                    "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
                    Self::put_value,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "getValue",
                    "(Ljava/lang/String;)Ljava/lang/String;",
                    Self::get_value,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "getValue",
                    "(Ljava/util/jar/Attributes$Name;)Ljava/lang/String;",
                    Self::get_value_by_name,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("entrySet", "()Ljava/util/Set;", Self::entry_set, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("size", "()I", Self::size, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("isEmpty", "()Z", Self::is_empty, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("containsKey", "(Ljava/lang/Object;)Z", Self::contains_key, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("containsValue", "(Ljava/lang/Object;)Z", Self::contains_value, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("get", "(Ljava/lang/Object;)Ljava/lang/Object;", Self::get, MethodAccessFlags::PUBLIC),
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
                JavaMethodProto::new("keySet", "()Ljava/util/Set;", Self::key_set, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("values", "()Ljava/util/Collection;", Self::values, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![JavaFieldProto::new("map", "Ljava/util/Map;", FieldAccessFlags::PROTECTED)],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.jar.Attributes::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        let map = jvm.new_class("java/util/HashMap", "()V", ()).await?;
        jvm.put_field(&mut this, "java/util/jar/Attributes", "map", "Ljava/util/Map;", map)
            .await?;

        Ok(())
    }

    async fn put_value(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
        value: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.jar.Attributes::putValue({this:?}, {name:?}, {value:?})");

        let name = jvm.new_class("java/util/jar/Attributes$Name", "(Ljava/lang/String;)V", (name,)).await?;
        let old = jvm
            .invoke_virtual(
                &this,
                "java/util/jar/Attributes",
                "put",
                "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                (name, value),
            )
            .await?;

        Ok(old)
    }

    async fn get_value(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.jar.Attributes::getValue({this:?}, {name:?})");

        let name = jvm.new_class("java/util/jar/Attributes$Name", "(Ljava/lang/String;)V", (name,)).await?;
        jvm.invoke_virtual(
            &this,
            "java/util/jar/Attributes",
            "get",
            "(Ljava/lang/Object;)Ljava/lang/Object;",
            (name,),
        )
        .await
    }

    async fn get_value_by_name(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<AttributesName>,
    ) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.jar.Attributes::getValue({this:?}, {name:?})");

        let value = jvm
            .invoke_virtual(
                &this,
                "java/util/jar/Attributes",
                "get",
                "(Ljava/lang/Object;)Ljava/lang/Object;",
                (name,),
            )
            .await?;

        Ok(value)
    }

    async fn entry_set(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Set>> {
        tracing::debug!("java.util.jar.Attributes::entrySet({this:?})");

        let map = jvm.get_field(&this, "java/util/jar/Attributes", "map", "Ljava/util/Map;").await?;
        jvm.invoke_virtual(&map, "java/util/Map", "entrySet", "()Ljava/util/Set;", ()).await
    }

    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let map = jvm.get_field(&this, "java/util/jar/Attributes", "map", "Ljava/util/Map;").await?;
        jvm.invoke_virtual(&map, "java/util/Map", "size", "()I", ()).await
    }

    async fn is_empty(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let map = jvm.get_field(&this, "java/util/jar/Attributes", "map", "Ljava/util/Map;").await?;
        jvm.invoke_virtual(&map, "java/util/Map", "isEmpty", "()Z", ()).await
    }

    async fn contains_key(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key: ClassInstanceRef<Object>) -> Result<bool> {
        let map = jvm.get_field(&this, "java/util/jar/Attributes", "map", "Ljava/util/Map;").await?;
        jvm.invoke_virtual(&map, "java/util/Map", "containsKey", "(Ljava/lang/Object;)Z", (key,))
            .await
    }

    async fn contains_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Object>) -> Result<bool> {
        let map = jvm.get_field(&this, "java/util/jar/Attributes", "map", "Ljava/util/Map;").await?;
        jvm.invoke_virtual(&map, "java/util/Map", "containsValue", "(Ljava/lang/Object;)Z", (value,))
            .await
    }

    async fn get(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key: ClassInstanceRef<Object>) -> Result<ClassInstanceRef<Object>> {
        let map = jvm.get_field(&this, "java/util/jar/Attributes", "map", "Ljava/util/Map;").await?;
        jvm.invoke_virtual(&map, "java/util/Map", "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (key,))
            .await
    }

    async fn put(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        key: ClassInstanceRef<Object>,
        value: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        if !key.is_null() && !jvm.is_instance(&**key, "java/util/jar/Attributes$Name") {
            return Err(jvm
                .exception("java/lang/ClassCastException", "Attribute key must be Attributes.Name")
                .await);
        }
        if !value.is_null() && !jvm.is_instance(&**value, "java/lang/String") {
            return Err(jvm.exception("java/lang/ClassCastException", "Attribute value must be String").await);
        }
        let map = jvm.get_field(&this, "java/util/jar/Attributes", "map", "Ljava/util/Map;").await?;
        jvm.invoke_virtual(
            &map,
            "java/util/Map",
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (key, value),
        )
        .await
    }

    async fn remove(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        key: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        let map = jvm.get_field(&this, "java/util/jar/Attributes", "map", "Ljava/util/Map;").await?;
        jvm.invoke_virtual(&map, "java/util/Map", "remove", "(Ljava/lang/Object;)Ljava/lang/Object;", (key,))
            .await
    }

    async fn put_all(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, source: ClassInstanceRef<Object>) -> Result<()> {
        if source.is_null() || !jvm.is_instance(&**source, "java/util/jar/Attributes") {
            return Err(jvm.exception("java/lang/ClassCastException", "Source must be Attributes").await);
        }
        let entries = jvm.invoke_virtual(&source, "java/util/Map", "entrySet", "()Ljava/util/Set;", ()).await?;
        let iterator = jvm
            .invoke_virtual(&entries, "java/util/Set", "iterator", "()Ljava/util/Iterator;", ())
            .await?;
        while jvm.invoke_virtual(&iterator, "java/util/Iterator", "hasNext", "()Z", ()).await? {
            let entry = jvm
                .invoke_virtual(&iterator, "java/util/Iterator", "next", "()Ljava/lang/Object;", ())
                .await?;
            let key: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&entry, "java/util/Map$Entry", "getKey", "()Ljava/lang/Object;", ())
                .await?;
            let value: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&entry, "java/util/Map$Entry", "getValue", "()Ljava/lang/Object;", ())
                .await?;
            let _: ClassInstanceRef<Object> = jvm
                .invoke_virtual(
                    &this,
                    "java/util/jar/Attributes",
                    "put",
                    "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                    (key, value),
                )
                .await?;
        }
        Ok(())
    }

    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let map = jvm.get_field(&this, "java/util/jar/Attributes", "map", "Ljava/util/Map;").await?;
        jvm.invoke_virtual(&map, "java/util/Map", "clear", "()V", ()).await
    }

    async fn key_set(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Set>> {
        let map = jvm.get_field(&this, "java/util/jar/Attributes", "map", "Ljava/util/Map;").await?;
        jvm.invoke_virtual(&map, "java/util/Map", "keySet", "()Ljava/util/Set;", ()).await
    }

    async fn values(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Collection>> {
        let map = jvm.get_field(&this, "java/util/jar/Attributes", "map", "Ljava/util/Map;").await?;
        jvm.invoke_virtual(&map, "java/util/Map", "values", "()Ljava/util/Collection;", ()).await
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        let map = jvm.get_field(&this, "java/util/jar/Attributes", "map", "Ljava/util/Map;").await?;
        jvm.invoke_virtual(&map, "java/lang/Object", "equals", "(Ljava/lang/Object;)Z", (other,))
            .await
    }

    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let map = jvm.get_field(&this, "java/util/jar/Attributes", "map", "Ljava/util/Map;").await?;
        jvm.invoke_virtual(&map, "java/lang/Object", "hashCode", "()I", ()).await
    }
}
