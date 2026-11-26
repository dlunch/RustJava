use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::RuntimeContext;

// public class java.util.FormatterClosedException
#[java_class(name = "java/util/FormatterClosedException", extends = "java/lang/IllegalStateException", public)]
pub struct FormatterClosedException;

#[java_methods]
impl FormatterClosedException {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/IllegalStateException", "<init>", "()V", ()).await
    }
}
