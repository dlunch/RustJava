use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::ClassAccessFlags;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

// abstract class java.util.AbstractMap
pub struct AbstractMap;

impl AbstractMap {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/AbstractMap",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/util/Map"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new_abstract("size", "()I", Default::default()),
                JavaMethodProto::new("isEmpty", "()Z", Self::is_empty, Default::default()),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::ABSTRACT,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.AbstractMap::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn is_empty(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.AbstractMap::isEmpty({this:?})");

        let size: i32 = jvm.invoke_virtual(&this, "size", "()I", ()).await?;

        Ok(size == 0)
    }
}
