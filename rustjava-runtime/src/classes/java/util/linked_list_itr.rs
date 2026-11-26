use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{lang::Object, util::LinkedList},
};

// class java.util.LinkedList$ListItr
#[java_class(name = "java/util/LinkedList$ListItr", implements("java/util/ListIterator"))]
pub struct LinkedListItr {
    #[field(private)]
    list: ClassInstanceRef<LinkedList>,
    #[field(private)]
    cursor: i32,
    #[field(private)]
    lastReturned: i32,
}

#[java_methods]
impl LinkedListItr {
    #[method(constructor, descriptor = "(Ljava/util/LinkedList;I)V")]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, list: ClassInstanceRef<Object>, index: i32) -> Result<()> {
        let size: i32 = jvm.invoke_virtual(&list, "java/util/LinkedList", "size", "()I", ()).await?;
        if index < 0 || index > size {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "list iterator index").await);
        }
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/util/LinkedList$ListItr", "list", "Ljava/util/LinkedList;", list)
            .await?;
        jvm.put_field(&mut this, "java/util/LinkedList$ListItr", "cursor", "I", index).await?;
        jvm.put_field(&mut this, "java/util/LinkedList$ListItr", "lastReturned", "I", -1).await
    }

    #[method(public)]
    async fn has_next(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let list: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/LinkedList$ListItr", "list", "Ljava/util/LinkedList;")
            .await?;
        let cursor: i32 = jvm.get_field(&this, "java/util/LinkedList$ListItr", "cursor", "I").await?;
        Ok(cursor < jvm.invoke_virtual::<_, i32>(&list, "java/util/LinkedList", "size", "()I", ()).await?)
    }

    #[method(public)]
    async fn next(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let list: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/LinkedList$ListItr", "list", "Ljava/util/LinkedList;")
            .await?;
        let cursor: i32 = jvm.get_field(&this, "java/util/LinkedList$ListItr", "cursor", "I").await?;
        if cursor >= jvm.invoke_virtual::<_, i32>(&list, "java/util/LinkedList", "size", "()I", ()).await? {
            return Err(jvm.exception("java/util/NoSuchElementException", "LinkedList iterator exhausted").await);
        }
        let value = jvm
            .invoke_virtual(&list, "java/util/LinkedList", "get", "(I)Ljava/lang/Object;", (cursor,))
            .await?;
        jvm.put_field(&mut this, "java/util/LinkedList$ListItr", "cursor", "I", cursor + 1)
            .await?;
        jvm.put_field(&mut this, "java/util/LinkedList$ListItr", "lastReturned", "I", cursor)
            .await?;
        Ok(value)
    }

    #[method(public)]
    async fn has_previous(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        Ok(jvm.get_field::<i32>(&this, "java/util/LinkedList$ListItr", "cursor", "I").await? > 0)
    }

    #[method(public)]
    async fn previous(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let cursor: i32 = jvm.get_field(&this, "java/util/LinkedList$ListItr", "cursor", "I").await?;
        if cursor <= 0 {
            return Err(jvm.exception("java/util/NoSuchElementException", "LinkedList iterator exhausted").await);
        }
        let index = cursor - 1;
        let list: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/LinkedList$ListItr", "list", "Ljava/util/LinkedList;")
            .await?;
        let value = jvm
            .invoke_virtual(&list, "java/util/LinkedList", "get", "(I)Ljava/lang/Object;", (index,))
            .await?;
        jvm.put_field(&mut this, "java/util/LinkedList$ListItr", "cursor", "I", index).await?;
        jvm.put_field(&mut this, "java/util/LinkedList$ListItr", "lastReturned", "I", index)
            .await?;
        Ok(value)
    }

    #[method(public)]
    async fn next_index(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "java/util/LinkedList$ListItr", "cursor", "I").await
    }

    #[method(public)]
    async fn previous_index(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(jvm.get_field::<i32>(&this, "java/util/LinkedList$ListItr", "cursor", "I").await? - 1)
    }

    #[method(public)]
    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let last_returned: i32 = jvm.get_field(&this, "java/util/LinkedList$ListItr", "lastReturned", "I").await?;
        if last_returned < 0 {
            return Err(jvm.exception("java/lang/IllegalStateException", "iterator state").await);
        }
        let list: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/LinkedList$ListItr", "list", "Ljava/util/LinkedList;")
            .await?;
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&list, "java/util/LinkedList", "remove", "(I)Ljava/lang/Object;", (last_returned,))
            .await?;
        let cursor: i32 = jvm.get_field(&this, "java/util/LinkedList$ListItr", "cursor", "I").await?;
        if last_returned < cursor {
            jvm.put_field(&mut this, "java/util/LinkedList$ListItr", "cursor", "I", cursor - 1)
                .await?;
        }
        jvm.put_field(&mut this, "java/util/LinkedList$ListItr", "lastReturned", "I", -1).await
    }

    #[method(public)]
    async fn set(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<()> {
        let last_returned: i32 = jvm.get_field(&this, "java/util/LinkedList$ListItr", "lastReturned", "I").await?;
        if last_returned < 0 {
            return Err(jvm.exception("java/lang/IllegalStateException", "iterator state").await);
        }
        let list: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/LinkedList$ListItr", "list", "Ljava/util/LinkedList;")
            .await?;
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &list,
                "java/util/LinkedList",
                "set",
                "(ILjava/lang/Object;)Ljava/lang/Object;",
                (last_returned, element),
            )
            .await?;
        Ok(())
    }

    #[method(public)]
    async fn add(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<()> {
        let cursor: i32 = jvm.get_field(&this, "java/util/LinkedList$ListItr", "cursor", "I").await?;
        let list: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/LinkedList$ListItr", "list", "Ljava/util/LinkedList;")
            .await?;
        let _: () = jvm
            .invoke_virtual(&list, "java/util/LinkedList", "add", "(ILjava/lang/Object;)V", (cursor, element))
            .await?;
        jvm.put_field(&mut this, "java/util/LinkedList$ListItr", "cursor", "I", cursor + 1)
            .await?;
        jvm.put_field(&mut this, "java/util/LinkedList$ListItr", "lastReturned", "I", -1).await
    }
}
