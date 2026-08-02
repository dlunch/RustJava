use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

// public class java.util.MissingFormatWidthException
pub struct MissingFormatWidthException;

impl MissingFormatWidthException {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/MissingFormatWidthException",
            parent_class: Some("java/util/IllegalFormatException"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "getFormatSpecifier",
                    "()Ljava/lang/String;",
                    Self::get_format_specifier,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "getMessage",
                    "()Ljava/lang/String;",
                    Self::get_format_specifier,
                    MethodAccessFlags::PUBLIC,
                ),
            ],
            fields: vec![JavaFieldProto::new("specifier", "Ljava/lang/String;", FieldAccessFlags::PRIVATE)],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, specifier: ClassInstanceRef<String>) -> Result<()> {
        if specifier.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "specifier is null").await);
        }
        let _: () = jvm.invoke_special(&this, "java/util/IllegalFormatException", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "specifier", "Ljava/lang/String;", specifier).await
    }

    async fn get_format_specifier(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "specifier", "Ljava/lang/String;").await
    }
}
