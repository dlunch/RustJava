use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::String};

// abstract class java.lang.VirtualMachineError
#[java_class(name = "java/lang/VirtualMachineError", extends = "java/lang/Error", abstract)]
pub struct VirtualMachineError;

#[java_methods]
impl VirtualMachineError {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.VirtualMachineError::<init>({this:?})");
        jvm.invoke_special(&this, "java/lang/Error", "<init>", "()V", ()).await
    }

    #[method(constructor, public)]
    async fn init_with_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.VirtualMachineError::<init>({this:?}, {message:?})");
        jvm.invoke_special(&this, "java/lang/Error", "<init>", "(Ljava/lang/String;)V", (message,))
            .await
    }
}
