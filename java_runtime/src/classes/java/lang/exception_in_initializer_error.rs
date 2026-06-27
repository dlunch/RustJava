use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::lang::{String, Throwable},
};

// class java.lang.ExceptionInInitializerError
pub struct ExceptionInInitializerError;

impl ExceptionInInitializerError {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/ExceptionInInitializerError",
            parent_class: Some("java/lang/LinkageError"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_with_message, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/Throwable;)V", Self::init_with_cause, Default::default()),
                JavaMethodProto::new("getException", "()Ljava/lang/Throwable;", Self::get_exception, Default::default()),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.ExceptionInInitializerError::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/LinkageError", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn init_with_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.ExceptionInInitializerError::<init>({this:?}, {message:?})");

        let _: () = jvm
            .invoke_special(&this, "java/lang/LinkageError", "<init>", "(Ljava/lang/String;)V", (message,))
            .await?;

        Ok(())
    }

    async fn init_with_cause(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, cause: ClassInstanceRef<Throwable>) -> Result<()> {
        tracing::debug!("java.lang.ExceptionInInitializerError::<init>({this:?}, {cause:?})");

        // unlike Throwable(Throwable), this keeps detailMessage null so toString is just the class name
        let _: () = jvm.invoke_special(&this, "java/lang/LinkageError", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "cause", "Ljava/lang/Throwable;", cause).await?;

        Ok(())
    }

    async fn get_exception(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Throwable>> {
        tracing::debug!("java.lang.ExceptionInInitializerError::getException({this:?})");

        jvm.get_field(&this, "cause", "Ljava/lang/Throwable;").await
    }
}
