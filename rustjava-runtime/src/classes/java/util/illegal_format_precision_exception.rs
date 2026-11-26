use alloc::string::ToString;

use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::String};

// public class java.util.IllegalFormatPrecisionException
#[java_class(name = "java/util/IllegalFormatPrecisionException", extends = "java/util/IllegalFormatException", public)]
pub struct IllegalFormatPrecisionException {
    #[field(private)]
    precision: i32,
}

#[java_methods]
impl IllegalFormatPrecisionException {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, precision: i32) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/util/IllegalFormatException", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/util/IllegalFormatPrecisionException", "precision", "I", precision)
            .await
    }

    #[method(public)]
    async fn get_precision(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "java/util/IllegalFormatPrecisionException", "precision", "I").await
    }

    #[method(public)]
    async fn get_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let precision: i32 = jvm
            .get_field(&this, "java/util/IllegalFormatPrecisionException", "precision", "I")
            .await?;
        Ok(JavaLangString::from_rust_string(jvm, &precision.to_string()).await?.into())
    }
}
