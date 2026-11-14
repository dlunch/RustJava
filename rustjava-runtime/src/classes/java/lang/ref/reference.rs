use alloc::vec;

use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm_types::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

// abstract class java.lang.ref.Reference
pub struct Reference;

impl Reference {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/ref/Reference",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/Object;)V", Self::init, MethodAccessFlags::empty()),
                JavaMethodProto::new("get", "()Ljava/lang/Object;", Self::get, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("clear", "()V", Self::clear, MethodAccessFlags::PUBLIC),
            ],
            // the garbage collector does not trace this field and clears it once the referent is otherwise unreachable
            fields: vec![JavaFieldProto::new("referent", "Ljava/lang/Object;", FieldAccessFlags::PRIVATE)],
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::ABSTRACT,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, referent: ClassInstanceRef<Object>) -> Result<()> {
        tracing::debug!("java.lang.ref.Reference::<init>({this:?}, {referent:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/lang/ref/Reference", "referent", "Ljava/lang/Object;", referent)
            .await?;

        Ok(())
    }

    async fn get(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.lang.ref.Reference::get({this:?})");

        jvm.get_field(&this, "java/lang/ref/Reference", "referent", "Ljava/lang/Object;").await
    }

    async fn clear(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.ref.Reference::clear({this:?})");

        jvm.put_field(
            &mut this,
            "java/lang/ref/Reference",
            "referent",
            "Ljava/lang/Object;",
            ClassInstanceRef::<Object>::new(None),
        )
        .await
    }
}
