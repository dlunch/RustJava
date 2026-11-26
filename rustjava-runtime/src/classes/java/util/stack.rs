use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::Object};

// class java.util.Stack
#[java_class(name = "java/util/Stack", extends = "java/util/Vector", public)]
pub struct Stack;

#[java_methods]
impl Stack {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::warn!("java.util.Stack::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/util/Vector", "<init>", "()V", ()).await?;

        Ok(())
    }

    #[method(public)]
    async fn empty(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::warn!("java.util.Stack::size({this:?})");

        let size: i32 = jvm.invoke_virtual(&this, "java/util/Stack", "size", "()I", ()).await?;
        let is_empty = size == 0;

        Ok(is_empty)
    }

    #[method(public, synchronized)]
    async fn peek(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::warn!("java.util.Stack::peek({this:?})");

        let size: i32 = jvm.invoke_virtual(&this, "java/util/Stack", "size", "()I", ()).await?;
        if size == 0 {
            return Err(jvm.exception("java/util/EmptyStackException", "").await);
        }

        let element = jvm
            .invoke_virtual(&this, "java/util/Stack", "elementAt", "(I)Ljava/lang/Object;", (size - 1,))
            .await?;
        Ok(element)
    }

    #[method(public, synchronized)]
    async fn pop(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::warn!("java.util.Stack::pop({this:?})");

        let size: i32 = jvm.invoke_virtual(&this, "java/util/Stack", "size", "()I", ()).await?;
        let element = jvm.invoke_virtual(&this, "java/util/Stack", "peek", "()Ljava/lang/Object;", ()).await?;
        let _: () = jvm
            .invoke_virtual(&this, "java/util/Stack", "removeElementAt", "(I)V", (size - 1,))
            .await?;

        Ok(element)
    }

    #[method(public)]
    async fn push(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        element: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        tracing::warn!("java.util.Stack::push({this:?}, {element:?})");

        let _: () = jvm
            .invoke_virtual(&this, "java/util/Stack", "addElement", "(Ljava/lang/Object;)V", (element.clone(),))
            .await?;

        Ok(element)
    }

    #[method(public, synchronized)]
    async fn search(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<i32> {
        tracing::warn!("java.util.Stack::search({this:?}, {element:?})");

        let i: i32 = jvm
            .invoke_virtual(&this, "java/util/Stack", "lastIndexOf", "(Ljava/lang/Object;)I", (element,))
            .await?;

        if i >= 0 {
            let size: i32 = jvm.invoke_virtual(&this, "java/util/Stack", "size", "()I", ()).await?;
            Ok(size - i)
        } else {
            Ok(-1)
        }
    }
}
