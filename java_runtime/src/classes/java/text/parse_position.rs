use alloc::{format, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::lang::{Object, String},
};

// public class java.text.ParsePosition
pub struct ParsePosition;

impl ParsePosition {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/text/ParsePosition",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(I)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getIndex", "()I", Self::get_index, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("setIndex", "(I)V", Self::set_index, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getErrorIndex", "()I", Self::get_error_index, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("setErrorIndex", "(I)V", Self::set_error_index, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("index", "I", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("errorIndex", "I", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, index: i32) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "index", "I", index).await?;
        jvm.put_field(&mut this, "errorIndex", "I", -1).await
    }

    async fn get_index(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "index", "I").await
    }

    async fn set_index(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, index: i32) -> Result<()> {
        jvm.put_field(&mut this, "index", "I", index).await
    }

    async fn get_error_index(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "errorIndex", "I").await
    }

    async fn set_error_index(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, index: i32) -> Result<()> {
        jvm.put_field(&mut this, "errorIndex", "I", index).await
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if other.is_null() || !jvm.is_instance(&**other, "java/text/ParsePosition") {
            return Ok(false);
        }

        let other: ClassInstanceRef<Self> = ClassInstanceRef::new(other.instance);
        let index: i32 = jvm.get_field(&this, "index", "I").await?;
        let other_index: i32 = jvm.get_field(&other, "index", "I").await?;
        let error_index: i32 = jvm.get_field(&this, "errorIndex", "I").await?;
        let other_error_index: i32 = jvm.get_field(&other, "errorIndex", "I").await?;
        Ok(index == other_index && error_index == other_error_index)
    }

    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let index: i32 = jvm.get_field(&this, "index", "I").await?;
        let error_index: i32 = jvm.get_field(&this, "errorIndex", "I").await?;
        Ok(index ^ error_index.rotate_left(16))
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let index: i32 = jvm.get_field(&this, "index", "I").await?;
        let error_index: i32 = jvm.get_field(&this, "errorIndex", "I").await?;
        Ok(
            JavaLangString::from_rust_string(jvm, &format!("java.text.ParsePosition[index={index},errorIndex={error_index}]"))
                .await?
                .into(),
        )
    }
}
