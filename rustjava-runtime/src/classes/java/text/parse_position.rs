use alloc::format;

use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::lang::{Object, String},
};

// public class java.text.ParsePosition
#[java_class(name = "java/text/ParsePosition", public)]
pub struct ParsePosition {
    #[field(private)]
    index: i32,
    #[field(private)]
    errorIndex: i32,
}

#[java_methods]
impl ParsePosition {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, index: i32) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/text/ParsePosition", "index", "I", index).await?;
        jvm.put_field(&mut this, "java/text/ParsePosition", "errorIndex", "I", -1).await
    }

    #[method(public)]
    async fn get_index(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "java/text/ParsePosition", "index", "I").await
    }

    #[method(public)]
    async fn set_index(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, index: i32) -> Result<()> {
        jvm.put_field(&mut this, "java/text/ParsePosition", "index", "I", index).await
    }

    #[method(public)]
    async fn get_error_index(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "java/text/ParsePosition", "errorIndex", "I").await
    }

    #[method(public)]
    async fn set_error_index(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, index: i32) -> Result<()> {
        jvm.put_field(&mut this, "java/text/ParsePosition", "errorIndex", "I", index).await
    }

    #[method(public)]
    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if other.is_null() || !jvm.is_instance(&**other, "java/text/ParsePosition") {
            return Ok(false);
        }

        let other: ClassInstanceRef<Self> = ClassInstanceRef::new(other.instance);
        let index: i32 = jvm.get_field(&this, "java/text/ParsePosition", "index", "I").await?;
        let other_index: i32 = jvm.get_field(&other, "java/text/ParsePosition", "index", "I").await?;
        let error_index: i32 = jvm.get_field(&this, "java/text/ParsePosition", "errorIndex", "I").await?;
        let other_error_index: i32 = jvm.get_field(&other, "java/text/ParsePosition", "errorIndex", "I").await?;
        Ok(index == other_index && error_index == other_error_index)
    }

    #[method(public)]
    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let index: i32 = jvm.get_field(&this, "java/text/ParsePosition", "index", "I").await?;
        let error_index: i32 = jvm.get_field(&this, "java/text/ParsePosition", "errorIndex", "I").await?;
        Ok(index ^ error_index.rotate_left(16))
    }

    #[method(public)]
    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let index: i32 = jvm.get_field(&this, "java/text/ParsePosition", "index", "I").await?;
        let error_index: i32 = jvm.get_field(&this, "java/text/ParsePosition", "errorIndex", "I").await?;
        Ok(
            JavaLangString::from_rust_string(jvm, &format!("java.text.ParsePosition[index={index},errorIndex={error_index}]"))
                .await?
                .into(),
        )
    }
}
