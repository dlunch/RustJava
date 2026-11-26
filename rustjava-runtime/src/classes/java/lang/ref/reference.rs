use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::Object};

// abstract class java.lang.ref.Reference
#[java_class(name = "java/lang/ref/Reference", public, abstract)]
pub struct Reference {
    // the garbage collector does not trace this field and clears it once the referent is otherwise unreachable
    #[field(private)]
    referent: ClassInstanceRef<Object>,
}

#[java_methods]
impl Reference {
    #[method(constructor)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, referent: ClassInstanceRef<Object>) -> Result<()> {
        tracing::debug!("java.lang.ref.Reference::<init>({this:?}, {referent:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/lang/ref/Reference", "referent", "Ljava/lang/Object;", referent)
            .await?;

        Ok(())
    }

    #[method(public)]
    async fn get(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.lang.ref.Reference::get({this:?})");

        jvm.get_field(&this, "java/lang/ref/Reference", "referent", "Ljava/lang/Object;").await
    }

    #[method(public)]
    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.ref.Reference::clear({this:?})");

        jvm.put_field(
            &mut this,
            "java/lang/ref/Reference",
            "referent",
            "Ljava/lang/Object;",
            ClassInstanceRef::<Object>::new(None),
        )
        .await
    }
}
