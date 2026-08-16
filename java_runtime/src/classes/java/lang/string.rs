use alloc::{
    format,
    string::{String as RustString, ToString},
    vec,
    vec::Vec,
};

use bytemuck::{cast_slice, cast_vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        lang::{Object, System},
        util::{
            Formatter, Locale,
            regex::{Matcher, Pattern},
        },
    },
};

use super::{CharSequence, StringBuffer};

// class java.lang.String
pub struct String;

impl String {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/String",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/io/Serializable", "java/lang/Comparable", "java/lang/CharSequence"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init_empty, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "([B)V", Self::init_with_byte_array, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "([C)V", Self::init_with_char_array, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "([CII)V", Self::init_with_partial_char_array, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(II[C)V", Self::init_with_shared_char_array, MethodAccessFlags::empty()),
                JavaMethodProto::new("<init>", "([BII)V", Self::init_with_partial_byte_array, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "<init>",
                    "([BLjava/lang/String;)V",
                    Self::init_with_byte_array_charset,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "<init>",
                    "([BIILjava/lang/String;)V",
                    Self::init_with_partial_byte_array_charset,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_with_string, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/lang/StringBuffer;)V",
                    Self::init_with_string_buffer,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "equalsIgnoreCase",
                    "(Ljava/lang/String;)Z",
                    Self::equals_ignore_case,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("compareTo", "(Ljava/lang/String;)I", Self::compare_to, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("compareTo", "(Ljava/lang/Object;)I", Self::compare_to, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "compareToIgnoreCase",
                    "(Ljava/lang/String;)I",
                    Self::compare_to_ignore_case,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("charAt", "(I)C", Self::char_at, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getBytes", "()[B", Self::get_bytes, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getBytes", "(Ljava/lang/String;)[B", Self::get_bytes_charset, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getChars", "(II[CI)V", Self::get_chars, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("toCharArray", "()[C", Self::to_char_array, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("toUpperCase", "()Ljava/lang/String;", Self::to_upper_case, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "toUpperCase",
                    "(Ljava/util/Locale;)Ljava/lang/String;",
                    Self::to_upper_case_locale,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("toLowerCase", "()Ljava/lang/String;", Self::to_lower_case, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "toLowerCase",
                    "(Ljava/util/Locale;)Ljava/lang/String;",
                    Self::to_lower_case_locale,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("length", "()I", Self::length, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "concat",
                    "(Ljava/lang/String;)Ljava/lang/String;",
                    Self::concat,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("substring", "(I)Ljava/lang/String;", Self::substring, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("substring", "(II)Ljava/lang/String;", Self::substring_with_end, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "subSequence",
                    "(II)Ljava/lang/CharSequence;",
                    Self::sub_sequence,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("replace", "(CC)Ljava/lang/String;", Self::replace, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("matches", "(Ljava/lang/String;)Z", Self::matches, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "replaceFirst",
                    "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
                    Self::replace_first,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "replaceAll",
                    "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
                    Self::replace_all,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("split", "(Ljava/lang/String;)[Ljava/lang/String;", Self::split, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "split",
                    "(Ljava/lang/String;I)[Ljava/lang/String;",
                    Self::split_with_limit,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "format",
                    "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;",
                    Self::format,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC | MethodAccessFlags::VARARGS,
                ),
                JavaMethodProto::new(
                    "format",
                    "(Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;",
                    Self::format_with_locale,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC | MethodAccessFlags::VARARGS,
                ),
                JavaMethodProto::new(
                    "regionMatches",
                    "(ILjava/lang/String;II)Z",
                    Self::region_matches_case_sensitive,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "regionMatches",
                    "(ZILjava/lang/String;II)Z",
                    Self::region_matches,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("valueOf", "(Z)Ljava/lang/String;", Self::value_of_boolean, MethodAccessFlags::STATIC),
                JavaMethodProto::new("valueOf", "(C)Ljava/lang/String;", Self::value_of_char, MethodAccessFlags::STATIC),
                JavaMethodProto::new("valueOf", "(I)Ljava/lang/String;", Self::value_of_integer, MethodAccessFlags::STATIC),
                JavaMethodProto::new("valueOf", "(J)Ljava/lang/String;", Self::value_of_long, MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "valueOf",
                    "(F)Ljava/lang/String;",
                    Self::value_of_float,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "valueOf",
                    "(D)Ljava/lang/String;",
                    Self::value_of_double,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("valueOf", "([C)Ljava/lang/String;", Self::value_of_char_array, MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "valueOf",
                    "([CII)Ljava/lang/String;",
                    Self::value_of_partial_char_array,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "valueOf",
                    "(Ljava/lang/Object;)Ljava/lang/String;",
                    Self::value_of_object,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("indexOf", "(I)I", Self::index_of, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("indexOf", "(II)I", Self::index_of_from, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("indexOf", "(Ljava/lang/String;)I", Self::index_of_string, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("indexOf", "(Ljava/lang/String;I)I", Self::index_of_string_from, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("lastIndexOf", "(I)I", Self::last_index_of, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("lastIndexOf", "(II)I", Self::last_index_of_from, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "lastIndexOf",
                    "(Ljava/lang/String;)I",
                    Self::last_index_of_string,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "lastIndexOf",
                    "(Ljava/lang/String;I)I",
                    Self::last_index_of_string_from,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "copyValueOf",
                    "([C)Ljava/lang/String;",
                    Self::copy_value_of,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "copyValueOf",
                    "([CII)Ljava/lang/String;",
                    Self::copy_value_of_range,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("trim", "()Ljava/lang/String;", Self::trim, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("startsWith", "(Ljava/lang/String;)Z", Self::starts_with, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "startsWith",
                    "(Ljava/lang/String;I)Z",
                    Self::starts_with_offset,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("endsWith", "(Ljava/lang/String;)Z", Self::ends_with, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "intern",
                    "()Ljava/lang/String;",
                    Self::intern,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::NATIVE,
                ),
            ],
            fields: vec![
                JavaFieldProto::new("value", "[C", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL),
                JavaFieldProto::new("offset", "I", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL),
                JavaFieldProto::new("count", "I", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL),
            ],
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::FINAL,
        }
    }

    async fn value_range(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<(ClassInstanceRef<Array<JavaChar>>, usize, usize)> {
        let value = jvm.get_field(this, "value", "[C").await?;
        let offset: i32 = jvm.get_field(this, "offset", "I").await?;
        let count: i32 = jvm.get_field(this, "count", "I").await?;

        // access flags are not enforced, so bytecode can leave a negative here, which would widen into a huge usize
        if offset < 0 || count < 0 {
            return Err(jvm
                .exception("java/lang/StringIndexOutOfBoundsException", &format!("offset {offset}, count {count}"))
                .await);
        }

        Ok((value, offset as _, count as _))
    }

    async fn init_with_byte_array(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Array<i8>>) -> Result<()> {
        tracing::debug!("java.lang.String::<init>({this:?}, {value:?})");

        let count = jvm.array_length(&value).await? as i32;

        let _: () = jvm
            .invoke_special(&this, "java/lang/String", "<init>", "([BII)V", (value, 0, count))
            .await?;

        Ok(())
    }

    async fn init_with_char_array(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        value: ClassInstanceRef<Array<u16>>,
    ) -> Result<()> {
        tracing::debug!("java.lang.String::<init>({this:?}, {value:?})");

        let count = jvm.array_length(&value).await? as i32;

        let _: () = jvm
            .invoke_special(&this, "java/lang/String", "<init>", "([CII)V", (value, 0, count))
            .await?;

        Ok(())
    }

    async fn init_with_partial_char_array(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        value: ClassInstanceRef<Array<u16>>,
        offset: i32,
        count: i32,
    ) -> Result<()> {
        tracing::debug!("java.lang.String::<init>({this:?}, {value:?}, {offset}, {count})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        if value.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "value is null").await);
        }
        let length = jvm.array_length(&value).await? as i32;
        if offset < 0 || count < 0 || offset > length - count {
            return Err(jvm
                .exception(
                    "java/lang/StringIndexOutOfBoundsException",
                    &format!("offset {offset}, count {count}, length {length}"),
                )
                .await);
        }

        let mut array = jvm.instantiate_array("C", count as _).await?;
        let data: Vec<JavaChar> = jvm.load_array(&value, offset as _, count as _).await?;
        jvm.store_array(&mut array, 0, data).await?;

        jvm.put_field(&mut this, "value", "[C", array).await?;
        jvm.put_field(&mut this, "offset", "I", 0).await?;
        jvm.put_field(&mut this, "count", "I", count).await?;

        Ok(())
    }

    // no validation; trusted internal callers pass a fresh array or an already-validated range
    async fn init_with_shared_char_array(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        offset: i32,
        count: i32,
        value: ClassInstanceRef<Array<JavaChar>>,
    ) -> Result<()> {
        tracing::debug!("java.lang.String::<init>({this:?}, {offset}, {count}, {value:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        if value.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "value is null").await);
        }
        // this constructor is reachable from bytecode, since the runtime does not enforce access flags
        let length = jvm.array_length(&value).await? as i64;
        if offset < 0 || count < 0 || offset as i64 + count as i64 > length {
            return Err(jvm
                .exception(
                    "java/lang/StringIndexOutOfBoundsException",
                    &format!("offset {offset}, count {count}, length {length}"),
                )
                .await);
        }

        jvm.put_field(&mut this, "value", "[C", value).await?;
        jvm.put_field(&mut this, "offset", "I", offset).await?;
        jvm.put_field(&mut this, "count", "I", count).await?;

        Ok(())
    }

    async fn init_with_partial_byte_array(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        value: ClassInstanceRef<Array<i8>>,
        offset: i32,
        count: i32,
    ) -> Result<()> {
        tracing::debug!("java.lang.String::<init>({this:?}, {value:?}, {offset}, {count})");

        let bytes: Vec<i8> = jvm.load_array(&value, offset as _, count as _).await?;

        let charset = System::get_charset(jvm).await?;
        let string = Self::decode_str(&charset, cast_slice(&bytes)).unwrap_or_else(|| RustString::from_utf8_lossy(cast_slice(&bytes)).into_owned());

        let utf16 = string.encode_utf16().collect::<Vec<_>>();

        let length = utf16.len();
        let mut array = jvm.instantiate_array("C", length).await?;
        jvm.store_array(&mut array, 0, utf16).await?;

        let _: () = jvm
            .invoke_special(&this, "java/lang/String", "<init>", "(II[C)V", (0, length as i32, array))
            .await?;

        Ok(())
    }

    async fn init_with_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.String::<init>({this:?}, {value:?})");

        let (original_value, offset, count) = Self::value_range(jvm, &value).await?;
        let length = jvm.array_length(&original_value).await?;

        // JDK 6 semantics: share a full-range original, but copy a substring so `new String(sub)` detaches from a large parent array
        let value = if offset == 0 && count == length {
            original_value
        } else {
            let chars: Vec<JavaChar> = jvm.load_array(&original_value, offset, count).await?;
            let mut array = jvm.instantiate_array("C", count).await?;
            jvm.store_array(&mut array, 0, chars).await?;

            array.into()
        };

        let _: () = jvm
            .invoke_special(&this, "java/lang/String", "<init>", "(II[C)V", (0, count as i32, value))
            .await?;

        Ok(())
    }

    async fn init_with_string_buffer(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        value: ClassInstanceRef<StringBuffer>,
    ) -> Result<()> {
        tracing::debug!("java.lang.String::<init>({this:?}, {value:?})");

        let string: ClassInstanceRef<Self> = jvm
            .invoke_virtual(&value, "java/lang/Object", "toString", "()Ljava/lang/String;", ())
            .await?;

        let _: () = jvm
            .invoke_special(&this, "java/lang/String", "<init>", "(Ljava/lang/String;)V", (string,))
            .await?;

        Ok(())
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.lang.String::equals({this:?}, {other:?})");

        if other.is_null() || !jvm.is_instance(&**other, "java/lang/String") {
            return Ok(false);
        }
        if this.identity() == other.identity() {
            return Ok(true);
        }

        let this_count: i32 = jvm.get_field(&this, "count", "I").await?;
        let other_count: i32 = jvm.get_field(&other, "count", "I").await?;
        if this_count != other_count {
            return Ok(false);
        }

        let this_chars = JavaLangString::to_utf16(jvm, &this).await?;
        let other_chars = JavaLangString::to_utf16(jvm, &other).await?;

        Ok(this_chars == other_chars)
    }

    async fn compare_to(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<i32> {
        tracing::debug!("java.lang.String::compareTo({this:?}, {other:?})");

        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "anotherString is null").await);
        }
        if !jvm.is_instance(&**other, "java/lang/String") {
            return Err(jvm.exception("java/lang/ClassCastException", &other.class_definition().name()).await);
        }

        let other: ClassInstanceRef<Self> = ClassInstanceRef::new(other.instance);
        let this_chars = JavaLangString::to_utf16(jvm, &this).await?;
        let other_chars = JavaLangString::to_utf16(jvm, &other).await?;

        for (&this_char, &other_char) in this_chars.iter().zip(&other_chars) {
            if this_char != other_char {
                return Ok(this_char as i32 - other_char as i32);
            }
        }

        Ok(this_chars.len() as i32 - other_chars.len() as i32)
    }

    async fn compare_to_ignore_case(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.lang.String::compareToIgnoreCase({this:?}, {other:?})");

        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "str is null").await);
        }

        let this_chars = JavaLangString::to_utf16(jvm, &this).await?;
        let other_chars = JavaLangString::to_utf16(jvm, &other).await?;

        for (&this_char, &other_char) in this_chars.iter().zip(&other_chars) {
            if this_char == other_char {
                continue;
            }

            let this_folded = if this_char <= 0x7f {
                (this_char as u8).to_ascii_lowercase() as JavaChar
            } else {
                char::from_u32(this_char as u32)
                    .and_then(|value| value.to_uppercase().next())
                    .and_then(|value| value.to_lowercase().next())
                    .map(|value| value as JavaChar)
                    .unwrap_or(this_char)
            };
            let other_folded = if other_char <= 0x7f {
                (other_char as u8).to_ascii_lowercase() as JavaChar
            } else {
                char::from_u32(other_char as u32)
                    .and_then(|value| value.to_uppercase().next())
                    .and_then(|value| value.to_lowercase().next())
                    .map(|value| value as JavaChar)
                    .unwrap_or(other_char)
            };
            if this_folded != other_folded {
                return Ok(this_folded as i32 - other_folded as i32);
            }
        }

        Ok(this_chars.len() as i32 - other_chars.len() as i32)
    }

    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.lang.String::hashCode({this:?})");

        let chars = JavaLangString::to_utf16(jvm, &this).await?;

        let hash = chars.iter().fold(0i32, |acc, &c| acc.wrapping_mul(31).wrapping_add(c as i32));

        Ok(hash)
    }

    async fn to_string(_jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::toString({this:?})");

        Ok(this)
    }

    async fn char_at(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<u16> {
        tracing::debug!("java.lang.String::charAt({this:?}, {index})");

        let (value, offset, count) = Self::value_range(jvm, &this).await?;
        if index < 0 || index as usize >= count {
            return Err(jvm
                .exception("java/lang/StringIndexOutOfBoundsException", &format!("index {index}, length {count}"))
                .await);
        }

        Ok(jvm.load_array(&value, offset + index as usize, 1).await?[0])
    }

    async fn concat(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        other: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::concat({this:?}, {other:?})");

        let this_string = JavaLangString::to_rust_string(jvm, &this.clone()).await?;
        let other_string = JavaLangString::to_rust_string(jvm, &other.clone()).await?;

        let concat = this_string + &other_string;

        Ok(JavaLangString::from_rust_string(jvm, &concat).await?.into())
    }

    async fn get_bytes(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Array<i8>>> {
        tracing::debug!("java.lang.String::getBytes({this:?})");

        let string = JavaLangString::to_rust_string(jvm, &this.clone()).await?;

        let charset = System::get_charset(jvm).await?;
        let bytes = cast_vec(Self::encode_str(&charset, &string).unwrap_or_else(|| string.as_bytes().to_vec()));

        let mut byte_array = jvm.instantiate_array("B", bytes.len()).await?;
        jvm.array_raw_buffer_mut(&mut byte_array).await?.write(0, &bytes)?;

        Ok(byte_array.into())
    }

    async fn get_chars(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        src_begin: i32,
        src_end: i32,
        mut dst: ClassInstanceRef<Array<JavaChar>>,
        dst_begin: i32,
    ) -> Result<()> {
        tracing::debug!("java.lang.String::getChars({this:?}, {src_begin}, {src_end}, {dst:?}, {dst_begin})");

        let (value, offset, count) = Self::value_range(jvm, &this).await?;
        if src_begin < 0 || src_begin > src_end || src_end as usize > count {
            return Err(jvm
                .exception(
                    "java/lang/StringIndexOutOfBoundsException",
                    &format!("begin {src_begin}, end {src_end}, length {count}"),
                )
                .await);
        }

        let chars: Vec<JavaChar> = jvm
            .load_array(&value, offset + src_begin as usize, (src_end - src_begin) as usize)
            .await?;
        jvm.store_array(&mut dst, dst_begin as _, chars).await?;

        Ok(())
    }

    async fn to_char_array(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Array<JavaChar>>> {
        tracing::debug!("java.lang.String::toCharArray({this:?})");

        let (value, offset, count) = Self::value_range(jvm, &this).await?;
        let chars: Vec<JavaChar> = jvm.load_array(&value, offset, count).await?;

        let mut array = jvm.instantiate_array("C", count).await?;
        jvm.store_array(&mut array, 0, chars).await?;

        Ok(array.into())
    }

    async fn length(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.lang.String::length({this:?})");

        jvm.get_field(&this, "count", "I").await
    }

    async fn substring(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, begin_index: i32) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::substring({this:?}, {begin_index})");

        let (value, offset, count) = Self::value_range(jvm, &this).await?;
        if begin_index < 0 || begin_index as usize > count {
            return Err(jvm
                .exception(
                    "java/lang/StringIndexOutOfBoundsException",
                    &format!("begin {begin_index}, length {count}"),
                )
                .await);
        }
        if begin_index == 0 {
            return Ok(this);
        }

        let new_string = jvm
            .new_class(
                "java/lang/String",
                "(II[C)V",
                ((offset + begin_index as usize) as i32, (count - begin_index as usize) as i32, value),
            )
            .await?;

        Ok(new_string.into())
    }

    async fn substring_with_end(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        begin_index: i32,
        end_index: i32,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::substring({this:?}, {begin_index}, {end_index})");

        let (value, offset, count) = Self::value_range(jvm, &this).await?;
        if begin_index < 0 || end_index as usize > count || begin_index > end_index {
            return Err(jvm
                .exception(
                    "java/lang/StringIndexOutOfBoundsException",
                    &format!("begin {begin_index}, end {end_index}, length {count}"),
                )
                .await);
        }
        if begin_index == 0 && end_index as usize == count {
            return Ok(this);
        }

        let new_string = jvm
            .new_class(
                "java/lang/String",
                "(II[C)V",
                ((offset + begin_index as usize) as i32, end_index - begin_index, value),
            )
            .await?;

        Ok(new_string.into())
    }

    async fn sub_sequence(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        begin_index: i32,
        end_index: i32,
    ) -> Result<ClassInstanceRef<CharSequence>> {
        tracing::debug!("java.lang.String::subSequence({this:?}, {begin_index}, {end_index})");

        jvm.invoke_virtual(&this, "java/lang/String", "substring", "(II)Ljava/lang/String;", (begin_index, end_index))
            .await
    }

    async fn value_of_char(jvm: &Jvm, _: &mut RuntimeContext, value: JavaChar) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::valueOf({value})");

        // build through [C so an unpaired surrogate is preserved
        let mut chars = jvm.instantiate_array("C", 1).await?;
        jvm.store_array(&mut chars, 0, [value]).await?;

        Ok(jvm.new_class("java/lang/String", "(II[C)V", (0, 1, chars)).await?.into())
    }

    async fn value_of_integer(jvm: &Jvm, _: &mut RuntimeContext, value: i32) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::valueOf({value})");

        let string = value.to_string();

        Ok(JavaLangString::from_rust_string(jvm, &string).await?.into())
    }

    async fn value_of_object(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<Object>) -> Result<ClassInstanceRef<Self>> {
        tracing::warn!("stub java.lang.String::valueOf({value:?})");

        Ok(if value.is_null() {
            JavaLangString::from_rust_string(jvm, "null").await?.into()
        } else {
            jvm.invoke_virtual(&value, "java/lang/Object", "toString", "()Ljava/lang/String;", ())
                .await?
        })
    }

    async fn index_of(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, ch: i32) -> Result<i32> {
        tracing::debug!("java.lang.String::indexOf({this:?}, {ch:?})");

        jvm.invoke_virtual(&this, "java/lang/String", "indexOf", "(II)I", (ch, 0)).await
    }

    async fn index_of_from(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, ch: i32, from_index: i32) -> Result<i32> {
        tracing::debug!("java.lang.String::indexOf({this:?}, {ch:?}, {from_index:?})");

        if !(0..=u16::MAX as i32).contains(&ch) {
            return Ok(-1);
        }

        let chars = JavaLangString::to_utf16(jvm, &this).await?;
        let from_index = from_index.max(0) as usize;
        let index = chars
            .get(from_index..)
            .and_then(|chars| chars.iter().position(|&value| value == ch as u16))
            .map(|index| index + from_index);

        Ok(index.map(|index| index as i32).unwrap_or(-1))
    }

    async fn index_of_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, str: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.lang.String::indexOf({this:?}, {str:?})");

        jvm.invoke_virtual(&this, "java/lang/String", "indexOf", "(Ljava/lang/String;I)I", (str, 0))
            .await
    }

    async fn index_of_string_from(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        str: ClassInstanceRef<Self>,
        from_index: i32,
    ) -> Result<i32> {
        tracing::debug!("java.lang.String::indexOf({this:?}, {str:?}, {from_index})");

        if str.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "str is null").await);
        }

        let chars = JavaLangString::to_utf16(jvm, &this).await?;
        let pattern = JavaLangString::to_utf16(jvm, &str).await?;
        let from_index = (from_index.max(0) as usize).min(chars.len());

        if pattern.is_empty() {
            return Ok(from_index as i32);
        }

        let index = chars[from_index..]
            .windows(pattern.len())
            .position(|window| window == pattern)
            .map(|index| index + from_index);

        Ok(index.map(|index| index as i32).unwrap_or(-1))
    }

    async fn last_index_of(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, ch: i32) -> Result<i32> {
        tracing::debug!("java.lang.String::lastIndexOf({this:?}, {ch:?})");

        if !(0..=u16::MAX as i32).contains(&ch) {
            return Ok(-1);
        }

        let chars = JavaLangString::to_utf16(jvm, &this).await?;
        let index = chars.iter().rposition(|&value| value == ch as u16).map(|index| index as i32);

        Ok(index.unwrap_or(-1))
    }

    async fn trim(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::trim({this:?})");

        let (value, offset, count) = Self::value_range(jvm, &this).await?;
        let chars: Vec<JavaChar> = jvm.load_array(&value, offset, count).await?;
        let start = chars.iter().position(|&value| value > 0x20).unwrap_or(chars.len());
        let end = chars.iter().rposition(|&value| value > 0x20).map(|index| index + 1).unwrap_or(start);
        if start == 0 && end == chars.len() {
            return Ok(this);
        }

        Ok(jvm
            .new_class("java/lang/String", "(II[C)V", ((offset + start) as i32, (end - start) as i32, value))
            .await?
            .into())
    }

    async fn to_upper_case(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::toUpperCase({this:?})");

        let string = JavaLangString::to_rust_string(jvm, &this.clone()).await?;

        let upper = string.to_uppercase().to_string();

        Ok(JavaLangString::from_rust_string(jvm, &upper).await?.into())
    }

    async fn to_upper_case_locale(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        locale: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::toUpperCase({this:?}, {locale:?})");

        if locale.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "locale is null").await);
        }

        jvm.invoke_virtual(&this, "java/lang/String", "toUpperCase", "()Ljava/lang/String;", ())
            .await
    }

    async fn starts_with(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, prefix: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.lang.String::startsWith({this:?}, {prefix:?})");

        jvm.invoke_virtual(&this, "java/lang/String", "startsWith", "(Ljava/lang/String;I)Z", (prefix, 0))
            .await
    }

    async fn starts_with_offset(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        prefix: ClassInstanceRef<Self>,
        offset: i32,
    ) -> Result<bool> {
        tracing::debug!("java.lang.String::startsWith({this:?}, {prefix:?}, {offset})");

        if prefix.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "prefix is null").await);
        }
        if offset < 0 {
            return Ok(false);
        }

        let chars = JavaLangString::to_utf16(jvm, &this).await?;
        let prefix = JavaLangString::to_utf16(jvm, &prefix).await?;

        Ok(chars.get(offset as usize..).is_some_and(|chars| chars.starts_with(&prefix)))
    }

    async fn init_empty(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.String::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        let array = jvm.instantiate_array("C", 0).await?;
        jvm.put_field(&mut this, "value", "[C", array).await?;
        jvm.put_field(&mut this, "offset", "I", 0).await?;
        jvm.put_field(&mut this, "count", "I", 0).await?;

        Ok(())
    }

    async fn init_with_byte_array_charset(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        value: ClassInstanceRef<Array<i8>>,
        charset_name: ClassInstanceRef<Self>,
    ) -> Result<()> {
        tracing::debug!("java.lang.String::<init>({this:?}, {value:?}, {charset_name:?})");

        let count = jvm.array_length(&value).await? as i32;

        let _: () = jvm
            .invoke_special(
                &this,
                "java/lang/String",
                "<init>",
                "([BIILjava/lang/String;)V",
                (value, 0, count, charset_name),
            )
            .await?;

        Ok(())
    }

    async fn init_with_partial_byte_array_charset(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        value: ClassInstanceRef<Array<i8>>,
        offset: i32,
        count: i32,
        charset_name: ClassInstanceRef<Self>,
    ) -> Result<()> {
        tracing::debug!("java.lang.String::<init>({this:?}, {value:?}, {offset}, {count}, {charset_name:?})");

        if charset_name.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "charsetName is null").await);
        }

        let bytes: Vec<i8> = jvm.load_array(&value, offset as _, count as _).await?;

        let charset = JavaLangString::to_rust_string(jvm, &charset_name).await?;
        let Some(string) = Self::decode_str(&charset, cast_slice(&bytes)) else {
            return Err(jvm.exception("java/io/UnsupportedEncodingException", &charset).await);
        };

        let utf16 = string.encode_utf16().collect::<Vec<_>>();

        let length = utf16.len();
        let mut array = jvm.instantiate_array("C", length).await?;
        jvm.store_array(&mut array, 0, utf16).await?;

        let _: () = jvm
            .invoke_special(&this, "java/lang/String", "<init>", "(II[C)V", (0, length as i32, array))
            .await?;

        Ok(())
    }

    async fn equals_ignore_case(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.lang.String::equalsIgnoreCase({this:?}, {other:?})");

        if other.is_null() {
            return Ok(false);
        }

        let this_string = JavaLangString::to_rust_string(jvm, &this).await?;
        let other_string = JavaLangString::to_rust_string(jvm, &other).await?;

        Ok(this_string.eq_ignore_ascii_case(&other_string) || this_string.to_lowercase() == other_string.to_lowercase())
    }

    async fn get_bytes_charset(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        charset_name: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<Array<i8>>> {
        tracing::debug!("java.lang.String::getBytes({this:?}, {charset_name:?})");

        if charset_name.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "charsetName is null").await);
        }

        let string = JavaLangString::to_rust_string(jvm, &this).await?;
        let charset = JavaLangString::to_rust_string(jvm, &charset_name).await?;

        let Some(bytes) = Self::encode_str(&charset, &string) else {
            return Err(jvm.exception("java/io/UnsupportedEncodingException", &charset).await);
        };
        let bytes = cast_vec(bytes);

        let mut byte_array = jvm.instantiate_array("B", bytes.len()).await?;
        jvm.array_raw_buffer_mut(&mut byte_array).await?.write(0, &bytes)?;

        Ok(byte_array.into())
    }

