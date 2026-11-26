use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{lang::Object, util::List},
};

// class java.util.AbstractList$SubList
#[java_class(name = "java/util/AbstractList$SubList", extends = "java/util/AbstractList")]
pub struct AbstractListSubList {
    #[field(private)]
    root: ClassInstanceRef<List>,
    #[field(private, final)]
    parent: ClassInstanceRef<Self>,
    #[field(private, final)]
    offset: i32,
    #[field(protected)]
    size: i32,
}

#[java_methods]
impl AbstractListSubList {
    #[method(constructor, descriptor = "(Ljava/util/List;Ljava/util/AbstractList$SubList;II)V")]
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
        jvm.put_field(&mut this, "java/util/AbstractList$SubList", "root", "Ljava/util/List;", root)
            .await?;
        jvm.put_field(
            &mut this,
            "java/util/AbstractList$SubList",
            "parent",
            "Ljava/util/AbstractList$SubList;",
            parent,
        )
        .await?;
        jvm.put_field(&mut this, "java/util/AbstractList$SubList", "offset", "I", offset).await?;
        jvm.put_field(&mut this, "java/util/AbstractList$SubList", "size", "I", size).await
    }

    #[method(public)]
    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "java/util/AbstractList$SubList", "size", "I").await
    }

    #[method(public)]
    async fn get(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Object>> {
        let size: i32 = jvm.get_field(&this, "java/util/AbstractList$SubList", "size", "I").await?;
        if index < 0 || index >= size {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "subList index").await);
        }
        let root: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/AbstractList$SubList", "root", "Ljava/util/List;").await?;
        let offset: i32 = jvm.get_field(&this, "java/util/AbstractList$SubList", "offset", "I").await?;
        jvm.invoke_virtual(&root, &root.class_definition().name(), "get", "(I)Ljava/lang/Object;", (offset + index,))
            .await
    }

    #[method(public)]
    async fn set(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        index: i32,
        element: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        let size: i32 = jvm.get_field(&this, "java/util/AbstractList$SubList", "size", "I").await?;
        if index < 0 || index >= size {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "subList index").await);
        }
        let root: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/AbstractList$SubList", "root", "Ljava/util/List;").await?;
        let offset: i32 = jvm.get_field(&this, "java/util/AbstractList$SubList", "offset", "I").await?;
        jvm.invoke_virtual(
            &root,
            &root.class_definition().name(),
            "set",
            "(ILjava/lang/Object;)Ljava/lang/Object;",
            (offset + index, element),
        )
        .await
    }

    #[method(public)]
    async fn add(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool> {
        let size: i32 = jvm.get_field(&this, "java/util/AbstractList$SubList", "size", "I").await?;
        let _: () = jvm
            .invoke_virtual(&this, "java/util/AbstractList$SubList", "add", "(ILjava/lang/Object;)V", (size, element))
            .await?;
        Ok(true)
    }

    #[method(name = "add", public)]
    async fn add_at(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32, element: ClassInstanceRef<Object>) -> Result<()> {
        let size: i32 = jvm.get_field(&this, "java/util/AbstractList$SubList", "size", "I").await?;
        if index < 0 || index > size {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "subList index").await);
        }
        let root: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/AbstractList$SubList", "root", "Ljava/util/List;").await?;
        let offset: i32 = jvm.get_field(&this, "java/util/AbstractList$SubList", "offset", "I").await?;
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

    #[method(descriptor = "(Ljava/util/Collection;)Z", public)]
    async fn add_all(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, collection: ClassInstanceRef<Object>) -> Result<bool> {
        let size: i32 = jvm.get_field(&this, "java/util/AbstractList$SubList", "size", "I").await?;
        jvm.invoke_virtual(
            &this,
            "java/util/AbstractList$SubList",
            "addAll",
            "(ILjava/util/Collection;)Z",
            (size, collection),
        )
        .await
    }

    #[method(name = "addAll", descriptor = "(ILjava/util/Collection;)Z", public)]
    async fn add_all_at(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        index: i32,
        collection: ClassInstanceRef<Object>,
    ) -> Result<bool> {
        let size: i32 = jvm.get_field(&this, "java/util/AbstractList$SubList", "size", "I").await?;
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
        let root: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/AbstractList$SubList", "root", "Ljava/util/List;").await?;
        let offset: i32 = jvm.get_field(&this, "java/util/AbstractList$SubList", "offset", "I").await?;
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

    #[method(name = "remove", public)]
    async fn remove_at(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Object>> {
        let size: i32 = jvm.get_field(&this, "java/util/AbstractList$SubList", "size", "I").await?;
        if index < 0 || index >= size {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "subList index").await);
        }
        let root: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/AbstractList$SubList", "root", "Ljava/util/List;").await?;
        let offset: i32 = jvm.get_field(&this, "java/util/AbstractList$SubList", "offset", "I").await?;
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

    #[method(name = "remove", public)]
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

    #[method(public)]
    async fn index_of(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<i32> {
        let size: i32 = jvm.get_field(&this, "java/util/AbstractList$SubList", "size", "I").await?;
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

    #[method(public)]
    async fn last_index_of(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<i32> {
        let size: i32 = jvm.get_field(&this, "java/util/AbstractList$SubList", "size", "I").await?;
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

    #[method(public)]
    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        while jvm.get_field::<i32>(&this, "java/util/AbstractList$SubList", "size", "I").await? > 0 {
            let _: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&this, "java/util/AbstractList$SubList", "remove", "(I)Ljava/lang/Object;", (0,))
                .await?;
        }
        Ok(())
    }

    #[method(descriptor = "()Ljava/util/Iterator;", public)]
    async fn iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let iterator = jvm.new_class("java/util/AbstractList$Itr", "(Ljava/util/List;I)V", (this, 0)).await?;
        Ok(iterator.into())
    }

    #[method(descriptor = "()Ljava/util/ListIterator;", public)]
    async fn list_iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let iterator = jvm.new_class("java/util/AbstractList$ListItr", "(Ljava/util/List;I)V", (this, 0)).await?;
        Ok(iterator.into())
    }

    #[method(name = "listIterator", descriptor = "(I)Ljava/util/ListIterator;", public)]
    async fn list_iterator_at(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Object>> {
        let iterator = jvm
            .new_class("java/util/AbstractList$ListItr", "(Ljava/util/List;I)V", (this, index))
            .await?;
        Ok(iterator.into())
    }

    #[method(descriptor = "(II)Ljava/util/List;", public)]
    async fn sub_list(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, from: i32, to: i32) -> Result<ClassInstanceRef<Object>> {
        let size: i32 = jvm.get_field(&this, "java/util/AbstractList$SubList", "size", "I").await?;
        if from < 0 || to > size {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "subList range").await);
        }
        if from > to {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "fromIndex > toIndex").await);
        }
        let root: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/AbstractList$SubList", "root", "Ljava/util/List;").await?;
        let offset: i32 = jvm.get_field(&this, "java/util/AbstractList$SubList", "offset", "I").await?;
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
            let size: i32 = jvm.get_field(&current, "java/util/AbstractList$SubList", "size", "I").await?;
            jvm.put_field(&mut current, "java/util/AbstractList$SubList", "size", "I", size + delta)
                .await?;
            current = jvm
                .get_field(&current, "java/util/AbstractList$SubList", "parent", "Ljava/util/AbstractList$SubList;")
                .await?;
        }
        Ok(())
    }
}
