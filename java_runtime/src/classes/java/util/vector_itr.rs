use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::MethodAccessFlags;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

// class java.util.Vector$Itr
pub struct VectorItr;

impl VectorItr {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Vector$Itr",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/util/Iterator"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/util/Vector;I)V", Self::init, Default::default()),
                JavaMethodProto::new("hasNext", "()Z", Self::has_next, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("next", "()Ljava/lang/Object;", Self::next, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("remove", "()V", Self::remove, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("list", "Ljava/util/Vector;", Default::default()),
                JavaFieldProto::new("cursor", "I", Default::default()),
                JavaFieldProto::new("lastReturned", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    pub fn list_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Vector$ListItr",
            parent_class: Some("java/util/Vector$Itr"),
            interfaces: vec!["java/util/ListIterator"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/util/Vector;I)V", Self::init, Default::default()),
                JavaMethodProto::new("hasPrevious", "()Z", Self::has_previous, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("previous", "()Ljava/lang/Object;", Self::previous, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("nextIndex", "()I", Self::next_index, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("previousIndex", "()I", Self::previous_index, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("set", "(Ljava/lang/Object;)V", Self::set, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("add", "(Ljava/lang/Object;)V", Self::add, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, list: ClassInstanceRef<Object>, index: i32) -> Result<()> {
        let size: i32 = jvm.invoke_virtual(&list, "size", "()I", ()).await?;
        if index < 0 || index > size {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "list iterator index").await);
        }
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "list", "Ljava/util/Vector;", list).await?;
        jvm.put_field(&mut this, "cursor", "I", index).await?;
        jvm.put_field(&mut this, "lastReturned", "I", -1).await
    }

    async fn has_next(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let list: ClassInstanceRef<Object> = jvm.get_field(&this, "list", "Ljava/util/Vector;").await?;
        let cursor: i32 = jvm.get_field(&this, "cursor", "I").await?;
        Ok(cursor < jvm.invoke_virtual::<_, i32>(&list, "size", "()I", ()).await?)
    }

    async fn next(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let list: ClassInstanceRef<Object> = jvm.get_field(&this, "list", "Ljava/util/Vector;").await?;
        let cursor: i32 = jvm.get_field(&this, "cursor", "I").await?;
        if cursor >= jvm.invoke_virtual::<_, i32>(&list, "size", "()I", ()).await? {
            return Err(jvm.exception("java/util/NoSuchElementException", "Vector iterator exhausted").await);
        }
        let element = jvm.invoke_virtual(&list, "get", "(I)Ljava/lang/Object;", (cursor,)).await?;
        jvm.put_field(&mut this, "cursor", "I", cursor + 1).await?;
        jvm.put_field(&mut this, "lastReturned", "I", cursor).await?;
        Ok(element)
    }

    async fn has_previous(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        Ok(jvm.get_field::<i32>(&this, "cursor", "I").await? > 0)
    }

    async fn previous(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let cursor: i32 = jvm.get_field(&this, "cursor", "I").await?;
        if cursor <= 0 {
            return Err(jvm.exception("java/util/NoSuchElementException", "Vector iterator exhausted").await);
        }
        let index = cursor - 1;
        let list: ClassInstanceRef<Object> = jvm.get_field(&this, "list", "Ljava/util/Vector;").await?;
        let element = jvm.invoke_virtual(&list, "get", "(I)Ljava/lang/Object;", (index,)).await?;
        jvm.put_field(&mut this, "cursor", "I", index).await?;
        jvm.put_field(&mut this, "lastReturned", "I", index).await?;
        Ok(element)
    }

    async fn next_index(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "cursor", "I").await
    }

    async fn previous_index(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(jvm.get_field::<i32>(&this, "cursor", "I").await? - 1)
    }

    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let last_returned: i32 = jvm.get_field(&this, "lastReturned", "I").await?;
        if last_returned < 0 {
            return Err(jvm.exception("java/lang/IllegalStateException", "iterator state").await);
        }
        let list: ClassInstanceRef<Object> = jvm.get_field(&this, "list", "Ljava/util/Vector;").await?;
        let _: ClassInstanceRef<Object> = jvm.invoke_virtual(&list, "remove", "(I)Ljava/lang/Object;", (last_returned,)).await?;
        let cursor: i32 = jvm.get_field(&this, "cursor", "I").await?;
        if last_returned < cursor {
            jvm.put_field(&mut this, "cursor", "I", cursor - 1).await?;
        }
        jvm.put_field(&mut this, "lastReturned", "I", -1).await
    }

    async fn set(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<()> {
        let last_returned: i32 = jvm.get_field(&this, "lastReturned", "I").await?;
        if last_returned < 0 {
            return Err(jvm.exception("java/lang/IllegalStateException", "iterator state").await);
        }
        let list: ClassInstanceRef<Object> = jvm.get_field(&this, "list", "Ljava/util/Vector;").await?;
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&list, "set", "(ILjava/lang/Object;)Ljava/lang/Object;", (last_returned, element))
            .await?;
        Ok(())
    }

    async fn add(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<()> {
        let cursor: i32 = jvm.get_field(&this, "cursor", "I").await?;
        let list: ClassInstanceRef<Object> = jvm.get_field(&this, "list", "Ljava/util/Vector;").await?;
        let _: () = jvm.invoke_virtual(&list, "add", "(ILjava/lang/Object;)V", (cursor, element)).await?;
        jvm.put_field(&mut this, "cursor", "I", cursor + 1).await?;
        jvm.put_field(&mut this, "lastReturned", "I", -1).await
    }
}
