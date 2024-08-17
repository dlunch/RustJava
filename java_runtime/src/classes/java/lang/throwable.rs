use alloc::{format, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{runtime::JavaLangString, ClassInstanceRef, Jvm, Result};

use crate::{classes::java::lang::String, RuntimeClassProto, RuntimeContext};

// class java.lang.Throwable
pub struct Throwable {}

impl Throwable {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Throwable",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_with_message, Default::default()),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("detailMessage", "Ljava/lang/String;", Default::default())],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.Throwable::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn init_with_message(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.Throwable::<init>({:?}, {:?})", &this, &message);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "detailMessage", "Ljava/lang/String;", message).await?;

        Ok(())
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.Throwable::toString({:?})", &this);

        let class = jvm.invoke_virtual(&this, "getClass", "()Ljava/lang/Class;", ()).await?;
        let class_name = jvm.invoke_virtual(&class, "getName", "()Ljava/lang/String;", ()).await?;

        let message = jvm.get_field(&this, "detailMessage", "Ljava/lang/String;").await?;

        let class_name = JavaLangString::to_rust_string(jvm, &class_name).await?;
        let message = JavaLangString::to_rust_string(jvm, &message).await?;

        let message = if message.is_empty() {
            class_name
        } else {
            format!("{}: {}", class_name, message)
        };

        let message = JavaLangString::from_rust_string(jvm, &message).await?;

        Ok(message.into())
    }
}

#[cfg(test)]
mod test {
    use jvm::{runtime::JavaLangString, Result};

    use crate::test::test_jvm;

    #[tokio::test]
    async fn test_to_string() -> Result<()> {
        let jvm = test_jvm().await?;

        let message = JavaLangString::from_rust_string(&jvm, "test message").await?;

        let throwable = jvm.new_class("java/lang/Throwable", "(Ljava/lang/String;)V", (message,)).await?;
        let to_string = jvm.invoke_virtual(&throwable, "toString", "()Ljava/lang/String;", ()).await?;

        let result = JavaLangString::to_rust_string(&jvm, &to_string).await?;

        assert_eq!(result, "java/lang/Throwable: test message");

        Ok(())
    }
}
