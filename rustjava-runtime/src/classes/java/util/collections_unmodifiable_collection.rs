use jvm::{Array, ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{lang::Object, util::Collection},
};

// class java.util.Collections$UnmodifiableCollection
#[java_class(
    name = "java/util/Collections$UnmodifiableCollection",
    implements("java/util/Collection", "java/io/Serializable")
)]
pub struct CollectionsUnmodifiableCollection {
    #[field(final)]
    c: ClassInstanceRef<Collection>,
}

#[java_methods]
impl CollectionsUnmodifiableCollection {
    #[method(constructor, descriptor = "(Ljava/util/Collection;)V")]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, collection: ClassInstanceRef<Object>) -> Result<()> {
        if collection.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "collection").await);
        }
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(
            &mut this,
            "java/util/Collections$UnmodifiableCollection",
            "c",
            "Ljava/util/Collection;",
            collection,
        )
        .await
    }

    #[method(public)]
    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let collection: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableCollection", "c", "Ljava/util/Collection;")
            .await?;
        jvm.invoke_virtual(&collection, &collection.class_definition().name(), "size", "()I", ())
            .await
    }

    #[method(public)]
    async fn is_empty(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let collection: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableCollection", "c", "Ljava/util/Collection;")
            .await?;
        jvm.invoke_virtual(&collection, &collection.class_definition().name(), "isEmpty", "()Z", ())
            .await
    }

    #[method(public)]
    async fn contains(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        let collection: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableCollection", "c", "Ljava/util/Collection;")
            .await?;
        jvm.invoke_virtual(
            &collection,
            &collection.class_definition().name(),
            "contains",
            "(Ljava/lang/Object;)Z",
            (element,),
        )
        .await
    }

    #[method(descriptor = "()Ljava/util/Iterator;", public)]
    async fn iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let collection: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableCollection", "c", "Ljava/util/Collection;")
            .await?;
        let iterator: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &collection,
                &collection.class_definition().name(),
                "iterator",
                "()Ljava/util/Iterator;",
                (),
            )
            .await?;
        Ok(jvm
            .new_class("java/util/Collections$UnmodifiableCollection$1", "(Ljava/util/Iterator;)V", (iterator,))
            .await?
            .into())
    }

    #[method(public)]
    async fn to_array(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Array<Object>>> {
        let collection: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableCollection", "c", "Ljava/util/Collection;")
            .await?;
        jvm.invoke_virtual(&collection, &collection.class_definition().name(), "toArray", "()[Ljava/lang/Object;", ())
            .await
    }

    #[method(name = "toArray", public)]
    async fn to_typed_array(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        array: ClassInstanceRef<Array<Object>>,
    ) -> Result<ClassInstanceRef<Array<Object>>> {
        let collection: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableCollection", "c", "Ljava/util/Collection;")
            .await?;
        jvm.invoke_virtual(
            &collection,
            &collection.class_definition().name(),
            "toArray",
            "([Ljava/lang/Object;)[Ljava/lang/Object;",
            (array,),
        )
        .await
    }

    #[method(descriptor = "(Ljava/util/Collection;)Z", public)]
    async fn contains_all(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, collection: ClassInstanceRef<Object>) -> Result<bool> {
        let backing: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableCollection", "c", "Ljava/util/Collection;")
            .await?;
        jvm.invoke_virtual(
            &backing,
            &backing.class_definition().name(),
            "containsAll",
            "(Ljava/util/Collection;)Z",
            (collection,),
        )
        .await
    }

    #[method(descriptor = "()Ljava/lang/String;", public)]
    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let collection: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableCollection", "c", "Ljava/util/Collection;")
            .await?;
        jvm.invoke_virtual(&collection, "java/lang/Object", "toString", "()Ljava/lang/String;", ())
            .await
    }

    #[method(public)]
    async fn add(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<bool> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "unmodifiable collection").await)
    }

    #[method(public)]
    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<bool> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "unmodifiable collection").await)
    }

    #[method(descriptor = "(Ljava/util/Collection;)Z", public)]
    async fn add_all(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<bool> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "unmodifiable collection").await)
    }

    #[method(descriptor = "(Ljava/util/Collection;)Z", public)]
    async fn remove_all(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<bool> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "unmodifiable collection").await)
    }

    #[method(descriptor = "(Ljava/util/Collection;)Z", public)]
    async fn retain_all(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<bool> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "unmodifiable collection").await)
    }

    #[method(public)]
    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<()> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "unmodifiable collection").await)
    }
}
