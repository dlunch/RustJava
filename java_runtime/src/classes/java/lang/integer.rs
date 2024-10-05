use alloc::{format, string::ToString, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::MethodAccessFlags;
use jvm::{runtime::JavaLangString, ClassInstanceRef, Jvm, Result};

use crate::{classes::java::lang::String, RuntimeClassProto, RuntimeContext};

// class java.lang.Integer
pub struct Integer;

impl Integer {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Integer",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(I)V", Self::init, Default::default()),
                JavaMethodProto::new("parseInt", "(Ljava/lang/String;)I", Self::parse_int, MethodAccessFlags::STATIC),
                JavaMethodProto::new("valueOf", "(I)Ljava/lang/Integer;", Self::value_of, MethodAccessFlags::STATIC),
                JavaMethodProto::new("intValue", "()I", Self::int_value, Default::default()),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, Default::default()),
                JavaMethodProto::new("toString", "(I)Ljava/lang/String;", Self::to_string_static, MethodAccessFlags::STATIC),
                JavaMethodProto::new("toHexString", "(I)Ljava/lang/String;", Self::to_hex_string, MethodAccessFlags::STATIC),
            ],
            fields: vec![JavaFieldProto::new("value", "I", Default::default())],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        tracing::debug!("java.lang.Integer::<init>({:?}, {:?})", &this, value);

        jvm.put_field(&mut this, "value", "I", value).await?;

        Ok(())
    }

    async fn int_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.lang.Integer::intValue({:?})", &this);

        let value = jvm.get_field(&this, "value", "I").await?;

        Ok(value)
    }

    async fn value_of(jvm: &Jvm, _: &mut RuntimeContext, value: i32) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.Integer::valueOf({:?})", value);

        let instance = jvm.new_class("java/lang/Integer", "(I)V", (value,)).await?;

        Ok(instance.into())
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.Integer::toString({:?})", &this);

        let value: i32 = jvm.get_field(&this, "value", "I").await?;

        let string = JavaLangString::from_rust_string(jvm, &value.to_string()).await?;

        Ok(string.into())
    }

    async fn to_string_static(jvm: &Jvm, _: &mut RuntimeContext, value: i32) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.Integer::toString({:?})", value);

        let string = JavaLangString::from_rust_string(jvm, &value.to_string()).await?;

        Ok(string.into())
    }

    async fn parse_int(jvm: &Jvm, _: &mut RuntimeContext, s: ClassInstanceRef<String>) -> Result<i32> {
        tracing::debug!("java.lang.Integer::parseInt({:?})", &s);

        let s = JavaLangString::to_rust_string(jvm, &s).await?;

        Ok(s.parse().unwrap())
    }

    async fn to_hex_string(jvm: &Jvm, _: &mut RuntimeContext, value: i32) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.Integer::toHexString({:?})", value);

        let string = JavaLangString::from_rust_string(jvm, &format!("{:x}", value)).await?;

        Ok(string.into())
    }
}

#[cfg(test)]
mod test {
    use jvm::{runtime::JavaLangString, Result};

    use crate::test::test_jvm;

    #[tokio::test]
    async fn test_parse_int() -> Result<()> {
        let jvm = test_jvm().await?;

        let string = JavaLangString::from_rust_string(&jvm, "42").await?;
        assert_eq!(
            42i32,
            jvm.invoke_static("java/lang/Integer", "parseInt", "(Ljava/lang/String;)I", (string,))
                .await?
        );

        Ok(())
    }
}
