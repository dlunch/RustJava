use alloc::{format, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::lang::{Object, String},
};

// public class java.text.FieldPosition
pub struct FieldPosition;

impl FieldPosition {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/text/FieldPosition",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(I)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getField", "()I", Self::get_field, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getBeginIndex", "()I", Self::get_begin_index, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getEndIndex", "()I", Self::get_end_index, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("setBeginIndex", "(I)V", Self::set_begin_index, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("setEndIndex", "(I)V", Self::set_end_index, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("field", "I", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("beginIndex", "I", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("endIndex", "I", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, field: i32) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "field", "I", field).await?;
        jvm.put_field(&mut this, "beginIndex", "I", 0).await?;
        jvm.put_field(&mut this, "endIndex", "I", 0).await
    }

    async fn get_field(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "field", "I").await
    }

    async fn get_begin_index(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "beginIndex", "I").await
    }

    async fn get_end_index(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "endIndex", "I").await
    }

    async fn set_begin_index(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        jvm.put_field(&mut this, "beginIndex", "I", value).await
    }

    async fn set_end_index(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        jvm.put_field(&mut this, "endIndex", "I", value).await
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if other.is_null() || !jvm.is_instance(&**other, "java/text/FieldPosition") {
            return Ok(false);
        }

        let other: ClassInstanceRef<Self> = ClassInstanceRef::new(other.instance);
        let field: i32 = jvm.get_field(&this, "field", "I").await?;
        let other_field: i32 = jvm.get_field(&other, "field", "I").await?;
        let begin_index: i32 = jvm.get_field(&this, "beginIndex", "I").await?;
        let other_begin_index: i32 = jvm.get_field(&other, "beginIndex", "I").await?;
        let end_index: i32 = jvm.get_field(&this, "endIndex", "I").await?;
        let other_end_index: i32 = jvm.get_field(&other, "endIndex", "I").await?;
        Ok(field == other_field && begin_index == other_begin_index && end_index == other_end_index)
    }

    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let field: i32 = jvm.get_field(&this, "field", "I").await?;
        let begin_index: i32 = jvm.get_field(&this, "beginIndex", "I").await?;
        let end_index: i32 = jvm.get_field(&this, "endIndex", "I").await?;
        Ok(field ^ begin_index.rotate_left(11) ^ end_index.rotate_left(22))
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let field: i32 = jvm.get_field(&this, "field", "I").await?;
        let begin_index: i32 = jvm.get_field(&this, "beginIndex", "I").await?;
        let end_index: i32 = jvm.get_field(&this, "endIndex", "I").await?;
        Ok(JavaLangString::from_rust_string(
            jvm,
            &format!("java.text.FieldPosition[field={field},beginIndex={begin_index},endIndex={end_index}]"),
        )
        .await?
        .into())
    }
}
