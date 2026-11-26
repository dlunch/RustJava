use alloc::format;

use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::String};

// public class java.util.IllegalFormatCodePointException
#[java_class(name = "java/util/IllegalFormatCodePointException", extends = "java/util/IllegalFormatException", public)]
pub struct IllegalFormatCodePointException {
    #[field(private)]
    codePoint: i32,
}

#[java_methods]
impl IllegalFormatCodePointException {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, code_point: i32) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/util/IllegalFormatException", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/util/IllegalFormatCodePointException", "codePoint", "I", code_point)
            .await
    }

    #[method(public)]
    async fn get_code_point(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "java/util/IllegalFormatCodePointException", "codePoint", "I").await
    }

    #[method(public)]
    async fn get_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let code_point: i32 = jvm
            .get_field(&this, "java/util/IllegalFormatCodePointException", "codePoint", "I")
            .await?;
        Ok(JavaLangString::from_rust_string(jvm, &format!("Code point = 0x{code_point:x}"))
            .await?
            .into())
    }
}
