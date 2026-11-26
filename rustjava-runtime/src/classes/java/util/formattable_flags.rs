use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::RuntimeContext;

// public class java.util.FormattableFlags
#[java_class(name = "java/util/FormattableFlags", extends = "java/lang/Object", public)]
pub struct FormattableFlags {
    #[field(public, static, final)]
    LEFT_JUSTIFY: i32,
    #[field(public, static, final)]
    UPPERCASE: i32,
    #[field(public, static, final)]
    ALTERNATE: i32,
}

#[java_methods]
impl FormattableFlags {
    #[method(static_initializer)]
    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        jvm.put_static_field("java/util/FormattableFlags", "LEFT_JUSTIFY", "I", 1).await?;
        jvm.put_static_field("java/util/FormattableFlags", "UPPERCASE", "I", 2).await?;
        jvm.put_static_field("java/util/FormattableFlags", "ALTERNATE", "I", 4).await
    }

    #[method(constructor, private)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }
}
