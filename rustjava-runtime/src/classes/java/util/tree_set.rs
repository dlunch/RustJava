use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{lang::Object, util::SortedMap},
};

// public class java.util.TreeSet
#[java_class(
    name = "java/util/TreeSet",
    extends = "java/util/AbstractSet",
    implements("java/util/SortedSet", "java/lang/Cloneable", "java/io/Serializable"),
    public
)]
pub struct TreeSet {
    #[field(private, transient)]
    m: ClassInstanceRef<SortedMap>,
    #[field(private, static, final)]
    PRESENT: ClassInstanceRef<Object>,
}

#[java_methods]
impl TreeSet {
    #[method(static_initializer)]
    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let present: ClassInstanceRef<Object> = jvm.new_class("java/lang/Object", "()V", ()).await?.into();
        jvm.put_static_field("java/util/TreeSet", "PRESENT", "Ljava/lang/Object;", present).await
    }

    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/util/AbstractSet", "<init>", "()V", ()).await?;
        let map: ClassInstanceRef<Object> = jvm.new_class("java/util/TreeMap", "()V", ()).await?.into();
        let mut this = this;
        jvm.put_field(&mut this, "java/util/TreeSet", "m", "Ljava/util/SortedMap;", map).await
    }

    #[method(constructor, descriptor = "(Ljava/util/Comparator;)V", public)]
    async fn init_comparator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, comparator: ClassInstanceRef<Object>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/util/AbstractSet", "<init>", "()V", ()).await?;
        let map: ClassInstanceRef<Object> = jvm
            .new_class("java/util/TreeMap", "(Ljava/util/Comparator;)V", (comparator,))
            .await?
            .into();
        let mut this = this;
        jvm.put_field(&mut this, "java/util/TreeSet", "m", "Ljava/util/SortedMap;", map).await
    }

    #[method(constructor, descriptor = "(Ljava/util/Collection;)V", public)]
    async fn init_collection(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, collection: ClassInstanceRef<Object>) -> Result<()> {
        if collection.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "collection").await);
        }
        let _: () = jvm.invoke_special(&this, "java/util/TreeSet", "<init>", "()V", ()).await?;
        let _: bool = jvm
            .invoke_virtual(&this, "java/util/TreeSet", "addAll", "(Ljava/util/Collection;)Z", (collection,))
            .await?;
        Ok(())
    }

    #[method(constructor, descriptor = "(Ljava/util/SortedSet;)V", public)]
    async fn init_sorted_set(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, set: ClassInstanceRef<Object>) -> Result<()> {
        if set.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "set").await);
        }
        let comparator: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&set, &set.class_definition().name(), "comparator", "()Ljava/util/Comparator;", ())
            .await?;
        let _: () = jvm
            .invoke_special(&this, "java/util/TreeSet", "<init>", "(Ljava/util/Comparator;)V", (comparator,))
            .await?;
        let _: bool = jvm
            .invoke_virtual(&this, "java/util/TreeSet", "addAll", "(Ljava/util/Collection;)Z", (set,))
            .await?;
        Ok(())
    }

    #[method(constructor, descriptor = "(Ljava/util/SortedMap;)V", private)]
    async fn init_sorted_map(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, map: ClassInstanceRef<Object>) -> Result<()> {
        if map.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "map").await);
        }
        let _: () = jvm.invoke_special(&this, "java/util/AbstractSet", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/util/TreeSet", "m", "Ljava/util/SortedMap;", map).await
    }

    #[method(public)]
    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/TreeSet", "m", "Ljava/util/SortedMap;").await?;
        jvm.invoke_virtual(&map, &map.class_definition().name(), "size", "()I", ()).await
    }

    #[method(public)]
    async fn contains(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/TreeSet", "m", "Ljava/util/SortedMap;").await?;
        jvm.invoke_virtual(&map, &map.class_definition().name(), "containsKey", "(Ljava/lang/Object;)Z", (element,))
            .await
    }

    #[method(public)]
    async fn add(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/TreeSet", "m", "Ljava/util/SortedMap;").await?;
        let present: ClassInstanceRef<Object> = jvm.get_static_field("java/util/TreeSet", "PRESENT", "Ljava/lang/Object;").await?;
        let old: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &map,
                &map.class_definition().name(),
                "put",
                "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                (element, present),
            )
            .await?;
        Ok(old.is_null())
    }

    #[method(public)]
    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/TreeSet", "m", "Ljava/util/SortedMap;").await?;
        if !jvm
            .invoke_virtual::<_, bool>(
                &map,
                &map.class_definition().name(),
                "containsKey",
                "(Ljava/lang/Object;)Z",
                (element.clone(),),
            )
            .await?
        {
            return Ok(false);
        }
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &map,
                &map.class_definition().name(),
                "remove",
                "(Ljava/lang/Object;)Ljava/lang/Object;",
                (element,),
            )
            .await?;
        Ok(true)
    }

    #[method(public)]
    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/TreeSet", "m", "Ljava/util/SortedMap;").await?;
        jvm.invoke_virtual(&map, &map.class_definition().name(), "clear", "()V", ()).await
    }

    #[method(descriptor = "()Ljava/util/Iterator;", public)]
    async fn iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/TreeSet", "m", "Ljava/util/SortedMap;").await?;
        let keys: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&map, &map.class_definition().name(), "keySet", "()Ljava/util/Set;", ())
            .await?;
        jvm.invoke_virtual(&keys, &keys.class_definition().name(), "iterator", "()Ljava/util/Iterator;", ())
            .await
    }

    #[method(descriptor = "()Ljava/util/Comparator;", public)]
    async fn comparator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/TreeSet", "m", "Ljava/util/SortedMap;").await?;
        jvm.invoke_virtual(&map, &map.class_definition().name(), "comparator", "()Ljava/util/Comparator;", ())
            .await
    }

    #[method(public)]
    async fn first(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/TreeSet", "m", "Ljava/util/SortedMap;").await?;
        jvm.invoke_virtual(&map, &map.class_definition().name(), "firstKey", "()Ljava/lang/Object;", ())
            .await
    }

    #[method(public)]
    async fn last(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/TreeSet", "m", "Ljava/util/SortedMap;").await?;
        jvm.invoke_virtual(&map, &map.class_definition().name(), "lastKey", "()Ljava/lang/Object;", ())
            .await
    }

    #[method(descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedSet;", public)]
    async fn sub_set(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        from: ClassInstanceRef<Object>,
        to: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/TreeSet", "m", "Ljava/util/SortedMap;").await?;
        let range: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &map,
                &map.class_definition().name(),
                "subMap",
                "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;",
                (from, to),
            )
            .await?;
        Ok(jvm.new_class("java/util/TreeSet", "(Ljava/util/SortedMap;)V", (range,)).await?.into())
    }

    #[method(descriptor = "(Ljava/lang/Object;)Ljava/util/SortedSet;", public)]
    async fn head_set(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        to: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/TreeSet", "m", "Ljava/util/SortedMap;").await?;
        let range: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &map,
                &map.class_definition().name(),
                "headMap",
                "(Ljava/lang/Object;)Ljava/util/SortedMap;",
                (to,),
            )
            .await?;
        Ok(jvm.new_class("java/util/TreeSet", "(Ljava/util/SortedMap;)V", (range,)).await?.into())
    }

    #[method(descriptor = "(Ljava/lang/Object;)Ljava/util/SortedSet;", public)]
    async fn tail_set(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        from: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/TreeSet", "m", "Ljava/util/SortedMap;").await?;
        let range: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &map,
                &map.class_definition().name(),
                "tailMap",
                "(Ljava/lang/Object;)Ljava/util/SortedMap;",
                (from,),
            )
            .await?;
        Ok(jvm.new_class("java/util/TreeSet", "(Ljava/util/SortedMap;)V", (range,)).await?.into())
    }
}
