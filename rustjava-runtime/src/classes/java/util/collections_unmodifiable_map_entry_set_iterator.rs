use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{lang::Object, util::Iterator},
};

// class java.util.Collections$UnmodifiableMap$UnmodifiableEntrySet$1
#[java_class(name = "java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$1", implements("java/util/Iterator"))]
pub struct CollectionsUnmodifiableMapEntrySetIterator {
    #[field(private, final)]
    i: ClassInstanceRef<Iterator>,
}

#[java_methods]
impl CollectionsUnmodifiableMapEntrySetIterator {
    #[method(constructor, descriptor = "(Ljava/util/Iterator;)V")]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, iterator: ClassInstanceRef<Object>) -> Result<()> {
        if iterator.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "iterator").await);
        }
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(
            &mut this,
            "java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$1",
            "i",
            "Ljava/util/Iterator;",
            iterator,
        )
        .await
    }

    #[method(public)]
    async fn has_next(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let iterator: ClassInstanceRef<Object> = jvm
            .get_field(
                &this,
                "java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$1",
                "i",
                "Ljava/util/Iterator;",
            )
            .await?;
        jvm.invoke_virtual(&iterator, &iterator.class_definition().name(), "hasNext", "()Z", ())
            .await
    }

    #[method(public)]
    async fn next(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let iterator: ClassInstanceRef<Object> = jvm
            .get_field(
                &this,
                "java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$1",
                "i",
                "Ljava/util/Iterator;",
            )
            .await?;
        let entry: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&iterator, &iterator.class_definition().name(), "next", "()Ljava/lang/Object;", ())
            .await?;
        Ok(jvm
            .new_class(
                "java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$UnmodifiableEntry",
                "(Ljava/util/Map$Entry;)V",
                (entry,),
            )
            .await?
            .into())
    }

    #[method(public)]
    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<()> {
        Err(jvm
            .exception("java/lang/UnsupportedOperationException", "unmodifiable entry iterator")
            .await)
    }
}
