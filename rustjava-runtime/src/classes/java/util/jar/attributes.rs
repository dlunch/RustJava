use alloc::boxed::Box;

use jvm::{ClassInstance, ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{
        lang::{Object, String},
        util::{Collection, Map, Set, jar::AttributesName},
    },
};

// class java.util.jar.Attributes
#[java_class(
    name = "java/util/jar/Attributes",
    extends = "java/lang/Object",
    implements("java/lang/Cloneable", "java/util/Map"),
    public
)]
pub struct Attributes {
    #[field(protected)]
    map: ClassInstanceRef<Map>,
}

#[java_methods]
impl Attributes {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.jar.Attributes::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        let map = jvm.new_class("java/util/HashMap", "()V", ()).await?;
        jvm.put_field(&mut this, "java/util/jar/Attributes", "map", "Ljava/util/Map;", map)
            .await?;

        Ok(())
    }

    #[method(public)]
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

    #[method(public)]
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

    #[method(name = "getValue", public)]
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

    #[method(public)]
    async fn entry_set(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Set>> {
        tracing::debug!("java.util.jar.Attributes::entrySet({this:?})");

        let map: Box<dyn ClassInstance> = jvm.get_field(&this, "java/util/jar/Attributes", "map", "Ljava/util/Map;").await?;
        jvm.invoke_virtual(&map, "java/util/Map", "entrySet", "()Ljava/util/Set;", ()).await
    }

    #[method(public)]
    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let map: Box<dyn ClassInstance> = jvm.get_field(&this, "java/util/jar/Attributes", "map", "Ljava/util/Map;").await?;
        jvm.invoke_virtual(&map, "java/util/Map", "size", "()I", ()).await
    }

    #[method(public)]
    async fn is_empty(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let map: Box<dyn ClassInstance> = jvm.get_field(&this, "java/util/jar/Attributes", "map", "Ljava/util/Map;").await?;
        jvm.invoke_virtual(&map, "java/util/Map", "isEmpty", "()Z", ()).await
    }

    #[method(public)]
    async fn contains_key(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key: ClassInstanceRef<Object>) -> Result<bool> {
        let map: Box<dyn ClassInstance> = jvm.get_field(&this, "java/util/jar/Attributes", "map", "Ljava/util/Map;").await?;
        jvm.invoke_virtual(&map, "java/util/Map", "containsKey", "(Ljava/lang/Object;)Z", (key,))
            .await
    }

    #[method(public)]
    async fn contains_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Object>) -> Result<bool> {
        let map: Box<dyn ClassInstance> = jvm.get_field(&this, "java/util/jar/Attributes", "map", "Ljava/util/Map;").await?;
        jvm.invoke_virtual(&map, "java/util/Map", "containsValue", "(Ljava/lang/Object;)Z", (value,))
            .await
    }

    #[method(public)]
    async fn get(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key: ClassInstanceRef<Object>) -> Result<ClassInstanceRef<Object>> {
        let map: Box<dyn ClassInstance> = jvm.get_field(&this, "java/util/jar/Attributes", "map", "Ljava/util/Map;").await?;
        jvm.invoke_virtual(&map, "java/util/Map", "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (key,))
            .await
    }

    #[method(public)]
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
        let map: Box<dyn ClassInstance> = jvm.get_field(&this, "java/util/jar/Attributes", "map", "Ljava/util/Map;").await?;
        jvm.invoke_virtual(
            &map,
            "java/util/Map",
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (key, value),
        )
        .await
    }

    #[method(public)]
    async fn remove(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        key: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        let map: Box<dyn ClassInstance> = jvm.get_field(&this, "java/util/jar/Attributes", "map", "Ljava/util/Map;").await?;
        jvm.invoke_virtual(&map, "java/util/Map", "remove", "(Ljava/lang/Object;)Ljava/lang/Object;", (key,))
            .await
    }

    #[method(descriptor = "(Ljava/util/Map;)V", public)]
    async fn put_all(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, source: ClassInstanceRef<Object>) -> Result<()> {
        if source.is_null() || !jvm.is_instance(&**source, "java/util/jar/Attributes") {
            return Err(jvm.exception("java/lang/ClassCastException", "Source must be Attributes").await);
        }
        let entries: Box<dyn ClassInstance> = jvm.invoke_virtual(&source, "java/util/Map", "entrySet", "()Ljava/util/Set;", ()).await?;
        let iterator: Box<dyn ClassInstance> = jvm
            .invoke_virtual(&entries, "java/util/Set", "iterator", "()Ljava/util/Iterator;", ())
            .await?;
        while jvm.invoke_virtual(&iterator, "java/util/Iterator", "hasNext", "()Z", ()).await? {
            let entry: Box<dyn ClassInstance> = jvm
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

    #[method(public)]
    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let map: Box<dyn ClassInstance> = jvm.get_field(&this, "java/util/jar/Attributes", "map", "Ljava/util/Map;").await?;
        jvm.invoke_virtual(&map, "java/util/Map", "clear", "()V", ()).await
    }

    #[method(public)]
    async fn key_set(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Set>> {
        let map: Box<dyn ClassInstance> = jvm.get_field(&this, "java/util/jar/Attributes", "map", "Ljava/util/Map;").await?;
        jvm.invoke_virtual(&map, "java/util/Map", "keySet", "()Ljava/util/Set;", ()).await
    }

    #[method(public)]
    async fn values(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Collection>> {
        let map: Box<dyn ClassInstance> = jvm.get_field(&this, "java/util/jar/Attributes", "map", "Ljava/util/Map;").await?;
        jvm.invoke_virtual(&map, "java/util/Map", "values", "()Ljava/util/Collection;", ()).await
    }

    #[method(public)]
    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        let map: Box<dyn ClassInstance> = jvm.get_field(&this, "java/util/jar/Attributes", "map", "Ljava/util/Map;").await?;
        jvm.invoke_virtual(&map, "java/lang/Object", "equals", "(Ljava/lang/Object;)Z", (other,))
            .await
    }

    #[method(public)]
    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let map: Box<dyn ClassInstance> = jvm.get_field(&this, "java/util/jar/Attributes", "map", "Ljava/util/Map;").await?;
        jvm.invoke_virtual(&map, "java/lang/Object", "hashCode", "()I", ()).await
    }
}
