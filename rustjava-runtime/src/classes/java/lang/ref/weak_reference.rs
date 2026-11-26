use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::Object};

// class java.lang.ref.WeakReference
#[java_class(name = "java/lang/ref/WeakReference", extends = "java/lang/ref/Reference", public)]
pub struct WeakReference;

#[java_methods]
impl WeakReference {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, referent: ClassInstanceRef<Object>) -> Result<()> {
        tracing::debug!("java.lang.ref.WeakReference::<init>({this:?}, {referent:?})");

        jvm.invoke_special(&this, "java/lang/ref/Reference", "<init>", "(Ljava/lang/Object;)V", (referent,))
            .await
    }
}
