use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::{ClassAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, JavaError, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        lang::{Object, String, StringBuffer},
        text::{FieldPosition, ParseException, ParsePosition},
    },
};

// public abstract class java.text.Format
pub struct Format;

impl Format {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/text/Format",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/io/Serializable", "java/lang/Cloneable"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PROTECTED),
                JavaMethodProto::new(
                    "format",
                    "(Ljava/lang/Object;)Ljava/lang/String;",
                    Self::format,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::FINAL,
                ),
                JavaMethodProto::new_abstract(
                    "format",
                    "(Ljava/lang/Object;Ljava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;",
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT,
                ),
                JavaMethodProto::new(
                    "parseObject",
                    "(Ljava/lang/String;)Ljava/lang/Object;",
                    Self::parse_object,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new_abstract(
                    "parseObject",
                    "(Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/lang/Object;",
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT,
                ),
                JavaMethodProto::new("clone", "()Ljava/lang/Object;", Self::clone, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::ABSTRACT,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }

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
                "format",
                "(Ljava/lang/Object;Ljava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;",
                (object, buffer, position),
            )
            .await?;
        jvm.invoke_virtual(&buffer, "toString", "()Ljava/lang/String;", ()).await
    }

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
                "parseObject",
                "(Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/lang/Object;",
                (source, position.clone()),
            )
            .await?;
        let index: i32 = jvm.invoke_virtual(&position, "getIndex", "()I", ()).await?;
        if index == 0 {
            let error_index: i32 = jvm.invoke_virtual(&position, "getErrorIndex", "()I", ()).await?;
            let message = JavaLangString::from_rust_string(jvm, "Format.parseObject(String) failed").await?;
            let exception: ClassInstanceRef<ParseException> = jvm
                .new_class("java/text/ParseException", "(Ljava/lang/String;I)V", (message, error_index))
                .await?
                .into();
            return Err(JavaError::JavaException(exception.into()));
        }
        Ok(result)
    }

    async fn clone(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        Ok(jvm.shallow_clone(&this)?.into())
    }
}
