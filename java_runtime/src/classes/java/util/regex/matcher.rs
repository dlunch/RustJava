use alloc::{string::String as RustString, vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::lang::{CharSequence, String, StringBuffer},
};

use super::Pattern;

#[derive(Clone, Copy)]
enum MatchMode {
    Full,
    Prefix,
    Find { start: i32, reset: bool },
}

// public final class java.util.regex.Matcher
pub struct Matcher;

impl Matcher {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/regex/Matcher",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/util/regex/Pattern;Ljava/lang/CharSequence;)V",
                    Self::init,
                    MethodAccessFlags::empty(),
                ),
                JavaMethodProto::new("pattern", "()Ljava/util/regex/Pattern;", Self::pattern, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("reset", "()Ljava/util/regex/Matcher;", Self::reset, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "reset",
                    "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;",
                    Self::reset_with_input,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("matches", "()Z", Self::matches, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("lookingAt", "()Z", Self::looking_at, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("find", "()Z", Self::find, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("find", "(I)Z", Self::find_from, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("start", "()I", Self::start, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("start", "(I)I", Self::start_group, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("end", "()I", Self::end, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("end", "(I)I", Self::end_group, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("group", "()Ljava/lang/String;", Self::group, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("group", "(I)Ljava/lang/String;", Self::group_index, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("groupCount", "()I", Self::group_count, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "appendReplacement",
                    "(Ljava/lang/StringBuffer;Ljava/lang/String;)Ljava/util/regex/Matcher;",
                    Self::append_replacement,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "appendTail",
                    "(Ljava/lang/StringBuffer;)Ljava/lang/StringBuffer;",
                    Self::append_tail,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "replaceAll",
                    "(Ljava/lang/String;)Ljava/lang/String;",
                    Self::replace_all,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "replaceFirst",
                    "(Ljava/lang/String;)Ljava/lang/String;",
                    Self::replace_first,
                    MethodAccessFlags::PUBLIC,
                ),
            ],
            fields: vec![
                JavaFieldProto::new(
                    "parentPattern",
                    "Ljava/util/regex/Pattern;",
                    FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new("text", "Ljava/lang/CharSequence;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("groups", "[I", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL),
                JavaFieldProto::new("searchPosition", "I", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("appendPosition", "I", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("hasMatch", "Z", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::FINAL,
        }
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        pattern: ClassInstanceRef<Pattern>,
        input: ClassInstanceRef<CharSequence>,
    ) -> Result<()> {
        tracing::debug!("java.util.regex.Matcher::<init>({this:?}, {pattern:?}, {input:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        if input.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "input is null").await);
        }

        let source: ClassInstanceRef<String> = jvm.get_field(&pattern, "pattern", "Ljava/lang/String;").await?;
        let flags: i32 = jvm.get_field(&pattern, "flags", "I").await?;
        let source_text = JavaLangString::to_rust_string(jvm, &source).await?;
        let regex = Pattern::build_regex(jvm, &source_text, &source, flags, false).await?;
        let mut groups = jvm.instantiate_array("I", regex.captures_len() * 2).await?;
        jvm.store_array(&mut groups, 0, vec![-1i32; regex.captures_len() * 2]).await?;

        jvm.put_field(&mut this, "parentPattern", "Ljava/util/regex/Pattern;", pattern).await?;
        jvm.put_field(&mut this, "text", "Ljava/lang/CharSequence;", input).await?;
        jvm.put_field(&mut this, "groups", "[I", groups).await?;
        jvm.put_field(&mut this, "searchPosition", "I", 0).await?;
        jvm.put_field(&mut this, "appendPosition", "I", 0).await?;
        jvm.put_field(&mut this, "hasMatch", "Z", false).await
    }

    async fn pattern(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Pattern>> {
        tracing::debug!("java.util.regex.Matcher::pattern({this:?})");

        jvm.get_field(&this, "parentPattern", "Ljava/util/regex/Pattern;").await
    }

    async fn reset(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.util.regex.Matcher::reset({this:?})");

        let mut groups: ClassInstanceRef<Array<i32>> = jvm.get_field(&this, "groups", "[I").await?;
        let length = jvm.array_length(&groups).await?;
        jvm.store_array(&mut groups, 0, vec![-1i32; length]).await?;
        jvm.put_field(&mut this, "searchPosition", "I", 0).await?;
        jvm.put_field(&mut this, "appendPosition", "I", 0).await?;
        jvm.put_field(&mut this, "hasMatch", "Z", false).await?;
        Ok(this)
    }

    async fn reset_with_input(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        input: ClassInstanceRef<CharSequence>,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.util.regex.Matcher::reset({this:?}, {input:?})");

        if input.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "input is null").await);
        }

        jvm.put_field(&mut this, "text", "Ljava/lang/CharSequence;", input).await?;
        let mut groups: ClassInstanceRef<Array<i32>> = jvm.get_field(&this, "groups", "[I").await?;
        let length = jvm.array_length(&groups).await?;
        jvm.store_array(&mut groups, 0, vec![-1i32; length]).await?;
        jvm.put_field(&mut this, "searchPosition", "I", 0).await?;
        jvm.put_field(&mut this, "appendPosition", "I", 0).await?;
        jvm.put_field(&mut this, "hasMatch", "Z", false).await?;
        Ok(this)
    }

    async fn matches(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.regex.Matcher::matches({this:?})");

        Self::execute_match(jvm, &mut this, MatchMode::Full).await
    }

    async fn looking_at(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.regex.Matcher::lookingAt({this:?})");

        Self::execute_match(jvm, &mut this, MatchMode::Prefix).await
    }

    async fn find(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.regex.Matcher::find({this:?})");

        let search_position: i32 = jvm.get_field(&this, "searchPosition", "I").await?;
        if search_position == -1 {
            return Ok(false);
        }

        let has_match: bool = jvm.get_field(&this, "hasMatch", "Z").await?;
        if has_match {
            let groups: ClassInstanceRef<Array<i32>> = jvm.get_field(&this, "groups", "[I").await?;
            let range: Vec<i32> = jvm.load_array(&groups, 0, 2).await?;
            if range[0] == range[1] && range[1] == search_position {
                let length = jvm.array_length(&groups).await?;
                let mut groups = groups;
                jvm.store_array(&mut groups, 0, vec![-1i32; length]).await?;
                jvm.put_field(&mut this, "searchPosition", "I", -1).await?;
                jvm.put_field(&mut this, "hasMatch", "Z", false).await?;
                return Ok(false);
            }
        }

        Self::execute_match(
            jvm,
            &mut this,
            MatchMode::Find {
                start: search_position,
                reset: false,
            },
        )
        .await
    }

    async fn find_from(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, start: i32) -> Result<bool> {
        tracing::debug!("java.util.regex.Matcher::find({this:?}, {start})");

        if start < 0 {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "Illegal start index").await);
        }
        let text: ClassInstanceRef<CharSequence> = jvm.get_field(&this, "text", "Ljava/lang/CharSequence;").await?;
        let length: i32 = jvm.invoke_virtual(&text, "length", "()I", ()).await?;
        if start > length {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "Illegal start index").await);
        }

        Self::execute_match(jvm, &mut this, MatchMode::Find { start, reset: true }).await
    }

    async fn start(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.regex.Matcher::start({this:?})");

        let (start, _) = Self::group_range(jvm, &this, 0).await?;
        Ok(start)
    }

    async fn start_group(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, group: i32) -> Result<i32> {
        tracing::debug!("java.util.regex.Matcher::start({this:?}, {group})");

        let (start, _) = Self::group_range(jvm, &this, group).await?;
        Ok(start)
    }

    async fn end(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.regex.Matcher::end({this:?})");

        let (_, end) = Self::group_range(jvm, &this, 0).await?;
        Ok(end)
    }

    async fn end_group(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, group: i32) -> Result<i32> {
        tracing::debug!("java.util.regex.Matcher::end({this:?}, {group})");

        let (_, end) = Self::group_range(jvm, &this, group).await?;
        Ok(end)
    }

    async fn group(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.regex.Matcher::group({this:?})");

        let (start, end) = Self::group_range(jvm, &this, 0).await?;
        let text: ClassInstanceRef<CharSequence> = jvm.get_field(&this, "text", "Ljava/lang/CharSequence;").await?;
        let group: ClassInstanceRef<CharSequence> = jvm
            .invoke_virtual(&text, "subSequence", "(II)Ljava/lang/CharSequence;", (start, end))
            .await?;
        jvm.invoke_virtual(&group, "toString", "()Ljava/lang/String;", ()).await
    }

    async fn group_index(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, group: i32) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.regex.Matcher::group({this:?}, {group})");

        let (start, end) = Self::group_range(jvm, &this, group).await?;
        if start < 0 {
            return Ok(None.into());
        }
        let text: ClassInstanceRef<CharSequence> = jvm.get_field(&this, "text", "Ljava/lang/CharSequence;").await?;
        let group: ClassInstanceRef<CharSequence> = jvm
            .invoke_virtual(&text, "subSequence", "(II)Ljava/lang/CharSequence;", (start, end))
            .await?;
        jvm.invoke_virtual(&group, "toString", "()Ljava/lang/String;", ()).await
    }

    async fn group_count(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.regex.Matcher::groupCount({this:?})");

        let groups: ClassInstanceRef<Array<i32>> = jvm.get_field(&this, "groups", "[I").await?;
        Ok(jvm.array_length(&groups).await? as i32 / 2 - 1)
    }

    async fn append_replacement(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        buffer: ClassInstanceRef<StringBuffer>,
        replacement: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.util.regex.Matcher::appendReplacement({this:?}, {buffer:?}, {replacement:?})");

        let has_match: bool = jvm.get_field(&this, "hasMatch", "Z").await?;
        if !has_match {
            return Err(jvm.exception("java/lang/IllegalStateException", "No match available").await);
        }

        let expanded = Self::expand_replacement(jvm, &this, &replacement).await?;
        let groups: ClassInstanceRef<Array<i32>> = jvm.get_field(&this, "groups", "[I").await?;
        let match_range: Vec<i32> = jvm.load_array(&groups, 0, 2).await?;
        let append_position: i32 = jvm.get_field(&this, "appendPosition", "I").await?;
        let text: ClassInstanceRef<CharSequence> = jvm.get_field(&this, "text", "Ljava/lang/CharSequence;").await?;
        let prefix: ClassInstanceRef<CharSequence> = jvm
            .invoke_virtual(&text, "subSequence", "(II)Ljava/lang/CharSequence;", (append_position, match_range[0]))
            .await?;
        let prefix: ClassInstanceRef<String> = jvm.invoke_virtual(&prefix, "toString", "()Ljava/lang/String;", ()).await?;
        let expanded = JavaLangString::from_utf16(jvm, expanded).await?;

        if buffer.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "buffer is null").await);
        }
        let _: ClassInstanceRef<StringBuffer> = jvm
            .invoke_virtual(&buffer, "append", "(Ljava/lang/String;)Ljava/lang/StringBuffer;", (prefix,))
            .await?;
        let _: ClassInstanceRef<StringBuffer> = jvm
            .invoke_virtual(&buffer, "append", "(Ljava/lang/String;)Ljava/lang/StringBuffer;", (expanded,))
            .await?;
        jvm.put_field(&mut this, "appendPosition", "I", match_range[1]).await?;
        Ok(this)
    }

    async fn append_tail(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        buffer: ClassInstanceRef<StringBuffer>,
    ) -> Result<ClassInstanceRef<StringBuffer>> {
        tracing::debug!("java.util.regex.Matcher::appendTail({this:?}, {buffer:?})");

        if buffer.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "buffer is null").await);
        }

        let append_position: i32 = jvm.get_field(&this, "appendPosition", "I").await?;
        let text: ClassInstanceRef<CharSequence> = jvm.get_field(&this, "text", "Ljava/lang/CharSequence;").await?;
        let length: i32 = jvm.invoke_virtual(&text, "length", "()I", ()).await?;
        let tail: ClassInstanceRef<CharSequence> = jvm
            .invoke_virtual(&text, "subSequence", "(II)Ljava/lang/CharSequence;", (append_position, length))
            .await?;
        let tail: ClassInstanceRef<String> = jvm.invoke_virtual(&tail, "toString", "()Ljava/lang/String;", ()).await?;
        jvm.invoke_virtual(&buffer, "append", "(Ljava/lang/String;)Ljava/lang/StringBuffer;", (tail,))
            .await
    }

    async fn replace_all(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        replacement: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.regex.Matcher::replaceAll({this:?}, {replacement:?})");

        let _: ClassInstanceRef<Self> = jvm.invoke_virtual(&this, "reset", "()Ljava/util/regex/Matcher;", ()).await?;
        if !jvm.invoke_virtual::<_, bool>(&this, "find", "()Z", ()).await? {
            let text: ClassInstanceRef<CharSequence> = jvm.get_field(&this, "text", "Ljava/lang/CharSequence;").await?;
            return jvm.invoke_virtual(&text, "toString", "()Ljava/lang/String;", ()).await;
        }

        let buffer: ClassInstanceRef<StringBuffer> = jvm.new_class("java/lang/StringBuffer", "()V", ()).await?.into();
        loop {
            let _: ClassInstanceRef<Self> = jvm
                .invoke_virtual(
                    &this,
                    "appendReplacement",
                    "(Ljava/lang/StringBuffer;Ljava/lang/String;)Ljava/util/regex/Matcher;",
                    (buffer.clone(), replacement.clone()),
                )
                .await?;
            if !jvm.invoke_virtual::<_, bool>(&this, "find", "()Z", ()).await? {
                break;
            }
        }
        let _: ClassInstanceRef<StringBuffer> = jvm
            .invoke_virtual(
                &this,
                "appendTail",
                "(Ljava/lang/StringBuffer;)Ljava/lang/StringBuffer;",
                (buffer.clone(),),
            )
            .await?;
        jvm.invoke_virtual(&buffer, "toString", "()Ljava/lang/String;", ()).await
    }

    async fn replace_first(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        replacement: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.regex.Matcher::replaceFirst({this:?}, {replacement:?})");

        let _: ClassInstanceRef<Self> = jvm.invoke_virtual(&this, "reset", "()Ljava/util/regex/Matcher;", ()).await?;
        if !jvm.invoke_virtual::<_, bool>(&this, "find", "()Z", ()).await? {
            let text: ClassInstanceRef<CharSequence> = jvm.get_field(&this, "text", "Ljava/lang/CharSequence;").await?;
            return jvm.invoke_virtual(&text, "toString", "()Ljava/lang/String;", ()).await;
        }

        let buffer: ClassInstanceRef<StringBuffer> = jvm.new_class("java/lang/StringBuffer", "()V", ()).await?.into();
        let _: ClassInstanceRef<Self> = jvm
            .invoke_virtual(
                &this,
                "appendReplacement",
                "(Ljava/lang/StringBuffer;Ljava/lang/String;)Ljava/util/regex/Matcher;",
                (buffer.clone(), replacement),
            )
            .await?;
        let _: ClassInstanceRef<StringBuffer> = jvm
            .invoke_virtual(
                &this,
                "appendTail",
                "(Ljava/lang/StringBuffer;)Ljava/lang/StringBuffer;",
                (buffer.clone(),),
            )
            .await?;
        jvm.invoke_virtual(&buffer, "toString", "()Ljava/lang/String;", ()).await
    }

    async fn group_range(jvm: &Jvm, this: &ClassInstanceRef<Self>, group: i32) -> Result<(i32, i32)> {
        let has_match: bool = jvm.get_field(this, "hasMatch", "Z").await?;
        if !has_match {
            return Err(jvm.exception("java/lang/IllegalStateException", "No match found").await);
        }

        let groups: ClassInstanceRef<Array<i32>> = jvm.get_field(this, "groups", "[I").await?;
        let length = jvm.array_length(&groups).await?;
        if group < 0 || group as usize >= length / 2 {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "No group with this index").await);
        }
        let range: Vec<i32> = jvm.load_array(&groups, group as usize * 2, 2).await?;
        Ok((range[0], range[1]))
    }

    async fn expand_replacement(jvm: &Jvm, this: &ClassInstanceRef<Self>, replacement: &ClassInstanceRef<String>) -> Result<Vec<u16>> {
        if replacement.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "replacement is null").await);
        }

        let replacement = JavaLangString::to_utf16(jvm, replacement).await?;
        let groups: ClassInstanceRef<Array<i32>> = jvm.get_field(this, "groups", "[I").await?;
        let group_count = jvm.array_length(&groups).await? / 2 - 1;
        let ranges: Vec<i32> = jvm.load_array(&groups, 0, (group_count + 1) * 2).await?;
        let text: ClassInstanceRef<CharSequence> = jvm.get_field(this, "text", "Ljava/lang/CharSequence;").await?;
        let mut expanded = Vec::new();
        let mut index = 0;

        while index < replacement.len() {
            match replacement[index] {
                value if value == '\\' as u16 => {
                    index += 1;
                    if index == replacement.len() {
                        return Err(jvm
                            .exception("java/lang/StringIndexOutOfBoundsException", "character to be escaped is missing")
                            .await);
                    }
                    expanded.push(replacement[index]);
                    index += 1;
                }
                value if value == '$' as u16 => {
                    index += 1;
                    if index == replacement.len() {
                        return Err(jvm
                            .exception("java/lang/StringIndexOutOfBoundsException", "group reference is missing")
                            .await);
                    }
                    if !(b'0' as u16..=b'9' as u16).contains(&replacement[index]) {
                        return Err(jvm.exception("java/lang/IllegalArgumentException", "Illegal group reference").await);
                    }

                    let mut group = (replacement[index] - '0' as u16) as usize;
                    index += 1;
                    if group > group_count {
                        return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "No group with this index").await);
                    }
                    while index < replacement.len() && (b'0' as u16..=b'9' as u16).contains(&replacement[index]) {
                        let candidate = group * 10 + (replacement[index] - '0' as u16) as usize;
                        if candidate > group_count {
                            break;
                        }
                        group = candidate;
                        index += 1;
                    }

                    let start = ranges[group * 2];
                    if start >= 0 {
                        let group: ClassInstanceRef<CharSequence> = jvm
                            .invoke_virtual(&text, "subSequence", "(II)Ljava/lang/CharSequence;", (start, ranges[group * 2 + 1]))
                            .await?;
                        let group: ClassInstanceRef<String> = jvm.invoke_virtual(&group, "toString", "()Ljava/lang/String;", ()).await?;
                        expanded.extend(JavaLangString::to_utf16(jvm, &group).await?);
                    }
                }
                value => {
                    expanded.push(value);
                    index += 1;
                }
            }
        }

        Ok(expanded)
    }

    async fn execute_match(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, mode: MatchMode) -> Result<bool> {
        if let MatchMode::Find { start, reset: true } = mode {
            let mut groups: ClassInstanceRef<Array<i32>> = jvm.get_field(this, "groups", "[I").await?;
            let length = jvm.array_length(&groups).await?;
            jvm.store_array(&mut groups, 0, vec![-1i32; length]).await?;
            jvm.put_field(this, "searchPosition", "I", start).await?;
            jvm.put_field(this, "appendPosition", "I", 0).await?;
            jvm.put_field(this, "hasMatch", "Z", false).await?;
        }

        let text: ClassInstanceRef<CharSequence> = jvm.get_field(this, "text", "Ljava/lang/CharSequence;").await?;
        let snapshot: ClassInstanceRef<String> = jvm.invoke_virtual(&text, "toString", "()Ljava/lang/String;", ()).await?;
        let utf16 = JavaLangString::to_utf16(jvm, &snapshot).await?;
        let rust = RustString::from_utf16_lossy(&utf16);

        if let MatchMode::Find { start, .. } = mode
            && start as usize > utf16.len()
        {
            let mut groups: ClassInstanceRef<Array<i32>> = jvm.get_field(this, "groups", "[I").await?;
            let length = jvm.array_length(&groups).await?;
            jvm.store_array(&mut groups, 0, vec![-1i32; length]).await?;
            jvm.put_field(this, "searchPosition", "I", -1).await?;
            jvm.put_field(this, "hasMatch", "Z", false).await?;
            return Ok(false);
        }

        let pattern: ClassInstanceRef<Pattern> = jvm.get_field(this, "parentPattern", "Ljava/util/regex/Pattern;").await?;
        let source: ClassInstanceRef<String> = jvm.get_field(&pattern, "pattern", "Ljava/lang/String;").await?;
        let flags: i32 = jvm.get_field(&pattern, "flags", "I").await?;
        let source_text = JavaLangString::to_rust_string(jvm, &source).await?;
        let regex = Pattern::build_regex(jvm, &source_text, &source, flags, matches!(mode, MatchMode::Full)).await?;

        let captures = match mode {
            MatchMode::Full => regex.captures(&rust),
            MatchMode::Prefix => regex
                .captures(&rust)
                .filter(|captures| captures.get(0).is_some_and(|matched| matched.start() == 0)),
            MatchMode::Find { start, .. } => regex.captures_at(&rust, Self::utf16_to_byte(&rust, start as usize)),
        };
        let Some(captures) = captures else {
            let mut groups: ClassInstanceRef<Array<i32>> = jvm.get_field(this, "groups", "[I").await?;
            let length = jvm.array_length(&groups).await?;
            jvm.store_array(&mut groups, 0, vec![-1i32; length]).await?;
            jvm.put_field(this, "searchPosition", "I", if matches!(mode, MatchMode::Find { .. }) { -1 } else { 0 })
                .await?;
            jvm.put_field(this, "hasMatch", "Z", false).await?;
            return Ok(false);
        };

        let matched = captures.get_match();
        let byte_range = (matched.start(), matched.end());
        let ranges = (0..regex.captures_len())
            .flat_map(|index| {
                if let Some(matched) = captures.get(index) {
                    [
                        Self::byte_to_utf16(&rust, matched.start()) as i32,
                        Self::byte_to_utf16(&rust, matched.end()) as i32,
                    ]
                } else {
                    [-1, -1]
                }
            })
            .collect::<Vec<_>>();
        let mut groups: ClassInstanceRef<Array<i32>> = jvm.get_field(this, "groups", "[I").await?;
        jvm.store_array(&mut groups, 0, ranges).await?;

        let end = Self::byte_to_utf16(&rust, byte_range.1) as i32;
        let search_position = if byte_range.0 == byte_range.1 && byte_range.1 < rust.len() {
            end + rust[byte_range.1..].chars().next().map(char::len_utf16).unwrap_or_default() as i32
        } else {
            end
        };
        jvm.put_field(this, "searchPosition", "I", search_position).await?;
        jvm.put_field(this, "hasMatch", "Z", true).await?;
        Ok(true)
    }

    fn byte_to_utf16(value: &str, byte: usize) -> usize {
        value[..byte].chars().map(char::len_utf16).sum()
    }

    fn utf16_to_byte(value: &str, index: usize) -> usize {
        let mut utf16 = 0;
        for (byte, character) in value.char_indices() {
            if utf16 == index {
                return byte;
            }
            utf16 += character.len_utf16();
            if utf16 >= index {
                return byte + character.len_utf8();
            }
        }
        value.len()
    }
}
