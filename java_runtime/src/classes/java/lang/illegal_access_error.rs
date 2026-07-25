use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::{ClassAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

// class java.lang.IllegalAccessError
pub struct IllegalAccessError;

impl IllegalAccessError {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/IllegalAccessError",
            parent_class: Some("java/lang/IncompatibleClassChangeError"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_with_message, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.IllegalAccessError::<init>({this:?})");

        jvm.invoke_special(&this, "java/lang/IncompatibleClassChangeError", "<init>", "()V", ())
            .await
    }

    async fn init_with_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.IllegalAccessError::<init>({this:?}, {message:?})");

        jvm.invoke_special(
            &this,
            "java/lang/IncompatibleClassChangeError",
            "<init>",
            "(Ljava/lang/String;)V",
            (message,),
        )
        .await
    }
}
