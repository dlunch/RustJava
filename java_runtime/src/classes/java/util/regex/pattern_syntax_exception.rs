use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::lang::{String, StringBuffer},
};

// public class java.util.regex.PatternSyntaxException
pub struct PatternSyntaxException;

impl PatternSyntaxException {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/regex/PatternSyntaxException",
            parent_class: Some("java/lang/IllegalArgumentException"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/lang/String;Ljava/lang/String;I)V",
                    Self::init,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("getDescription", "()Ljava/lang/String;", Self::get_description, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getPattern", "()Ljava/lang/String;", Self::get_pattern, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getIndex", "()I", Self::get_index, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getMessage", "()Ljava/lang/String;", Self::get_message, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("desc", "Ljava/lang/String;", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL),
                JavaFieldProto::new("pattern", "Ljava/lang/String;", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL),
                JavaFieldProto::new("index", "I", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL),
                JavaFieldProto::new(
                    "nl",
                    "Ljava/lang/String;",
                    FieldAccessFlags::PRIVATE | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        tracing::debug!("java.util.regex.PatternSyntaxException::<clinit>()");

        let key = JavaLangString::from_rust_string(jvm, "line.separator").await?;
        let line_separator: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/System", "getProperty", "(Ljava/lang/String;)Ljava/lang/String;", (key,))
            .await?;
        jvm.put_static_field("java/util/regex/PatternSyntaxException", "nl", "Ljava/lang/String;", line_separator)
            .await
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        description: ClassInstanceRef<String>,
        pattern: ClassInstanceRef<String>,
        index: i32,
    ) -> Result<()> {
        tracing::debug!("java.util.regex.PatternSyntaxException::<init>({this:?}, {description:?}, {pattern:?}, {index})");

        let _: () = jvm
            .invoke_special(&this, "java/lang/IllegalArgumentException", "<init>", "()V", ())
            .await?;
        jvm.put_field(&mut this, "desc", "Ljava/lang/String;", description).await?;
        jvm.put_field(&mut this, "pattern", "Ljava/lang/String;", pattern).await?;
        jvm.put_field(&mut this, "index", "I", index).await
    }

    async fn get_description(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.regex.PatternSyntaxException::getDescription({this:?})");

        jvm.get_field(&this, "desc", "Ljava/lang/String;").await
    }

    async fn get_pattern(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.regex.PatternSyntaxException::getPattern({this:?})");

        jvm.get_field(&this, "pattern", "Ljava/lang/String;").await
    }

    async fn get_index(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.regex.PatternSyntaxException::getIndex({this:?})");

        jvm.get_field(&this, "index", "I").await
    }

    async fn get_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.regex.PatternSyntaxException::getMessage({this:?})");

        let description: ClassInstanceRef<String> = jvm.get_field(&this, "desc", "Ljava/lang/String;").await?;
        let pattern: ClassInstanceRef<String> = jvm.get_field(&this, "pattern", "Ljava/lang/String;").await?;
        let index: i32 = jvm.get_field(&this, "index", "I").await?;
        let line_separator: ClassInstanceRef<String> = jvm
            .get_static_field("java/util/regex/PatternSyntaxException", "nl", "Ljava/lang/String;")
            .await?;
        let buffer: ClassInstanceRef<StringBuffer> = jvm.new_class("java/lang/StringBuffer", "()V", ()).await?.into();

        let _: ClassInstanceRef<StringBuffer> = jvm
            .invoke_virtual(
                &buffer,
                "java/lang/StringBuffer",
                "append",
                "(Ljava/lang/String;)Ljava/lang/StringBuffer;",
                (description,),
            )
            .await?;
        if index >= 0 {
            let near_index = JavaLangString::from_rust_string(jvm, " near index ").await?;
            let _: ClassInstanceRef<StringBuffer> = jvm
                .invoke_virtual(
                    &buffer,
                    "java/lang/StringBuffer",
                    "append",
                    "(Ljava/lang/String;)Ljava/lang/StringBuffer;",
                    (near_index,),
                )
                .await?;
            let _: ClassInstanceRef<StringBuffer> = jvm
                .invoke_virtual(&buffer, "java/lang/StringBuffer", "append", "(I)Ljava/lang/StringBuffer;", (index,))
                .await?;
        }
        let _: ClassInstanceRef<StringBuffer> = jvm
            .invoke_virtual(
                &buffer,
                "java/lang/StringBuffer",
                "append",
                "(Ljava/lang/String;)Ljava/lang/StringBuffer;",
                (line_separator.clone(),),
            )
            .await?;
        let _: ClassInstanceRef<StringBuffer> = jvm
            .invoke_virtual(
                &buffer,
                "java/lang/StringBuffer",
                "append",
                "(Ljava/lang/String;)Ljava/lang/StringBuffer;",
                (pattern.clone(),),
            )
            .await?;

        if index >= 0 {
            let _: ClassInstanceRef<StringBuffer> = jvm
                .invoke_virtual(
                    &buffer,
                    "java/lang/StringBuffer",
                    "append",
                    "(Ljava/lang/String;)Ljava/lang/StringBuffer;",
                    (line_separator,),
                )
                .await?;
            for _ in 0..index {
                let _: ClassInstanceRef<StringBuffer> = jvm
                    .invoke_virtual(
                        &buffer,
                        "java/lang/StringBuffer",
                        "append",
                        "(C)Ljava/lang/StringBuffer;",
                        (' ' as JavaChar,),
                    )
                    .await?;
            }
            let _: ClassInstanceRef<StringBuffer> = jvm
                .invoke_virtual(
                    &buffer,
                    "java/lang/StringBuffer",
                    "append",
                    "(C)Ljava/lang/StringBuffer;",
                    ('^' as JavaChar,),
                )
                .await?;
        }

        jvm.invoke_virtual(&buffer, "java/lang/Object", "toString", "()Ljava/lang/String;", ())
            .await
    }
}
