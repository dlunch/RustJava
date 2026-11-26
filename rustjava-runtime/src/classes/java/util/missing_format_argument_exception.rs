use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::String};

// public class java.util.MissingFormatArgumentException
#[java_class(name = "java/util/MissingFormatArgumentException", extends = "java/util/IllegalFormatException", public)]
pub struct MissingFormatArgumentException {
    #[field(private)]
    specifier: ClassInstanceRef<String>,
}

#[java_methods]
impl MissingFormatArgumentException {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, specifier: ClassInstanceRef<String>) -> Result<()> {
        if specifier.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "specifier is null").await);
        }
        let _: () = jvm.invoke_special(&this, "java/util/IllegalFormatException", "<init>", "()V", ()).await?;
        jvm.put_field(
            &mut this,
            "java/util/MissingFormatArgumentException",
            "specifier",
            "Ljava/lang/String;",
            specifier,
        )
        .await
    }

    #[method(public)]
    async fn get_format_specifier(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "java/util/MissingFormatArgumentException", "specifier", "Ljava/lang/String;")
            .await
    }

    #[method(public)]
    async fn get_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let specifier: ClassInstanceRef<String> = jvm
            .get_field(&this, "java/util/MissingFormatArgumentException", "specifier", "Ljava/lang/String;")
            .await?;
        JavaLangString::from_rust_string(
            jvm,
            &alloc::format!("Format specifier '{}'", JavaLangString::to_rust_string(jvm, &specifier).await?),
        )
        .await
        .map(Into::into)
    }
}
