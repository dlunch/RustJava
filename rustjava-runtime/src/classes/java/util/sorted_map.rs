use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{lang::Object, util::Comparator},
};

// interface java.util.SortedMap
#[java_class(name = "java/util/SortedMap", implements("java/util/Map"), public, interface, abstract)]
pub struct SortedMap;

#[java_methods]
impl SortedMap {
    #[method(public)]
    async fn comparator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Comparator>>;

    #[method(public)]
    async fn sub_map(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        from_key: ClassInstanceRef<Object>,
        to_key: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Self>>;

    #[method(public)]
    async fn head_map(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        to_key: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Self>>;

    #[method(public)]
    async fn tail_map(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        from_key: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Self>>;

    #[method(public)]
    async fn first_key(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>>;

    #[method(public)]
    async fn last_key(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>>;
}
