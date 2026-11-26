use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::RuntimeContext;

// abstract class java.util.Dictionary
#[java_class(name = "java/util/Dictionary", abstract)]
pub struct Dictionary;

#[java_methods]
impl Dictionary {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Dictionary::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }
}
