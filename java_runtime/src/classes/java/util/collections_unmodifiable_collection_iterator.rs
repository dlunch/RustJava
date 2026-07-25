use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

// class java.util.Collections$UnmodifiableCollection$1
pub struct CollectionsUnmodifiableCollectionIterator;

impl CollectionsUnmodifiableCollectionIterator {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Collections$UnmodifiableCollection$1",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/util/Iterator"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/util/Iterator;)V", Self::init, Default::default()),
                JavaMethodProto::new("hasNext", "()Z", Self::has_next, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("next", "()Ljava/lang/Object;", Self::next, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("remove", "()V", Self::remove, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![JavaFieldProto::new(
                "i",
                "Ljava/util/Iterator;",
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
        jvm.put_field(&mut this, "i", "Ljava/util/Iterator;", iterator).await
    }

    async fn has_next(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let iterator: ClassInstanceRef<Object> = jvm.get_field(&this, "i", "Ljava/util/Iterator;").await?;
        jvm.invoke_virtual(&iterator, "hasNext", "()Z", ()).await
    }

    async fn next(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let iterator: ClassInstanceRef<Object> = jvm.get_field(&this, "i", "Ljava/util/Iterator;").await?;
        jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await
    }

    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<()> {
        Err(jvm.exception("java/lang/UnsupportedOperationException", "unmodifiable iterator").await)
    }
}
