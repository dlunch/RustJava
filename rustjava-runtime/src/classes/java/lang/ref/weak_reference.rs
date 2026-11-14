use alloc::vec;

use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::JavaMethodProto;
use jvm_types::{ClassAccessFlags, MethodAccessFlags};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

// class java.lang.ref.WeakReference
pub struct WeakReference;

impl WeakReference {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/ref/WeakReference",
            parent_class: Some("java/lang/ref/Reference"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new(
                "<init>",
                "(Ljava/lang/Object;)V",
                Self::init,
                MethodAccessFlags::PUBLIC,
            )],
            fields: vec![],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, referent: ClassInstanceRef<Object>) -> Result<()> {
        tracing::debug!("java.lang.ref.WeakReference::<init>({this:?}, {referent:?})");

        jvm.invoke_special(&this, "java/lang/ref/Reference", "<init>", "(Ljava/lang/Object;)V", (referent,))
            .await
    }
}
