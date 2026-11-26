use alloc::{boxed::Box, format, string::ToString, vec::Vec};

use jvm::{Array, ClassInstance, ClassInstanceRef, JavaError, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};
use regex::{Regex, RegexBuilder};

use crate::{
    RuntimeContext,
    classes::java::lang::{CharSequence, String},
};

use super::Matcher;

// public final class java.util.regex.Pattern
#[java_class(name = "java/util/regex/Pattern", implements("java/io/Serializable"), public, final)]
pub struct Pattern {
    #[field(public, static, final)]
    UNIX_LINES: i32,
    #[field(public, static, final)]
    CASE_INSENSITIVE: i32,
    #[field(public, static, final)]
    COMMENTS: i32,
    #[field(public, static, final)]
    MULTILINE: i32,
    #[field(public, static, final)]
    DOTALL: i32,
    #[field(public, static, final)]
    UNICODE_CASE: i32,
    #[field(public, static, final)]
    CANON_EQ: i32,
    #[field(private, final)]
    pattern: ClassInstanceRef<String>,
    #[field(private, final)]
    flags: i32,
}

#[java_methods]
impl Pattern {
    #[method(static_initializer)]
    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        tracing::debug!("java.util.regex.Pattern::<clinit>()");

        jvm.put_static_field("java/util/regex/Pattern", "UNIX_LINES", "I", 1i32).await?;
        jvm.put_static_field("java/util/regex/Pattern", "CASE_INSENSITIVE", "I", 2i32).await?;
        jvm.put_static_field("java/util/regex/Pattern", "COMMENTS", "I", 4i32).await?;
        jvm.put_static_field("java/util/regex/Pattern", "MULTILINE", "I", 8i32).await?;
        jvm.put_static_field("java/util/regex/Pattern", "DOTALL", "I", 32i32).await?;
        jvm.put_static_field("java/util/regex/Pattern", "UNICODE_CASE", "I", 64i32).await?;
        jvm.put_static_field("java/util/regex/Pattern", "CANON_EQ", "I", 128i32).await
    }

    #[method(constructor, private)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, pattern: ClassInstanceRef<String>, flags: i32) -> Result<()> {
        tracing::debug!("java.util.regex.Pattern::<init>({this:?}, {pattern:?}, {flags})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/util/regex/Pattern", "pattern", "Ljava/lang/String;", pattern)
            .await?;
        jvm.put_field(&mut this, "java/util/regex/Pattern", "flags", "I", flags).await
    }

    #[method(public, static)]
    async fn compile(jvm: &Jvm, _: &mut RuntimeContext, pattern: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.util.regex.Pattern::compile({pattern:?})");

        jvm.invoke_static(
            "java/util/regex/Pattern",
            "compile",
            "(Ljava/lang/String;I)Ljava/util/regex/Pattern;",
            (pattern, 0),
        )
        .await
    }

    #[method(name = "compile", public, static)]
    async fn compile_with_flags(jvm: &Jvm, _: &mut RuntimeContext, pattern: ClassInstanceRef<String>, flags: i32) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.util.regex.Pattern::compile({pattern:?}, {flags})");

        if flags & !0xef != 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "Unknown flag").await);
        }
        if pattern.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "regex is null").await);
        }

        let source = JavaLangString::to_rust_string(jvm, &pattern).await?;
        let _ = Self::build_regex(jvm, &source, &pattern, flags, false).await?;

        Ok(jvm
            .new_class("java/util/regex/Pattern", "(Ljava/lang/String;I)V", (pattern, flags))
            .await?
            .into())
    }

    #[method(public)]
    async fn pattern(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.regex.Pattern::pattern({this:?})");

        jvm.get_field(&this, "java/util/regex/Pattern", "pattern", "Ljava/lang/String;").await
    }

    #[method(public)]
    async fn flags(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.regex.Pattern::flags({this:?})");

        jvm.get_field(&this, "java/util/regex/Pattern", "flags", "I").await
    }

    #[method(public)]
    async fn matcher(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        input: ClassInstanceRef<CharSequence>,
    ) -> Result<ClassInstanceRef<Matcher>> {
        tracing::debug!("java.util.regex.Pattern::matcher({this:?}, {input:?})");

        Ok(jvm
            .new_class(
                "java/util/regex/Matcher",
                "(Ljava/util/regex/Pattern;Ljava/lang/CharSequence;)V",
                (this, input),
            )
            .await?
            .into())
    }

    #[method(public, static)]
    async fn matches(jvm: &Jvm, _: &mut RuntimeContext, pattern: ClassInstanceRef<String>, input: ClassInstanceRef<CharSequence>) -> Result<bool> {
        tracing::debug!("java.util.regex.Pattern::matches({pattern:?}, {input:?})");

        let pattern: ClassInstanceRef<Self> = jvm
            .invoke_static(
                "java/util/regex/Pattern",
                "compile",
                "(Ljava/lang/String;)Ljava/util/regex/Pattern;",
                (pattern,),
            )
            .await?;
        let matcher: ClassInstanceRef<Matcher> = jvm
            .invoke_virtual(
                &pattern,
                "java/util/regex/Pattern",
                "matcher",
                "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;",
                (input,),
            )
            .await?;
        jvm.invoke_virtual(&matcher, "java/util/regex/Matcher", "matches", "()Z", ()).await
    }

    #[method(public)]
    async fn split(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        input: ClassInstanceRef<CharSequence>,
    ) -> Result<ClassInstanceRef<Array<String>>> {
        tracing::debug!("java.util.regex.Pattern::split({this:?}, {input:?})");

        jvm.invoke_virtual(
            &this,
            "java/util/regex/Pattern",
            "split",
            "(Ljava/lang/CharSequence;I)[Ljava/lang/String;",
            (input, 0),
        )
        .await
    }

    #[method(name = "split", public)]
    async fn split_with_limit(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        input: ClassInstanceRef<CharSequence>,
        limit: i32,
    ) -> Result<ClassInstanceRef<Array<String>>> {
        tracing::debug!("java.util.regex.Pattern::split({this:?}, {input:?}, {limit})");

        let matcher: ClassInstanceRef<Matcher> = jvm
            .invoke_virtual(
                &this,
                "java/util/regex/Pattern",
                "matcher",
                "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;",
                (input.clone(),),
            )
            .await?;
        let input_length: i32 = jvm.invoke_virtual(&input, &input.class_definition().name(), "length", "()I", ()).await?;
        let match_limited = limit > 0;
        let mut index = 0;
        let mut parts: Vec<Box<dyn ClassInstance>> = Vec::new();

        while jvm
            .invoke_virtual::<_, bool>(&matcher, "java/util/regex/Matcher", "find", "()Z", ())
            .await?
        {
            if !match_limited || parts.len() < (limit - 1) as usize {
                let start: i32 = jvm.invoke_virtual(&matcher, "java/util/regex/Matcher", "start", "()I", ()).await?;
                let part: ClassInstanceRef<CharSequence> = jvm
                    .invoke_virtual(
                        &input,
                        &input.class_definition().name(),
                        "subSequence",
                        "(II)Ljava/lang/CharSequence;",
                        (index, start),
                    )
                    .await?;
                parts.push(
                    jvm.invoke_virtual(&part, "java/lang/Object", "toString", "()Ljava/lang/String;", ())
                        .await?,
                );
                index = jvm.invoke_virtual(&matcher, "java/util/regex/Matcher", "end", "()I", ()).await?;
            } else if parts.len() == (limit - 1) as usize {
                let part: ClassInstanceRef<CharSequence> = jvm
                    .invoke_virtual(
                        &input,
                        &input.class_definition().name(),
                        "subSequence",
                        "(II)Ljava/lang/CharSequence;",
                        (index, input_length),
                    )
                    .await?;
                parts.push(
                    jvm.invoke_virtual(&part, "java/lang/Object", "toString", "()Ljava/lang/String;", ())
                        .await?,
                );
                index = jvm.invoke_virtual(&matcher, "java/util/regex/Matcher", "end", "()I", ()).await?;
            }
        }

        if index == 0 {
            parts.clear();
            parts.push(
                jvm.invoke_virtual(&input, "java/lang/Object", "toString", "()Ljava/lang/String;", ())
                    .await?,
            );
        } else {
            if !match_limited || parts.len() < limit as usize {
                let part: ClassInstanceRef<CharSequence> = jvm
                    .invoke_virtual(
                        &input,
                        &input.class_definition().name(),
                        "subSequence",
                        "(II)Ljava/lang/CharSequence;",
                        (index, input_length),
                    )
                    .await?;
                parts.push(
                    jvm.invoke_virtual(&part, "java/lang/Object", "toString", "()Ljava/lang/String;", ())
                        .await?,
                );
            }
            if limit == 0 {
                while let Some(part) = parts.last() {
                    if jvm
                        .invoke_virtual::<_, i32>(part, &part.class_definition().name(), "length", "()I", ())
                        .await?
                        != 0
                    {
                        break;
                    }
                    parts.pop();
                }
            }
        }

        let mut result: ClassInstanceRef<Array<String>> = jvm.instantiate_array("Ljava/lang/String;", parts.len()).await?.into();
        jvm.store_array(&mut result, 0, parts).await?;
        Ok(result)
    }

    pub(crate) async fn build_regex(jvm: &Jvm, source: &str, original: &ClassInstanceRef<String>, flags: i32, full_match: bool) -> Result<Regex> {
        let expression = if full_match { format!("\\A(?:{source})\\z") } else { source.to_string() };
        let mut builder = RegexBuilder::new(&expression);
        builder
            .case_insensitive(flags & 2 != 0)
            .ignore_whitespace(flags & 4 != 0)
            .multi_line(flags & 8 != 0)
            .dot_matches_new_line(flags & 32 != 0);
        let regex = builder.build();
        let regex = if full_match && regex.is_err() {
            let expression = format!("\\A(?:{source}\n)\\z");
            let mut builder = RegexBuilder::new(&expression);
            builder
                .case_insensitive(flags & 2 != 0)
                .ignore_whitespace(flags & 4 != 0)
                .multi_line(flags & 8 != 0)
                .dot_matches_new_line(flags & 32 != 0);
            builder.build()
        } else {
            regex
        };

        match regex {
            Ok(regex) => Ok(regex),
            Err(error) => {
                let description = JavaLangString::from_rust_string(jvm, &error.to_string()).await?;
                let exception = jvm
                    .new_class(
                        "java/util/regex/PatternSyntaxException",
                        "(Ljava/lang/String;Ljava/lang/String;I)V",
                        (description, original.clone(), -1),
                    )
                    .await?;
                Err(JavaError::JavaException(exception))
            }
        }
    }
}
