use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{lang::Object, util::Comparator},
};

// interface java.util.SortedSet
#[java_class(name = "java/util/SortedSet", implements("java/util/Set"), public, interface, abstract)]
pub struct SortedSet;

#[java_methods]
impl SortedSet {
    #[method(public)]
    async fn comparator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Comparator>>;

    #[method(public)]
    async fn sub_set(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        from: ClassInstanceRef<Object>,
        to: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Self>>;

    #[method(public)]
    async fn head_set(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        to: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Self>>;

    #[method(public)]
    async fn tail_set(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        from: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Self>>;

    #[method(public)]
    async fn first(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>>;

    #[method(public)]
    async fn last(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>>;
}
