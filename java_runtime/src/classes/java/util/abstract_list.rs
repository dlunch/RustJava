use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

// abstract class java.util.AbstractList
pub struct AbstractList;

impl AbstractList {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/AbstractList",
            parent_class: Some("java/util/AbstractCollection"),
            interfaces: vec!["java/util/List"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PROTECTED),
                JavaMethodProto::new("add", "(Ljava/lang/Object;)Z", Self::add, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("addAll", "(ILjava/util/Collection;)Z", Self::add_all, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("indexOf", "(Ljava/lang/Object;)I", Self::index_of, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("lastIndexOf", "(Ljava/lang/Object;)I", Self::last_index_of, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("iterator", "()Ljava/util/Iterator;", Self::iterator, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "listIterator",
                    "()Ljava/util/ListIterator;",
                    Self::list_iterator,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "listIterator",
                    "(I)Ljava/util/ListIterator;",
                    Self::list_iterator_at,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("subList", "(II)Ljava/util/List;", Self::sub_list, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::ABSTRACT,
        }
    }

    async fn add(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        let size: i32 = jvm.invoke_virtual(&this, "size", "()I", ()).await?;
        let _: () = jvm.invoke_virtual(&this, "add", "(ILjava/lang/Object;)V", (size, element)).await?;
        Ok(true)
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.AbstractList::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/util/AbstractCollection", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn add_all(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        index: i32,
        collection: ClassInstanceRef<Object>,
    ) -> Result<bool> {
        tracing::debug!("java.util.AbstractList::addAll({this:?}, {index:?}, {collection:?})");

        let size: i32 = jvm.invoke_virtual(&this, "size", "()I", ()).await?;
        if index < 0 || index > size {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "index").await);
        }
        if collection.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "collection").await);
        }

        let elements: ClassInstanceRef<Array<Object>> = jvm.invoke_virtual(&collection, "toArray", "()[Ljava/lang/Object;", ()).await?;
        let count = jvm.array_length(&elements).await?;
        for (offset, element) in jvm
            .load_array::<ClassInstanceRef<Object>>(&elements, 0, count)
            .await?
            .into_iter()
            .enumerate()
        {
            let _: () = jvm
                .invoke_virtual(&this, "add", "(ILjava/lang/Object;)V", (index + offset as i32, element))
                .await?;
        }

        Ok(count != 0)
    }

    async fn last_index_of(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<i32> {
        tracing::debug!("java.util.AbstractList::lastIndexOf({this:?}, {element:?})");

        let size: i32 = jvm.invoke_virtual(&this, "size", "()I", ()).await?;
        for index in (0..size).rev() {
            let current: ClassInstanceRef<Object> = jvm.invoke_virtual(&this, "get", "(I)Ljava/lang/Object;", (index,)).await?;
            if element.is_null() {
                if current.is_null() {
                    return Ok(index);
                }
            } else if !current.is_null()
                && jvm
                    .invoke_virtual::<_, bool>(&element, "equals", "(Ljava/lang/Object;)Z", (current,))
                    .await?
            {
                return Ok(index);
            }
        }

        Ok(-1)
    }

    async fn index_of(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<i32> {
        let size: i32 = jvm.invoke_virtual(&this, "size", "()I", ()).await?;
        for index in 0..size {
            let current: ClassInstanceRef<Object> = jvm.invoke_virtual(&this, "get", "(I)Ljava/lang/Object;", (index,)).await?;
            if element.is_null() {
                if current.is_null() {
                    return Ok(index);
                }
            } else if !current.is_null()
                && jvm
                    .invoke_virtual::<_, bool>(&element, "equals", "(Ljava/lang/Object;)Z", (current,))
                    .await?
            {
                return Ok(index);
            }
        }
        Ok(-1)
    }

    async fn iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let iterator = jvm.new_class("java/util/AbstractList$Itr", "(Ljava/util/List;I)V", (this, 0)).await?;
        Ok(iterator.into())
    }

    async fn list_iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let iterator = jvm.new_class("java/util/AbstractList$ListItr", "(Ljava/util/List;I)V", (this, 0)).await?;
        Ok(iterator.into())
    }

    async fn list_iterator_at(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Object>> {
        let iterator = jvm
            .new_class("java/util/AbstractList$ListItr", "(Ljava/util/List;I)V", (this, index))
            .await?;
        Ok(iterator.into())
    }

    async fn sub_list(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, from: i32, to: i32) -> Result<ClassInstanceRef<Object>> {
        let size: i32 = jvm.invoke_virtual(&this, "size", "()I", ()).await?;
        if from < 0 || to > size {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "subList range").await);
        }
        if from > to {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "fromIndex > toIndex").await);
        }
        let parent: ClassInstanceRef<crate::classes::java::util::AbstractListSubList> = None.into();
        let sub_list = jvm
            .new_class(
                "java/util/AbstractList$SubList",
                "(Ljava/util/List;Ljava/util/AbstractList$SubList;II)V",
                (this, parent, from, to - from),
            )
            .await?;
        Ok(sub_list.into())
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if other.is_null() {
            return Ok(false);
        }
        if this.identity() == other.identity() {
            return Ok(true);
        }
        if !jvm.is_instance(other.as_ref(), "java/util/List") {
            return Ok(false);
        }

        let left: ClassInstanceRef<Object> = jvm.invoke_virtual(&this, "listIterator", "()Ljava/util/ListIterator;", ()).await?;
        let right: ClassInstanceRef<Object> = jvm.invoke_virtual(&other, "listIterator", "()Ljava/util/ListIterator;", ()).await?;
        loop {
            let left_has_next: bool = jvm.invoke_virtual(&left, "hasNext", "()Z", ()).await?;
            let right_has_next: bool = jvm.invoke_virtual(&right, "hasNext", "()Z", ()).await?;
            if !left_has_next || !right_has_next {
                return Ok(left_has_next == right_has_next);
            }

            let left_element: ClassInstanceRef<Object> = jvm.invoke_virtual(&left, "next", "()Ljava/lang/Object;", ()).await?;
            let right_element: ClassInstanceRef<Object> = jvm.invoke_virtual(&right, "next", "()Ljava/lang/Object;", ()).await?;
            let equal = if left_element.is_null() {
                right_element.is_null()
            } else if right_element.is_null() {
                false
            } else {
                jvm.invoke_virtual::<_, bool>(&left_element, "equals", "(Ljava/lang/Object;)Z", (right_element,))
                    .await?
            };
            if !equal {
                return Ok(false);
            }
        }
    }

    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&this, "iterator", "()Ljava/util/Iterator;", ()).await?;
        let mut hash = 1i32;
        while jvm.invoke_virtual::<_, bool>(&iterator, "hasNext", "()Z", ()).await? {
            let element: ClassInstanceRef<Object> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await?;
            let element_hash = if element.is_null() {
                0
            } else {
                jvm.invoke_virtual(&element, "hashCode", "()I", ()).await?
            };
            hash = hash.wrapping_mul(31).wrapping_add(element_hash);
        }
        Ok(hash)
    }
}

