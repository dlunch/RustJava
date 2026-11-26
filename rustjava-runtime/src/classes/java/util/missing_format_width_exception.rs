use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::String};

// public class java.util.MissingFormatWidthException
#[java_class(name = "java/util/MissingFormatWidthException", extends = "java/util/IllegalFormatException", public)]
pub struct MissingFormatWidthException {
    #[field(private)]
    specifier: ClassInstanceRef<String>,
}

#[java_methods]
impl MissingFormatWidthException {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, specifier: ClassInstanceRef<String>) -> Result<()> {
        if specifier.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "specifier is null").await);
        }
        let _: () = jvm.invoke_special(&this, "java/util/IllegalFormatException", "<init>", "()V", ()).await?;
        jvm.put_field(
            &mut this,
            "java/util/MissingFormatWidthException",
            "specifier",
            "Ljava/lang/String;",
            specifier,
        )
        .await
    }

    #[method(public)]
    #[method(name = "getMessage", public)]
    async fn get_format_specifier(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "java/util/MissingFormatWidthException", "specifier", "Ljava/lang/String;")
            .await
    }
}
