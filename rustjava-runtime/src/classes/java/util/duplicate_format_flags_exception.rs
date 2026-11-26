use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::String};

// public class java.util.DuplicateFormatFlagsException
#[java_class(name = "java/util/DuplicateFormatFlagsException", extends = "java/util/IllegalFormatException", public)]
pub struct DuplicateFormatFlagsException {
    #[field(private)]
    flags: ClassInstanceRef<String>,
}

#[java_methods]
impl DuplicateFormatFlagsException {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, flags: ClassInstanceRef<String>) -> Result<()> {
        if flags.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "flags is null").await);
        }
        let _: () = jvm.invoke_special(&this, "java/util/IllegalFormatException", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/util/DuplicateFormatFlagsException", "flags", "Ljava/lang/String;", flags)
            .await
    }

    #[method(public)]
    async fn get_flags(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "java/util/DuplicateFormatFlagsException", "flags", "Ljava/lang/String;")
            .await
    }

    #[method(public)]
    async fn get_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let flags: ClassInstanceRef<String> = jvm
            .get_field(&this, "java/util/DuplicateFormatFlagsException", "flags", "Ljava/lang/String;")
            .await?;
        JavaLangString::from_rust_string(jvm, &alloc::format!("Flags = '{}'", JavaLangString::to_rust_string(jvm, &flags).await?))
            .await
            .map(Into::into)
    }
}
