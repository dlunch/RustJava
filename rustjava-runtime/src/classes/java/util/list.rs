use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{
        lang::Object,
        util::{Collection, ListIterator},
    },
};

// interface java.util.List
#[java_class(name = "java/util/List", implements("java/util/Collection"), public, interface, abstract)]
pub struct List;

#[java_methods]
impl List {
    #[method(public)]
    async fn get(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Object>>;

    #[method(public)]
    async fn set(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        index: i32,
        element: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>>;

    #[method(public)]
    async fn add(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32, element: ClassInstanceRef<Object>) -> Result<()>;

    #[method(public)]
    async fn add_all(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        index: i32,
        collection: ClassInstanceRef<Collection>,
    ) -> Result<bool>;

    #[method(public)]
    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Object>>;

    #[method(public)]
    async fn index_of(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<i32>;

    #[method(public)]
    async fn last_index_of(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<i32>;

    #[method(public)]
    async fn list_iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<ListIterator>>;

    #[method(name = "listIterator", public)]
    async fn list_iterator_at(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<ListIterator>>;

    #[method(public)]
    async fn sub_list(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, from: i32, to: i32) -> Result<ClassInstanceRef<Self>>;
}
