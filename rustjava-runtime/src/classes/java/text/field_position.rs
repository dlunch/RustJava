use alloc::format;

use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::lang::{Object, String},
};

// public class java.text.FieldPosition
#[java_class(name = "java/text/FieldPosition", public)]
pub struct FieldPosition {
    #[field(private)]
    field: i32,
    #[field(private)]
    beginIndex: i32,
    #[field(private)]
    endIndex: i32,
}

#[java_methods]
impl FieldPosition {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, field: i32) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/text/FieldPosition", "field", "I", field).await?;
        jvm.put_field(&mut this, "java/text/FieldPosition", "beginIndex", "I", 0).await?;
        jvm.put_field(&mut this, "java/text/FieldPosition", "endIndex", "I", 0).await
    }

    #[method(public)]
    async fn get_field(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "java/text/FieldPosition", "field", "I").await
    }

    #[method(public)]
    async fn get_begin_index(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "java/text/FieldPosition", "beginIndex", "I").await
    }

    #[method(public)]
    async fn get_end_index(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "java/text/FieldPosition", "endIndex", "I").await
    }

    #[method(public)]
    async fn set_begin_index(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        jvm.put_field(&mut this, "java/text/FieldPosition", "beginIndex", "I", value).await
    }

    #[method(public)]
    async fn set_end_index(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        jvm.put_field(&mut this, "java/text/FieldPosition", "endIndex", "I", value).await
    }

    #[method(public)]
    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if other.is_null() || !jvm.is_instance(&**other, "java/text/FieldPosition") {
            return Ok(false);
        }

        let other: ClassInstanceRef<Self> = ClassInstanceRef::new(other.instance);
        let field: i32 = jvm.get_field(&this, "java/text/FieldPosition", "field", "I").await?;
        let other_field: i32 = jvm.get_field(&other, "java/text/FieldPosition", "field", "I").await?;
        let begin_index: i32 = jvm.get_field(&this, "java/text/FieldPosition", "beginIndex", "I").await?;
        let other_begin_index: i32 = jvm.get_field(&other, "java/text/FieldPosition", "beginIndex", "I").await?;
        let end_index: i32 = jvm.get_field(&this, "java/text/FieldPosition", "endIndex", "I").await?;
        let other_end_index: i32 = jvm.get_field(&other, "java/text/FieldPosition", "endIndex", "I").await?;
        Ok(field == other_field && begin_index == other_begin_index && end_index == other_end_index)
    }

    #[method(public)]
    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let field: i32 = jvm.get_field(&this, "java/text/FieldPosition", "field", "I").await?;
        let begin_index: i32 = jvm.get_field(&this, "java/text/FieldPosition", "beginIndex", "I").await?;
        let end_index: i32 = jvm.get_field(&this, "java/text/FieldPosition", "endIndex", "I").await?;
        Ok(field ^ begin_index.rotate_left(11) ^ end_index.rotate_left(22))
    }

    #[method(public)]
    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let field: i32 = jvm.get_field(&this, "java/text/FieldPosition", "field", "I").await?;
        let begin_index: i32 = jvm.get_field(&this, "java/text/FieldPosition", "beginIndex", "I").await?;
        let end_index: i32 = jvm.get_field(&this, "java/text/FieldPosition", "endIndex", "I").await?;
        Ok(JavaLangString::from_rust_string(
            jvm,
            &format!("java.text.FieldPosition[field={field},beginIndex={begin_index},endIndex={end_index}]"),
        )
        .await?
        .into())
    }
}
