use alloc::{format, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{lang::Object, util::LinkedListEntry},
};

// class java.util.LinkedList
pub struct LinkedList;

impl LinkedList {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/LinkedList",
            parent_class: Some("java/util/AbstractList"),
            interfaces: vec!["java/util/List", "java/lang/Cloneable", "java/io/Serializable"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(Ljava/util/Collection;)V", Self::init_collection, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("addFirst", "(Ljava/lang/Object;)V", Self::add_first, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("addLast", "(Ljava/lang/Object;)V", Self::add_last, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getFirst", "()Ljava/lang/Object;", Self::get_first, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getLast", "()Ljava/lang/Object;", Self::get_last, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("removeFirst", "()Ljava/lang/Object;", Self::remove_first, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("removeLast", "()Ljava/lang/Object;", Self::remove_last, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("size", "()I", Self::size, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("get", "(I)Ljava/lang/Object;", Self::get, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("set", "(ILjava/lang/Object;)Ljava/lang/Object;", Self::set, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("add", "(ILjava/lang/Object;)V", Self::add_at, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("add", "(Ljava/lang/Object;)Z", Self::add, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("remove", "(I)Ljava/lang/Object;", Self::remove_at, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("remove", "(Ljava/lang/Object;)Z", Self::remove_object, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("indexOf", "(Ljava/lang/Object;)I", Self::index_of, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("lastIndexOf", "(Ljava/lang/Object;)I", Self::last_index_of, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("clear", "()V", Self::clear, MethodAccessFlags::PUBLIC),
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
            ],
            fields: vec![
                JavaFieldProto::new("header", "Ljava/util/LinkedList$Entry;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("size", "I", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/util/AbstractList", "<init>", "()V", ()).await?;
        let null_object: ClassInstanceRef<Object> = None.into();
        let null_entry: ClassInstanceRef<LinkedListEntry> = None.into();
        let mut header: ClassInstanceRef<LinkedListEntry> = jvm
            .new_class(
                "java/util/LinkedList$Entry",
                "(Ljava/lang/Object;Ljava/util/LinkedList$Entry;Ljava/util/LinkedList$Entry;)V",
                (null_object, null_entry.clone(), null_entry),
            )
            .await?
            .into();
        let header_next = header.clone();
        jvm.put_field(&mut header, "next", "Ljava/util/LinkedList$Entry;", header_next).await?;
        let header_previous = header.clone();
        jvm.put_field(&mut header, "previous", "Ljava/util/LinkedList$Entry;", header_previous)
            .await?;
        jvm.put_field(&mut this, "header", "Ljava/util/LinkedList$Entry;", header).await?;
        jvm.put_field(&mut this, "size", "I", 0).await
    }

