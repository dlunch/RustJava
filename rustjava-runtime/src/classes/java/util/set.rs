use jvm::{Array, ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{
        lang::Object,
        util::{Collection, Iterator},
    },
};

// interface java.util.Set
#[java_class(name = "java/util/Set", implements("java/util/Collection"), public, interface, abstract)]
pub struct Set;

#[java_methods]
impl Set {
    #[method(public)]
    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32>;

    #[method(public)]
    async fn is_empty(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool>;

    #[method(public)]
    async fn contains(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool>;

    #[method(public)]
    async fn iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Iterator>>;

    #[method(public)]
    async fn to_array(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Array<Object>>>;

    #[method(name = "toArray", public)]
    async fn to_typed_array(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        destination: ClassInstanceRef<Array<Object>>,
    ) -> Result<ClassInstanceRef<Array<Object>>>;

    #[method(public)]
    async fn add(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool>;

    #[method(public)]
    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<bool>;

    #[method(public)]
    async fn contains_all(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, collection: ClassInstanceRef<Collection>) -> Result<bool>;

    #[method(public)]
    async fn add_all(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, collection: ClassInstanceRef<Collection>) -> Result<bool>;

    #[method(public)]
    async fn retain_all(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, collection: ClassInstanceRef<Collection>) -> Result<bool>;

    #[method(public)]
    async fn remove_all(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, collection: ClassInstanceRef<Collection>) -> Result<bool>;

    #[method(public)]
    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()>;

    #[method(public)]
    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool>;

    #[method(public)]
    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32>;
}
