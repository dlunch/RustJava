use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

// class java.util.Collections$UnmodifiableList$1
pub struct CollectionsUnmodifiableListIterator;

impl CollectionsUnmodifiableListIterator {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Collections$UnmodifiableList$1",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/util/ListIterator"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/util/ListIterator;)V", Self::init, Default::default()),
                JavaMethodProto::new("hasNext", "()Z", Self::has_next, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("next", "()Ljava/lang/Object;", Self::next, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("hasPrevious", "()Z", Self::has_previous, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("previous", "()Ljava/lang/Object;", Self::previous, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("nextIndex", "()I", Self::next_index, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("previousIndex", "()I", Self::previous_index, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("remove", "()V", Self::remove, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("set", "(Ljava/lang/Object;)V", Self::set, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("add", "(Ljava/lang/Object;)V", Self::add, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![JavaFieldProto::new(
                "i",
                "Ljava/util/ListIterator;",
                FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL,
            )],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, iterator: ClassInstanceRef<Object>) -> Result<()> {
        if iterator.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "iterator").await);
        }
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "i", "Ljava/util/ListIterator;", iterator).await
    }

    async fn has_next(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let iterator: ClassInstanceRef<Object> = jvm.get_field(&this, "i", "Ljava/util/ListIterator;").await?;
        jvm.invoke_virtual(&iterator, "hasNext", "()Z", ()).await
    }

    async fn next(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let iterator: ClassInstanceRef<Object> = jvm.get_field(&this, "i", "Ljava/util/ListIterator;").await?;
        jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await
    }

    async fn has_previous(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let iterator: ClassInstanceRef<Object> = jvm.get_field(&this, "i", "Ljava/util/ListIterator;").await?;
        jvm.invoke_virtual(&iterator, "hasPrevious", "()Z", ()).await
    }

    async fn previous(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let iterator: ClassInstanceRef<Object> = jvm.get_field(&this, "i", "Ljava/util/ListIterator;").await?;
        jvm.invoke_virtual(&iterator, "previous", "()Ljava/lang/Object;", ()).await
    }

    async fn next_index(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let iterator: ClassInstanceRef<Object> = jvm.get_field(&this, "i", "Ljava/util/ListIterator;").await?;
        jvm.invoke_virtual(&iterator, "nextIndex", "()I", ()).await
    }

    async fn previous_index(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let iterator: ClassInstanceRef<Object> = jvm.get_field(&this, "i", "Ljava/util/ListIterator;").await?;
        jvm.invoke_virtual(&iterator, "previousIndex", "()I", ()).await
    }

    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<()> {
        Err(jvm
            .exception("java/lang/UnsupportedOperationException", "unmodifiable list iterator")
            .await)
    }

    async fn set(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<()> {
        Err(jvm
            .exception("java/lang/UnsupportedOperationException", "unmodifiable list iterator")
            .await)
    }

    async fn add(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<()> {
        Err(jvm
            .exception("java/lang/UnsupportedOperationException", "unmodifiable list iterator")
            .await)
    }
}
