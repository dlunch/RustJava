use alloc::vec;

use jvm::{Array, ClassInstanceRef, Jvm, Result};
use jvm_class_proto::JavaMethodProto;
use jvm_types::{ClassAccessFlags, MethodAccessFlags};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

use super::HashMap;

// class java.util.HashMap$KeyIterator
pub struct HashMapKeyIterator;

impl HashMapKeyIterator {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/HashMap$KeyIterator",
            parent_class: Some("java/util/HashMap$HashIterator"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/util/HashMap;[Ljava/lang/Object;)V",
                    Self::init,
                    MethodAccessFlags::empty(),
                ),
                JavaMethodProto::new("next", "()Ljava/lang/Object;", Self::next, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::FINAL,
        }
    }

    async fn next(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let entry = jvm
            .invoke_special(&this, "java/util/HashMap$HashIterator", "next", "()Ljava/lang/Object;", ())
            .await?;
        jvm.get_field(&entry, "java/util/HashMap$Entry", "key", "Ljava/lang/Object;").await
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        map: ClassInstanceRef<HashMap>,
        elements: ClassInstanceRef<Array<Object>>,
    ) -> Result<()> {
        tracing::debug!("java.util.HashMap$KeyIterator::<init>({this:?}, {elements:?})");

        jvm.invoke_special(
            &this,
            "java/util/HashMap$HashIterator",
            "<init>",
            "(Ljava/util/HashMap;[Ljava/lang/Object;)V",
            (map, elements),
        )
        .await
    }
}
