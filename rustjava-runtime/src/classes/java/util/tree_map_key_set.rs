use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{lang::Object, util::SortedMap},
};

// class java.util.TreeMap$KeySet
#[java_class(name = "java/util/TreeMap$KeySet", extends = "java/util/AbstractSet", final)]
pub struct TreeMapKeySet {
    #[field(private)]
    map: ClassInstanceRef<SortedMap>,
}

#[java_methods]
impl TreeMapKeySet {
    #[method(constructor, descriptor = "(Ljava/util/SortedMap;)V")]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, map: ClassInstanceRef<Object>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/util/AbstractSet", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/util/TreeMap$KeySet", "map", "Ljava/util/SortedMap;", map)
            .await
    }

    #[method(public)]
    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/TreeMap$KeySet", "map", "Ljava/util/SortedMap;").await?;
        jvm.invoke_virtual(&map, &map.class_definition().name(), "size", "()I", ()).await
    }

    #[method(public)]
    async fn contains(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key: ClassInstanceRef<Object>) -> Result<bool> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/TreeMap$KeySet", "map", "Ljava/util/SortedMap;").await?;
        jvm.invoke_virtual(&map, &map.class_definition().name(), "containsKey", "(Ljava/lang/Object;)Z", (key,))
            .await
    }

    #[method(public)]
    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key: ClassInstanceRef<Object>) -> Result<bool> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/TreeMap$KeySet", "map", "Ljava/util/SortedMap;").await?;
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
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/TreeMap$KeySet", "map", "Ljava/util/SortedMap;").await?;
        jvm.invoke_virtual(&map, &map.class_definition().name(), "clear", "()V", ()).await
    }

    #[method(descriptor = "()Ljava/util/Iterator;", public)]
    async fn iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/TreeMap$KeySet", "map", "Ljava/util/SortedMap;").await?;
        jvm.invoke_virtual(&map, &map.class_definition().name(), "keyIterator", "()Ljava/util/Iterator;", ())
            .await
    }
}
