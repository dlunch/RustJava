use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::Object};

// interface java.util.ListIterator
#[java_class(name = "java/util/ListIterator", implements("java/util/Iterator"), public, interface, abstract)]
pub struct ListIterator;

#[java_methods]
impl ListIterator {
    #[method(public)]
    async fn has_next(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool>;

    #[method(public)]
    async fn next(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>>;

    #[method(public)]
    async fn has_previous(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool>;

    #[method(public)]
    async fn previous(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>>;

    #[method(public)]
    async fn next_index(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32>;

    #[method(public)]
    async fn previous_index(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32>;

    #[method(public)]
    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()>;

    #[method(public)]
    async fn set(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<()>;

    #[method(public)]
    async fn add(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<()>;
}
