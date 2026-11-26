use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{lang::Object, util::SortedMap},
};

// class java.util.Collections$UnmodifiableSortedMap
#[java_class(
    name = "java/util/Collections$UnmodifiableSortedMap",
    extends = "java/util/Collections$UnmodifiableMap",
    implements("java/util/SortedMap", "java/io/Serializable")
)]
pub struct CollectionsUnmodifiableSortedMap {
    #[field(private, final)]
    sm: ClassInstanceRef<SortedMap>,
}

#[java_methods]
impl CollectionsUnmodifiableSortedMap {
    #[method(constructor, descriptor = "(Ljava/util/SortedMap;)V")]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, map: ClassInstanceRef<Object>) -> Result<()> {
        let _: () = jvm
            .invoke_special(
                &this,
                "java/util/Collections$UnmodifiableMap",
                "<init>",
                "(Ljava/util/Map;)V",
                (map.clone(),),
            )
            .await?;
        jvm.put_field(
            &mut this,
            "java/util/Collections$UnmodifiableSortedMap",
            "sm",
            "Ljava/util/SortedMap;",
            map,
        )
        .await
    }

    #[method(descriptor = "()Ljava/util/Comparator;", public)]
    async fn comparator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableSortedMap", "sm", "Ljava/util/SortedMap;")
            .await?;
        jvm.invoke_virtual(&map, &map.class_definition().name(), "comparator", "()Ljava/util/Comparator;", ())
            .await
    }

    #[method(public)]
    async fn first_key(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableSortedMap", "sm", "Ljava/util/SortedMap;")
            .await?;
        jvm.invoke_virtual(&map, &map.class_definition().name(), "firstKey", "()Ljava/lang/Object;", ())
            .await
    }

    #[method(public)]
    async fn last_key(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableSortedMap", "sm", "Ljava/util/SortedMap;")
            .await?;
        jvm.invoke_virtual(&map, &map.class_definition().name(), "lastKey", "()Ljava/lang/Object;", ())
            .await
    }

    #[method(descriptor = "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;", public)]
    async fn sub_map(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        from: ClassInstanceRef<Object>,
        to: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableSortedMap", "sm", "Ljava/util/SortedMap;")
            .await?;
        let range: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &map,
                &map.class_definition().name(),
                "subMap",
                "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;",
                (from, to),
            )
            .await?;
        Ok(jvm
            .new_class("java/util/Collections$UnmodifiableSortedMap", "(Ljava/util/SortedMap;)V", (range,))
            .await?
            .into())
    }

    #[method(descriptor = "(Ljava/lang/Object;)Ljava/util/SortedMap;", public)]
    async fn head_map(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        to: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableSortedMap", "sm", "Ljava/util/SortedMap;")
            .await?;
        let range: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &map,
                &map.class_definition().name(),
                "headMap",
                "(Ljava/lang/Object;)Ljava/util/SortedMap;",
                (to,),
            )
            .await?;
        Ok(jvm
            .new_class("java/util/Collections$UnmodifiableSortedMap", "(Ljava/util/SortedMap;)V", (range,))
            .await?
            .into())
    }

    #[method(descriptor = "(Ljava/lang/Object;)Ljava/util/SortedMap;", public)]
    async fn tail_map(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        from: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        let map: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableSortedMap", "sm", "Ljava/util/SortedMap;")
            .await?;
        let range: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &map,
                &map.class_definition().name(),
                "tailMap",
                "(Ljava/lang/Object;)Ljava/util/SortedMap;",
                (from,),
            )
            .await?;
        Ok(jvm
            .new_class("java/util/Collections$UnmodifiableSortedMap", "(Ljava/util/SortedMap;)V", (range,))
            .await?
            .into())
    }
}
