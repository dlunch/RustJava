use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::Object};

use super::HashMap;

// class java.util.HashMap$EntrySet
#[java_class(name = "java/util/HashMap$EntrySet", extends = "java/util/AbstractSet", final)]
pub struct HashMapEntrySet {
    #[field(private)]
    map: ClassInstanceRef<HashMap>,
}

#[java_methods]
impl HashMapEntrySet {
    #[method(constructor)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, map: ClassInstanceRef<HashMap>) -> Result<()> {
        tracing::debug!("java.util.HashMap$EntrySet::<init>({this:?}, {map:?})");

        let _: () = jvm.invoke_special(&this, "java/util/AbstractSet", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/util/HashMap$EntrySet", "map", "Ljava/util/HashMap;", map)
            .await?;

        Ok(())
    }

    #[method(public, final)]
    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.HashMap$EntrySet::size({this:?})");

        let map: ClassInstanceRef<HashMap> = jvm.get_field(&this, "java/util/HashMap$EntrySet", "map", "Ljava/util/HashMap;").await?;

        jvm.invoke_virtual(&map, "java/util/HashMap", "size", "()I", ()).await
    }

    #[method(public)]
    async fn is_empty(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.HashMap$EntrySet::isEmpty({this:?})");

        let map: ClassInstanceRef<HashMap> = jvm.get_field(&this, "java/util/HashMap$EntrySet", "map", "Ljava/util/HashMap;").await?;

        jvm.invoke_virtual(&map, "java/util/HashMap", "isEmpty", "()Z", ()).await
    }

    #[method(public, final)]
    async fn contains(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, candidate: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.HashMap$EntrySet::contains({this:?}, {candidate:?})");

        if candidate.is_null() || !jvm.is_instance(&**candidate, "java/util/Map$Entry") {
            return Ok(false);
        }

        let map: ClassInstanceRef<HashMap> = jvm.get_field(&this, "java/util/HashMap$EntrySet", "map", "Ljava/util/HashMap;").await?;
        let candidate_key: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&candidate, &candidate.class_definition().name(), "getKey", "()Ljava/lang/Object;", ())
            .await?;

        let entry = HashMap::find_entry(jvm, &map, &candidate_key).await?;
        if entry.is_null() {
            return Ok(false);
        }

        let entry_key: ClassInstanceRef<Object> = jvm.get_field(&entry, "java/util/HashMap$Entry", "key", "Ljava/lang/Object;").await?;
        if !Self::object_equals(jvm, &entry_key, &candidate_key).await? {
            return Ok(false);
        }

        let candidate_value: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&candidate, &candidate.class_definition().name(), "getValue", "()Ljava/lang/Object;", ())
            .await?;
        let entry_value: ClassInstanceRef<Object> = jvm.get_field(&entry, "java/util/HashMap$Entry", "value", "Ljava/lang/Object;").await?;

        Self::object_equals(jvm, &entry_value, &candidate_value).await
    }

    #[method(public)]
    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, candidate: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.HashMap$EntrySet::remove({this:?}, {candidate:?})");

        if candidate.is_null() || !jvm.is_instance(&**candidate, "java/util/Map$Entry") {
            return Ok(false);
        }

        let map: ClassInstanceRef<HashMap> = jvm.get_field(&this, "java/util/HashMap$EntrySet", "map", "Ljava/util/HashMap;").await?;
        let candidate_key: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&candidate, &candidate.class_definition().name(), "getKey", "()Ljava/lang/Object;", ())
            .await?;
        let entry = HashMap::find_entry(jvm, &map, &candidate_key).await?;
        if entry.is_null() {
            return Ok(false);
        }

        let entry_key: ClassInstanceRef<Object> = jvm.get_field(&entry, "java/util/HashMap$Entry", "key", "Ljava/lang/Object;").await?;
        if !Self::object_equals(jvm, &entry_key, &candidate_key).await? {
            return Ok(false);
        }

        let candidate_value: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&candidate, &candidate.class_definition().name(), "getValue", "()Ljava/lang/Object;", ())
            .await?;
        let entry_value: ClassInstanceRef<Object> = jvm.get_field(&entry, "java/util/HashMap$Entry", "value", "Ljava/lang/Object;").await?;
        if !Self::object_equals(jvm, &entry_value, &candidate_value).await? {
            return Ok(false);
        }

        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &map,
                "java/util/HashMap",
                "remove",
                "(Ljava/lang/Object;)Ljava/lang/Object;",
                (candidate_key,),
            )
            .await?;

        Ok(true)
    }

    #[method(public, final)]
    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.HashMap$EntrySet::clear({this:?})");

        let map: ClassInstanceRef<HashMap> = jvm.get_field(&this, "java/util/HashMap$EntrySet", "map", "Ljava/util/HashMap;").await?;

        jvm.invoke_virtual(&map, "java/util/HashMap", "clear", "()V", ()).await
    }

    #[method(descriptor = "()Ljava/util/Iterator;", public, final)]
    async fn iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.HashMap$EntrySet::iterator({this:?})");

        let map: ClassInstanceRef<HashMap> = jvm.get_field(&this, "java/util/HashMap$EntrySet", "map", "Ljava/util/HashMap;").await?;

        jvm.invoke_virtual(&map, "java/util/HashMap", "entryIterator", "()Ljava/util/Iterator;", ())
            .await
    }

    async fn object_equals(jvm: &Jvm, left: &ClassInstanceRef<Object>, right: &ClassInstanceRef<Object>) -> Result<bool> {
        if left.is_null() {
            return Ok(right.is_null());
        }

        jvm.invoke_virtual(left, "java/lang/Object", "equals", "(Ljava/lang/Object;)Z", (right.clone(),))
            .await
    }
}
