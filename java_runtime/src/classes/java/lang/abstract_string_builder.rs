use alloc::{format, string::ToString, vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::lang::{CharSequence, Object, String, StringBuffer},
};

// abstract class java.lang.AbstractStringBuilder
pub struct AbstractStringBuilder;

impl AbstractStringBuilder {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/AbstractStringBuilder",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/lang/Appendable", "java/lang/CharSequence"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::empty()),
                JavaMethodProto::new("<init>", "(I)V", Self::init_with_capacity, MethodAccessFlags::empty()),
                JavaMethodProto::new("length", "()I", Self::length, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("capacity", "()I", Self::capacity, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("ensureCapacity", "(I)V", Self::ensure_capacity, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("trimToSize", "()V", Self::trim_to_size, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("setLength", "(I)V", Self::set_length, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("charAt", "(I)C", Self::char_at, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("codePointAt", "(I)I", Self::code_point_at, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("codePointBefore", "(I)I", Self::code_point_before, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("codePointCount", "(II)I", Self::code_point_count, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("offsetByCodePoints", "(II)I", Self::offset_by_code_points, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getChars", "(II[CI)V", Self::get_chars, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("setCharAt", "(IC)V", Self::set_char_at, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "append",
                    "(Ljava/lang/Object;)Ljava/lang/AbstractStringBuilder;",
                    Self::append_object,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "(Ljava/lang/String;)Ljava/lang/AbstractStringBuilder;",
                    Self::append_string,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "(Ljava/lang/StringBuffer;)Ljava/lang/AbstractStringBuilder;",
                    Self::append_string_buffer,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "(Ljava/lang/CharSequence;)Ljava/lang/AbstractStringBuilder;",
                    Self::append_char_sequence,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "(Ljava/lang/CharSequence;II)Ljava/lang/AbstractStringBuilder;",
                    Self::append_char_sequence_range,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "([C)Ljava/lang/AbstractStringBuilder;",
                    Self::append_char_array,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "([CII)Ljava/lang/AbstractStringBuilder;",
                    Self::append_char_array_range,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "(Z)Ljava/lang/AbstractStringBuilder;",
                    Self::append_boolean,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "(C)Ljava/lang/AbstractStringBuilder;",
                    Self::append_char,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "(I)Ljava/lang/AbstractStringBuilder;",
                    Self::append_int,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "(J)Ljava/lang/AbstractStringBuilder;",
                    Self::append_long,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "(F)Ljava/lang/AbstractStringBuilder;",
                    Self::append_float,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "(D)Ljava/lang/AbstractStringBuilder;",
                    Self::append_double,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "appendCodePoint",
                    "(I)Ljava/lang/AbstractStringBuilder;",
                    Self::append_code_point,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("delete", "(II)Ljava/lang/AbstractStringBuilder;", Self::delete, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "deleteCharAt",
                    "(I)Ljava/lang/AbstractStringBuilder;",
                    Self::delete_char_at,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "replace",
                    "(IILjava/lang/String;)Ljava/lang/AbstractStringBuilder;",
                    Self::replace,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("substring", "(I)Ljava/lang/String;", Self::substring, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("substring", "(II)Ljava/lang/String;", Self::substring_range, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "subSequence",
                    "(II)Ljava/lang/CharSequence;",
                    Self::sub_sequence,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(I[CII)Ljava/lang/AbstractStringBuilder;",
                    Self::insert_char_array_range,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(ILjava/lang/Object;)Ljava/lang/AbstractStringBuilder;",
                    Self::insert_object,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(ILjava/lang/String;)Ljava/lang/AbstractStringBuilder;",
                    Self::insert_string,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(I[C)Ljava/lang/AbstractStringBuilder;",
                    Self::insert_char_array,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(ILjava/lang/CharSequence;)Ljava/lang/AbstractStringBuilder;",
                    Self::insert_char_sequence,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(ILjava/lang/CharSequence;II)Ljava/lang/AbstractStringBuilder;",
                    Self::insert_char_sequence_range,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(IZ)Ljava/lang/AbstractStringBuilder;",
                    Self::insert_boolean,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(IC)Ljava/lang/AbstractStringBuilder;",
                    Self::insert_char,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(II)Ljava/lang/AbstractStringBuilder;",
                    Self::insert_int,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(IJ)Ljava/lang/AbstractStringBuilder;",
                    Self::insert_long,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(IF)Ljava/lang/AbstractStringBuilder;",
                    Self::insert_float,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(ID)Ljava/lang/AbstractStringBuilder;",
                    Self::insert_double,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("indexOf", "(Ljava/lang/String;)I", Self::index_of, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("indexOf", "(Ljava/lang/String;I)I", Self::index_of_from, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("lastIndexOf", "(Ljava/lang/String;)I", Self::last_index_of, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "lastIndexOf",
                    "(Ljava/lang/String;I)I",
                    Self::last_index_of_from,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("reverse", "()Ljava/lang/AbstractStringBuilder;", Self::reverse, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "append",
                    "(Ljava/lang/CharSequence;)Ljava/lang/Appendable;",
                    Self::append_char_sequence,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "(Ljava/lang/CharSequence;II)Ljava/lang/Appendable;",
                    Self::append_char_sequence_range,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "(C)Ljava/lang/Appendable;",
                    Self::append_char,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
            ],
            fields: vec![
                JavaFieldProto::new("value", "[C", FieldAccessFlags::empty()),
                JavaFieldProto::new("count", "I", FieldAccessFlags::empty()),
            ],
            access_flags: ClassAccessFlags::ABSTRACT,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }

    async fn init_with_capacity(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, capacity: i32) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        if capacity < 0 {
            return Err(jvm.exception("java/lang/NegativeArraySizeException", &capacity.to_string()).await);
        }
        let value = jvm.instantiate_array("C", capacity as usize).await?;
        jvm.put_field(&mut this, "value", "[C", value).await?;
        jvm.put_field(&mut this, "count", "I", 0).await
    }

    async fn characters(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<Vec<JavaChar>> {
        let count: i32 = jvm.get_field(this, "count", "I").await?;
        let value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(this, "value", "[C").await?;
        jvm.load_array(&value, 0, count as usize).await
    }

    async fn replace_characters(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, characters: Vec<JavaChar>) -> Result<()> {
        let value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(this, "value", "[C").await?;
        let old_capacity = jvm.array_length(&value).await?;
        if characters.len() > old_capacity {
            let new_capacity = characters.len().max(old_capacity.saturating_mul(2).saturating_add(2));
            let mut new_value = jvm.instantiate_array("C", new_capacity).await?;
            jvm.store_array(&mut new_value, 0, characters.clone()).await?;
            jvm.put_field(this, "value", "[C", new_value).await?;
        } else if !characters.is_empty() {
            let mut value = value;
            jvm.store_array(&mut value, 0, characters.clone()).await?;
        }
        jvm.put_field(this, "count", "I", characters.len() as i32).await
    }

    async fn insert_characters(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, offset: i32, inserted: Vec<JavaChar>) -> Result<()> {
        let mut characters = Self::characters(jvm, this).await?;
        if offset < 0 || offset as usize > characters.len() {
            return Err(jvm
                .exception(
                    "java/lang/StringIndexOutOfBoundsException",
                    &format!("offset {offset}, length {}", characters.len()),
                )
                .await);
        }
        characters.splice(offset as usize..offset as usize, inserted);
        Self::replace_characters(jvm, this, characters).await
    }

    async fn char_sequence_characters(jvm: &Jvm, sequence: &ClassInstanceRef<CharSequence>) -> Result<Vec<JavaChar>> {
        if sequence.is_null() {
            return Ok("null".encode_utf16().collect());
        }
        if jvm.is_instance(&***sequence, "java/lang/String") {
            return JavaLangString::to_utf16(jvm, sequence).await;
        }
        let length: i32 = jvm
            .invoke_virtual(sequence, &sequence.class_definition().name(), "length", "()I", ())
            .await?;
        let mut characters = Vec::with_capacity(length as usize);
        for index in 0..length {
            characters.push(
                jvm.invoke_virtual(sequence, &sequence.class_definition().name(), "charAt", "(I)C", (index,))
                    .await?,
            );
        }
        Ok(characters)
    }

    async fn length(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "count", "I").await
    }

    async fn capacity(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "value", "[C").await?;
        Ok(jvm.array_length(&value).await? as i32)
    }

    async fn ensure_capacity(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, minimum_capacity: i32) -> Result<()> {
        if minimum_capacity <= 0 {
            return Ok(());
        }
        let value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "value", "[C").await?;
        let old_capacity = jvm.array_length(&value).await? as i32;
        if minimum_capacity <= old_capacity {
            return Ok(());
        }
        let new_capacity = minimum_capacity.max(old_capacity.saturating_mul(2).saturating_add(2));
        let characters = Self::characters(jvm, &this).await?;
        let mut new_value = jvm.instantiate_array("C", new_capacity as usize).await?;
        jvm.store_array(&mut new_value, 0, characters).await?;
        jvm.put_field(&mut this, "value", "[C", new_value).await
    }

    async fn trim_to_size(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let characters = Self::characters(jvm, &this).await?;
        let mut value = jvm.instantiate_array("C", characters.len()).await?;
        jvm.store_array(&mut value, 0, characters).await?;
        jvm.put_field(&mut this, "value", "[C", value).await
    }

    async fn set_length(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, new_length: i32) -> Result<()> {
        if new_length < 0 {
            return Err(jvm.exception("java/lang/StringIndexOutOfBoundsException", &new_length.to_string()).await);
        }
        let mut characters = Self::characters(jvm, &this).await?;
        characters.resize(new_length as usize, 0);
        Self::replace_characters(jvm, &mut this, characters).await
    }

    async fn char_at(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<JavaChar> {
        let characters = Self::characters(jvm, &this).await?;
        if index < 0 || index as usize >= characters.len() {
            return Err(jvm
                .exception(
                    "java/lang/StringIndexOutOfBoundsException",
                    &format!("index {index}, length {}", characters.len()),
                )
                .await);
        }
        Ok(characters[index as usize])
    }

    async fn code_point_at(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<i32> {
        let characters = Self::characters(jvm, &this).await?;
        if index < 0 || index as usize >= characters.len() {
            return Err(jvm.exception("java/lang/StringIndexOutOfBoundsException", &index.to_string()).await);
        }
        let first = characters[index as usize];
        if (0xd800..=0xdbff).contains(&first) && (index as usize + 1) < characters.len() {
            let second = characters[index as usize + 1];
            if (0xdc00..=0xdfff).contains(&second) {
                return Ok((((first as i32 - 0xd800) << 10) | (second as i32 - 0xdc00)) + 0x10000);
            }
        }
        Ok(first as i32)
    }

    async fn code_point_before(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<i32> {
        let characters = Self::characters(jvm, &this).await?;
        if index <= 0 || index as usize > characters.len() {
            return Err(jvm.exception("java/lang/StringIndexOutOfBoundsException", &index.to_string()).await);
        }
        let second = characters[index as usize - 1];
        if (0xdc00..=0xdfff).contains(&second) && index > 1 {
            let first = characters[index as usize - 2];
            if (0xd800..=0xdbff).contains(&first) {
                return Ok((((first as i32 - 0xd800) << 10) | (second as i32 - 0xdc00)) + 0x10000);
            }
        }
        Ok(second as i32)
    }

    async fn code_point_count(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, begin: i32, end: i32) -> Result<i32> {
        let characters = Self::characters(jvm, &this).await?;
        if begin < 0 || end > characters.len() as i32 || begin > end {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "invalid code point range").await);
        }
        let mut index = begin as usize;
        let mut count = 0;
        while index < end as usize {
            if (0xd800..=0xdbff).contains(&characters[index]) && index + 1 < end as usize && (0xdc00..=0xdfff).contains(&characters[index + 1]) {
                index += 2;
            } else {
                index += 1;
            }
            count += 1;
        }
        Ok(count)
    }

    async fn offset_by_code_points(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        index: i32,
        code_point_offset: i32,
    ) -> Result<i32> {
        let characters = Self::characters(jvm, &this).await?;
        if index < 0 || index as usize > characters.len() {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "invalid index").await);
        }
        let mut cursor = index as usize;
        if code_point_offset >= 0 {
            for _ in 0..code_point_offset {
                if cursor >= characters.len() {
                    return Err(jvm
                        .exception("java/lang/IndexOutOfBoundsException", "code point offset exceeds length")
                        .await);
                }
                if (0xd800..=0xdbff).contains(&characters[cursor])
                    && cursor + 1 < characters.len()
                    && (0xdc00..=0xdfff).contains(&characters[cursor + 1])
                {
                    cursor += 2;
                } else {
                    cursor += 1;
                }
            }
        } else {
            for _ in code_point_offset..0 {
                if cursor == 0 {
                    return Err(jvm
                        .exception("java/lang/IndexOutOfBoundsException", "code point offset precedes start")
                        .await);
                }
                cursor -= 1;
                if (0xdc00..=0xdfff).contains(&characters[cursor]) && cursor > 0 && (0xd800..=0xdbff).contains(&characters[cursor - 1]) {
                    cursor -= 1;
                }
            }
        }
        Ok(cursor as i32)
    }

    async fn get_chars(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        source_begin: i32,
        source_end: i32,
        mut destination: ClassInstanceRef<Array<JavaChar>>,
        destination_begin: i32,
    ) -> Result<()> {
        let characters = Self::characters(jvm, &this).await?;
        if source_begin < 0 || source_end > characters.len() as i32 || source_begin > source_end {
            return Err(jvm.exception("java/lang/StringIndexOutOfBoundsException", "invalid source range").await);
        }
        if destination.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "destination is null").await);
        }
        let copy_length = source_end - source_begin;
        let destination_length = jvm.array_length(&destination).await? as i32;
        if destination_begin < 0 || destination_begin > destination_length - copy_length {
            return Err(jvm
                .exception("java/lang/ArrayIndexOutOfBoundsException", "invalid destination range")
                .await);
        }
        jvm.store_array(
            &mut destination,
            destination_begin as usize,
            characters[source_begin as usize..source_end as usize].to_vec(),
        )
        .await
    }

    async fn set_char_at(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, index: i32, character: JavaChar) -> Result<()> {
        let mut characters = Self::characters(jvm, &this).await?;
        if index < 0 || index as usize >= characters.len() {
            return Err(jvm.exception("java/lang/StringIndexOutOfBoundsException", &index.to_string()).await);
        }
        characters[index as usize] = character;
        Self::replace_characters(jvm, &mut this, characters).await
    }

    async fn append_object(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        object: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Self>> {
        let string: ClassInstanceRef<String> = if object.is_null() {
            JavaLangString::from_rust_string(jvm, "null").await?.into()
        } else {
            jvm.invoke_virtual(&object, "java/lang/Object", "toString", "()Ljava/lang/String;", ())
                .await?
        };
        jvm.invoke_virtual(
            &this,
            "java/lang/AbstractStringBuilder",
            "append",
            "(Ljava/lang/String;)Ljava/lang/AbstractStringBuilder;",
            (string,),
        )
        .await
    }

    async fn append_string(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        string: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Self>> {
        let mut characters = Self::characters(jvm, &this).await?;
        if string.is_null() {
            characters.extend("null".encode_utf16());
        } else {
            characters.extend(JavaLangString::to_utf16(jvm, &string).await?);
        }
        Self::replace_characters(jvm, &mut this, characters).await?;
        Ok(this)
    }

    async fn append_string_buffer(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        buffer: ClassInstanceRef<StringBuffer>,
    ) -> Result<ClassInstanceRef<Self>> {
        let string: ClassInstanceRef<String> = if buffer.is_null() {
            JavaLangString::from_rust_string(jvm, "null").await?.into()
        } else {
            jvm.invoke_virtual(&buffer, "java/lang/Object", "toString", "()Ljava/lang/String;", ())
                .await?
        };
        jvm.invoke_virtual(
            &this,
            "java/lang/AbstractStringBuilder",
            "append",
            "(Ljava/lang/String;)Ljava/lang/AbstractStringBuilder;",
            (string,),
        )
        .await
    }

    async fn append_char_sequence(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        sequence: ClassInstanceRef<CharSequence>,
    ) -> Result<ClassInstanceRef<Self>> {
        let mut characters = Self::characters(jvm, &this).await?;
        characters.extend(Self::char_sequence_characters(jvm, &sequence).await?);
        Self::replace_characters(jvm, &mut this, characters).await?;
        Ok(this)
    }

    async fn append_char_sequence_range(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        sequence: ClassInstanceRef<CharSequence>,
        start: i32,
        end: i32,
    ) -> Result<ClassInstanceRef<Self>> {
        let source = Self::char_sequence_characters(jvm, &sequence).await?;
        if start < 0 || end > source.len() as i32 || start > end {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "invalid CharSequence range").await);
        }
        let mut characters = Self::characters(jvm, &this).await?;
        characters.extend_from_slice(&source[start as usize..end as usize]);
        Self::replace_characters(jvm, &mut this, characters).await?;
        Ok(this)
    }

    async fn append_char_array(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        array: ClassInstanceRef<Array<JavaChar>>,
    ) -> Result<ClassInstanceRef<Self>> {
        if array.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "str is null").await);
        }
        let length = jvm.array_length(&array).await? as i32;
        jvm.invoke_virtual(
            &this,
            "java/lang/AbstractStringBuilder",
            "append",
            "([CII)Ljava/lang/AbstractStringBuilder;",
            (array, 0, length),
        )
        .await
    }

    async fn append_char_array_range(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        array: ClassInstanceRef<Array<JavaChar>>,
        offset: i32,
        length: i32,
    ) -> Result<ClassInstanceRef<Self>> {
        if array.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "str is null").await);
        }
        let array_length = jvm.array_length(&array).await? as i32;
        if offset < 0 || length < 0 || offset > array_length - length {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "invalid char array range").await);
        }
        let mut characters = Self::characters(jvm, &this).await?;
        characters.extend(jvm.load_array::<JavaChar>(&array, offset as usize, length as usize).await?);
        Self::replace_characters(jvm, &mut this, characters).await?;
        Ok(this)
    }

    async fn append_boolean(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: bool) -> Result<ClassInstanceRef<Self>> {
        let string = JavaLangString::from_rust_string(jvm, if value { "true" } else { "false" }).await?;
        jvm.invoke_virtual(
            &this,
            "java/lang/AbstractStringBuilder",
            "append",
            "(Ljava/lang/String;)Ljava/lang/AbstractStringBuilder;",
            (string,),
        )
        .await
    }

    async fn append_char(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: JavaChar) -> Result<ClassInstanceRef<Self>> {
        let mut characters = Self::characters(jvm, &this).await?;
        characters.push(value);
        Self::replace_characters(jvm, &mut this, characters).await?;
        Ok(this)
    }

    async fn append_int(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i32) -> Result<ClassInstanceRef<Self>> {
        let string = JavaLangString::from_rust_string(jvm, &value.to_string()).await?;
        jvm.invoke_virtual(
            &this,
            "java/lang/AbstractStringBuilder",
            "append",
            "(Ljava/lang/String;)Ljava/lang/AbstractStringBuilder;",
            (string,),
        )
        .await
    }

    async fn append_long(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i64) -> Result<ClassInstanceRef<Self>> {
        let string = JavaLangString::from_rust_string(jvm, &value.to_string()).await?;
        jvm.invoke_virtual(
            &this,
            "java/lang/AbstractStringBuilder",
            "append",
            "(Ljava/lang/String;)Ljava/lang/AbstractStringBuilder;",
            (string,),
        )
        .await
    }

    async fn append_float(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: f32) -> Result<ClassInstanceRef<Self>> {
        let string: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/Float", "toString", "(F)Ljava/lang/String;", (value,))
            .await?;
        jvm.invoke_virtual(
            &this,
            "java/lang/AbstractStringBuilder",
            "append",
            "(Ljava/lang/String;)Ljava/lang/AbstractStringBuilder;",
            (string,),
        )
        .await
    }

    async fn append_double(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: f64) -> Result<ClassInstanceRef<Self>> {
        let string: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/Double", "toString", "(D)Ljava/lang/String;", (value,))
            .await?;
        jvm.invoke_virtual(
            &this,
            "java/lang/AbstractStringBuilder",
            "append",
            "(Ljava/lang/String;)Ljava/lang/AbstractStringBuilder;",
            (string,),
        )
        .await
    }

    async fn append_code_point(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        code_point: i32,
    ) -> Result<ClassInstanceRef<Self>> {
        if !(0..=0x10ffff).contains(&code_point) {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "invalid code point").await);
        }
        let mut characters = Self::characters(jvm, &this).await?;
        if code_point < 0x10000 {
            characters.push(code_point as JavaChar);
        } else {
            let value = code_point - 0x10000;
            characters.push((0xd800 + (value >> 10)) as JavaChar);
            characters.push((0xdc00 + (value & 0x3ff)) as JavaChar);
        }
        Self::replace_characters(jvm, &mut this, characters).await?;
        Ok(this)
    }

    async fn delete(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, start: i32, end: i32) -> Result<ClassInstanceRef<Self>> {
        let mut characters = Self::characters(jvm, &this).await?;
        let end = end.min(characters.len() as i32);
        if start < 0 || start > end {
            return Err(jvm.exception("java/lang/StringIndexOutOfBoundsException", "invalid delete range").await);
        }
        characters.drain(start as usize..end as usize);
        Self::replace_characters(jvm, &mut this, characters).await?;
        Ok(this)
    }

    async fn delete_char_at(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Self>> {
        let mut characters = Self::characters(jvm, &this).await?;
        if index < 0 || index as usize >= characters.len() {
            return Err(jvm.exception("java/lang/StringIndexOutOfBoundsException", &index.to_string()).await);
        }
        characters.remove(index as usize);
        Self::replace_characters(jvm, &mut this, characters).await?;
        Ok(this)
    }

    async fn replace(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        start: i32,
        end: i32,
        string: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Self>> {
        let mut characters = Self::characters(jvm, &this).await?;
        let end = end.min(characters.len() as i32);
        if start < 0 || start > characters.len() as i32 || start > end {
            return Err(jvm.exception("java/lang/StringIndexOutOfBoundsException", "invalid replace range").await);
        }
        if string.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "str is null").await);
        }
        characters.splice(start as usize..end as usize, JavaLangString::to_utf16(jvm, &string).await?);
        Self::replace_characters(jvm, &mut this, characters).await?;
        Ok(this)
    }

    async fn substring(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, start: i32) -> Result<ClassInstanceRef<String>> {
        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        jvm.invoke_virtual(
            &this,
            "java/lang/AbstractStringBuilder",
            "substring",
            "(II)Ljava/lang/String;",
            (start, count),
        )
        .await
    }

    async fn substring_range(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        start: i32,
        end: i32,
    ) -> Result<ClassInstanceRef<String>> {
        let characters = Self::characters(jvm, &this).await?;
        if start < 0 || end > characters.len() as i32 || start > end {
            return Err(jvm
                .exception("java/lang/StringIndexOutOfBoundsException", "invalid substring range")
                .await);
        }
        Ok(JavaLangString::from_utf16(jvm, characters[start as usize..end as usize].to_vec())
            .await?
            .into())
    }

    async fn sub_sequence(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        start: i32,
        end: i32,
    ) -> Result<ClassInstanceRef<CharSequence>> {
        jvm.invoke_virtual(
            &this,
            "java/lang/AbstractStringBuilder",
            "substring",
            "(II)Ljava/lang/String;",
            (start, end),
        )
        .await
    }

    async fn insert_char_array_range(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        offset: i32,
        array: ClassInstanceRef<Array<JavaChar>>,
        source_offset: i32,
        length: i32,
    ) -> Result<ClassInstanceRef<Self>> {
        if array.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "str is null").await);
        }
        let array_length = jvm.array_length(&array).await? as i32;
        if source_offset < 0 || length < 0 || source_offset > array_length - length {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "invalid char array range").await);
        }
        let inserted = jvm.load_array(&array, source_offset as usize, length as usize).await?;
        Self::insert_characters(jvm, &mut this, offset, inserted).await?;
        Ok(this)
    }

    async fn insert_object(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        offset: i32,
        object: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Self>> {
        let string: ClassInstanceRef<String> = if object.is_null() {
            JavaLangString::from_rust_string(jvm, "null").await?.into()
        } else {
            jvm.invoke_virtual(&object, "java/lang/Object", "toString", "()Ljava/lang/String;", ())
                .await?
        };
        jvm.invoke_virtual(
            &this,
            "java/lang/AbstractStringBuilder",
            "insert",
            "(ILjava/lang/String;)Ljava/lang/AbstractStringBuilder;",
            (offset, string),
        )
        .await
    }

    async fn insert_string(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        offset: i32,
        string: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Self>> {
        let inserted = if string.is_null() {
            "null".encode_utf16().collect()
        } else {
            JavaLangString::to_utf16(jvm, &string).await?
        };
        Self::insert_characters(jvm, &mut this, offset, inserted).await?;
        Ok(this)
    }

    async fn insert_char_array(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        offset: i32,
        array: ClassInstanceRef<Array<JavaChar>>,
    ) -> Result<ClassInstanceRef<Self>> {
        if array.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "str is null").await);
        }
        let length = jvm.array_length(&array).await? as i32;
        jvm.invoke_virtual(
            &this,
            "java/lang/AbstractStringBuilder",
            "insert",
            "(I[CII)Ljava/lang/AbstractStringBuilder;",
            (offset, array, 0, length),
        )
        .await
    }

    async fn insert_char_sequence(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        offset: i32,
        sequence: ClassInstanceRef<CharSequence>,
    ) -> Result<ClassInstanceRef<Self>> {
        let inserted = Self::char_sequence_characters(jvm, &sequence).await?;
        Self::insert_characters(jvm, &mut this, offset, inserted).await?;
        Ok(this)
    }

    async fn insert_char_sequence_range(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        offset: i32,
        sequence: ClassInstanceRef<CharSequence>,
        start: i32,
        end: i32,
    ) -> Result<ClassInstanceRef<Self>> {
        let source = Self::char_sequence_characters(jvm, &sequence).await?;
        if start < 0 || end > source.len() as i32 || start > end {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "invalid CharSequence range").await);
        }
        Self::insert_characters(jvm, &mut this, offset, source[start as usize..end as usize].to_vec()).await?;
        Ok(this)
    }

    async fn insert_boolean(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        offset: i32,
        value: bool,
    ) -> Result<ClassInstanceRef<Self>> {
        let string = JavaLangString::from_rust_string(jvm, if value { "true" } else { "false" }).await?;
        jvm.invoke_virtual(
            &this,
            "java/lang/AbstractStringBuilder",
            "insert",
            "(ILjava/lang/String;)Ljava/lang/AbstractStringBuilder;",
            (offset, string),
        )
        .await
    }

    async fn insert_char(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        offset: i32,
        value: JavaChar,
    ) -> Result<ClassInstanceRef<Self>> {
        Self::insert_characters(jvm, &mut this, offset, vec![value]).await?;
        Ok(this)
    }

    async fn insert_int(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, offset: i32, value: i32) -> Result<ClassInstanceRef<Self>> {
        let string = JavaLangString::from_rust_string(jvm, &value.to_string()).await?;
        jvm.invoke_virtual(
            &this,
            "java/lang/AbstractStringBuilder",
            "insert",
            "(ILjava/lang/String;)Ljava/lang/AbstractStringBuilder;",
            (offset, string),
        )
        .await
    }

    async fn insert_long(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, offset: i32, value: i64) -> Result<ClassInstanceRef<Self>> {
        let string = JavaLangString::from_rust_string(jvm, &value.to_string()).await?;
        jvm.invoke_virtual(
            &this,
            "java/lang/AbstractStringBuilder",
            "insert",
            "(ILjava/lang/String;)Ljava/lang/AbstractStringBuilder;",
            (offset, string),
        )
        .await
    }

    async fn insert_float(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        offset: i32,
        value: f32,
    ) -> Result<ClassInstanceRef<Self>> {
        let string: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/Float", "toString", "(F)Ljava/lang/String;", (value,))
            .await?;
        jvm.invoke_virtual(
            &this,
            "java/lang/AbstractStringBuilder",
            "insert",
            "(ILjava/lang/String;)Ljava/lang/AbstractStringBuilder;",
            (offset, string),
        )
        .await
    }

    async fn insert_double(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        offset: i32,
        value: f64,
    ) -> Result<ClassInstanceRef<Self>> {
        let string: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/Double", "toString", "(D)Ljava/lang/String;", (value,))
            .await?;
        jvm.invoke_virtual(
            &this,
            "java/lang/AbstractStringBuilder",
            "insert",
            "(ILjava/lang/String;)Ljava/lang/AbstractStringBuilder;",
            (offset, string),
        )
        .await
    }

    async fn index_of(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, string: ClassInstanceRef<String>) -> Result<i32> {
        jvm.invoke_virtual(&this, "java/lang/AbstractStringBuilder", "indexOf", "(Ljava/lang/String;I)I", (string, 0))
            .await
    }

    async fn index_of_from(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        string: ClassInstanceRef<String>,
        from_index: i32,
    ) -> Result<i32> {
        if string.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "str is null").await);
        }
        let characters = Self::characters(jvm, &this).await?;
        let needle = JavaLangString::to_utf16(jvm, &string).await?;
        let start = from_index.max(0) as usize;
        if needle.is_empty() {
            return Ok(start.min(characters.len()) as i32);
        }
        if start > characters.len().saturating_sub(needle.len()) {
            return Ok(-1);
        }
        Ok(characters[start..]
            .windows(needle.len())
            .position(|candidate| candidate == needle)
            .map_or(-1, |index| (start + index) as i32))
    }

    async fn last_index_of(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, string: ClassInstanceRef<String>) -> Result<i32> {
        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        jvm.invoke_virtual(
            &this,
            "java/lang/AbstractStringBuilder",
            "lastIndexOf",
            "(Ljava/lang/String;I)I",
            (string, count),
        )
        .await
    }

    async fn last_index_of_from(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        string: ClassInstanceRef<String>,
        from_index: i32,
    ) -> Result<i32> {
        if string.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "str is null").await);
        }
        let characters = Self::characters(jvm, &this).await?;
        let needle = JavaLangString::to_utf16(jvm, &string).await?;
        if needle.is_empty() {
            return Ok(from_index.min(characters.len() as i32).max(-1));
        }
        if from_index < 0 || needle.len() > characters.len() {
            return Ok(-1);
        }
        let last_start = (characters.len() - needle.len()).min(from_index as usize);
        for index in (0..=last_start).rev() {
            if characters[index..index + needle.len()] == needle {
                return Ok(index as i32);
            }
        }
        Ok(-1)
    }

    async fn reverse(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        let mut characters = Self::characters(jvm, &this).await?;
        characters.reverse();
        let mut index = 0;
        while index + 1 < characters.len() {
            if (0xdc00..=0xdfff).contains(&characters[index]) && (0xd800..=0xdbff).contains(&characters[index + 1]) {
                characters.swap(index, index + 1);
                index += 2;
            } else {
                index += 1;
            }
        }
        Self::replace_characters(jvm, &mut this, characters).await?;
        Ok(this)
    }
}
