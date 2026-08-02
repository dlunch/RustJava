use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

// public class java.util.DuplicateFormatFlagsException
pub struct DuplicateFormatFlagsException;

impl DuplicateFormatFlagsException {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/DuplicateFormatFlagsException",
            parent_class: Some("java/util/IllegalFormatException"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getFlags", "()Ljava/lang/String;", Self::get_flags, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getMessage", "()Ljava/lang/String;", Self::get_message, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![JavaFieldProto::new("flags", "Ljava/lang/String;", FieldAccessFlags::PRIVATE)],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, flags: ClassInstanceRef<String>) -> Result<()> {
        if flags.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "flags is null").await);
        }
        let _: () = jvm.invoke_special(&this, "java/util/IllegalFormatException", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "flags", "Ljava/lang/String;", flags).await
    }

    async fn get_flags(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "flags", "Ljava/lang/String;").await
    }

    async fn get_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let flags: ClassInstanceRef<String> = jvm.get_field(&this, "flags", "Ljava/lang/String;").await?;
        JavaLangString::from_rust_string(jvm, &alloc::format!("Flags = '{}'", JavaLangString::to_rust_string(jvm, &flags).await?))
            .await
            .map(Into::into)
    }
}
