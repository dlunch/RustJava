use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{lang::Object, util::List},
};

// class java.util.ArrayList$Itr
#[java_class(name = "java/util/ArrayList$Itr", extends = "java/lang/Object", implements("java/util/Iterator"))]
pub struct ArrayListItr {
    #[field(private)]
    list: ClassInstanceRef<List>,
    cursor: i32,
    #[field(private)]
    lastReturned: i32,
}

#[java_methods]
impl ArrayListItr {
    #[method(constructor, descriptor = "(Ljava/util/List;I)V")]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, list: ClassInstanceRef<Object>, index: i32) -> Result<()> {
        tracing::debug!("java.util.ArrayList$Itr::<init>({this:?}, {list:?}, {index:?})");

        let size: i32 = jvm.invoke_virtual(&list, &list.class_definition().name(), "size", "()I", ()).await?;
        if index < 0 || index > size {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "list iterator index").await);
        }
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/util/ArrayList$Itr", "list", "Ljava/util/List;", list)
            .await?;
        jvm.put_field(&mut this, "java/util/ArrayList$Itr", "cursor", "I", index).await?;
        jvm.put_field(&mut this, "java/util/ArrayList$Itr", "lastReturned", "I", -1).await
    }

    #[method(public)]
    async fn has_next(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let list: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/ArrayList$Itr", "list", "Ljava/util/List;").await?;
        let cursor: i32 = jvm.get_field(&this, "java/util/ArrayList$Itr", "cursor", "I").await?;
        let size: i32 = jvm.invoke_virtual(&list, &list.class_definition().name(), "size", "()I", ()).await?;
        Ok(cursor < size)
    }

    #[method(public)]
    async fn next(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let list: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/ArrayList$Itr", "list", "Ljava/util/List;").await?;
        let cursor: i32 = jvm.get_field(&this, "java/util/ArrayList$Itr", "cursor", "I").await?;
        let size: i32 = jvm.invoke_virtual(&list, &list.class_definition().name(), "size", "()I", ()).await?;
        if cursor >= size {
            return Err(jvm.exception("java/util/NoSuchElementException", "ArrayList iterator exhausted").await);
        }
        let element = jvm
            .invoke_virtual(&list, &list.class_definition().name(), "get", "(I)Ljava/lang/Object;", (cursor,))
            .await?;
        jvm.put_field(&mut this, "java/util/ArrayList$Itr", "cursor", "I", cursor + 1).await?;
        jvm.put_field(&mut this, "java/util/ArrayList$Itr", "lastReturned", "I", cursor).await?;
        Ok(element)
    }

    #[method(public)]
    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let last_returned: i32 = jvm.get_field(&this, "java/util/ArrayList$Itr", "lastReturned", "I").await?;
        if last_returned < 0 {
            return Err(jvm.exception("java/lang/IllegalStateException", "iterator state").await);
        }
        let list: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/ArrayList$Itr", "list", "Ljava/util/List;").await?;
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &list,
                &list.class_definition().name(),
                "remove",
                "(I)Ljava/lang/Object;",
                (last_returned,),
            )
            .await?;
        let cursor: i32 = jvm.get_field(&this, "java/util/ArrayList$Itr", "cursor", "I").await?;
        if last_returned < cursor {
            jvm.put_field(&mut this, "java/util/ArrayList$Itr", "cursor", "I", cursor - 1).await?;
        }
        jvm.put_field(&mut this, "java/util/ArrayList$Itr", "lastReturned", "I", -1).await
    }
}

// class java.util.ArrayList$ListItr
#[java_class(
    name = "java/util/ArrayList$ListItr",
    extends = "java/util/ArrayList$Itr",
    implements("java/util/ListIterator")
)]
pub struct ArrayListListItr;

#[java_methods]
impl ArrayListListItr {
    #[method(constructor, descriptor = "(Ljava/util/List;I)V")]
    async fn init(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, list: ClassInstanceRef<Object>, index: i32) -> Result<()> {
        ArrayListItr::init(jvm, context, ClassInstanceRef::new(this.instance), list, index).await
    }

    #[method(public)]
    async fn has_previous(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        Ok(jvm.get_field::<i32>(&this, "java/util/ArrayList$Itr", "cursor", "I").await? > 0)
    }

    #[method(public)]
    async fn previous(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let cursor: i32 = jvm.get_field(&this, "java/util/ArrayList$Itr", "cursor", "I").await?;
        if cursor <= 0 {
            return Err(jvm.exception("java/util/NoSuchElementException", "ArrayList iterator exhausted").await);
        }
        let index = cursor - 1;
        let list: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/ArrayList$Itr", "list", "Ljava/util/List;").await?;
        let element = jvm
            .invoke_virtual(&list, &list.class_definition().name(), "get", "(I)Ljava/lang/Object;", (index,))
            .await?;
        jvm.put_field(&mut this, "java/util/ArrayList$Itr", "cursor", "I", index).await?;
        jvm.put_field(&mut this, "java/util/ArrayList$Itr", "lastReturned", "I", index).await?;
        Ok(element)
    }

    #[method(public)]
    async fn next_index(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "java/util/ArrayList$Itr", "cursor", "I").await
    }

    #[method(public)]
    async fn previous_index(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(jvm.get_field::<i32>(&this, "java/util/ArrayList$Itr", "cursor", "I").await? - 1)
    }

    #[method(public)]
    async fn set(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<()> {
        let last_returned: i32 = jvm.get_field(&this, "java/util/ArrayList$Itr", "lastReturned", "I").await?;
        if last_returned < 0 {
            return Err(jvm.exception("java/lang/IllegalStateException", "iterator state").await);
        }
        let list: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/ArrayList$Itr", "list", "Ljava/util/List;").await?;
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &list,
                &list.class_definition().name(),
                "set",
                "(ILjava/lang/Object;)Ljava/lang/Object;",
                (last_returned, element),
            )
            .await?;
        Ok(())
    }

    #[method(public)]
    async fn add(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<()> {
        let cursor: i32 = jvm.get_field(&this, "java/util/ArrayList$Itr", "cursor", "I").await?;
        let list: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/ArrayList$Itr", "list", "Ljava/util/List;").await?;
        let _: () = jvm
            .invoke_virtual(&list, &list.class_definition().name(), "add", "(ILjava/lang/Object;)V", (cursor, element))
            .await?;
        jvm.put_field(&mut this, "java/util/ArrayList$Itr", "cursor", "I", cursor + 1).await?;
        jvm.put_field(&mut this, "java/util/ArrayList$Itr", "lastReturned", "I", -1).await
    }
}
