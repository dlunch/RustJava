use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

// class java.util.Collections$UnmodifiableSortedSet
pub struct CollectionsUnmodifiableSortedSet;

impl CollectionsUnmodifiableSortedSet {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Collections$UnmodifiableSortedSet",
            parent_class: Some("java/util/Collections$UnmodifiableSet"),
            interfaces: vec!["java/util/SortedSet", "java/io/Serializable"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/util/SortedSet;)V", Self::init, MethodAccessFlags::empty()),
                JavaMethodProto::new("comparator", "()Ljava/util/Comparator;", Self::comparator, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("first", "()Ljava/lang/Object;", Self::first, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("last", "()Ljava/lang/Object;", Self::last, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "subSet",
                    "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedSet;",
                    Self::sub_set,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "headSet",
                    "(Ljava/lang/Object;)Ljava/util/SortedSet;",
                    Self::head_set,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "tailSet",
                    "(Ljava/lang/Object;)Ljava/util/SortedSet;",
                    Self::tail_set,
                    MethodAccessFlags::PUBLIC,
                ),
            ],
            fields: vec![JavaFieldProto::new(
                "ss",
                "Ljava/util/SortedSet;",
                FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL,
            )],
            access_flags: ClassAccessFlags::empty(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, set: ClassInstanceRef<Object>) -> Result<()> {
        let _: () = jvm
            .invoke_special(
                &this,
                "java/util/Collections$UnmodifiableSet",
                "<init>",
                "(Ljava/util/Set;)V",
                (set.clone(),),
            )
            .await?;
        jvm.put_field(&mut this, "ss", "Ljava/util/SortedSet;", set).await
    }

    async fn comparator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let set: ClassInstanceRef<Object> = jvm.get_field(&this, "ss", "Ljava/util/SortedSet;").await?;
        jvm.invoke_virtual(&set, &set.class_definition().name(), "comparator", "()Ljava/util/Comparator;", ())
            .await
    }

    async fn first(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let set: ClassInstanceRef<Object> = jvm.get_field(&this, "ss", "Ljava/util/SortedSet;").await?;
        jvm.invoke_virtual(&set, &set.class_definition().name(), "first", "()Ljava/lang/Object;", ())
            .await
    }

    async fn last(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let set: ClassInstanceRef<Object> = jvm.get_field(&this, "ss", "Ljava/util/SortedSet;").await?;
        jvm.invoke_virtual(&set, &set.class_definition().name(), "last", "()Ljava/lang/Object;", ())
            .await
    }

    async fn sub_set(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        from: ClassInstanceRef<Object>,
        to: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        let set: ClassInstanceRef<Object> = jvm.get_field(&this, "ss", "Ljava/util/SortedSet;").await?;
        let range: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &set,
                &set.class_definition().name(),
                "subSet",
                "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedSet;",
                (from, to),
            )
            .await?;
        Ok(jvm
            .new_class("java/util/Collections$UnmodifiableSortedSet", "(Ljava/util/SortedSet;)V", (range,))
            .await?
            .into())
    }

    async fn head_set(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        to: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        let set: ClassInstanceRef<Object> = jvm.get_field(&this, "ss", "Ljava/util/SortedSet;").await?;
        let range: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &set,
                &set.class_definition().name(),
                "headSet",
                "(Ljava/lang/Object;)Ljava/util/SortedSet;",
                (to,),
            )
            .await?;
        Ok(jvm
            .new_class("java/util/Collections$UnmodifiableSortedSet", "(Ljava/util/SortedSet;)V", (range,))
            .await?
            .into())
    }

    async fn tail_set(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        from: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        let set: ClassInstanceRef<Object> = jvm.get_field(&this, "ss", "Ljava/util/SortedSet;").await?;
        let range: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &set,
                &set.class_definition().name(),
                "tailSet",
                "(Ljava/lang/Object;)Ljava/util/SortedSet;",
                (from,),
            )
            .await?;
        Ok(jvm
            .new_class("java/util/Collections$UnmodifiableSortedSet", "(Ljava/util/SortedSet;)V", (range,))
            .await?
            .into())
    }
}
