use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::String};

// class java.lang.StackOverflowError
#[java_class(name = "java/lang/StackOverflowError", extends = "java/lang/VirtualMachineError", public)]
pub struct StackOverflowError;

#[java_methods]
impl StackOverflowError {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.StackOverflowError::<init>({this:?})");

        jvm.invoke_special(&this, "java/lang/VirtualMachineError", "<init>", "()V", ()).await
    }

    #[method(constructor, public)]
    async fn init_with_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.StackOverflowError::<init>({this:?}, {message:?})");

        jvm.invoke_special(&this, "java/lang/VirtualMachineError", "<init>", "(Ljava/lang/String;)V", (message,))
            .await
    }
}
