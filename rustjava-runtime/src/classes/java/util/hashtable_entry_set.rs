use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::Object};

use super::Hashtable;

// class java.util.Hashtable$EntrySet
#[java_class(name = "java/util/Hashtable$EntrySet", extends = "java/util/AbstractSet")]
pub struct HashtableEntrySet {
    #[field(private)]
    map: ClassInstanceRef<Hashtable>,
}

#[java_methods]
impl HashtableEntrySet {
    #[method(constructor, private)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, map: ClassInstanceRef<Hashtable>) -> Result<()> {
        tracing::debug!("java.util.Hashtable$EntrySet::<init>({this:?}, {map:?})");

        let _: () = jvm.invoke_special(&this, "java/util/AbstractSet", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/util/Hashtable$EntrySet", "map", "Ljava/util/Hashtable;", map)
            .await?;

        Ok(())
    }

    #[method(public)]
    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.Hashtable$EntrySet::size({this:?})");

        let map: ClassInstanceRef<Hashtable> = jvm
            .get_field(&this, "java/util/Hashtable$EntrySet", "map", "Ljava/util/Hashtable;")
            .await?;

        jvm.invoke_virtual(&map, "java/util/Hashtable", "size", "()I", ()).await
    }

    #[method(public)]
    async fn is_empty(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.Hashtable$EntrySet::isEmpty({this:?})");

        let map: ClassInstanceRef<Hashtable> = jvm
            .get_field(&this, "java/util/Hashtable$EntrySet", "map", "Ljava/util/Hashtable;")
            .await?;

        jvm.invoke_virtual(&map, "java/util/Hashtable", "isEmpty", "()Z", ()).await
    }

    #[method(public)]
    async fn contains(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, candidate: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Hashtable$EntrySet::contains({this:?}, {candidate:?})");

        if candidate.is_null() || !jvm.is_instance(&**candidate, "java/util/Map$Entry") {
            return Ok(false);
        }

        let candidate_key: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&candidate, &candidate.class_definition().name(), "getKey", "()Ljava/lang/Object;", ())
            .await?;
        if candidate_key.is_null() {
            return Ok(false);
        }

        let map: ClassInstanceRef<Hashtable> = jvm
            .get_field(&this, "java/util/Hashtable$EntrySet", "map", "Ljava/util/Hashtable;")
            .await?;
        let entry = Hashtable::find_entry(jvm, &map, &candidate_key).await?;
        if entry.is_null() {
            return Ok(false);
        }

        let candidate_value: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&candidate, &candidate.class_definition().name(), "getValue", "()Ljava/lang/Object;", ())
            .await?;
        let entry_value: ClassInstanceRef<Object> = jvm.get_field(&entry, "java/util/Hashtable$Entry", "value", "Ljava/lang/Object;").await?;

        Self::object_equals(jvm, &entry_value, &candidate_value).await
    }

    #[method(public)]
    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, candidate: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Hashtable$EntrySet::remove({this:?}, {candidate:?})");

        if candidate.is_null() || !jvm.is_instance(&**candidate, "java/util/Map$Entry") {
            return Ok(false);
        }

        let candidate_key: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&candidate, &candidate.class_definition().name(), "getKey", "()Ljava/lang/Object;", ())
            .await?;
        if candidate_key.is_null() {
            return Ok(false);
        }
        let map: ClassInstanceRef<Hashtable> = jvm
            .get_field(&this, "java/util/Hashtable$EntrySet", "map", "Ljava/util/Hashtable;")
            .await?;
        let entry = Hashtable::find_entry(jvm, &map, &candidate_key).await?;
        if entry.is_null() {
            return Ok(false);
        }
        let candidate_value: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&candidate, &candidate.class_definition().name(), "getValue", "()Ljava/lang/Object;", ())
            .await?;
        let entry_value: ClassInstanceRef<Object> = jvm.get_field(&entry, "java/util/Hashtable$Entry", "value", "Ljava/lang/Object;").await?;
        if !Self::object_equals(jvm, &entry_value, &candidate_value).await? {
            return Ok(false);
        }

        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &map,
                "java/util/Hashtable",
                "remove",
                "(Ljava/lang/Object;)Ljava/lang/Object;",
                (candidate_key,),
            )
            .await?;

        Ok(true)
    }

    #[method(public)]
    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Hashtable$EntrySet::clear({this:?})");

        let map: ClassInstanceRef<Hashtable> = jvm
            .get_field(&this, "java/util/Hashtable$EntrySet", "map", "Ljava/util/Hashtable;")
            .await?;

        jvm.invoke_virtual(&map, "java/util/Hashtable", "clear", "()V", ()).await
    }

    #[method(descriptor = "()Ljava/util/Iterator;", public)]
    async fn iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Hashtable$EntrySet::iterator({this:?})");

        let map: ClassInstanceRef<Hashtable> = jvm
            .get_field(&this, "java/util/Hashtable$EntrySet", "map", "Ljava/util/Hashtable;")
            .await?;
        let snapshot = Hashtable::entries_snapshot(jvm, &map).await?;
        let iterator = jvm
            .new_class("java/util/Hashtable$Enumerator", "([Ljava/lang/Object;)V", (snapshot,))
            .await?;

        Ok(iterator.into())
    }

    async fn object_equals(jvm: &Jvm, left: &ClassInstanceRef<Object>, right: &ClassInstanceRef<Object>) -> Result<bool> {
        if left.is_null() {
            return Ok(right.is_null());
        }

        jvm.invoke_virtual(left, "java/lang/Object", "equals", "(Ljava/lang/Object;)Z", (right.clone(),))
            .await
    }
}
