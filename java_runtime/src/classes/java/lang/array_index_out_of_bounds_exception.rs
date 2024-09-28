use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{classes::java::lang::String, RuntimeClassProto, RuntimeContext};

// class java.lang.ArrayIndexOutOfBoundsException
pub struct ArrayIndexOutOfBoundsException {}

impl ArrayIndexOutOfBoundsException {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/ArrayIndexOutOfBoundsException",
            parent_class: Some("java/lang/IndexOutOfBoundsException"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_with_message, Default::default()),
            ],
            fields: vec![],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.ArrayIndexOutOfBoundsException::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/lang/RuntimeException", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn init_with_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.ArrayIndexOutOfBoundsException::<init>({:?}, {:?})", &this, &message);

        let _: () = jvm
            .invoke_special(
                &this,
                "java/lang/IndexOutOfBoundsException",
                "<init>",
                "(Ljava/lang/String;)V",
                (message,),
            )
            .await?;

        Ok(())
    }
}
