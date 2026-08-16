use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

// class java.util.Collections$UnmodifiableList
pub struct CollectionsUnmodifiableList;

impl CollectionsUnmodifiableList {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Collections$UnmodifiableList",
            parent_class: Some("java/util/Collections$UnmodifiableCollection"),
            interfaces: vec!["java/util/List"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/util/List;)V", Self::init, MethodAccessFlags::empty()),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("get", "(I)Ljava/lang/Object;", Self::get, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("indexOf", "(Ljava/lang/Object;)I", Self::index_of, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("lastIndexOf", "(Ljava/lang/Object;)I", Self::last_index_of, MethodAccessFlags::PUBLIC),
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
                JavaMethodProto::new("set", "(ILjava/lang/Object;)Ljava/lang/Object;", Self::set, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("add", "(ILjava/lang/Object;)V", Self::add, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("addAll", "(ILjava/util/Collection;)Z", Self::add_all, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("remove", "(I)Ljava/lang/Object;", Self::remove, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![JavaFieldProto::new("list", "Ljava/util/List;", FieldAccessFlags::FINAL)],
            access_flags: ClassAccessFlags::empty(),
        }
    }

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
        jvm.put_field(&mut this, "list", "Ljava/util/List;", list).await
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if !other.is_null() && this.identity() == other.identity() {
            return Ok(true);
        }
        let list: ClassInstanceRef<Object> = jvm.get_field(&this, "list", "Ljava/util/List;").await?;
        jvm.invoke_virtual(&list, "java/lang/Object", "equals", "(Ljava/lang/Object;)Z", (other,))
            .await
    }

    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let list: ClassInstanceRef<Object> = jvm.get_field(&this, "list", "Ljava/util/List;").await?;
        jvm.invoke_virtual(&list, "java/lang/Object", "hashCode", "()I", ()).await
    }

    async fn get(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Object>> {
        let list: ClassInstanceRef<Object> = jvm.get_field(&this, "list", "Ljava/util/List;").await?;
        jvm.invoke_virtual(&list, &list.class_definition().name(), "get", "(I)Ljava/lang/Object;", (index,))
            .await
    }

    async fn index_of(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<i32> {
        let list: ClassInstanceRef<Object> = jvm.get_field(&this, "list", "Ljava/util/List;").await?;
        jvm.invoke_virtual(&list, &list.class_definition().name(), "indexOf", "(Ljava/lang/Object;)I", (element,))
            .await
    }

    async fn last_index_of(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<i32> {
        let list: ClassInstanceRef<Object> = jvm.get_field(&this, "list", "Ljava/util/List;").await?;
        jvm.invoke_virtual(&list, &list.class_definition().name(), "lastIndexOf", "(Ljava/lang/Object;)I", (element,))
            .await
    }

    async fn list_iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let list: ClassInstanceRef<Object> = jvm.get_field(&this, "list", "Ljava/util/List;").await?;
        let iterator: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&list, &list.class_definition().name(), "listIterator", "()Ljava/util/ListIterator;", ())
            .await?;
        Ok(jvm
            .new_class("java/util/Collections$UnmodifiableList$1", "(Ljava/util/ListIterator;)V", (iterator,))
            .await?
            .into())
    }

    async fn list_iterator_at(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Object>> {
        let list: ClassInstanceRef<Object> = jvm.get_field(&this, "list", "Ljava/util/List;").await?;
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

    async fn sub_list(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, from: i32, to: i32) -> Result<ClassInstanceRef<Object>> {
        let list: ClassInstanceRef<Object> = jvm.get_field(&this, "list", "Ljava/util/List;").await?;
        let sub_list: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&list, &list.class_definition().name(), "subList", "(II)Ljava/util/List;", (from, to))
            .await?;
        Ok(jvm
            .new_class("java/util/Collections$UnmodifiableList", "(Ljava/util/List;)V", (sub_list,))
            .await?
            .into())
    }

    async fn set(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        _: ClassInstanceRef<Self>,
        _: i32,
        _: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "unmodifiable list").await)
    }

    async fn add(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: i32, _: ClassInstanceRef<Object>) -> Result<()> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "unmodifiable list").await)
    }

    async fn add_all(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: i32, _: ClassInstanceRef<Object>) -> Result<bool> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "unmodifiable list").await)
    }

    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: i32) -> Result<ClassInstanceRef<Object>> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "unmodifiable list").await)
    }
}
