use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

// class java.io.EOFException
pub struct EOFException;

impl EOFException {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/EOFException",
            parent_class: Some("java/io/IOException"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_with_message, Default::default()),
            ],
            fields: vec![],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.EOFException::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/io/IOException", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn init_with_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.io.EOFException::<init>({:?}, {:?})", &this, &message);

        let _: () = jvm
            .invoke_special(&this, "java/io/IOException", "<init>", "(Ljava/lang/String;)V", (message,))
            .await?;

        Ok(())
    }
}
