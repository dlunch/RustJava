use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{lang::Object, util::ListIterator},
};

// class java.util.Collections$UnmodifiableList$1
#[java_class(
    name = "java/util/Collections$UnmodifiableList$1",
    extends = "java/lang/Object",
    implements("java/util/ListIterator")
)]
pub struct CollectionsUnmodifiableListIterator {
    #[field(private, final)]
    i: ClassInstanceRef<ListIterator>,
}

#[java_methods]
impl CollectionsUnmodifiableListIterator {
    #[method(constructor, descriptor = "(Ljava/util/ListIterator;)V")]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, iterator: ClassInstanceRef<Object>) -> Result<()> {
        if iterator.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "iterator").await);
        }
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(
            &mut this,
            "java/util/Collections$UnmodifiableList$1",
            "i",
            "Ljava/util/ListIterator;",
            iterator,
        )
        .await
    }

    #[method(public)]
    async fn has_next(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let iterator: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableList$1", "i", "Ljava/util/ListIterator;")
            .await?;
        jvm.invoke_virtual(&iterator, &iterator.class_definition().name(), "hasNext", "()Z", ())
            .await
    }

    #[method(public)]
    async fn next(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let iterator: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableList$1", "i", "Ljava/util/ListIterator;")
            .await?;
        jvm.invoke_virtual(&iterator, &iterator.class_definition().name(), "next", "()Ljava/lang/Object;", ())
            .await
    }

    #[method(public)]
    async fn has_previous(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let iterator: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableList$1", "i", "Ljava/util/ListIterator;")
            .await?;
        jvm.invoke_virtual(&iterator, &iterator.class_definition().name(), "hasPrevious", "()Z", ())
            .await
    }

    #[method(public)]
    async fn previous(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let iterator: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableList$1", "i", "Ljava/util/ListIterator;")
            .await?;
        jvm.invoke_virtual(&iterator, &iterator.class_definition().name(), "previous", "()Ljava/lang/Object;", ())
            .await
    }

    #[method(public)]
    async fn next_index(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let iterator: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableList$1", "i", "Ljava/util/ListIterator;")
            .await?;
        jvm.invoke_virtual(&iterator, &iterator.class_definition().name(), "nextIndex", "()I", ())
            .await
    }

    #[method(public)]
    async fn previous_index(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let iterator: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableList$1", "i", "Ljava/util/ListIterator;")
            .await?;
        jvm.invoke_virtual(&iterator, &iterator.class_definition().name(), "previousIndex", "()I", ())
            .await
    }

    #[method(public)]
    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<()> {
        Err(jvm
            .exception("java/lang/UnsupportedOperationException", "unmodifiable list iterator")
            .await)
    }

    #[method(public)]
    async fn set(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<()> {
        Err(jvm
            .exception("java/lang/UnsupportedOperationException", "unmodifiable list iterator")
            .await)
    }

    #[method(public)]
    async fn add(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<()> {
        Err(jvm
            .exception("java/lang/UnsupportedOperationException", "unmodifiable list iterator")
            .await)
    }
}
