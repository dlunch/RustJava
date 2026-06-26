use core::cmp::Ordering;

use alloc::{
    format,
    string::{String as RustString, ToString},
    vec,
    vec::Vec,
};

use bytemuck::{cast_slice, cast_vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::MethodAccessFlags;
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::lang::{Object, System},
};

use super::StringBuffer;

// class java.lang.String
pub struct String;

impl String {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/String",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init_empty, Default::default()),
                JavaMethodProto::new("<init>", "([B)V", Self::init_with_byte_array, Default::default()),
                JavaMethodProto::new("<init>", "([C)V", Self::init_with_char_array, Default::default()),
                JavaMethodProto::new("<init>", "([CII)V", Self::init_with_partial_char_array, Default::default()),
                JavaMethodProto::new("<init>", "([BII)V", Self::init_with_partial_byte_array, Default::default()),
                JavaMethodProto::new(
                    "<init>",
                    "([BLjava/lang/String;)V",
                    Self::init_with_byte_array_charset,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "<init>",
                    "([BIILjava/lang/String;)V",
                    Self::init_with_partial_byte_array_charset,
                    Default::default(),
                ),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_with_string, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/StringBuffer;)V", Self::init_with_string_buffer, Default::default()),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, Default::default()),
                JavaMethodProto::new("equalsIgnoreCase", "(Ljava/lang/String;)Z", Self::equals_ignore_case, Default::default()),
                JavaMethodProto::new("compareTo", "(Ljava/lang/String;)I", Self::compare_to, Default::default()),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, Default::default()),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, Default::default()),
                JavaMethodProto::new("charAt", "(I)C", Self::char_at, Default::default()),
                JavaMethodProto::new("getBytes", "()[B", Self::get_bytes, Default::default()),
                JavaMethodProto::new("getBytes", "(Ljava/lang/String;)[B", Self::get_bytes_charset, Default::default()),
                JavaMethodProto::new("getChars", "(II[CI)V", Self::get_chars, Default::default()),
                JavaMethodProto::new("toCharArray", "()[C", Self::to_char_array, Default::default()),
                JavaMethodProto::new("toUpperCase", "()Ljava/lang/String;", Self::to_upper_case, Default::default()),
                JavaMethodProto::new("toLowerCase", "()Ljava/lang/String;", Self::to_lower_case, Default::default()),
                JavaMethodProto::new("length", "()I", Self::length, Default::default()),
                JavaMethodProto::new("concat", "(Ljava/lang/String;)Ljava/lang/String;", Self::concat, Default::default()),
                JavaMethodProto::new("substring", "(I)Ljava/lang/String;", Self::substring, Default::default()),
                JavaMethodProto::new("substring", "(II)Ljava/lang/String;", Self::substring_with_end, Default::default()),
                JavaMethodProto::new("replace", "(CC)Ljava/lang/String;", Self::replace, Default::default()),
                JavaMethodProto::new("regionMatches", "(ZILjava/lang/String;II)Z", Self::region_matches, Default::default()),
                JavaMethodProto::new("valueOf", "(Z)Ljava/lang/String;", Self::value_of_boolean, MethodAccessFlags::STATIC),
                JavaMethodProto::new("valueOf", "(C)Ljava/lang/String;", Self::value_of_char, MethodAccessFlags::STATIC),
                JavaMethodProto::new("valueOf", "(I)Ljava/lang/String;", Self::value_of_integer, MethodAccessFlags::STATIC),
                JavaMethodProto::new("valueOf", "(J)Ljava/lang/String;", Self::value_of_long, MethodAccessFlags::STATIC),
                JavaMethodProto::new("valueOf", "(F)Ljava/lang/String;", Self::value_of_float, MethodAccessFlags::STATIC),
                JavaMethodProto::new("valueOf", "(D)Ljava/lang/String;", Self::value_of_double, MethodAccessFlags::STATIC),
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
                JavaMethodProto::new("indexOf", "(I)I", Self::index_of, Default::default()),
                JavaMethodProto::new("indexOf", "(II)I", Self::index_of_from, Default::default()),
                JavaMethodProto::new("indexOf", "(Ljava/lang/String;)I", Self::index_of_string, Default::default()),
                JavaMethodProto::new("indexOf", "(Ljava/lang/String;I)I", Self::index_of_string_from, Default::default()),
                JavaMethodProto::new("lastIndexOf", "(I)I", Self::last_index_of, Default::default()),
                JavaMethodProto::new("lastIndexOf", "(II)I", Self::last_index_of_from, Default::default()),
                JavaMethodProto::new("trim", "()Ljava/lang/String;", Self::trim, Default::default()),
                JavaMethodProto::new("startsWith", "(Ljava/lang/String;)Z", Self::starts_with, Default::default()),
                JavaMethodProto::new("startsWith", "(Ljava/lang/String;I)Z", Self::starts_with_offset, Default::default()),
                JavaMethodProto::new("endsWith", "(Ljava/lang/String;)Z", Self::ends_with, Default::default()),
                JavaMethodProto::new("intern", "()Ljava/lang/String;", Self::intern, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("value", "[C", Default::default())],
            access_flags: Default::default(),
        }
    }

    async fn init_with_byte_array(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Array<i8>>) -> Result<()> {
        tracing::debug!("java.lang.String::<init>({:?}, {:?})", &this, &value);

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
        tracing::debug!("java.lang.String::<init>({:?}, {:?})", &this, &value);

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
        tracing::debug!("java.lang.String::<init>({:?}, {:?}, {}, {})", &this, &value, offset, count);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        let mut array = jvm.instantiate_array("C", count as _).await?;
        jvm.put_field(&mut this, "value", "[C", array.clone()).await?;

        let data: Vec<JavaChar> = jvm.load_array(&value, offset as _, count as _).await?;
        jvm.store_array(&mut array, 0, data).await?; // TODO we should store value, offset, count like in java

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
        tracing::debug!("java.lang.String::<init>({:?}, {:?}, {}, {})", &this, &value, offset, count);

        let bytes: Vec<i8> = jvm.load_array(&value, offset as _, count as _).await?;

        let charset = System::get_charset(jvm).await?;
        let string = Self::decode_str(&charset, cast_slice(&bytes));

        let utf16 = string.encode_utf16().collect::<Vec<_>>();

        let mut array = jvm.instantiate_array("C", utf16.len()).await?;
        jvm.store_array(&mut array, 0, utf16).await?;

        let _: () = jvm.invoke_special(&this, "java/lang/String", "<init>", "([C)V", [array.into()]).await?;

        Ok(())
    }

    async fn init_with_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.String::<init>({:?}, {:?})", &this, &value);

        let chars: ClassInstanceRef<Array<JavaChar>> = jvm.invoke_virtual(&value, "toCharArray", "()[C", ()).await?;

        let _: () = jvm.invoke_special(&this, "java/lang/String", "<init>", "([C)V", (chars,)).await?;

        Ok(())
    }

    async fn init_with_string_buffer(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        value: ClassInstanceRef<StringBuffer>,
    ) -> Result<()> {
        tracing::debug!("java.lang.String::<init>({:?}, {:?})", &this, &value);

        let string: ClassInstanceRef<Self> = jvm.invoke_virtual(&value, "toString", "()Ljava/lang/String;", ()).await?;

        let _: () = jvm
            .invoke_special(&this, "java/lang/String", "<init>", "(Ljava/lang/String;)V", (string,))
            .await?;

        Ok(())
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.lang.String::equals({:?}, {:?})", &this, &other);

        if other.is_null() {
            return Ok(false);
        }

        let other_string = JavaLangString::to_rust_string(jvm, &other).await?;
        let this_string = JavaLangString::to_rust_string(jvm, &this).await?;

        if this_string == other_string { Ok(true) } else { Ok(false) }
    }

    async fn compare_to(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.lang.String::compareTo({:?}, {:?})", &this, &other);

        let other_string = JavaLangString::to_rust_string(jvm, &other).await?;
        let this_string = JavaLangString::to_rust_string(jvm, &this).await?;

        let compare_result = this_string.cmp(&other_string);

        match compare_result {
            Ordering::Less => Ok(-1),
            Ordering::Equal => Ok(0),
            Ordering::Greater => Ok(1),
        }
    }

    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.lang.String::hashCode({:?})", &this);

        let chars = jvm.get_field(&this, "value", "[C").await?;
        let chars: Vec<JavaChar> = jvm.load_array(&chars, 0, jvm.array_length(&chars).await? as _).await?;

        let hash = chars.iter().fold(0i32, |acc, &c| acc.wrapping_mul(31).wrapping_add(c as i32));

        Ok(hash)
    }

    async fn to_string(_jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::toString({:?})", &this);

        Ok(this)
    }

    async fn char_at(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<u16> {
        tracing::debug!("java.lang.String::charAt({:?}, {})", &this, index);

        let value = jvm.get_field(&this, "value", "[C").await?;

        Ok(jvm.load_array(&value, index as _, 1).await?[0])
    }

    async fn concat(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        other: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::concat({:?}, {:?})", &this, &other);

        let this_string = JavaLangString::to_rust_string(jvm, &this.clone()).await?;
        let other_string = JavaLangString::to_rust_string(jvm, &other.clone()).await?;

        let concat = this_string + &other_string;

        Ok(JavaLangString::from_rust_string(jvm, &concat).await?.into())
    }

    async fn get_bytes(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Array<i8>>> {
        tracing::debug!("java.lang.String::getBytes({:?})", &this);

        let string = JavaLangString::to_rust_string(jvm, &this.clone()).await?;

        let charset = System::get_charset(jvm).await?;
        let bytes = cast_vec(Self::encode_str(&charset, &string));

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
        tracing::debug!(
            "java.lang.String::getChars({:?}, {}, {}, {:?}, {})",
            &this,
            src_begin,
            src_end,
            &dst,
            dst_begin
        );

        let value = jvm.get_field(&this, "value", "[C").await?;

        let count = src_end - src_begin;
        let chars: Vec<JavaChar> = jvm.load_array(&value, src_begin as _, count as _).await?;
        jvm.store_array(&mut dst, dst_begin as _, chars).await?;

        Ok(())
    }

    async fn to_char_array(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Array<JavaChar>>> {
        tracing::debug!("java.lang.String::toCharArray({:?})", &this);

        let value = jvm.get_field(&this, "value", "[C").await?;

        Ok(value)
    }

    async fn length(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.lang.String::length({:?})", &this);

        let value = jvm.get_field(&this, "value", "[C").await?;

        Ok(jvm.array_length(&value).await? as _)
    }

    async fn substring(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, begin_index: i32) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::substring({:?}, {})", &this, begin_index);

        let string = JavaLangString::to_rust_string(jvm, &this.clone()).await?;

        // java string indices are in utf-16 code units
        let utf16 = string.encode_utf16().collect::<Vec<_>>();

        let length = utf16.len() as i32;
        if begin_index < 0 || begin_index > length {
            return Err(jvm
                .exception(
                    "java/lang/StringIndexOutOfBoundsException",
                    &format!("begin {begin_index}, length {length}"),
                )
                .await);
        }

        let substr = RustString::from_utf16_lossy(&utf16[begin_index as usize..]); // TODO buffer sharing

        Ok(JavaLangString::from_rust_string(jvm, &substr).await?.into())
    }

    async fn substring_with_end(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        begin_index: i32,
        end_index: i32,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::substring({:?}, {}, {})", &this, begin_index, end_index);

        let string = JavaLangString::to_rust_string(jvm, &this.clone()).await?;

        // java string indices are in utf-16 code units
        let utf16 = string.encode_utf16().collect::<Vec<_>>();

        let length = utf16.len() as i32;
        if begin_index < 0 || end_index > length || begin_index > end_index {
            return Err(jvm
                .exception(
                    "java/lang/StringIndexOutOfBoundsException",
                    &format!("begin {begin_index}, end {end_index}, length {length}"),
                )
                .await);
        }

        let substr = RustString::from_utf16_lossy(&utf16[begin_index as usize..end_index as usize]); // TODO buffer sharing

        Ok(JavaLangString::from_rust_string(jvm, &substr).await?.into())
    }

    async fn value_of_char(jvm: &Jvm, _: &mut RuntimeContext, value: JavaChar) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::valueOf({})", value);

        let string = RustString::from_utf16(&[value]).unwrap();

        Ok(JavaLangString::from_rust_string(jvm, &string).await?.into())
    }

    async fn value_of_integer(jvm: &Jvm, _: &mut RuntimeContext, value: i32) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::valueOf({})", value);

        let string = value.to_string();

        Ok(JavaLangString::from_rust_string(jvm, &string).await?.into())
    }

    async fn value_of_object(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<Object>) -> Result<ClassInstanceRef<Self>> {
        tracing::warn!("stub java.lang.String::valueOf({:?})", &value);

        Ok(if value.is_null() {
            JavaLangString::from_rust_string(jvm, "null").await?.into()
        } else {
            jvm.invoke_virtual(&value, "toString", "()Ljava/lang/String;", ()).await?
        })
    }

    async fn index_of(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, ch: i32) -> Result<i32> {
        tracing::debug!("java.lang.String::indexOf({:?}, {:?})", &this, ch);

        jvm.invoke_virtual(&this, "indexOf", "(II)I", (ch, 0)).await
    }

    async fn index_of_from(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, ch: i32, from_index: i32) -> Result<i32> {
        tracing::debug!("java.lang.String::indexOf({:?}, {:?}, {:?})", &this, ch, from_index);

        let this_string = JavaLangString::to_rust_string(jvm, &this.clone()).await?;

        let index = this_string
            .chars()
            .skip(from_index as usize)
            .position(|x| x as u32 == ch as u32)
            .map(|x| x as i32 + from_index);

        Ok(index.unwrap_or(-1))
    }

    async fn index_of_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, str: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.lang.String::indexOf({:?}, {:?})", &this, &str);

        jvm.invoke_virtual(&this, "indexOf", "(Ljava/lang/String;I)I", (str, 0)).await
    }

    async fn index_of_string_from(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        str: ClassInstanceRef<Self>,
        from_index: i32,
    ) -> Result<i32> {
        tracing::debug!("java.lang.String::indexOf({:?}, {:?}, {})", &this, &str, from_index);

        let this_string = JavaLangString::to_rust_string(jvm, &this.clone()).await?;
        let str_string = JavaLangString::to_rust_string(jvm, &str.clone()).await?;

        tracing::trace!("this_string: {:?}", this_string);
        tracing::trace!("str_string: {:?}", str_string);

        let chars = this_string.chars().skip(from_index as usize).collect::<Vec<_>>();
        let str_chars = str_string.chars().collect::<Vec<_>>();
        let index = chars.windows(str_chars.len()).position(|x| x == str_chars).map(|x| x as i32 + from_index);

        Ok(index.unwrap_or(-1))
    }

    async fn last_index_of(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, ch: i32) -> Result<i32> {
        tracing::debug!("java.lang.String::lastIndexOf({:?}, {:?})", &this, ch);

        let this_string = JavaLangString::to_rust_string(jvm, &this.clone()).await?;

        let index = this_string
            .chars()
            .collect::<Vec<_>>() // TODO i think we don't need collect..
            .into_iter()
            .rposition(|x| x as u32 == ch as u32)
            .map(|x| x as i32);

        Ok(index.unwrap_or(-1))
    }

    async fn trim(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::trim({:?})", &this);

        let string = JavaLangString::to_rust_string(jvm, &this.clone()).await?;

        let trimmed = string.trim().to_string();

        Ok(JavaLangString::from_rust_string(jvm, &trimmed).await?.into()) // TODO buffer sharing
    }

    async fn to_upper_case(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::toUpperCase({:?})", &this);

        let string = JavaLangString::to_rust_string(jvm, &this.clone()).await?;

        let upper = string.to_uppercase().to_string();

        Ok(JavaLangString::from_rust_string(jvm, &upper).await?.into())
    }

    async fn starts_with(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, prefix: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.lang.String::startsWith({:?}, {:?})", &this, &prefix);

        jvm.invoke_virtual(&this, "startsWith", "(Ljava/lang/String;I)Z", (prefix, 0)).await
    }

    async fn starts_with_offset(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        prefix: ClassInstanceRef<Self>,
        offset: i32,
    ) -> Result<bool> {
        tracing::debug!("java.lang.String::startsWith({:?}, {:?}, {})", &this, &prefix, offset);

        let this_string = JavaLangString::to_rust_string(jvm, &this.clone())
            .await?
            .chars()
            .skip(offset as usize)
            .collect::<RustString>();
        let prefix_string = JavaLangString::to_rust_string(jvm, &prefix.clone()).await?;

        Ok(this_string.starts_with(&prefix_string))
    }

    async fn init_empty(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.String::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        let array = jvm.instantiate_array("C", 0).await?;
        jvm.put_field(&mut this, "value", "[C", array).await?;

        Ok(())
    }

    async fn init_with_byte_array_charset(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        value: ClassInstanceRef<Array<i8>>,
        charset_name: ClassInstanceRef<Self>,
    ) -> Result<()> {
        tracing::debug!("java.lang.String::<init>({:?}, {:?}, {:?})", &this, &value, &charset_name);

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
        tracing::debug!(
            "java.lang.String::<init>({:?}, {:?}, {}, {}, {:?})",
            &this,
            &value,
            offset,
            count,
            &charset_name
        );

        if charset_name.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "charsetName is null").await);
        }

        let bytes: Vec<i8> = jvm.load_array(&value, offset as _, count as _).await?;

        let charset = JavaLangString::to_rust_string(jvm, &charset_name).await?;
        let string = Self::decode_str(&charset, cast_slice(&bytes));

        let utf16 = string.encode_utf16().collect::<Vec<_>>();

        let mut array = jvm.instantiate_array("C", utf16.len()).await?;
        jvm.store_array(&mut array, 0, utf16).await?;

        let _: () = jvm.invoke_special(&this, "java/lang/String", "<init>", "([C)V", [array.into()]).await?;

        Ok(())
    }

    async fn equals_ignore_case(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.lang.String::equalsIgnoreCase({:?}, {:?})", &this, &other);

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
        tracing::debug!("java.lang.String::getBytes({:?}, {:?})", &this, &charset_name);

        if charset_name.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "charsetName is null").await);
        }

        let string = JavaLangString::to_rust_string(jvm, &this).await?;
        let charset = JavaLangString::to_rust_string(jvm, &charset_name).await?;

        let bytes = cast_vec(Self::encode_str(&charset, &string));

        let mut byte_array = jvm.instantiate_array("B", bytes.len()).await?;
        jvm.array_raw_buffer_mut(&mut byte_array).await?.write(0, &bytes)?;

        Ok(byte_array.into())
    }

    async fn to_lower_case(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::toLowerCase({:?})", &this);

        let string = JavaLangString::to_rust_string(jvm, &this).await?;
        let lower = string.to_lowercase();

        Ok(JavaLangString::from_rust_string(jvm, &lower).await?.into())
    }

    async fn replace(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        old_char: JavaChar,
        new_char: JavaChar,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::replace({:?}, {}, {})", &this, old_char, new_char);

        let value = jvm.get_field(&this, "value", "[C").await?;
        let length = jvm.array_length(&value).await?;
        let chars: Vec<JavaChar> = jvm.load_array(&value, 0, length).await?;

        let replaced: Vec<JavaChar> = chars.into_iter().map(|c| if c == old_char { new_char } else { c }).collect();

        let mut array = jvm.instantiate_array("C", replaced.len()).await?;
        jvm.store_array(&mut array, 0, replaced).await?;

        let new_string = jvm.new_class("java/lang/String", "([C)V", (array,)).await?;

        Ok(new_string.into())
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
        tracing::debug!(
            "java.lang.String::regionMatches({:?}, {}, {}, {:?}, {}, {})",
            &this,
            ignore_case,
            toffset,
            &other,
            ooffset,
            len
        );

        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "other is null").await);
        }

        if toffset < 0 || ooffset < 0 || len < 0 {
            return Ok(false);
        }

        let this_string = JavaLangString::to_rust_string(jvm, &this).await?;
        let other_string = JavaLangString::to_rust_string(jvm, &other).await?;

        let this_chars: Vec<u16> = this_string.encode_utf16().collect();
        let other_chars: Vec<u16> = other_string.encode_utf16().collect();

        let end_t = toffset as usize + len as usize;
        let end_o = ooffset as usize + len as usize;
        if end_t > this_chars.len() || end_o > other_chars.len() {
            return Ok(false);
        }

        let this_slice = &this_chars[toffset as usize..end_t];
        let other_slice = &other_chars[ooffset as usize..end_o];

        if ignore_case {
            let to_lower = |c: u16| -> u16 {
                char::from_u32(c as u32)
                    .map(|ch| ch.to_lowercase().next().unwrap_or(ch) as u32 as u16)
                    .unwrap_or(c)
            };
            Ok(this_slice.iter().copied().map(to_lower).eq(other_slice.iter().copied().map(to_lower)))
        } else {
            Ok(this_slice == other_slice)
        }
    }

    async fn last_index_of_from(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, ch: i32, from_index: i32) -> Result<i32> {
        tracing::debug!("java.lang.String::lastIndexOf({:?}, {}, {})", &this, ch, from_index);

        if from_index < 0 {
            return Ok(-1);
        }

        let this_string = JavaLangString::to_rust_string(jvm, &this).await?;
        let chars: Vec<char> = this_string.chars().collect();
        let end = (from_index as usize + 1).min(chars.len());

        let index = chars[..end].iter().rposition(|&c| c as u32 == ch as u32).map(|x| x as i32);

        Ok(index.unwrap_or(-1))
    }

    async fn ends_with(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, suffix: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.lang.String::endsWith({:?}, {:?})", &this, &suffix);

        if suffix.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "suffix is null").await);
        }

        let this_string = JavaLangString::to_rust_string(jvm, &this).await?;
        let suffix_string = JavaLangString::to_rust_string(jvm, &suffix).await?;

        Ok(this_string.ends_with(&suffix_string))
    }

    async fn intern(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::intern({:?})", &this);

        let value = JavaLangString::to_rust_string(jvm, &this).await?;

        Ok(jvm.intern_string(&value).await?.into())
    }

    async fn value_of_boolean(jvm: &Jvm, _: &mut RuntimeContext, value: bool) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::valueOf({})", value);

        let string = if value { "true" } else { "false" };
        Ok(JavaLangString::from_rust_string(jvm, string).await?.into())
    }

    async fn value_of_long(jvm: &Jvm, _: &mut RuntimeContext, value: i64) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::valueOf({})", value);

        Ok(JavaLangString::from_rust_string(jvm, &value.to_string()).await?.into())
    }

    async fn value_of_float(jvm: &Jvm, _: &mut RuntimeContext, value: f32) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::valueOf({})", value);

        Ok(JavaLangString::from_rust_string(jvm, &value.to_string()).await?.into())
    }

    async fn value_of_double(jvm: &Jvm, _: &mut RuntimeContext, value: f64) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::valueOf({})", value);

        Ok(JavaLangString::from_rust_string(jvm, &value.to_string()).await?.into())
    }

    async fn value_of_char_array(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<Array<JavaChar>>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::valueOf({:?})", &value);

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
        tracing::debug!("java.lang.String::valueOf({:?}, {}, {})", &value, offset, count);

        let new_string = jvm.new_class("java/lang/String", "([CII)V", (value, offset, count)).await?;

        Ok(new_string.into())
    }

    fn decode_str(charset: &str, bytes: &[u8]) -> RustString {
        match charset.to_ascii_uppercase().replace('_', "-").as_str() {
            "UTF-8" | "UTF8" => RustString::from_utf8_lossy(bytes).into_owned(),
            "EUC-KR" | "EUCKR" | "KS-C-5601-1987" | "MS949" | "CP949" => encoding_rs::EUC_KR.decode(bytes).0.to_string(),
            "ISO-8859-1" | "LATIN1" | "US-ASCII" | "ASCII" => bytes.iter().map(|&b| b as char).collect(),
            _ => unimplemented!("unsupported charset: {}", charset),
        }
    }

    fn encode_str(charset: &str, string: &str) -> Vec<u8> {
        match charset.to_ascii_uppercase().replace('_', "-").as_str() {
            "UTF-8" | "UTF8" => string.as_bytes().to_vec(),
            "EUC-KR" | "EUCKR" | "KS-C-5601-1987" | "MS949" | "CP949" => encoding_rs::EUC_KR.encode(string).0.to_vec(),
            "ISO-8859-1" | "LATIN1" => string.chars().map(|c| if (c as u32) <= 0xff { c as u8 } else { b'?' }).collect(),
            "US-ASCII" | "ASCII" => string.chars().map(|c| if c.is_ascii() { c as u8 } else { b'?' }).collect(),
            _ => unimplemented!("unsupported charset: {}", charset),
        }
    }
}
