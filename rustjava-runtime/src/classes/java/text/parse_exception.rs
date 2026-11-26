use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::String};

// public class java.text.ParseException
#[java_class(name = "java/text/ParseException", extends = "java/lang/Exception", public)]
pub struct ParseException {
    #[field(private)]
    errorOffset: i32,
}

#[java_methods]
impl ParseException {
    #[method(constructor, public)]
    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        message: ClassInstanceRef<String>,
        error_offset: i32,
    ) -> Result<()> {
        let _: () = jvm
            .invoke_special(&this, "java/lang/Exception", "<init>", "(Ljava/lang/String;)V", (message,))
            .await?;
        jvm.put_field(&mut this, "java/text/ParseException", "errorOffset", "I", error_offset)
            .await
    }

    #[method(public)]
    async fn get_error_offset(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "java/text/ParseException", "errorOffset", "I").await
    }
}
