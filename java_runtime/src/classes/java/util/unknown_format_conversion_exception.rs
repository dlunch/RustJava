use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

// public class java.util.UnknownFormatConversionException
pub struct UnknownFormatConversionException;

impl UnknownFormatConversionException {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/UnknownFormatConversionException",
            parent_class: Some("java/util/IllegalFormatException"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getConversion", "()Ljava/lang/String;", Self::get_conversion, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getMessage", "()Ljava/lang/String;", Self::get_message, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![JavaFieldProto::new("conversion", "Ljava/lang/String;", FieldAccessFlags::PRIVATE)],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, conversion: ClassInstanceRef<String>) -> Result<()> {
        if conversion.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "conversion is null").await);
        }
        let _: () = jvm.invoke_special(&this, "java/util/IllegalFormatException", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "conversion", "Ljava/lang/String;", conversion).await
    }

    async fn get_conversion(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "conversion", "Ljava/lang/String;").await
    }

    async fn get_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let conversion: ClassInstanceRef<String> = jvm.get_field(&this, "conversion", "Ljava/lang/String;").await?;
        JavaLangString::from_rust_string(
            jvm,
            &alloc::format!("Conversion = '{}'", JavaLangString::to_rust_string(jvm, &conversion).await?),
        )
        .await
        .map(Into::into)
    }
}
