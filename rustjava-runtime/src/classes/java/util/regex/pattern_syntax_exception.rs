use jvm::{ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::lang::{String, StringBuffer},
};

// public class java.util.regex.PatternSyntaxException
#[java_class(name = "java/util/regex/PatternSyntaxException", extends = "java/lang/IllegalArgumentException", public)]
pub struct PatternSyntaxException {
    #[field(private, final)]
    desc: ClassInstanceRef<String>,
    #[field(private, final)]
    pattern: ClassInstanceRef<String>,
    #[field(private, final)]
    index: i32,
    #[field(private, static, final)]
    nl: ClassInstanceRef<String>,
}

#[java_methods]
impl PatternSyntaxException {
    #[method(static_initializer)]
    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        tracing::debug!("java.util.regex.PatternSyntaxException::<clinit>()");

        let key = JavaLangString::from_rust_string(jvm, "line.separator").await?;
        let line_separator: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/System", "getProperty", "(Ljava/lang/String;)Ljava/lang/String;", (key,))
            .await?;
        jvm.put_static_field("java/util/regex/PatternSyntaxException", "nl", "Ljava/lang/String;", line_separator)
            .await
    }

    #[method(constructor, public)]
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
        jvm.put_field(
            &mut this,
            "java/util/regex/PatternSyntaxException",
            "desc",
            "Ljava/lang/String;",
            description,
        )
        .await?;
        jvm.put_field(
            &mut this,
            "java/util/regex/PatternSyntaxException",
            "pattern",
            "Ljava/lang/String;",
            pattern,
        )
        .await?;
        jvm.put_field(&mut this, "java/util/regex/PatternSyntaxException", "index", "I", index)
            .await
    }

    #[method(public)]
    async fn get_description(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.regex.PatternSyntaxException::getDescription({this:?})");

        jvm.get_field(&this, "java/util/regex/PatternSyntaxException", "desc", "Ljava/lang/String;")
            .await
    }

    #[method(public)]
    async fn get_pattern(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.regex.PatternSyntaxException::getPattern({this:?})");

        jvm.get_field(&this, "java/util/regex/PatternSyntaxException", "pattern", "Ljava/lang/String;")
            .await
    }

    #[method(public)]
    async fn get_index(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.regex.PatternSyntaxException::getIndex({this:?})");

        jvm.get_field(&this, "java/util/regex/PatternSyntaxException", "index", "I").await
    }

    #[method(public)]
    async fn get_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.regex.PatternSyntaxException::getMessage({this:?})");

        let description: ClassInstanceRef<String> = jvm
            .get_field(&this, "java/util/regex/PatternSyntaxException", "desc", "Ljava/lang/String;")
            .await?;
        let pattern: ClassInstanceRef<String> = jvm
            .get_field(&this, "java/util/regex/PatternSyntaxException", "pattern", "Ljava/lang/String;")
            .await?;
        let index: i32 = jvm.get_field(&this, "java/util/regex/PatternSyntaxException", "index", "I").await?;
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