    async fn to_lower_case(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::toLowerCase({this:?})");

        let string = JavaLangString::to_rust_string(jvm, &this).await?;
        let lower = string.to_lowercase();

        Ok(JavaLangString::from_rust_string(jvm, &lower).await?.into())
    }

    async fn to_lower_case_locale(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        locale: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::toLowerCase({this:?}, {locale:?})");

        if locale.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "locale is null").await);
        }

        jvm.invoke_virtual(&this, "java/lang/String", "toLowerCase", "()Ljava/lang/String;", ())
            .await
    }

    async fn replace(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        old_char: JavaChar,
        new_char: JavaChar,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::replace({this:?}, {old_char}, {new_char})");

        let chars = JavaLangString::to_utf16(jvm, &this).await?;

        let replaced: Vec<JavaChar> = chars.into_iter().map(|c| if c == old_char { new_char } else { c }).collect();

        let length = replaced.len();
        let mut array = jvm.instantiate_array("C", length).await?;
        jvm.store_array(&mut array, 0, replaced).await?;

        let new_string = jvm.new_class("java/lang/String", "(II[C)V", (0, length as i32, array)).await?;

        Ok(new_string.into())
    }

    async fn matches(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, regex: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.lang.String::matches({this:?}, {regex:?})");

        let input: ClassInstanceRef<CharSequence> = ClassInstanceRef::new(this.instance);
        jvm.invoke_static(
            "java/util/regex/Pattern",
            "matches",
            "(Ljava/lang/String;Ljava/lang/CharSequence;)Z",
            (regex, input),
        )
        .await
    }

    async fn replace_first(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        regex: ClassInstanceRef<Self>,
        replacement: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::replaceFirst({this:?}, {regex:?}, {replacement:?})");

        let pattern: ClassInstanceRef<Pattern> = jvm
            .invoke_static(
                "java/util/regex/Pattern",
                "compile",
                "(Ljava/lang/String;)Ljava/util/regex/Pattern;",
                (regex,),
            )
            .await?;
        let input: ClassInstanceRef<CharSequence> = ClassInstanceRef::new(this.instance);
        let matcher: ClassInstanceRef<Matcher> = jvm
            .invoke_virtual(
                &pattern,
                "java/util/regex/Pattern",
                "matcher",
                "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;",
                (input,),
            )
            .await?;
        jvm.invoke_virtual(
            &matcher,
            "java/util/regex/Matcher",
            "replaceFirst",
            "(Ljava/lang/String;)Ljava/lang/String;",
            (replacement,),
        )
        .await
    }

    async fn replace_all(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        regex: ClassInstanceRef<Self>,
        replacement: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::replaceAll({this:?}, {regex:?}, {replacement:?})");

        let pattern: ClassInstanceRef<Pattern> = jvm
            .invoke_static(
                "java/util/regex/Pattern",
                "compile",
                "(Ljava/lang/String;)Ljava/util/regex/Pattern;",
                (regex,),
            )
            .await?;
        let input: ClassInstanceRef<CharSequence> = ClassInstanceRef::new(this.instance);
        let matcher: ClassInstanceRef<Matcher> = jvm
            .invoke_virtual(
                &pattern,
                "java/util/regex/Pattern",
                "matcher",
                "(Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;",
                (input,),
            )
            .await?;
        jvm.invoke_virtual(
            &matcher,
            "java/util/regex/Matcher",
            "replaceAll",
            "(Ljava/lang/String;)Ljava/lang/String;",
            (replacement,),
        )
        .await
    }

    async fn split(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        regex: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<Array<Self>>> {
        tracing::debug!("java.lang.String::split({this:?}, {regex:?})");

        let pattern: ClassInstanceRef<Pattern> = jvm
            .invoke_static(
                "java/util/regex/Pattern",
                "compile",
                "(Ljava/lang/String;)Ljava/util/regex/Pattern;",
                (regex,),
            )
            .await?;
        let input: ClassInstanceRef<CharSequence> = ClassInstanceRef::new(this.instance);
        jvm.invoke_virtual(
            &pattern,
            "java/util/regex/Pattern",
            "split",
            "(Ljava/lang/CharSequence;)[Ljava/lang/String;",
            (input,),
        )
        .await
    }

    async fn split_with_limit(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        regex: ClassInstanceRef<Self>,
        limit: i32,
    ) -> Result<ClassInstanceRef<Array<Self>>> {
        tracing::debug!("java.lang.String::split({this:?}, {regex:?}, {limit})");

        let pattern: ClassInstanceRef<Pattern> = jvm
            .invoke_static(
                "java/util/regex/Pattern",
                "compile",
                "(Ljava/lang/String;)Ljava/util/regex/Pattern;",
                (regex,),
            )
            .await?;
        let input: ClassInstanceRef<CharSequence> = ClassInstanceRef::new(this.instance);
        jvm.invoke_virtual(
            &pattern,
            "java/util/regex/Pattern",
            "split",
            "(Ljava/lang/CharSequence;I)[Ljava/lang/String;",
            (input, limit),
        )
        .await
    }

    async fn format(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        format: ClassInstanceRef<Self>,
        arguments: ClassInstanceRef<Array<Object>>,
    ) -> Result<ClassInstanceRef<Self>> {
        let formatter: ClassInstanceRef<Formatter> = jvm.new_class("java/util/Formatter", "()V", ()).await?.into();
        let _: ClassInstanceRef<Formatter> = jvm
            .invoke_virtual(
                &formatter,
                "java/util/Formatter",
                "format",
                "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/util/Formatter;",
                (format, arguments),
            )
            .await?;
        jvm.invoke_virtual(&formatter, "java/util/Formatter", "toString", "()Ljava/lang/String;", ())
            .await
    }

    async fn format_with_locale(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        locale: ClassInstanceRef<Locale>,
        format: ClassInstanceRef<Self>,
        arguments: ClassInstanceRef<Array<Object>>,
    ) -> Result<ClassInstanceRef<Self>> {
        let formatter: ClassInstanceRef<Formatter> = jvm.new_class("java/util/Formatter", "(Ljava/util/Locale;)V", (locale,)).await?.into();
        let _: ClassInstanceRef<Formatter> = jvm
            .invoke_virtual(
                &formatter,
                "java/util/Formatter",
                "format",
                "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/util/Formatter;",
                (format, arguments),
            )
            .await?;
        jvm.invoke_virtual(&formatter, "java/util/Formatter", "toString", "()Ljava/lang/String;", ())
            .await
    }

    #[allow(clippy::too_many_arguments)]
    async fn region_matches(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        ignore_case: bool,
        toffset: i32,
        other: ClassInstanceRef<Self>,
        ooffset: i32,
        len: i32,
    ) -> Result<bool> {
        tracing::debug!("java.lang.String::regionMatches({this:?}, {ignore_case}, {toffset}, {other:?}, {ooffset}, {len})");

        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "other is null").await);
        }

        if toffset < 0 || ooffset < 0 {
            return Ok(false);
        }

        let (this_value, this_offset, this_count) = Self::value_range(jvm, &this).await?;
        let (other_value, other_offset, other_count) = Self::value_range(jvm, &other).await?;
        // widened like the jdk does, so a len near i32::MAX fails the bounds test instead of overflowing
        if toffset as i64 > this_count as i64 - len as i64 || ooffset as i64 > other_count as i64 - len as i64 {
            return Ok(false);
        }
        // the jdk's comparison loop never runs for a non-positive len, so an in-range region trivially matches
        if len <= 0 {
            return Ok(true);
        }

        let this_chars: Vec<JavaChar> = jvm.load_array(&this_value, this_offset + toffset as usize, len as usize).await?;
        let other_chars: Vec<JavaChar> = jvm.load_array(&other_value, other_offset + ooffset as usize, len as usize).await?;

        if ignore_case {
            let to_lower = |c: JavaChar| -> JavaChar {
                char::from_u32(c as u32)
                    .map(|ch| ch.to_lowercase().next().unwrap_or(ch) as u32 as u16)
                    .unwrap_or(c)
            };
            Ok(this_chars.iter().copied().map(to_lower).eq(other_chars.iter().copied().map(to_lower)))
        } else {
            Ok(this_chars == other_chars)
        }
    }

    async fn region_matches_case_sensitive(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        toffset: i32,
        other: ClassInstanceRef<Self>,
        ooffset: i32,
        len: i32,
    ) -> Result<bool> {
        tracing::debug!("java.lang.String::regionMatches({this:?}, {toffset}, {other:?}, {ooffset}, {len})");

        jvm.invoke_virtual(
            &this,
            "java/lang/String",
            "regionMatches",
            "(ZILjava/lang/String;II)Z",
            (false, toffset, other, ooffset, len),
        )
        .await
    }

    async fn last_index_of_from(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, ch: i32, from_index: i32) -> Result<i32> {
        tracing::debug!("java.lang.String::lastIndexOf({this:?}, {ch}, {from_index})");

        if from_index < 0 {
            return Ok(-1);
        }

        if !(0..=u16::MAX as i32).contains(&ch) {
            return Ok(-1);
        }

        let chars = JavaLangString::to_utf16(jvm, &this).await?;
        let end = (from_index as usize + 1).min(chars.len());

        let index = chars[..end].iter().rposition(|&value| value == ch as u16).map(|index| index as i32);

        Ok(index.unwrap_or(-1))
    }

    async fn last_index_of_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, str: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.lang.String::lastIndexOf({this:?}, {str:?})");

        let length: i32 = jvm.get_field(&this, "count", "I").await?;
        jvm.invoke_virtual(&this, "java/lang/String", "lastIndexOf", "(Ljava/lang/String;I)I", (str, length))
            .await
    }

    async fn last_index_of_string_from(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        str: ClassInstanceRef<Self>,
        from_index: i32,
    ) -> Result<i32> {
        tracing::debug!("java.lang.String::lastIndexOf({this:?}, {str:?}, {from_index})");

        if str.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "str is null").await);
        }
        if from_index < 0 {
            return Ok(-1);
        }

        let chars = JavaLangString::to_utf16(jvm, &this).await?;
        let pattern = JavaLangString::to_utf16(jvm, &str).await?;

        if pattern.is_empty() {
            return Ok((from_index as usize).min(chars.len()) as i32);
        }
        if pattern.len() > chars.len() {
            return Ok(-1);
        }

        let last_start = (from_index as usize).min(chars.len() - pattern.len());
        for index in (0..=last_start).rev() {
            if chars[index..].starts_with(&pattern) {
                return Ok(index as i32);
            }
        }

        Ok(-1)
    }

    async fn ends_with(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, suffix: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.lang.String::endsWith({this:?}, {suffix:?})");

        if suffix.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "suffix is null").await);
        }

        let this_chars = JavaLangString::to_utf16(jvm, &this).await?;
        let suffix_chars = JavaLangString::to_utf16(jvm, &suffix).await?;

        Ok(this_chars.ends_with(&suffix_chars))
    }

    async fn intern(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::intern({this:?})");

        let utf16 = JavaLangString::to_utf16(jvm, &this).await?;

        let receiver = this.instance.unwrap();

        Ok(jvm.intern_string_instance(receiver, &utf16).into())
    }

    async fn value_of_boolean(jvm: &Jvm, _: &mut RuntimeContext, value: bool) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::valueOf({value})");

        let string = if value { "true" } else { "false" };
        Ok(JavaLangString::from_rust_string(jvm, string).await?.into())
    }

    async fn value_of_long(jvm: &Jvm, _: &mut RuntimeContext, value: i64) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::valueOf({value})");

        Ok(JavaLangString::from_rust_string(jvm, &value.to_string()).await?.into())
    }

    async fn value_of_float(jvm: &Jvm, _: &mut RuntimeContext, value: f32) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::valueOf({value})");

        jvm.invoke_static("java/lang/Float", "toString", "(F)Ljava/lang/String;", (value,)).await
    }

    async fn value_of_double(jvm: &Jvm, _: &mut RuntimeContext, value: f64) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::valueOf({value})");

        jvm.invoke_static("java/lang/Double", "toString", "(D)Ljava/lang/String;", (value,)).await
    }

    async fn value_of_char_array(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<Array<JavaChar>>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::valueOf({value:?})");

        let new_string = jvm.new_class("java/lang/String", "([C)V", (value,)).await?;

        Ok(new_string.into())
    }

    async fn value_of_partial_char_array(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        value: ClassInstanceRef<Array<JavaChar>>,
        offset: i32,
        count: i32,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::valueOf({value:?}, {offset}, {count})");

        let new_string = jvm.new_class("java/lang/String", "([CII)V", (value, offset, count)).await?;

        Ok(new_string.into())
    }

    async fn copy_value_of(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<Array<JavaChar>>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::copyValueOf({value:?})");

        if value.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "data is null").await);
        }

        let length = jvm.array_length(&value).await? as i32;
        Ok(jvm.new_class("java/lang/String", "([CII)V", (value, 0, length)).await?.into())
    }

    async fn copy_value_of_range(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        value: ClassInstanceRef<Array<JavaChar>>,
        offset: i32,
        count: i32,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::copyValueOf({value:?}, {offset}, {count})");

        if value.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "data is null").await);
        }
        let length = jvm.array_length(&value).await? as i32;
        if offset < 0 || count < 0 || offset > length - count {
            return Err(jvm
                .exception(
                    "java/lang/StringIndexOutOfBoundsException",
                    &format!("offset {offset}, count {count}, length {length}"),
                )
                .await);
        }

        Ok(jvm.new_class("java/lang/String", "([CII)V", (value, offset, count)).await?.into())
    }

    fn decode_str(charset: &str, bytes: &[u8]) -> Option<RustString> {
        Some(match charset.to_ascii_uppercase().replace('_', "-").as_str() {
            "UTF-8" | "UTF8" => RustString::from_utf8_lossy(bytes).into_owned(),
            "EUC-KR" | "EUCKR" | "KS-C-5601-1987" | "MS949" | "CP949" => encoding_rs::EUC_KR.decode(bytes).0.to_string(),
            "ISO-8859-1" | "LATIN1" | "US-ASCII" | "ASCII" => bytes.iter().map(|&b| b as char).collect(),
            _ => return None,
        })
    }

    fn encode_str(charset: &str, string: &str) -> Option<Vec<u8>> {
        Some(match charset.to_ascii_uppercase().replace('_', "-").as_str() {
            "UTF-8" | "UTF8" => string.as_bytes().to_vec(),
            "EUC-KR" | "EUCKR" | "KS-C-5601-1987" | "MS949" | "CP949" => encoding_rs::EUC_KR.encode(string).0.to_vec(),
            "ISO-8859-1" | "LATIN1" => string.chars().map(|c| if (c as u32) <= 0xff { c as u8 } else { b'?' }).collect(),
            "US-ASCII" | "ASCII" => string.chars().map(|c| if c.is_ascii() { c as u8 } else { b'?' }).collect(),
            _ => return None,
        })
    }
}
