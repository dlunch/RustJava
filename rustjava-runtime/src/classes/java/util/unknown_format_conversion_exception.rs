use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::String};

// public class java.util.UnknownFormatConversionException
#[java_class(name = "java/util/UnknownFormatConversionException", extends = "java/util/IllegalFormatException", public)]
pub struct UnknownFormatConversionException {
    #[field(private)]
    conversion: ClassInstanceRef<String>,
}

#[java_methods]
impl UnknownFormatConversionException {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, conversion: ClassInstanceRef<String>) -> Result<()> {
        if conversion.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "conversion is null").await);
        }
        let _: () = jvm.invoke_special(&this, "java/util/IllegalFormatException", "<init>", "()V", ()).await?;
        jvm.put_field(
            &mut this,
            "java/util/UnknownFormatConversionException",
            "conversion",
            "Ljava/lang/String;",
            conversion,
        )
        .await
    }

    #[method(public)]
    async fn get_conversion(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "java/util/UnknownFormatConversionException", "conversion", "Ljava/lang/String;")
            .await
    }

    #[method(public)]
    async fn get_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let conversion: ClassInstanceRef<String> = jvm
            .get_field(&this, "java/util/UnknownFormatConversionException", "conversion", "Ljava/lang/String;")
            .await?;
        JavaLangString::from_rust_string(
            jvm,
            &alloc::format!("Conversion = '{}'", JavaLangString::to_rust_string(jvm, &conversion).await?),
        )
        .await
        .map(Into::into)
    }
}