    async fn init_collection(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        collection: ClassInstanceRef<Object>,
    ) -> Result<()> {
        if collection.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "collection").await);
        }
        Self::init(jvm, context, this.clone()).await?;
        let _: bool = jvm.invoke_virtual(&this, "addAll", "(Ljava/util/Collection;)Z", (collection,)).await?;
        Ok(())
    }

    async fn add_first(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<()> {
        let header: ClassInstanceRef<LinkedListEntry> = jvm.get_field(&this, "header", "Ljava/util/LinkedList$Entry;").await?;
        let first = jvm.get_field(&header, "next", "Ljava/util/LinkedList$Entry;").await?;
        Self::add_before(jvm, &this, element, first).await
    }

    async fn add_last(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<()> {
        let header = jvm.get_field(&this, "header", "Ljava/util/LinkedList$Entry;").await?;
        Self::add_before(jvm, &this, element, header).await
    }

    async fn get_first(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        if jvm.get_field::<i32>(&this, "size", "I").await? == 0 {
            return Err(jvm.exception("java/util/NoSuchElementException", "LinkedList is empty").await);
        }
        let header: ClassInstanceRef<LinkedListEntry> = jvm.get_field(&this, "header", "Ljava/util/LinkedList$Entry;").await?;
        let first: ClassInstanceRef<LinkedListEntry> = jvm.get_field(&header, "next", "Ljava/util/LinkedList$Entry;").await?;
        jvm.get_field(&first, "element", "Ljava/lang/Object;").await
    }

    async fn get_last(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        if jvm.get_field::<i32>(&this, "size", "I").await? == 0 {
            return Err(jvm.exception("java/util/NoSuchElementException", "LinkedList is empty").await);
        }
        let header: ClassInstanceRef<LinkedListEntry> = jvm.get_field(&this, "header", "Ljava/util/LinkedList$Entry;").await?;
        let last: ClassInstanceRef<LinkedListEntry> = jvm.get_field(&header, "previous", "Ljava/util/LinkedList$Entry;").await?;
        jvm.get_field(&last, "element", "Ljava/lang/Object;").await
    }

    async fn remove_first(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        if jvm.get_field::<i32>(&this, "size", "I").await? == 0 {
            return Err(jvm.exception("java/util/NoSuchElementException", "LinkedList is empty").await);
        }
        let header: ClassInstanceRef<LinkedListEntry> = jvm.get_field(&this, "header", "Ljava/util/LinkedList$Entry;").await?;
        let first = jvm.get_field(&header, "next", "Ljava/util/LinkedList$Entry;").await?;
        Self::remove_entry(jvm, &this, first).await
    }

    async fn remove_last(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        if jvm.get_field::<i32>(&this, "size", "I").await? == 0 {
            return Err(jvm.exception("java/util/NoSuchElementException", "LinkedList is empty").await);
        }
        let header: ClassInstanceRef<LinkedListEntry> = jvm.get_field(&this, "header", "Ljava/util/LinkedList$Entry;").await?;
        let last = jvm.get_field(&header, "previous", "Ljava/util/LinkedList$Entry;").await?;
        Self::remove_entry(jvm, &this, last).await
    }

    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "size", "I").await
    }

    async fn get(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Object>> {
        let entry = Self::entry_at(jvm, &this, index).await?;
        jvm.get_field(&entry, "element", "Ljava/lang/Object;").await
    }

    async fn set(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        index: i32,
        element: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        let mut entry = Self::entry_at(jvm, &this, index).await?;
        let old = jvm.get_field(&entry, "element", "Ljava/lang/Object;").await?;
        jvm.put_field(&mut entry, "element", "Ljava/lang/Object;", element).await?;
        Ok(old)
    }

    async fn add_at(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32, element: ClassInstanceRef<Object>) -> Result<()> {
        let size: i32 = jvm.get_field(&this, "size", "I").await?;
        if index < 0 || index > size {
            return Err(jvm
                .exception("java/lang/IndexOutOfBoundsException", &format!("Index: {index}, Size: {size}"))
                .await);
        }
        let successor = if index == size {
            jvm.get_field(&this, "header", "Ljava/util/LinkedList$Entry;").await?
        } else {
            Self::entry_at(jvm, &this, index).await?
        };
        Self::add_before(jvm, &this, element, successor).await
    }

    async fn add(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        let header = jvm.get_field(&this, "header", "Ljava/util/LinkedList$Entry;").await?;
        Self::add_before(jvm, &this, element, header).await?;
        Ok(true)
    }

    async fn remove_at(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Object>> {
        let entry = Self::entry_at(jvm, &this, index).await?;
        Self::remove_entry(jvm, &this, entry).await
    }

    async fn remove_object(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        let size: i32 = jvm.get_field(&this, "size", "I").await?;
        for index in 0..size {
            let entry = Self::entry_at(jvm, &this, index).await?;
            let current: ClassInstanceRef<Object> = jvm.get_field(&entry, "element", "Ljava/lang/Object;").await?;
            let equal = if element.is_null() {
                current.is_null()
            } else {
                jvm.invoke_virtual::<_, bool>(&element, "equals", "(Ljava/lang/Object;)Z", (current,))
                    .await?
            };
            if equal {
                let _: ClassInstanceRef<Object> = Self::remove_entry(jvm, &this, entry).await?;
                return Ok(true);
            }
        }
        Ok(false)
    }

    async fn index_of(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<i32> {
        let size: i32 = jvm.get_field(&this, "size", "I").await?;
        for index in 0..size {
            let entry = Self::entry_at(jvm, &this, index).await?;
            let current: ClassInstanceRef<Object> = jvm.get_field(&entry, "element", "Ljava/lang/Object;").await?;
            if (element.is_null() && current.is_null())
                || (!element.is_null()
                    && jvm
                        .invoke_virtual::<_, bool>(&element, "equals", "(Ljava/lang/Object;)Z", (current,))
                        .await?)
            {
                return Ok(index);
            }
        }
        Ok(-1)
    }

    async fn last_index_of(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<i32> {
        let size: i32 = jvm.get_field(&this, "size", "I").await?;
        for index in (0..size).rev() {
            let entry = Self::entry_at(jvm, &this, index).await?;
            let current: ClassInstanceRef<Object> = jvm.get_field(&entry, "element", "Ljava/lang/Object;").await?;
            if (element.is_null() && current.is_null())
                || (!element.is_null()
                    && jvm
                        .invoke_virtual::<_, bool>(&element, "equals", "(Ljava/lang/Object;)Z", (current,))
                        .await?)
            {
                return Ok(index);
            }
        }
        Ok(-1)
    }

    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        while jvm.get_field::<i32>(&this, "size", "I").await? > 0 {
            let _: ClassInstanceRef<Object> = jvm.invoke_virtual(&this, "removeFirst", "()Ljava/lang/Object;", ()).await?;
        }
        Ok(())
    }

    async fn iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let iterator = jvm
            .new_class("java/util/LinkedList$ListItr", "(Ljava/util/LinkedList;I)V", (this, 0))
            .await?;
        Ok(iterator.into())
    }

    async fn list_iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let iterator = jvm
            .new_class("java/util/LinkedList$ListItr", "(Ljava/util/LinkedList;I)V", (this, 0))
            .await?;
        Ok(iterator.into())
    }

    async fn list_iterator_at(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Object>> {
        let iterator = jvm
            .new_class("java/util/LinkedList$ListItr", "(Ljava/util/LinkedList;I)V", (this, index))
            .await?;
        Ok(iterator.into())
    }

    async fn entry_at(jvm: &Jvm, this: &ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<LinkedListEntry>> {
        let size: i32 = jvm.get_field(this, "size", "I").await?;
        if index < 0 || index >= size {
            return Err(jvm
                .exception("java/lang/IndexOutOfBoundsException", &format!("Index: {index}, Size: {size}"))
                .await);
        }
        let header: ClassInstanceRef<LinkedListEntry> = jvm.get_field(this, "header", "Ljava/util/LinkedList$Entry;").await?;
        if index < size / 2 {
            let mut entry: ClassInstanceRef<LinkedListEntry> = jvm.get_field(&header, "next", "Ljava/util/LinkedList$Entry;").await?;
            for _ in 0..index {
                entry = jvm.get_field(&entry, "next", "Ljava/util/LinkedList$Entry;").await?;
            }
            Ok(entry)
        } else {
            let mut entry: ClassInstanceRef<LinkedListEntry> = jvm.get_field(&header, "previous", "Ljava/util/LinkedList$Entry;").await?;
            for _ in (index + 1)..size {
                entry = jvm.get_field(&entry, "previous", "Ljava/util/LinkedList$Entry;").await?;
            }
            Ok(entry)
        }
    }

    async fn add_before(
        jvm: &Jvm,
        this: &ClassInstanceRef<Self>,
        element: ClassInstanceRef<Object>,
        mut successor: ClassInstanceRef<LinkedListEntry>,
    ) -> Result<()> {
        let mut predecessor: ClassInstanceRef<LinkedListEntry> = jvm.get_field(&successor, "previous", "Ljava/util/LinkedList$Entry;").await?;
        let entry: ClassInstanceRef<LinkedListEntry> = jvm
            .new_class(
                "java/util/LinkedList$Entry",
                "(Ljava/lang/Object;Ljava/util/LinkedList$Entry;Ljava/util/LinkedList$Entry;)V",
                (element, successor.clone(), predecessor.clone()),
            )
            .await?
            .into();
        jvm.put_field(&mut predecessor, "next", "Ljava/util/LinkedList$Entry;", entry.clone())
            .await?;
        jvm.put_field(&mut successor, "previous", "Ljava/util/LinkedList$Entry;", entry.clone())
            .await?;
        let size: i32 = jvm.get_field(this, "size", "I").await?;
        let mut list = this.clone();
        jvm.put_field(&mut list, "size", "I", size + 1).await?;
        Ok(())
    }

    async fn remove_entry(
        jvm: &Jvm,
        this: &ClassInstanceRef<Self>,
        mut entry: ClassInstanceRef<LinkedListEntry>,
    ) -> Result<ClassInstanceRef<Object>> {
        let mut previous: ClassInstanceRef<LinkedListEntry> = jvm.get_field(&entry, "previous", "Ljava/util/LinkedList$Entry;").await?;
        let mut next: ClassInstanceRef<LinkedListEntry> = jvm.get_field(&entry, "next", "Ljava/util/LinkedList$Entry;").await?;
        jvm.put_field(&mut previous, "next", "Ljava/util/LinkedList$Entry;", next.clone()).await?;
        jvm.put_field(&mut next, "previous", "Ljava/util/LinkedList$Entry;", previous).await?;
        let element = jvm.get_field(&entry, "element", "Ljava/lang/Object;").await?;
        let null_object: ClassInstanceRef<Object> = None.into();
        let null_entry: ClassInstanceRef<LinkedListEntry> = None.into();
        jvm.put_field(&mut entry, "element", "Ljava/lang/Object;", null_object).await?;
        jvm.put_field(&mut entry, "next", "Ljava/util/LinkedList$Entry;", null_entry.clone())
            .await?;
        jvm.put_field(&mut entry, "previous", "Ljava/util/LinkedList$Entry;", null_entry).await?;
        let size: i32 = jvm.get_field(this, "size", "I").await?;
        let mut list = this.clone();
        jvm.put_field(&mut list, "size", "I", size - 1).await?;
        Ok(element)
    }
}
