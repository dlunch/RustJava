use alloc::{format, vec};

use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::JavaMethodProto;
use jvm_types::{ClassAccessFlags, MethodAccessFlags};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

// class java.lang.ArrayIndexOutOfBoundsException
pub struct ArrayIndexOutOfBoundsException;

impl ArrayIndexOutOfBoundsException {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/ArrayIndexOutOfBoundsException",
            parent_class: Some("java/lang/IndexOutOfBoundsException"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_with_message, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(I)V", Self::init_with_index, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.ArrayIndexOutOfBoundsException::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/RuntimeException", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn init_with_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.ArrayIndexOutOfBoundsException::<init>({this:?}, {message:?})");

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

    async fn init_with_index(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<()> {
        tracing::debug!("java.lang.ArrayIndexOutOfBoundsException::<init>({this:?}, {index:?})");

        let message = JavaLangString::from_rust_string(jvm, &format!("Array index out of range: {index}")).await?;
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
