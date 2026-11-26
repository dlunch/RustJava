use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::Object};

// class java.util.Collections$UnmodifiableSet
#[java_class(
    name = "java/util/Collections$UnmodifiableSet",
    extends = "java/util/Collections$UnmodifiableCollection",
    implements("java/util/Set", "java/io/Serializable")
)]
pub struct CollectionsUnmodifiableSet;

#[java_methods]
impl CollectionsUnmodifiableSet {
    #[method(constructor, descriptor = "(Ljava/util/Set;)V")]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, set: ClassInstanceRef<Object>) -> Result<()> {
        jvm.invoke_special(
            &this,
            "java/util/Collections$UnmodifiableCollection",
            "<init>",
            "(Ljava/util/Collection;)V",
            (set,),
        )
        .await
    }

    #[method(public)]
    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if !other.is_null() && this.identity() == other.identity() {
            return Ok(true);
        }
        let set: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableSet", "c", "Ljava/util/Collection;")
            .await?;
        jvm.invoke_virtual(&set, "java/lang/Object", "equals", "(Ljava/lang/Object;)Z", (other,))
            .await
    }

    #[method(public)]
    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let set: ClassInstanceRef<Object> = jvm
            .get_field(&this, "java/util/Collections$UnmodifiableSet", "c", "Ljava/util/Collection;")
            .await?;
        jvm.invoke_virtual(&set, "java/lang/Object", "hashCode", "()I", ()).await
    }
}
