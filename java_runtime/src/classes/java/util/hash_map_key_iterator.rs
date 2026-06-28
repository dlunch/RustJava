use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

// class java.util.HashMap$KeyIterator
pub struct HashMapKeyIterator;

impl HashMapKeyIterator {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/HashMap$KeyIterator",
            parent_class: Some("java/util/HashMap$HashIterator"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new("<init>", "([Ljava/lang/Object;)V", Self::init, Default::default())],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, elements: ClassInstanceRef<Array<Object>>) -> Result<()> {
        tracing::debug!("java.util.HashMap$KeyIterator::<init>({this:?}, {elements:?})");

        jvm.invoke_special(&this, "java/util/HashMap$HashIterator", "<init>", "([Ljava/lang/Object;)V", (elements,))
            .await
    }
}
