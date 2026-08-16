use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

// class java.util.AbstractList$SubList
pub struct AbstractListSubList;

impl AbstractListSubList {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/AbstractList$SubList",
            parent_class: Some("java/util/AbstractList"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/util/List;Ljava/util/AbstractList$SubList;II)V",
                    Self::init,
                    MethodAccessFlags::empty(),
                ),
                JavaMethodProto::new("size", "()I", Self::size, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("get", "(I)Ljava/lang/Object;", Self::get, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("set", "(ILjava/lang/Object;)Ljava/lang/Object;", Self::set, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("add", "(Ljava/lang/Object;)Z", Self::add, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("add", "(ILjava/lang/Object;)V", Self::add_at, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("addAll", "(Ljava/util/Collection;)Z", Self::add_all, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("addAll", "(ILjava/util/Collection;)Z", Self::add_all_at, MethodAccessFlags::PUBLIC),
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
                JavaMethodProto::new("subList", "(II)Ljava/util/List;", Self::sub_list, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("root", "Ljava/util/List;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new(
                    "parent",
                    "Ljava/util/AbstractList$SubList;",
                    FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new("offset", "I", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL),
                JavaFieldProto::new("size", "I", FieldAccessFlags::PROTECTED),
            ],
            access_flags: ClassAccessFlags::empty(),
        }
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        root: ClassInstanceRef<Object>,
        parent: ClassInstanceRef<Self>,
        offset: i32,
        size: i32,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/util/AbstractList", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "root", "Ljava/util/List;", root).await?;
        jvm.put_field(&mut this, "parent", "Ljava/util/AbstractList$SubList;", parent).await?;
        jvm.put_field(&mut this, "offset", "I", offset).await?;
        jvm.put_field(&mut this, "size", "I", size).await
    }

    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "size", "I").await
    }

    async fn get(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Object>> {
        let size: i32 = jvm.get_field(&this, "size", "I").await?;
        if index < 0 || index >= size {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "subList index").await);
        }
        let root: ClassInstanceRef<Object> = jvm.get_field(&this, "root", "Ljava/util/List;").await?;
        let offset: i32 = jvm.get_field(&this, "offset", "I").await?;
        jvm.invoke_virtual(&root, &root.class_definition().name(), "get", "(I)Ljava/lang/Object;", (offset + index,))
            .await
    }

    async fn set(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        index: i32,
        element: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        let size: i32 = jvm.get_field(&this, "size", "I").await?;
        if index < 0 || index >= size {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "subList index").await);
        }
        let root: ClassInstanceRef<Object> = jvm.get_field(&this, "root", "Ljava/util/List;").await?;
        let offset: i32 = jvm.get_field(&this, "offset", "I").await?;
        jvm.invoke_virtual(
            &root,
            &root.class_definition().name(),
            "set",
            "(ILjava/lang/Object;)Ljava/lang/Object;",
            (offset + index, element),
        )
        .await
    }

    async fn add(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        let size: i32 = jvm.get_field(&this, "size", "I").await?;
        let _: () = jvm
            .invoke_virtual(&this, "java/util/AbstractList$SubList", "add", "(ILjava/lang/Object;)V", (size, element))
            .await?;
        Ok(true)
    }

    async fn add_at(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32, element: ClassInstanceRef<Object>) -> Result<()> {
        let size: i32 = jvm.get_field(&this, "size", "I").await?;
        if index < 0 || index > size {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "subList index").await);
        }
        let root: ClassInstanceRef<Object> = jvm.get_field(&this, "root", "Ljava/util/List;").await?;
        let offset: i32 = jvm.get_field(&this, "offset", "I").await?;
        let _: () = jvm
            .invoke_virtual(
                &root,
                &root.class_definition().name(),
                "add",
                "(ILjava/lang/Object;)V",
                (offset + index, element),
            )
            .await?;
        Self::update_sizes(jvm, this, 1).await
    }

    async fn add_all(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, collection: ClassInstanceRef<Object>) -> Result<bool> {
        let size: i32 = jvm.get_field(&this, "size", "I").await?;
        jvm.invoke_virtual(
            &this,
            "java/util/AbstractList$SubList",
            "addAll",
            "(ILjava/util/Collection;)Z",
            (size, collection),
        )
        .await
    }

    async fn add_all_at(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        index: i32,
        collection: ClassInstanceRef<Object>,
    ) -> Result<bool> {
        let size: i32 = jvm.get_field(&this, "size", "I").await?;
        if index < 0 || index > size {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "subList index").await);
        }
        if collection.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "collection").await);
        }
        if jvm
            .invoke_virtual::<_, i32>(&collection, &collection.class_definition().name(), "size", "()I", ())
            .await?
            == 0
        {
            return Ok(false);
        }
        let root: ClassInstanceRef<Object> = jvm.get_field(&this, "root", "Ljava/util/List;").await?;
        let offset: i32 = jvm.get_field(&this, "offset", "I").await?;
        let old_root_size: i32 = jvm.invoke_virtual(&root, &root.class_definition().name(), "size", "()I", ()).await?;
        let modified: bool = jvm
            .invoke_virtual(
                &root,
                &root.class_definition().name(),
                "addAll",
                "(ILjava/util/Collection;)Z",
                (offset + index, collection),
            )
            .await?;
        if modified {
            let new_root_size: i32 = jvm.invoke_virtual(&root, &root.class_definition().name(), "size", "()I", ()).await?;
            Self::update_sizes(jvm, this, new_root_size - old_root_size).await?;
        }
        Ok(modified)
    }

    async fn remove_at(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Object>> {
        let size: i32 = jvm.get_field(&this, "size", "I").await?;
        if index < 0 || index >= size {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "subList index").await);
        }
        let root: ClassInstanceRef<Object> = jvm.get_field(&this, "root", "Ljava/util/List;").await?;
        let offset: i32 = jvm.get_field(&this, "offset", "I").await?;
        let removed = jvm
            .invoke_virtual(
                &root,
                &root.class_definition().name(),
                "remove",
                "(I)Ljava/lang/Object;",
                (offset + index,),
            )
            .await?;
        Self::update_sizes(jvm, this, -1).await?;
        Ok(removed)
    }

    async fn remove_object(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        let index: i32 = jvm
            .invoke_virtual(&this, "java/util/AbstractList$SubList", "indexOf", "(Ljava/lang/Object;)I", (element,))
            .await?;
        if index < 0 {
            return Ok(false);
        }
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&this, "java/util/AbstractList$SubList", "remove", "(I)Ljava/lang/Object;", (index,))
            .await?;
        Ok(true)
    }

    async fn index_of(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<i32> {
        let size: i32 = jvm.get_field(&this, "size", "I").await?;
        for index in 0..size {
            let current: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&this, "java/util/AbstractList$SubList", "get", "(I)Ljava/lang/Object;", (index,))
                .await?;
            if (element.is_null() && current.is_null())
                || (!element.is_null()
                    && jvm
                        .invoke_virtual::<_, bool>(&element, "java/lang/Object", "equals", "(Ljava/lang/Object;)Z", (current,))
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
            let current: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&this, "java/util/AbstractList$SubList", "get", "(I)Ljava/lang/Object;", (index,))
                .await?;
            if (element.is_null() && current.is_null())
                || (!element.is_null()
                    && jvm
                        .invoke_virtual::<_, bool>(&element, "java/lang/Object", "equals", "(Ljava/lang/Object;)Z", (current,))
                        .await?)
            {
                return Ok(index);
            }
        }
        Ok(-1)
    }

    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        while jvm.get_field::<i32>(&this, "size", "I").await? > 0 {
            let _: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&this, "java/util/AbstractList$SubList", "remove", "(I)Ljava/lang/Object;", (0,))
                .await?;
        }
        Ok(())
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
        let size: i32 = jvm.get_field(&this, "size", "I").await?;
        if from < 0 || to > size {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "subList range").await);
        }
        if from > to {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "fromIndex > toIndex").await);
        }
        let root: ClassInstanceRef<Object> = jvm.get_field(&this, "root", "Ljava/util/List;").await?;
        let offset: i32 = jvm.get_field(&this, "offset", "I").await?;
        let sub_list = jvm
            .new_class(
                "java/util/AbstractList$SubList",
                "(Ljava/util/List;Ljava/util/AbstractList$SubList;II)V",
                (root, this, offset + from, to - from),
            )
            .await?;
        Ok(sub_list.into())
    }

    async fn update_sizes(jvm: &Jvm, mut current: ClassInstanceRef<Self>, delta: i32) -> Result<()> {
        while !current.is_null() {
            let size: i32 = jvm.get_field(&current, "size", "I").await?;
            jvm.put_field(&mut current, "size", "I", size + delta).await?;
            current = jvm.get_field(&current, "parent", "Ljava/util/AbstractList$SubList;").await?;
        }
        Ok(())
    }
}
