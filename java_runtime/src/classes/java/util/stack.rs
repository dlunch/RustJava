use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{classes::java::lang::Object, RuntimeClassProto, RuntimeContext};

// class java.util.Stack
pub struct Stack;

impl Stack {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Stack",
            parent_class: Some("java/util/Vector"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("empty", "()Z", Self::empty, Default::default()),
                JavaMethodProto::new("peek", "()Ljava/lang/Object;", Self::peek, Default::default()),
                JavaMethodProto::new("pop", "()Ljava/lang/Object;", Self::pop, Default::default()),
                JavaMethodProto::new("push", "(Ljava/lang/Object;)Ljava/lang/Object;", Self::push, Default::default()),
                JavaMethodProto::new("search", "(Ljava/lang/Object;)I", Self::search, Default::default()),
            ],
            fields: vec![],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::warn!("java.util.Stack::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/util/Vector", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn empty(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::warn!("java.util.Stack::size({:?})", &this);

        let size: i32 = jvm.invoke_virtual(&this, "size", "()I", ()).await?;
        let is_empty = size == 0;

        Ok(is_empty)
    }

    async fn peek(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::warn!("java.util.Stack::peek({:?})", &this);

        let size: i32 = jvm.invoke_virtual(&this, "size", "()I", ()).await?;
        if size == 0 {
            return Err(jvm.exception("java/util/EmptyStackException", "").await);
        }

        let element = jvm.invoke_virtual(&this, "elementAt", "(I)Ljava/lang/Object;", (size - 1,)).await?;
        Ok(element)
    }

    async fn pop(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::warn!("java.util.Stack::pop({:?})", &this);

        let size: i32 = jvm.invoke_virtual(&this, "size", "()I", ()).await?;
        let element = jvm.invoke_virtual(&this, "peek", "()Ljava/lang/Object;", ()).await?;
        let _: () = jvm.invoke_virtual(&this, "removeElementAt", "(I)V", (size - 1,)).await?;

        Ok(element)
    }

    async fn push(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        element: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        tracing::warn!("java.util.Stack::push({:?}, {:?})", &this, &element);

        let _: () = jvm
            .invoke_virtual(&this, "addElement", "(Ljava/lang/Object;)V", (element.clone(),))
            .await?;

        Ok(element)
    }

    async fn search(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>) -> Result<i32> {
        tracing::warn!("java.util.Stack::search({:?}, {:?})", &this, &element);

        let i: i32 = jvm.invoke_virtual(&this, "lastIndexOf", "(Ljava/lang/Object;)I", (element,)).await?;

        if i >= 0 {
            let size: i32 = jvm.invoke_virtual(&this, "size", "()I", ()).await?;
            Ok(size - i)
        } else {
            Ok(-1)
        }
    }
}

#[cfg(test)]
mod test {
    use jvm::{runtime::JavaLangString, ClassInstanceRef, Result};

    use crate::{classes::java::lang::Object, test::test_jvm};

    #[tokio::test]
    async fn test_stack_push_pop() -> Result<()> {
        let jvm = test_jvm().await?;

        let stack = jvm.new_class("java/util/Stack", "()V", ()).await?;

        let element1 = JavaLangString::from_rust_string(&jvm, "testValue1").await?;
        let element2 = JavaLangString::from_rust_string(&jvm, "testValue2").await?;

        let _: ClassInstanceRef<Object> = jvm.invoke_virtual(&stack, "push", "(Ljava/lang/Object;)Ljava/lang/Object;", (element1.clone(),)).await?;
        let _: ClassInstanceRef<Object> = jvm.invoke_virtual(&stack, "push", "(Ljava/lang/Object;)Ljava/lang/Object;", (element2.clone(),)).await?;

        let size: i32 = jvm.invoke_virtual(&stack, "size", "()I", ()).await?;
        assert_eq!(size, 2);

        let popped: ClassInstanceRef<Object> = jvm.invoke_virtual(&stack, "pop", "()Ljava/lang/Object;", ()).await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &popped).await?, "testValue2");

        let popped: ClassInstanceRef<Object> = jvm.invoke_virtual(&stack, "pop", "()Ljava/lang/Object;", ()).await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &popped).await?, "testValue1");

        Ok(())
    }

    #[tokio::test]
    async fn test_stack_peek() -> Result<()> {
        let jvm = test_jvm().await?;

        let stack = jvm.new_class("java/util/Stack", "()V", ()).await?;

        let element1 = JavaLangString::from_rust_string(&jvm, "testValue1").await?;
        let element2 = JavaLangString::from_rust_string(&jvm, "testValue2").await?;

        let _: ClassInstanceRef<Object> = jvm.invoke_virtual(&stack, "push", "(Ljava/lang/Object;)Ljava/lang/Object;", (element1.clone(),)).await?;
        let _: ClassInstanceRef<Object> = jvm.invoke_virtual(&stack, "push", "(Ljava/lang/Object;)Ljava/lang/Object;", (element2.clone(),)).await?;

        let size: i32 = jvm.invoke_virtual(&stack, "size", "()I", ()).await?;
        assert_eq!(size, 2);

        let peek: ClassInstanceRef<Object> = jvm.invoke_virtual(&stack, "peek", "()Ljava/lang/Object;", ()).await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &peek).await?, "testValue2");

        let peek: ClassInstanceRef<Object> = jvm.invoke_virtual(&stack, "pop", "()Ljava/lang/Object;", ()).await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &peek).await?, "testValue2");

        Ok(())
    }

    #[tokio::test]
    async fn test_stack_search() -> Result<()> {
        let jvm = test_jvm().await?;

        let stack = jvm.new_class("java/util/Stack", "()V", ()).await?;

        let element1 = JavaLangString::from_rust_string(&jvm, "testValue1").await?;
        let element2 = JavaLangString::from_rust_string(&jvm, "testValue2").await?;
        let element3 = JavaLangString::from_rust_string(&jvm, "testValue3").await?;
        let element4 = JavaLangString::from_rust_string(&jvm, "testValue3").await?;

        let _: ClassInstanceRef<Object> = jvm.invoke_virtual(&stack, "push", "(Ljava/lang/Object;)Ljava/lang/Object;", (element1.clone(),)).await?;
        let _: ClassInstanceRef<Object> = jvm.invoke_virtual(&stack, "push", "(Ljava/lang/Object;)Ljava/lang/Object;", (element2.clone(),)).await?;
        let _: ClassInstanceRef<Object> = jvm.invoke_virtual(&stack, "push", "(Ljava/lang/Object;)Ljava/lang/Object;", (element3.clone(),)).await?;
        let _: ClassInstanceRef<Object> = jvm.invoke_virtual(&stack, "push", "(Ljava/lang/Object;)Ljava/lang/Object;", (element1.clone(),)).await?;
        
        let size: i32 = jvm.invoke_virtual(&stack, "size", "()I", ()).await?;
        assert_eq!(size, 4);

        let peek: i32 = jvm.invoke_virtual(&stack, "search", "(Ljava/lang/Object;)I", (element2.clone(),)).await?;
        assert_eq!(peek, 3);

        let peek: i32 = jvm.invoke_virtual(&stack, "search", "(Ljava/lang/Object;)I", (element1.clone(),)).await?;
        assert_eq!(peek, 1);

        let peek: i32 = jvm.invoke_virtual(&stack, "search", "(Ljava/lang/Object;)I", (element4.clone(),)).await?;
        assert_eq!(peek, -1);
        
        Ok(())
    }
}
