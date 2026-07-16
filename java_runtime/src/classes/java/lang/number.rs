use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::{ClassAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

// abstract class java.lang.Number
pub struct Number;

impl Number {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Number",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/io/Serializable"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("byteValue", "()B", Self::byte_value, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("shortValue", "()S", Self::short_value, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new_abstract("intValue", "()I", MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new_abstract("longValue", "()J", MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new_abstract("floatValue", "()F", MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new_abstract("doubleValue", "()D", MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::ABSTRACT,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }

    async fn byte_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i8> {
        let value: i32 = jvm.invoke_virtual(&this, "intValue", "()I", ()).await?;
        Ok(value as i8)
    }

    async fn short_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i16> {
        let value: i32 = jvm.invoke_virtual(&this, "intValue", "()I", ()).await?;
        Ok(value as i16)
    }
}
