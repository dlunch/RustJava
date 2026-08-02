use alloc::{string::ToString, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

// public class java.util.IllegalFormatWidthException
pub struct IllegalFormatWidthException;

impl IllegalFormatWidthException {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/IllegalFormatWidthException",
            parent_class: Some("java/util/IllegalFormatException"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(I)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getWidth", "()I", Self::get_width, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getMessage", "()Ljava/lang/String;", Self::get_message, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![JavaFieldProto::new("width", "I", FieldAccessFlags::PRIVATE)],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, width: i32) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/util/IllegalFormatException", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "width", "I", width).await
    }

    async fn get_width(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "width", "I").await
    }

    async fn get_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let width: i32 = jvm.get_field(&this, "width", "I").await?;
        Ok(JavaLangString::from_rust_string(jvm, &width.to_string()).await?.into())
    }
}
