use jvm::{ClassInstanceRef, JavaError, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{
        lang::{Object, String, StringBuffer},
        text::{FieldPosition, ParseException, ParsePosition},
    },
};

// public abstract class java.text.Format
#[java_class(
    name = "java/text/Format",
    extends = "java/lang/Object",
    implements("java/io/Serializable", "java/lang/Cloneable"),
    public,
    abstract
)]
pub struct Format;

#[java_methods]
impl Format {
    #[method(constructor, protected)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }

    #[method(public, final)]
    async fn format(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        object: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<String>> {
        let buffer: ClassInstanceRef<StringBuffer> = jvm.new_class("java/lang/StringBuffer", "()V", ()).await?.into();
        let position: ClassInstanceRef<FieldPosition> = jvm.new_class("java/text/FieldPosition", "(I)V", (0,)).await?.into();
        let buffer: ClassInstanceRef<StringBuffer> = jvm
            .invoke_virtual(
                &this,
                "java/text/Format",
                "format",
                "(Ljava/lang/Object;Ljava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;",
                (object, buffer, position),
            )
            .await?;
        jvm.invoke_virtual(&buffer, "java/lang/Object", "toString", "()Ljava/lang/String;", ())
            .await
    }

    #[method(name = "format", public)]
    async fn format_object(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        object: ClassInstanceRef<Object>,
        buffer: ClassInstanceRef<StringBuffer>,
        position: ClassInstanceRef<FieldPosition>,
    ) -> Result<ClassInstanceRef<StringBuffer>>;

    #[method(public)]
    async fn parse_object(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        source: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Object>> {
        if source.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "source").await);
        }

        let position: ClassInstanceRef<ParsePosition> = jvm.new_class("java/text/ParsePosition", "(I)V", (0,)).await?.into();
        let result: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &this,
                "java/text/Format",
                "parseObject",
                "(Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/lang/Object;",
                (source, position.clone()),
            )
            .await?;
        let index: i32 = jvm.invoke_virtual(&position, "java/text/ParsePosition", "getIndex", "()I", ()).await?;
        if index == 0 {
            let error_index: i32 = jvm
                .invoke_virtual(&position, "java/text/ParsePosition", "getErrorIndex", "()I", ())
                .await?;
            let message = JavaLangString::from_rust_string(jvm, "Format.parseObject(String) failed").await?;
            let exception: ClassInstanceRef<ParseException> = jvm
                .new_class("java/text/ParseException", "(Ljava/lang/String;I)V", (message, error_index))
                .await?
                .into();
            return Err(JavaError::JavaException(exception.into()));
        }
        Ok(result)
    }

    #[method(name = "parseObject", public)]
    async fn parse_object_with_position(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        source: ClassInstanceRef<String>,
        position: ClassInstanceRef<ParsePosition>,
    ) -> Result<ClassInstanceRef<Object>>;

    #[method(public)]
    async fn clone(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        Ok(jvm.shallow_clone(&this)?.into())
    }
}
