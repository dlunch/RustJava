use jvm::{Array, ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::Object};

use super::HashMap;

const DEFAULT_INITIAL_CAPACITY: i32 = 16;

// class java.util.HashSet
#[java_class(
    name = "java/util/HashSet",
    extends = "java/util/AbstractSet",
    implements("java/lang/Cloneable", "java/io/Serializable"),
    public
)]
pub struct HashSet {
    #[field(transient)]
    map: ClassInstanceRef<HashMap>,
    #[field(private, final)]
    present: ClassInstanceRef<Object>,
}

#[java_methods]
impl HashSet {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.HashSet::<init>({this:?})");

        let _: () = jvm
            .invoke_special(&this, "java/util/HashSet", "<init>", "(I)V", (DEFAULT_INITIAL_CAPACITY,))
            .await?;

        Ok(())
    }

    #[method(constructor, public)]
    async fn init_with_capacity(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, capacity: i32) -> Result<()> {
        tracing::debug!("java.util.HashSet::<init>({this:?}, {capacity:?})");

        let _: () = jvm.invoke_special(&this, "java/util/AbstractSet", "<init>", "()V", ()).await?;

        let map: ClassInstanceRef<HashMap> = jvm.new_class("java/util/HashMap", "(I)V", (capacity,)).await?.into();
        let present: ClassInstanceRef<Object> = jvm.new_class("java/lang/Object", "()V", ()).await?.into();

        jvm.put_field(&mut this, "java/util/HashSet", "map", "Ljava/util/HashMap;", map).await?;
        jvm.put_field(&mut this, "java/util/HashSet", "present", "Ljava/lang/Object;", present)
            .await?;

        Ok(())
    }

    #[method(constructor, descriptor = "(Ljava/util/Collection;)V", public)]
    async fn init_from_collection(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        collection: ClassInstanceRef<Object>,
    ) -> Result<()> {
        tracing::debug!("java.util.HashSet::<init>({this:?}, {collection:?})");

        if collection.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "collection").await);
        }
        let size: i32 = jvm
            .invoke_virtual(&collection, &collection.class_definition().name(), "size", "()I", ())
            .await?;
        let capacity = size.saturating_mul(2).max(DEFAULT_INITIAL_CAPACITY);
        let _: () = jvm.invoke_special(&this, "java/util/HashSet", "<init>", "(I)V", (capacity,)).await?;
        let _: bool = jvm
            .invoke_virtual(&this, "java/util/HashSet", "addAll", "(Ljava/util/Collection;)Z", (collection,))
            .await?;

        Ok(())
    }

    #[method(public)]
    async fn add(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.HashSet::add({this:?}, {element:?})");

        let map: ClassInstanceRef<HashMap> = jvm.get_field(&this, "java/util/HashSet", "map", "Ljava/util/HashMap;").await?;
        let present: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/HashSet", "present", "Ljava/lang/Object;").await?;
        let old: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &map,
                "java/util/HashMap",
                "put",
                "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                (element, present),
            )
            .await?;

        Ok(old.is_null())
    }

    #[method(public)]
    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.HashSet::remove({this:?}, {element:?})");

        let map: ClassInstanceRef<HashMap> = jvm.get_field(&this, "java/util/HashSet", "map", "Ljava/util/HashMap;").await?;
        let old: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&map, "java/util/HashMap", "remove", "(Ljava/lang/Object;)Ljava/lang/Object;", (element,))
            .await?;

        Ok(!old.is_null())
    }

    #[method(public)]
    async fn contains(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.HashSet::contains({this:?}, {element:?})");

        let map: ClassInstanceRef<HashMap> = jvm.get_field(&this, "java/util/HashSet", "map", "Ljava/util/HashMap;").await?;

        jvm.invoke_virtual(&map, "java/util/HashMap", "containsKey", "(Ljava/lang/Object;)Z", (element,))
            .await
    }

    #[method(public)]
    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.HashSet::size({this:?})");

        let map: ClassInstanceRef<HashMap> = jvm.get_field(&this, "java/util/HashSet", "map", "Ljava/util/HashMap;").await?;

        jvm.invoke_virtual(&map, "java/util/HashMap", "size", "()I", ()).await
    }

    #[method(public)]
    async fn is_empty(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.HashSet::isEmpty({this:?})");

        let map: ClassInstanceRef<HashMap> = jvm.get_field(&this, "java/util/HashSet", "map", "Ljava/util/HashMap;").await?;

        jvm.invoke_virtual(&map, "java/util/HashMap", "isEmpty", "()Z", ()).await
    }

    #[method(public)]
    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.HashSet::clear({this:?})");

        let map: ClassInstanceRef<HashMap> = jvm.get_field(&this, "java/util/HashSet", "map", "Ljava/util/HashMap;").await?;

        jvm.invoke_virtual(&map, "java/util/HashMap", "clear", "()V", ()).await
    }

    #[method(descriptor = "()Ljava/util/Iterator;", public)]
    async fn iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.HashSet::iterator({this:?})");

        let key_set = Self::key_set(jvm, &this).await?;

        jvm.invoke_virtual(&key_set, &key_set.class_definition().name(), "iterator", "()Ljava/util/Iterator;", ())
            .await
    }

    #[method(public)]
    async fn to_array(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Array<Object>>> {
        tracing::debug!("java.util.HashSet::toArray({this:?})");

        let key_set = Self::key_set(jvm, &this).await?;

        jvm.invoke_virtual(&key_set, &key_set.class_definition().name(), "toArray", "()[Ljava/lang/Object;", ())
            .await
    }

    async fn key_set(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<HashMap> = jvm.get_field(this, "java/util/HashSet", "map", "Ljava/util/HashMap;").await?;

        jvm.invoke_virtual(&map, "java/util/HashMap", "keySet", "()Ljava/util/Set;", ()).await
    }
}
