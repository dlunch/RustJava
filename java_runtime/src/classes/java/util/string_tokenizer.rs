use alloc::{vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::lang::{Object, String},
};

// class java.util.StringTokenizer
pub struct StringTokenizer;

impl StringTokenizer {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/StringTokenizer",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/util/Enumeration"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/lang/String;Ljava/lang/String;)V",
                    Self::init_with_delimiters,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/lang/String;Ljava/lang/String;Z)V",
                    Self::init_with_delimiters_and_return,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("hasMoreTokens", "()Z", Self::has_more_tokens, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("nextToken", "()Ljava/lang/String;", Self::next_token, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "nextToken",
                    "(Ljava/lang/String;)Ljava/lang/String;",
                    Self::next_token_with_delimiters,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("countTokens", "()I", Self::count_tokens, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("hasMoreElements", "()Z", Self::has_more_elements, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("nextElement", "()Ljava/lang/Object;", Self::next_element, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("str", "Ljava/lang/String;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("delimiters", "Ljava/lang/String;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("currentPosition", "I", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("maxPosition", "I", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("returnDelimiters", "Z", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

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
        let value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&string, "value", "[C").await?;
        let max_position = jvm.array_length(&value).await? as i32;
        jvm.put_field(&mut this, "str", "Ljava/lang/String;", string).await?;
        jvm.put_field(&mut this, "delimiters", "Ljava/lang/String;", delimiters).await?;
        jvm.put_field(&mut this, "currentPosition", "I", 0).await?;
        jvm.put_field(&mut this, "maxPosition", "I", max_position).await?;
        jvm.put_field(&mut this, "returnDelimiters", "Z", return_delimiters).await
    }

    async fn has_more_tokens(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.StringTokenizer::hasMoreTokens({this:?})");

        let string: ClassInstanceRef<String> = jvm.get_field(&this, "str", "Ljava/lang/String;").await?;
        let delimiters: ClassInstanceRef<String> = jvm.get_field(&this, "delimiters", "Ljava/lang/String;").await?;
        let position: i32 = jvm.get_field(&this, "currentPosition", "I").await?;
        let max_position: i32 = jvm.get_field(&this, "maxPosition", "I").await?;
        let return_delimiters: bool = jvm.get_field(&this, "returnDelimiters", "Z").await?;
        Ok(
            Self::token_bounds(jvm, &string, &delimiters, position as usize, max_position as usize, return_delimiters)
                .await?
                .is_some(),
        )
    }

    async fn next_token(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.StringTokenizer::nextToken({this:?})");

        let string: ClassInstanceRef<String> = jvm.get_field(&this, "str", "Ljava/lang/String;").await?;
        let delimiters: ClassInstanceRef<String> = jvm.get_field(&this, "delimiters", "Ljava/lang/String;").await?;
        let position: i32 = jvm.get_field(&this, "currentPosition", "I").await?;
        let max_position: i32 = jvm.get_field(&this, "maxPosition", "I").await?;
        let return_delimiters: bool = jvm.get_field(&this, "returnDelimiters", "Z").await?;
        let Some((start, end)) = Self::token_bounds(jvm, &string, &delimiters, position as usize, max_position as usize, return_delimiters).await?
        else {
            return Err(jvm.exception("java/util/NoSuchElementException", "StringTokenizer exhausted").await);
        };

        let string_value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&string, "value", "[C").await?;
        let token_chars: Vec<JavaChar> = jvm.load_array(&string_value, start, end - start).await?;
        let mut token_value = jvm.instantiate_array("C", token_chars.len()).await?;
        jvm.store_array(&mut token_value, 0, token_chars).await?;
        let token = jvm.new_class("java/lang/String", "([C)V", (token_value,)).await?;
        jvm.put_field(&mut this, "currentPosition", "I", end as i32).await?;
        Ok(token.into())
    }

    async fn next_token_with_delimiters(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        delimiters: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.StringTokenizer::nextToken({this:?}, {delimiters:?})");

        jvm.put_field(&mut this, "delimiters", "Ljava/lang/String;", delimiters).await?;
        jvm.invoke_virtual(&this, "nextToken", "()Ljava/lang/String;", ()).await
    }

    async fn count_tokens(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.StringTokenizer::countTokens({this:?})");

        let string: ClassInstanceRef<String> = jvm.get_field(&this, "str", "Ljava/lang/String;").await?;
        let delimiters: ClassInstanceRef<String> = jvm.get_field(&this, "delimiters", "Ljava/lang/String;").await?;
        let mut position: i32 = jvm.get_field(&this, "currentPosition", "I").await?;
        let max_position: i32 = jvm.get_field(&this, "maxPosition", "I").await?;
        let return_delimiters: bool = jvm.get_field(&this, "returnDelimiters", "Z").await?;
        let mut count = 0;
        while let Some((_, end)) = Self::token_bounds(jvm, &string, &delimiters, position as usize, max_position as usize, return_delimiters).await? {
            count += 1;
            position = end as i32;
        }
        Ok(count)
    }

    async fn has_more_elements(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.invoke_virtual(&this, "hasMoreTokens", "()Z", ()).await
    }

    async fn next_element(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let token: ClassInstanceRef<String> = jvm.invoke_virtual(&this, "nextToken", "()Ljava/lang/String;", ()).await?;
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

        let string_value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(string, "value", "[C").await?;
        let delimiter_value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(delimiters, "value", "[C").await?;
        let string_chars: Vec<JavaChar> = jvm.load_array(&string_value, 0, jvm.array_length(&string_value).await?).await?;
        let delimiter_chars: Vec<JavaChar> = jvm.load_array(&delimiter_value, 0, jvm.array_length(&delimiter_value).await?).await?;
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
