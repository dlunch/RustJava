use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::String};

// class java.lang.OutOfMemoryError
#[java_class(name = "java/lang/OutOfMemoryError", extends = "java/lang/VirtualMachineError", public)]
pub struct OutOfMemoryError;

#[java_methods]
impl OutOfMemoryError {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.OutOfMemoryError::<init>({this:?})");
        jvm.invoke_special(&this, "java/lang/VirtualMachineError", "<init>", "()V", ()).await
    }

    #[method(constructor, public)]
    async fn init_with_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.OutOfMemoryError::<init>({this:?}, {message:?})");
        jvm.invoke_special(&this, "java/lang/VirtualMachineError", "<init>", "(Ljava/lang/String;)V", (message,))
            .await
    }
}
