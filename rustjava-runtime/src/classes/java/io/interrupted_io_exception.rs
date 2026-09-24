use alloc::vec;

use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm_types::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

// class java.io.InterruptedIOException
pub struct InterruptedIOException;

impl InterruptedIOException {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/InterruptedIOException",
            parent_class: Some("java/io/IOException"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_with_message, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![JavaFieldProto::new("bytesTransferred", "I", FieldAccessFlags::PUBLIC)],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.InterruptedIOException::<init>({this:?})");
        let _: () = jvm.invoke_special(&this, "java/io/IOException", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/io/InterruptedIOException", "bytesTransferred", "I", 0)
            .await
    }

    async fn init_with_message(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.io.InterruptedIOException::<init>({this:?}, {message:?})");
        let _: () = jvm
            .invoke_special(&this, "java/io/IOException", "<init>", "(Ljava/lang/String;)V", (message,))
            .await?;
        jvm.put_field(&mut this, "java/io/InterruptedIOException", "bytesTransferred", "I", 0)
            .await
    }
}
