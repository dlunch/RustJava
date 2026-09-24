use alloc::{format, vec};

use jvm::{ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm_types::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

// public class java.util.FormatFlagsConversionMismatchException
pub struct FormatFlagsConversionMismatchException;

impl FormatFlagsConversionMismatchException {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/FormatFlagsConversionMismatchException",
            parent_class: Some("java/util/IllegalFormatException"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/String;C)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getFlags", "()Ljava/lang/String;", Self::get_flags, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getConversion", "()C", Self::get_conversion, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getMessage", "()Ljava/lang/String;", Self::get_message, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("flags", "Ljava/lang/String;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("conversion", "C", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        flags: ClassInstanceRef<String>,
        conversion: JavaChar,
    ) -> Result<()> {
        if flags.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "flags is null").await);
        }
        let _: () = jvm.invoke_special(&this, "java/util/IllegalFormatException", "<init>", "()V", ()).await?;
        jvm.put_field(
            &mut this,
            "java/util/FormatFlagsConversionMismatchException",
            "flags",
            "Ljava/lang/String;",
            flags,
        )
        .await?;
        jvm.put_field(
            &mut this,
            "java/util/FormatFlagsConversionMismatchException",
            "conversion",
            "C",
            conversion,
        )
        .await
    }

    async fn get_flags(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "java/util/FormatFlagsConversionMismatchException", "flags", "Ljava/lang/String;")
            .await
    }

    async fn get_conversion(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<JavaChar> {
        jvm.get_field(&this, "java/util/FormatFlagsConversionMismatchException", "conversion", "C")
            .await
    }

    async fn get_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let flags: ClassInstanceRef<String> = jvm
            .get_field(&this, "java/util/FormatFlagsConversionMismatchException", "flags", "Ljava/lang/String;")
            .await?;
        let conversion: JavaChar = jvm
            .get_field(&this, "java/util/FormatFlagsConversionMismatchException", "conversion", "C")
            .await?;
        Ok(JavaLangString::from_rust_string(
            jvm,
            &format!(
                "Conversion = {}, Flags = {}",
                char::from_u32(conversion as u32).unwrap_or('\u{fffd}'),
                JavaLangString::to_rust_string(jvm, &flags).await?
            ),
        )
        .await?
        .into())
    }
}
