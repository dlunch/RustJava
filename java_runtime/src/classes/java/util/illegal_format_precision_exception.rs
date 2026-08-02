use alloc::{string::ToString, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

// public class java.util.IllegalFormatPrecisionException
pub struct IllegalFormatPrecisionException;

impl IllegalFormatPrecisionException {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/IllegalFormatPrecisionException",
            parent_class: Some("java/util/IllegalFormatException"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(I)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getPrecision", "()I", Self::get_precision, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getMessage", "()Ljava/lang/String;", Self::get_message, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![JavaFieldProto::new("precision", "I", FieldAccessFlags::PRIVATE)],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, precision: i32) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/util/IllegalFormatException", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "precision", "I", precision).await
    }

    async fn get_precision(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "precision", "I").await
    }

    async fn get_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let precision: i32 = jvm.get_field(&this, "precision", "I").await?;
        Ok(JavaLangString::from_rust_string(jvm, &precision.to_string()).await?.into())
    }
}
