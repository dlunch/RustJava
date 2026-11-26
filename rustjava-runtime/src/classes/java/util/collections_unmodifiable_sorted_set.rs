use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{lang::Object, util::SortedSet},
};

// class java.util.Collections$UnmodifiableSortedSet
#[java_class(
    name = "java/util/Collections$UnmodifiableSortedSet",
    extends = "java/util/Collections$UnmodifiableSet",
    implements("java/util/SortedSet", "java/io/Serializable")
)]
pub struct CollectionsUnmodifiableSortedSet {
    #[field(private, final)]
    ss: ClassInstanceRef<SortedSet>,
}

#[java_methods]
impl CollectionsUnmodifiableSortedSet {
    #[method(constructor, descriptor = "(Ljava/util/SortedSet;)V")]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, set: ClassInstanceRef<Object>) -> Result<()> {
        let _: () = jvm
            .invoke_special(
                &this,
                "java/util/Collections$UnmodifiableSet",
                "<init>",
                "(Ljava/util/Set;)V",
                (set.clone(),),
            )
            .await?;
        jvm.put_field(
            &mut this,
            "java/util/Collections$UnmodifiableSortedSet",
            "ss",
            "Ljava/util/SortedSet;",
            set,
        )
        .await
    }

    #[method(descriptor = "()Ljava/util/Comparator;", public)]
    async fn comparator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let set: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableSortedSet", "ss", "Ljava/util/SortedSet;")
            .await?;
        jvm.invoke_virtual(&set, &set.class_definition().name(), "comparator", "()Ljava/util/Comparator;", ())
            .await
    }

    #[method(public)]
    async fn first(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let set: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableSortedSet", "ss", "Ljava/util/SortedSet;")
            .await?;
        jvm.invoke_virtual(&set, &set.class_definition().name(), "first", "()Ljava/lang/Object;", ())
            .await
    }

    #[method(public)]
    async fn last(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let set: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableSortedSet", "ss", "Ljava/util/SortedSet;")
            .await?;
        jvm.invoke_virtual(&set, &set.class_definition().name(), "last", "()Ljava/lang/Object;", ())
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
        let set: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableSortedSet", "ss", "Ljava/util/SortedSet;")
            .await?;
        let range: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &set,
                &set.class_definition().name(),
                "subSet",
                "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedSet;",
                (from, to),
            )
            .await?;
        Ok(jvm
            .new_class("java/util/Collections$UnmodifiableSortedSet", "(Ljava/util/SortedSet;)V", (range,))
            .await?
            .into())
    }

    #[method(descriptor = "(Ljava/lang/Object;)Ljava/util/SortedSet;", public)]
    async fn head_set(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        to: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        let set: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableSortedSet", "ss", "Ljava/util/SortedSet;")
            .await?;
        let range: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &set,
                &set.class_definition().name(),
                "headSet",
                "(Ljava/lang/Object;)Ljava/util/SortedSet;",
                (to,),
            )
            .await?;
        Ok(jvm
            .new_class("java/util/Collections$UnmodifiableSortedSet", "(Ljava/util/SortedSet;)V", (range,))
            .await?
            .into())
    }

    #[method(descriptor = "(Ljava/lang/Object;)Ljava/util/SortedSet;", public)]
    async fn tail_set(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        from: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        let set: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableSortedSet", "ss", "Ljava/util/SortedSet;")
            .await?;
        let range: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &set,
                &set.class_definition().name(),
                "tailSet",
                "(Ljava/lang/Object;)Ljava/util/SortedSet;",
                (from,),
            )
            .await?;
        Ok(jvm
            .new_class("java/util/Collections$UnmodifiableSortedSet", "(Ljava/util/SortedSet;)V", (range,))
            .await?
            .into())
    }
}
