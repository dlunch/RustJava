use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

// class java.lang.IllegalThreadStateException
pub struct IllegalThreadStateException;

impl IllegalThreadStateException {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/IllegalThreadStateException",
            parent_class: Some("java/lang/IllegalArgumentException"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_with_message, Default::default()),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.IllegalThreadStateException::<init>({this:?})");
        jvm.invoke_special(&this, "java/lang/IllegalArgumentException", "<init>", "()V", ()).await
    }

    async fn init_with_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.IllegalThreadStateException::<init>({this:?}, {message:?})");
        jvm.invoke_special(&this, "java/lang/IllegalArgumentException", "<init>", "(Ljava/lang/String;)V", (message,))
            .await
    }
}
