use alloc::{format, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::lang::{Class, String},
};

// public class java.util.IllegalFormatConversionException
pub struct IllegalFormatConversionException;

impl IllegalFormatConversionException {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/IllegalFormatConversionException",
            parent_class: Some("java/util/IllegalFormatException"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(CLjava/lang/Class;)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getConversion", "()C", Self::get_conversion, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "getArgumentClass",
                    "()Ljava/lang/Class;",
                    Self::get_argument_class,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("getMessage", "()Ljava/lang/String;", Self::get_message, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("conversion", "C", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("argumentClass", "Ljava/lang/Class;", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        conversion: JavaChar,
        argument_class: ClassInstanceRef<Class>,
    ) -> Result<()> {
        if argument_class.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "argumentClass is null").await);
        }
        let _: () = jvm.invoke_special(&this, "java/util/IllegalFormatException", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "conversion", "C", conversion).await?;
        jvm.put_field(&mut this, "argumentClass", "Ljava/lang/Class;", argument_class).await
    }

    async fn get_conversion(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<JavaChar> {
        jvm.get_field(&this, "conversion", "C").await
    }

    async fn get_argument_class(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Class>> {
        jvm.get_field(&this, "argumentClass", "Ljava/lang/Class;").await
    }

    async fn get_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let conversion: JavaChar = jvm.get_field(&this, "conversion", "C").await?;
        let argument_class: ClassInstanceRef<Class> = jvm.get_field(&this, "argumentClass", "Ljava/lang/Class;").await?;
        let class_name: ClassInstanceRef<String> = jvm.invoke_virtual(&argument_class, "getName", "()Ljava/lang/String;", ()).await?;
        Ok(JavaLangString::from_rust_string(
            jvm,
            &format!(
                "{} != {}",
                char::from_u32(conversion as u32).unwrap_or('\u{fffd}'),
                JavaLangString::to_rust_string(jvm, &class_name).await?
            ),
        )
        .await?
        .into())
    }
}
