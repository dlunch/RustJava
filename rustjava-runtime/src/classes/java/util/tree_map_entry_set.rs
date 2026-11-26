use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{lang::Object, util::SortedMap},
};

// class java.util.TreeMap$EntrySet
#[java_class(name = "java/util/TreeMap$EntrySet", extends = "java/util/AbstractSet")]
pub struct TreeMapEntrySet {
    #[field(private)]
    map: ClassInstanceRef<SortedMap>,
}

#[java_methods]
impl TreeMapEntrySet {
    #[method(constructor, descriptor = "(Ljava/util/SortedMap;)V")]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, map: ClassInstanceRef<Object>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/util/AbstractSet", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/util/TreeMap$EntrySet", "map", "Ljava/util/SortedMap;", map)
            .await
    }

    #[method(public)]
    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/TreeMap$EntrySet", "map", "Ljava/util/SortedMap;").await?;
        jvm.invoke_virtual(&map, &map.class_definition().name(), "size", "()I", ()).await
    }

    #[method(public)]
    async fn contains(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, candidate: ClassInstanceRef<Object>) -> Result<bool> {
        if candidate.is_null() || !jvm.is_instance(candidate.as_ref(), "java/util/Map$Entry") {
            return Ok(false);
        }
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/TreeMap$EntrySet", "map", "Ljava/util/SortedMap;").await?;
        let value: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&candidate, &candidate.class_definition().name(), "getValue", "()Ljava/lang/Object;", ())
            .await?;
        let key: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&candidate, &candidate.class_definition().name(), "getKey", "()Ljava/lang/Object;", ())
            .await?;
        if !jvm
            .invoke_virtual::<_, bool>(
                &map,
                &map.class_definition().name(),
                "containsKey",
                "(Ljava/lang/Object;)Z",
                (key.clone(),),
            )
            .await?
        {
            return Ok(false);
        }
        let stored: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &map,
                &map.class_definition().name(),
                "get",
                "(Ljava/lang/Object;)Ljava/lang/Object;",
                (key,),
            )
            .await?;
        if stored.is_null() {
            Ok(value.is_null())
        } else {
            jvm.invoke_virtual(&stored, "java/lang/Object", "equals", "(Ljava/lang/Object;)Z", (value,))
                .await
        }
    }

    #[method(public)]
    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, candidate: ClassInstanceRef<Object>) -> Result<bool> {
        if candidate.is_null() || !jvm.is_instance(candidate.as_ref(), "java/util/Map$Entry") {
            return Ok(false);
        }
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/TreeMap$EntrySet", "map", "Ljava/util/SortedMap;").await?;
        let value: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&candidate, &candidate.class_definition().name(), "getValue", "()Ljava/lang/Object;", ())
            .await?;
        let key: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&candidate, &candidate.class_definition().name(), "getKey", "()Ljava/lang/Object;", ())
            .await?;
        if !jvm
            .invoke_virtual::<_, bool>(
                &map,
                &map.class_definition().name(),
                "containsKey",
                "(Ljava/lang/Object;)Z",
                (key.clone(),),
            )
            .await?
        {
            return Ok(false);
        }
        let stored: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &map,
                &map.class_definition().name(),
                "get",
                "(Ljava/lang/Object;)Ljava/lang/Object;",
                (key.clone(),),
            )
            .await?;
        let equal = if stored.is_null() {
            value.is_null()
        } else {
            jvm.invoke_virtual::<_, bool>(&stored, "java/lang/Object", "equals", "(Ljava/lang/Object;)Z", (value,))
                .await?
        };
        if !equal {
            return Ok(false);
        }
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &map,
                &map.class_definition().name(),
                "remove",
                "(Ljava/lang/Object;)Ljava/lang/Object;",
                (key,),
            )
            .await?;
        Ok(true)
    }

    #[method(public)]
    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/TreeMap$EntrySet", "map", "Ljava/util/SortedMap;").await?;
        jvm.invoke_virtual(&map, &map.class_definition().name(), "clear", "()V", ()).await
    }

    #[method(descriptor = "()Ljava/util/Iterator;", public)]
    async fn iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/TreeMap$EntrySet", "map", "Ljava/util/SortedMap;").await?;
        jvm.invoke_virtual(&map, &map.class_definition().name(), "entryIterator", "()Ljava/util/Iterator;", ())
            .await
    }
}
