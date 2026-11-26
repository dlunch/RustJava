use alloc::format;

use jvm::{ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::String};

// public class java.util.FormatFlagsConversionMismatchException
#[java_class(
    name = "java/util/FormatFlagsConversionMismatchException",
    extends = "java/util/IllegalFormatException",
    public
)]
pub struct FormatFlagsConversionMismatchException {
    #[field(private)]
    flags: ClassInstanceRef<String>,
    #[field(private)]
    conversion: JavaChar,
}

#[java_methods]
impl FormatFlagsConversionMismatchException {
    #[method(constructor, public)]
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

    #[method(public)]
    async fn get_flags(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "java/util/FormatFlagsConversionMismatchException", "flags", "Ljava/lang/String;")
            .await
    }

    #[method(public)]
    async fn get_conversion(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<JavaChar> {
        jvm.get_field(&this, "java/util/FormatFlagsConversionMismatchException", "conversion", "C")
            .await
    }

    #[method(public)]
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
