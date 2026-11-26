use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::lang::{Object, String},
};

// class java.util.StringTokenizer
#[java_class(
    name = "java/util/StringTokenizer",
    extends = "java/lang/Object",
    implements("java/util/Enumeration"),
    public
)]
pub struct StringTokenizer {
    #[field(private)]
    str: ClassInstanceRef<String>,
    #[field(private)]
    delimiters: ClassInstanceRef<String>,
    #[field(private)]
    currentPosition: i32,
    #[field(private)]
    maxPosition: i32,
    #[field(private)]
    returnDelimiters: bool,
}

#[java_methods]
impl StringTokenizer {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, string: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.util.StringTokenizer::<init>({this:?}, {string:?})");

        let delimiters = JavaLangString::from_rust_string(jvm, " \t\n\r\u{000c}").await?;
        jvm.invoke_special(
            &this,
            "java/util/StringTokenizer",
            "<init>",
            "(Ljava/lang/String;Ljava/lang/String;Z)V",
            (string, delimiters, false),
        )
        .await
    }

    #[method(constructor, public)]
    async fn init_with_delimiters(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        string: ClassInstanceRef<String>,
        delimiters: ClassInstanceRef<String>,
    ) -> Result<()> {
        tracing::debug!("java.util.StringTokenizer::<init>({this:?}, {string:?}, {delimiters:?})");

        jvm.invoke_special(
            &this,
            "java/util/StringTokenizer",
            "<init>",
            "(Ljava/lang/String;Ljava/lang/String;Z)V",
            (string, delimiters, false),
        )
        .await
    }

    #[method(constructor, public)]
    async fn init_with_delimiters_and_return(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        string: ClassInstanceRef<String>,
        delimiters: ClassInstanceRef<String>,
        return_delimiters: bool,
    ) -> Result<()> {
        tracing::debug!("java.util.StringTokenizer::<init>({this:?}, {string:?}, {delimiters:?}, {return_delimiters:?})");

        if string.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "str").await);
        }

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        let max_position: i32 = jvm.invoke_virtual(&string, "java/lang/String", "length", "()I", ()).await?;
        jvm.put_field(&mut this, "java/util/StringTokenizer", "str", "Ljava/lang/String;", string)
            .await?;
        jvm.put_field(&mut this, "java/util/StringTokenizer", "delimiters", "Ljava/lang/String;", delimiters)
            .await?;
        jvm.put_field(&mut this, "java/util/StringTokenizer", "currentPosition", "I", 0).await?;
        jvm.put_field(&mut this, "java/util/StringTokenizer", "maxPosition", "I", max_position)
            .await?;
        jvm.put_field(&mut this, "java/util/StringTokenizer", "returnDelimiters", "Z", return_delimiters)
            .await
    }

    #[method(public)]
    async fn has_more_tokens(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.StringTokenizer::hasMoreTokens({this:?})");

        let string: ClassInstanceRef<String> = jvm.get_field(&this, "java/util/StringTokenizer", "str", "Ljava/lang/String;").await?;
        let delimiters: ClassInstanceRef<String> = jvm
            .get_field(&this, "java/util/StringTokenizer", "delimiters", "Ljava/lang/String;")
            .await?;
        let position: i32 = jvm.get_field(&this, "java/util/StringTokenizer", "currentPosition", "I").await?;
        let max_position: i32 = jvm.get_field(&this, "java/util/StringTokenizer", "maxPosition", "I").await?;
        let return_delimiters: bool = jvm.get_field(&this, "java/util/StringTokenizer", "returnDelimiters", "Z").await?;
        Ok(
            Self::token_bounds(jvm, &string, &delimiters, position as usize, max_position as usize, return_delimiters)
                .await?
                .is_some(),
        )
    }

    #[method(public)]
    async fn next_token(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.StringTokenizer::nextToken({this:?})");

        let string: ClassInstanceRef<String> = jvm.get_field(&this, "java/util/StringTokenizer", "str", "Ljava/lang/String;").await?;
        let delimiters: ClassInstanceRef<String> = jvm
            .get_field(&this, "java/util/StringTokenizer", "delimiters", "Ljava/lang/String;")
            .await?;
        let position: i32 = jvm.get_field(&this, "java/util/StringTokenizer", "currentPosition", "I").await?;
        let max_position: i32 = jvm.get_field(&this, "java/util/StringTokenizer", "maxPosition", "I").await?;
        let return_delimiters: bool = jvm.get_field(&this, "java/util/StringTokenizer", "returnDelimiters", "Z").await?;
        let Some((start, end)) = Self::token_bounds(jvm, &string, &delimiters, position as usize, max_position as usize, return_delimiters).await?
        else {
            return Err(jvm.exception("java/util/NoSuchElementException", "StringTokenizer exhausted").await);
        };

        let token: ClassInstanceRef<String> = jvm
            .invoke_virtual(
                &string,
                "java/lang/String",
                "substring",
                "(II)Ljava/lang/String;",
                (start as i32, end as i32),
            )
            .await?;
        jvm.put_field(&mut this, "java/util/StringTokenizer", "currentPosition", "I", end as i32)
            .await?;
        Ok(token)
    }

    #[method(name = "nextToken", public)]
    async fn next_token_with_delimiters(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        delimiters: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.StringTokenizer::nextToken({this:?}, {delimiters:?})");

        jvm.put_field(&mut this, "java/util/StringTokenizer", "delimiters", "Ljava/lang/String;", delimiters)
            .await?;
        jvm.invoke_virtual(&this, "java/util/StringTokenizer", "nextToken", "()Ljava/lang/String;", ())
            .await
    }

    #[method(public)]
    async fn count_tokens(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.StringTokenizer::countTokens({this:?})");

        let string: ClassInstanceRef<String> = jvm.get_field(&this, "java/util/StringTokenizer", "str", "Ljava/lang/String;").await?;
        let delimiters: ClassInstanceRef<String> = jvm
            .get_field(&this, "java/util/StringTokenizer", "delimiters", "Ljava/lang/String;")
            .await?;
        let mut position: i32 = jvm.get_field(&this, "java/util/StringTokenizer", "currentPosition", "I").await?;
        let max_position: i32 = jvm.get_field(&this, "java/util/StringTokenizer", "maxPosition", "I").await?;
        let return_delimiters: bool = jvm.get_field(&this, "java/util/StringTokenizer", "returnDelimiters", "Z").await?;
        let mut count = 0;
        while let Some((_, end)) = Self::token_bounds(jvm, &string, &delimiters, position as usize, max_position as usize, return_delimiters).await? {
            count += 1;
            position = end as i32;
        }
        Ok(count)
    }

    #[method(public)]
    async fn has_more_elements(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.invoke_virtual(&this, "java/util/StringTokenizer", "hasMoreTokens", "()Z", ()).await
    }

    #[method(public)]
    async fn next_element(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let token: ClassInstanceRef<String> = jvm
            .invoke_virtual(&this, "java/util/StringTokenizer", "nextToken", "()Ljava/lang/String;", ())
            .await?;
        Ok(ClassInstanceRef::new(token.instance))
    }

    async fn token_bounds(
        jvm: &Jvm,
        string: &ClassInstanceRef<String>,
        delimiters: &ClassInstanceRef<String>,
        mut position: usize,
        max_position: usize,
        return_delimiters: bool,
    ) -> Result<Option<(usize, usize)>> {
        if delimiters.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "delimiters").await);
        }

        let string_chars = JavaLangString::to_utf16(jvm, string).await?;
        let delimiter_chars = JavaLangString::to_utf16(jvm, delimiters).await?;
        let max_position = max_position.min(string_chars.len());

        if !return_delimiters {
            while position < max_position && delimiter_chars.contains(&string_chars[position]) {
                position += 1;
            }
        }
        if position >= max_position {
            return Ok(None);
        }

        let start = position;
        if return_delimiters && delimiter_chars.contains(&string_chars[position]) {
            return Ok(Some((start, start + 1)));
        }
        while position < max_position && !delimiter_chars.contains(&string_chars[position]) {
            position += 1;
        }
        Ok(Some((start, position)))
    }
}