// classes java.util.AbstractList$Itr and java.util.AbstractList$ListItr
pub struct AbstractListItr;

impl AbstractListItr {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/AbstractList$Itr",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/util/Iterator"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/util/List;I)V", Self::init, Default::default()),
                JavaMethodProto::new("hasNext", "()Z", Self::has_next, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("next", "()Ljava/lang/Object;", Self::next, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("remove", "()V", Self::remove, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("list", "Ljava/util/List;", Default::default()),
                JavaFieldProto::new("cursor", "I", Default::default()),
                JavaFieldProto::new("lastReturned", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    pub fn list_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/AbstractList$ListItr",
            parent_class: Some("java/util/AbstractList$Itr"),
            interfaces: vec!["java/util/ListIterator"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/util/List;I)V", Self::init, Default::default()),
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
        jvm.put_field(&mut this, "list", "Ljava/util/List;", list).await?;
        jvm.put_field(&mut this, "cursor", "I", index).await?;
        jvm.put_field(&mut this, "lastReturned", "I", -1).await
    }

    async fn has_next(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let list: ClassInstanceRef<Object> = jvm.get_field(&this, "list", "Ljava/util/List;").await?;
        let cursor: i32 = jvm.get_field(&this, "cursor", "I").await?;
        Ok(cursor < jvm.invoke_virtual::<_, i32>(&list, "size", "()I", ()).await?)
    }

    async fn next(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let list: ClassInstanceRef<Object> = jvm.get_field(&this, "list", "Ljava/util/List;").await?;
        let cursor: i32 = jvm.get_field(&this, "cursor", "I").await?;
        if cursor >= jvm.invoke_virtual::<_, i32>(&list, "size", "()I", ()).await? {
            return Err(jvm.exception("java/util/NoSuchElementException", "AbstractList iterator exhausted").await);
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
            return Err(jvm.exception("java/util/NoSuchElementException", "AbstractList iterator exhausted").await);
        }
        let index = cursor - 1;
        let list: ClassInstanceRef<Object> = jvm.get_field(&this, "list", "Ljava/util/List;").await?;
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
        let list: ClassInstanceRef<Object> = jvm.get_field(&this, "list", "Ljava/util/List;").await?;
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
        let list: ClassInstanceRef<Object> = jvm.get_field(&this, "list", "Ljava/util/List;").await?;
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&list, "set", "(ILjava/lang/Object;)Ljava/lang/Object;", (last_returned, element))
            .await?;
        Ok(())
    }

    async fn add(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<()> {
        let cursor: i32 = jvm.get_field(&this, "cursor", "I").await?;
        let list: ClassInstanceRef<Object> = jvm.get_field(&this, "list", "Ljava/util/List;").await?;
        let _: () = jvm.invoke_virtual(&list, "add", "(ILjava/lang/Object;)V", (cursor, element)).await?;
        jvm.put_field(&mut this, "cursor", "I", cursor + 1).await?;
        jvm.put_field(&mut this, "lastReturned", "I", -1).await
    }
}
