use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{lang::Object, util::SortedMap},
};

// class java.util.TreeMap$Values
#[java_class(name = "java/util/TreeMap$Values", extends = "java/util/AbstractCollection")]
pub struct TreeMapValues {
    #[field(private)]
    map: ClassInstanceRef<SortedMap>,
}

#[java_methods]
impl TreeMapValues {
    #[method(constructor, descriptor = "(Ljava/util/SortedMap;)V")]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, map: ClassInstanceRef<Object>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/util/AbstractCollection", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/util/TreeMap$Values", "map", "Ljava/util/SortedMap;", map)
            .await
    }

    #[method(public)]
    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/TreeMap$Values", "map", "Ljava/util/SortedMap;").await?;
        jvm.invoke_virtual(&map, &map.class_definition().name(), "size", "()I", ()).await
    }

    #[method(public)]
    async fn contains(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Object>) -> Result<bool> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/TreeMap$Values", "map", "Ljava/util/SortedMap;").await?;
        jvm.invoke_virtual(&map, &map.class_definition().name(), "containsValue", "(Ljava/lang/Object;)Z", (value,))
            .await
    }

    #[method(public)]
    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Object>) -> Result<bool> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/TreeMap$Values", "map", "Ljava/util/SortedMap;").await?;
        let iterator: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&map, &map.class_definition().name(), "valueIterator", "()Ljava/util/Iterator;", ())
            .await?;
        while jvm
            .invoke_virtual::<_, bool>(&iterator, &iterator.class_definition().name(), "hasNext", "()Z", ())
            .await?
        {
            let current: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&iterator, &iterator.class_definition().name(), "next", "()Ljava/lang/Object;", ())
                .await?;
            let equal = if current.is_null() {
                value.is_null()
            } else {
                jvm.invoke_virtual(&current, "java/lang/Object", "equals", "(Ljava/lang/Object;)Z", (value.clone(),))
                    .await?
            };
            if equal {
                let _: () = jvm
                    .invoke_virtual(&iterator, &iterator.class_definition().name(), "remove", "()V", ())
                    .await?;
                return Ok(true);
            }
        }
        Ok(false)
    }

    #[method(public)]
    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/TreeMap$Values", "map", "Ljava/util/SortedMap;").await?;
        jvm.invoke_virtual(&map, &map.class_definition().name(), "clear", "()V", ()).await
    }

    #[method(descriptor = "()Ljava/util/Iterator;", public)]
    async fn iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/TreeMap$Values", "map", "Ljava/util/SortedMap;").await?;
        jvm.invoke_virtual(&map, &map.class_definition().name(), "valueIterator", "()Ljava/util/Iterator;", ())
            .await
    }
}
