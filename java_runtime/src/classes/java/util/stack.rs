use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

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
