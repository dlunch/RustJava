use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::Object};

use super::Hashtable;

// class java.util.Hashtable$Values
#[java_class(name = "java/util/Hashtable$Values", extends = "java/util/AbstractCollection")]
pub struct HashtableValues {
    #[field(private)]
    map: ClassInstanceRef<Hashtable>,
}

#[java_methods]
impl HashtableValues {
    #[method(constructor)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, map: ClassInstanceRef<Hashtable>) -> Result<()> {
        tracing::debug!("java.util.Hashtable$Values::<init>({this:?}, {map:?})");

        let _: () = jvm.invoke_special(&this, "java/util/AbstractCollection", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/util/Hashtable$Values", "map", "Ljava/util/Hashtable;", map)
            .await?;

        Ok(())
    }

    #[method(public)]
    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.Hashtable$Values::size({this:?})");

        let map: ClassInstanceRef<Hashtable> = jvm.get_field(&this, "java/util/Hashtable$Values", "map", "Ljava/util/Hashtable;").await?;

        jvm.invoke_virtual(&map, "java/util/Hashtable", "size", "()I", ()).await
    }

    #[method(public)]
    async fn is_empty(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.Hashtable$Values::isEmpty({this:?})");

        let map: ClassInstanceRef<Hashtable> = jvm.get_field(&this, "java/util/Hashtable$Values", "map", "Ljava/util/Hashtable;").await?;

        jvm.invoke_virtual(&map, "java/util/Hashtable", "isEmpty", "()Z", ()).await
    }

    #[method(public)]
    async fn contains(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Hashtable$Values::contains({this:?}, {value:?})");

        let map: ClassInstanceRef<Hashtable> = jvm.get_field(&this, "java/util/Hashtable$Values", "map", "Ljava/util/Hashtable;").await?;

        jvm.invoke_virtual(&map, "java/util/Hashtable", "containsValue", "(Ljava/lang/Object;)Z", (value,))
            .await
    }

    #[method(public)]
    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Hashtable$Values::remove({this:?}, {value:?})");

        if value.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Hashtable value is null").await);
        }

        let map: ClassInstanceRef<Hashtable> = jvm.get_field(&this, "java/util/Hashtable$Values", "map", "Ljava/util/Hashtable;").await?;
        let entries = Hashtable::entries_snapshot(jvm, &map).await?;
        let count = jvm.array_length(&entries).await?;
        for entry in jvm.load_array::<ClassInstanceRef<Object>>(&entries, 0, count).await? {
            let entry_value: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&entry, &entry.class_definition().name(), "getValue", "()Ljava/lang/Object;", ())
                .await?;
            let equal: bool = jvm
                .invoke_virtual(&value, "java/lang/Object", "equals", "(Ljava/lang/Object;)Z", (entry_value,))
                .await?;
            if equal {
                let key: ClassInstanceRef<Object> = jvm
                    .invoke_virtual(&entry, &entry.class_definition().name(), "getKey", "()Ljava/lang/Object;", ())
                    .await?;
                let _: ClassInstanceRef<Object> = jvm
                    .invoke_virtual(&map, "java/util/Hashtable", "remove", "(Ljava/lang/Object;)Ljava/lang/Object;", (key,))
                    .await?;
                return Ok(true);
            }
        }

        Ok(false)
    }

    #[method(public)]
    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Hashtable$Values::clear({this:?})");

        let map: ClassInstanceRef<Hashtable> = jvm.get_field(&this, "java/util/Hashtable$Values", "map", "Ljava/util/Hashtable;").await?;

        jvm.invoke_virtual(&map, "java/util/Hashtable", "clear", "()V", ()).await
    }

    #[method(descriptor = "()Ljava/util/Iterator;", public)]
    async fn iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Hashtable$Values::iterator({this:?})");

        let map: ClassInstanceRef<Hashtable> = jvm.get_field(&this, "java/util/Hashtable$Values", "map", "Ljava/util/Hashtable;").await?;
        let snapshot = Hashtable::values_snapshot(jvm, &map).await?;
        let iterator = jvm
            .new_class("java/util/Hashtable$Enumerator", "([Ljava/lang/Object;)V", (snapshot,))
            .await?;

        Ok(iterator.into())
    }
}
