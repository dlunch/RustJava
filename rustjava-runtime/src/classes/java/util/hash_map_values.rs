use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::Object};

use super::HashMap;

// class java.util.HashMap$Values
#[java_class(name = "java/util/HashMap$Values", extends = "java/util/AbstractCollection", final)]
pub struct HashMapValues {
    #[field(private)]
    map: ClassInstanceRef<HashMap>,
}

#[java_methods]
impl HashMapValues {
    #[method(constructor)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, map: ClassInstanceRef<HashMap>) -> Result<()> {
        tracing::debug!("java.util.HashMap$Values::<init>({this:?}, {map:?})");

        let _: () = jvm.invoke_special(&this, "java/util/AbstractCollection", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/util/HashMap$Values", "map", "Ljava/util/HashMap;", map)
            .await?;

        Ok(())
    }

    #[method(public, final)]
    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.HashMap$Values::size({this:?})");

        let map: ClassInstanceRef<HashMap> = jvm.get_field(&this, "java/util/HashMap$Values", "map", "Ljava/util/HashMap;").await?;

        jvm.invoke_virtual(&map, "java/util/HashMap", "size", "()I", ()).await
    }

    #[method(public)]
    async fn is_empty(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.HashMap$Values::isEmpty({this:?})");

        let map: ClassInstanceRef<HashMap> = jvm.get_field(&this, "java/util/HashMap$Values", "map", "Ljava/util/HashMap;").await?;

        jvm.invoke_virtual(&map, "java/util/HashMap", "isEmpty", "()Z", ()).await
    }

    #[method(public, final)]
    async fn contains(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.HashMap$Values::contains({this:?}, {value:?})");

        let map: ClassInstanceRef<HashMap> = jvm.get_field(&this, "java/util/HashMap$Values", "map", "Ljava/util/HashMap;").await?;

        jvm.invoke_virtual(&map, "java/util/HashMap", "containsValue", "(Ljava/lang/Object;)Z", (value,))
            .await
    }

    #[method(public)]
    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.HashMap$Values::remove({this:?}, {value:?})");

        let map: ClassInstanceRef<HashMap> = jvm.get_field(&this, "java/util/HashMap$Values", "map", "Ljava/util/HashMap;").await?;
        let iterator: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&map, "java/util/HashMap", "entryIterator", "()Ljava/util/Iterator;", ())
            .await?;
        while jvm
            .invoke_virtual::<_, bool>(&iterator, &iterator.class_definition().name(), "hasNext", "()Z", ())
            .await?
        {
            let entry: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&iterator, &iterator.class_definition().name(), "next", "()Ljava/lang/Object;", ())
                .await?;
            let entry_value: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&entry, &entry.class_definition().name(), "getValue", "()Ljava/lang/Object;", ())
                .await?;
            let equal = if value.is_null() {
                entry_value.is_null()
            } else if entry_value.is_null() {
                false
            } else {
                jvm.invoke_virtual(&value, "java/lang/Object", "equals", "(Ljava/lang/Object;)Z", (entry_value,))
                    .await?
            };
            if equal {
                let key: ClassInstanceRef<Object> = jvm
                    .invoke_virtual(&entry, &entry.class_definition().name(), "getKey", "()Ljava/lang/Object;", ())
                    .await?;
                let _: ClassInstanceRef<Object> = jvm
                    .invoke_virtual(&map, "java/util/HashMap", "remove", "(Ljava/lang/Object;)Ljava/lang/Object;", (key,))
                    .await?;
                return Ok(true);
            }
        }

        Ok(false)
    }

    #[method(public, final)]
    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.HashMap$Values::clear({this:?})");

        let map: ClassInstanceRef<HashMap> = jvm.get_field(&this, "java/util/HashMap$Values", "map", "Ljava/util/HashMap;").await?;

        jvm.invoke_virtual(&map, "java/util/HashMap", "clear", "()V", ()).await
    }

    #[method(descriptor = "()Ljava/util/Iterator;", public, final)]
    async fn iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.HashMap$Values::iterator({this:?})");

        let map: ClassInstanceRef<HashMap> = jvm.get_field(&this, "java/util/HashMap$Values", "map", "Ljava/util/HashMap;").await?;

        jvm.invoke_virtual(&map, "java/util/HashMap", "valueIterator", "()Ljava/util/Iterator;", ())
            .await
    }
}
