use alloc::vec;

use java_class_proto::{JavaMethodProto, JavaResult};
use jvm::{ClassInstanceRef, Jvm};

use crate::{RuntimeClassProto, RuntimeContext};

// class java.lang.Object
pub struct Object {}

impl Object {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: None,
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getClass", "()Ljava/lang/Class;", Self::get_class, Default::default()),
            ],
            fields: vec![],
        }
    }

    async fn init(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> JavaResult<()> {
        tracing::debug!("java.lang.Object::<init>({:?})", &this);

        Ok(())
    }

    async fn get_class(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> JavaResult<ClassInstanceRef<Self>> {
        tracing::warn!("stub java.lang.Object::get_class({:?})", &this);

        let result = jvm.new_class("java/lang/Class", "()V", []).await?;

        Ok(result.into())
    }
}
