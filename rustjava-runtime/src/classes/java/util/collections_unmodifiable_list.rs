use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{lang::Object, util::List},
};

// class java.util.Collections$UnmodifiableList
#[java_class(
    name = "java/util/Collections$UnmodifiableList",
    extends = "java/util/Collections$UnmodifiableCollection",
    implements("java/util/List")
)]
pub struct CollectionsUnmodifiableList {
    #[field(final)]
    list: ClassInstanceRef<List>,
}

#[java_methods]
impl CollectionsUnmodifiableList {
    #[method(constructor, descriptor = "(Ljava/util/List;)V")]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, list: ClassInstanceRef<Object>) -> Result<()> {
        let _: () = jvm
            .invoke_special(
                &this,
                "java/util/Collections$UnmodifiableCollection",
                "<init>",
                "(Ljava/util/Collection;)V",
                (list.clone(),),
            )
            .await?;
        jvm.put_field(&mut this, "java/util/Collections$UnmodifiableList", "list", "Ljava/util/List;", list)
            .await
    }

    #[method(public)]
    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if !other.is_null() && this.identity() == other.identity() {
            return Ok(true);
        }
        let list: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableList", "list", "Ljava/util/List;")
            .await?;
        jvm.invoke_virtual(&list, "java/lang/Object", "equals", "(Ljava/lang/Object;)Z", (other,))
            .await
    }

    #[method(public)]
    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let list: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableList", "list", "Ljava/util/List;")
            .await?;
        jvm.invoke_virtual(&list, "java/lang/Object", "hashCode", "()I", ()).await
    }

    #[method(public)]
    async fn get(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Object>> {
        let list: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableList", "list", "Ljava/util/List;")
            .await?;
        jvm.invoke_virtual(&list, &list.class_definition().name(), "get", "(I)Ljava/lang/Object;", (index,))
            .await
    }

    #[method(public)]
    async fn index_of(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<i32> {
        let list: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableList", "list", "Ljava/util/List;")
            .await?;
        jvm.invoke_virtual(&list, &list.class_definition().name(), "indexOf", "(Ljava/lang/Object;)I", (element,))
            .await
    }

    #[method(public)]
    async fn last_index_of(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<i32> {
        let list: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableList", "list", "Ljava/util/List;")
            .await?;
        jvm.invoke_virtual(&list, &list.class_definition().name(), "lastIndexOf", "(Ljava/lang/Object;)I", (element,))
            .await
    }

    #[method(descriptor = "()Ljava/util/ListIterator;", public)]
    async fn list_iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let list: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableList", "list", "Ljava/util/List;")
            .await?;
        let iterator: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&list, &list.class_definition().name(), "listIterator", "()Ljava/util/ListIterator;", ())
            .await?;
        Ok(jvm
            .new_class("java/util/Collections$UnmodifiableList$1", "(Ljava/util/ListIterator;)V", (iterator,))
            .await?
            .into())
    }

    #[method(name = "listIterator", descriptor = "(I)Ljava/util/ListIterator;", public)]
    async fn list_iterator_at(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Object>> {
        let list: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableList", "list", "Ljava/util/List;")
            .await?;
        let iterator: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &list,
                &list.class_definition().name(),
                "listIterator",
                "(I)Ljava/util/ListIterator;",
                (index,),
            )
            .await?;
        Ok(jvm
            .new_class("java/util/Collections$UnmodifiableList$1", "(Ljava/util/ListIterator;)V", (iterator,))
            .await?
            .into())
    }

    #[method(descriptor = "(II)Ljava/util/List;", public)]
    async fn sub_list(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, from: i32, to: i32) -> Result<ClassInstanceRef<Object>> {
        let list: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableList", "list", "Ljava/util/List;")
            .await?;
        let sub_list: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&list, &list.class_definition().name(), "subList", "(II)Ljava/util/List;", (from, to))
            .await?;
        Ok(jvm
            .new_class("java/util/Collections$UnmodifiableList", "(Ljava/util/List;)V", (sub_list,))
            .await?
            .into())
    }

    #[method(public)]
    async fn set(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _: ClassInstanceRef<Self>,
        _: i32,
        _: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "unmodifiable list").await)
    }

    #[method(public)]
    async fn add(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: i32, _: ClassInstanceRef<Object>) -> Result<()> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "unmodifiable list").await)
    }

    #[method(descriptor = "(ILjava/util/Collection;)Z", public)]
    async fn add_all(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: i32, _: ClassInstanceRef<Object>) -> Result<bool> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "unmodifiable list").await)
    }

    #[method(public)]
    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: i32) -> Result<ClassInstanceRef<Object>> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "unmodifiable list").await)
    }
}
