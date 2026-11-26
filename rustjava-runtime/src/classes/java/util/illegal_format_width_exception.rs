use alloc::string::ToString;

use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::String};

// public class java.util.IllegalFormatWidthException
#[java_class(name = "java/util/IllegalFormatWidthException", extends = "java/util/IllegalFormatException", public)]
pub struct IllegalFormatWidthException {
    #[field(private)]
    width: i32,
}

#[java_methods]
impl IllegalFormatWidthException {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, width: i32) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/util/IllegalFormatException", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/util/IllegalFormatWidthException", "width", "I", width)
            .await
    }

    #[method(public)]
    async fn get_width(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "java/util/IllegalFormatWidthException", "width", "I").await
    }

    #[method(public)]
    async fn get_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let width: i32 = jvm.get_field(&this, "java/util/IllegalFormatWidthException", "width", "I").await?;
        Ok(JavaLangString::from_rust_string(jvm, &width.to_string()).await?.into())
    }
}
