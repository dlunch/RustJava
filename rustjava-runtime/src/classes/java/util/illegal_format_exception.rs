use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::RuntimeContext;

// public class java.util.IllegalFormatException
#[java_class(name = "java/util/IllegalFormatException", extends = "java/lang/IllegalArgumentException", public)]
pub struct IllegalFormatException;

#[java_methods]
impl IllegalFormatException {
    #[method(constructor)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/IllegalArgumentException", "<init>", "()V", ()).await
    }
}
