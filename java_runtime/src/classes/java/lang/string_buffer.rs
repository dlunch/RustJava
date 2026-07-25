use alloc::{string::ToString, vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::lang::{Object, String},
};

// public final class java.lang.StringBuffer
pub struct StringBuffer;

impl StringBuffer {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/StringBuffer",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(I)V", Self::init_with_capacity, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_with_string, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "append",
                    "(Ljava/lang/String;)Ljava/lang/StringBuffer;",
                    Self::append_string,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "append",
                    "(Ljava/lang/Object;)Ljava/lang/StringBuffer;",
                    Self::append_object,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "append",
                    "(Z)Ljava/lang/StringBuffer;",
                    Self::append_boolean,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "append",
                    "(C)Ljava/lang/StringBuffer;",
                    Self::append_character,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "append",
                    "(I)Ljava/lang/StringBuffer;",
                    Self::append_integer,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "append",
                    "(J)Ljava/lang/StringBuffer;",
                    Self::append_long,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "append",
                    "(F)Ljava/lang/StringBuffer;",
                    Self::append_float,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "append",
                    "(D)Ljava/lang/StringBuffer;",
                    Self::append_double,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "append",
                    "([C)Ljava/lang/StringBuffer;",
                    Self::append_char_array_all,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "append",
                    "([CII)Ljava/lang/StringBuffer;",
                    Self::append_char_array,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(ILjava/lang/String;)Ljava/lang/StringBuffer;",
                    Self::insert_string,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(ILjava/lang/Object;)Ljava/lang/StringBuffer;",
                    Self::insert_object,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(IZ)Ljava/lang/StringBuffer;",
                    Self::insert_boolean,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(IC)Ljava/lang/StringBuffer;",
                    Self::insert_character,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(II)Ljava/lang/StringBuffer;",
                    Self::insert_integer,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(IJ)Ljava/lang/StringBuffer;",
                    Self::insert_long,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(IF)Ljava/lang/StringBuffer;",
                    Self::insert_float,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(ID)Ljava/lang/StringBuffer;",
                    Self::insert_double,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(I[C)Ljava/lang/StringBuffer;",
                    Self::insert_char_array,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "delete",
                    "(II)Ljava/lang/StringBuffer;",
                    Self::delete,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "deleteCharAt",
                    "(I)Ljava/lang/StringBuffer;",
                    Self::delete_char_at,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "replace",
                    "(IILjava/lang/String;)Ljava/lang/StringBuffer;",
                    Self::replace,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "substring",
                    "(I)Ljava/lang/String;",
                    Self::substring,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "substring",
                    "(II)Ljava/lang/String;",
                    Self::substring_range,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "reverse",
                    "()Ljava/lang/StringBuffer;",
                    Self::reverse,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "toString",
                    "()Ljava/lang/String;",
                    Self::to_string,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "capacity",
                    "()I",
                    Self::capacity,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "ensureCapacity",
                    "(I)V",
                    Self::ensure_capacity,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "setLength",
                    "(I)V",
                    Self::set_length,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new("length", "()I", Self::length, MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED),
                JavaMethodProto::new(
                    "charAt",
                    "(I)C",
                    Self::char_at,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "setCharAt",
                    "(IC)V",
                    Self::set_char_at,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "getChars",
                    "(II[CI)V",
                    Self::get_chars,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
            ],
            fields: vec![
                JavaFieldProto::new("value", "[C", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("count", "I", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::FINAL,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.StringBuffer::<init>({this:?})");

        jvm.invoke_special(&this, "java/lang/StringBuffer", "<init>", "(I)V", (16,)).await
    }

    async fn init_with_capacity(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, capacity: i32) -> Result<()> {
        tracing::debug!("java.lang.StringBuffer::<init>({this:?}, {capacity})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        if capacity < 0 {
            return Err(jvm.exception("java/lang/NegativeArraySizeException", &capacity.to_string()).await);
        }

        let value = jvm.instantiate_array("C", capacity as usize).await?;
        jvm.put_field(&mut this, "value", "[C", value).await?;
        jvm.put_field(&mut this, "count", "I", 0).await?;
        Ok(())
    }

    async fn init_with_string(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, string: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.StringBuffer::<init>({this:?}, {string:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        if string.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "str is null").await);
        }

        let string_value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&string, "value", "[C").await?;
        let count = jvm.array_length(&string_value).await?;
        let chars: Vec<JavaChar> = jvm.load_array(&string_value, 0, count).await?;
        let mut value = jvm.instantiate_array("C", count + 16).await?;
        jvm.store_array(&mut value, 0, chars).await?;
        jvm.put_field(&mut this, "value", "[C", value).await?;
        jvm.put_field(&mut this, "count", "I", count as i32).await?;
        Ok(())
    }

    async fn append_string(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        string: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({this:?}, {string:?})");

        let chars = if string.is_null() {
            "null".encode_utf16().collect()
        } else {
            let value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&string, "value", "[C").await?;
            jvm.load_array(&value, 0, jvm.array_length(&value).await?).await?
        };
        Self::append_utf16(jvm, &mut this, chars).await?;
        Ok(this)
    }

    async fn append_object(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        object: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({this:?}, {object:?})");

        let string: ClassInstanceRef<String> = if object.is_null() {
            JavaLangString::from_rust_string(jvm, "null").await?.into()
        } else {
            jvm.invoke_virtual(&object, "toString", "()Ljava/lang/String;", ()).await?
        };
        jvm.invoke_virtual(&this, "append", "(Ljava/lang/String;)Ljava/lang/StringBuffer;", (string,))
            .await
    }

    async fn append_boolean(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: bool) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({this:?}, {value})");

        let string = JavaLangString::from_rust_string(jvm, if value { "true" } else { "false" }).await?;
        jvm.invoke_virtual(&this, "append", "(Ljava/lang/String;)Ljava/lang/StringBuffer;", (string,))
            .await
    }

    async fn append_character(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        value: JavaChar,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({this:?}, {value})");

        Self::append_utf16(jvm, &mut this, vec![value]).await?;
        Ok(this)
    }

    async fn append_integer(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i32) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({this:?}, {value})");

        let string = JavaLangString::from_rust_string(jvm, &value.to_string()).await?;
        jvm.invoke_virtual(&this, "append", "(Ljava/lang/String;)Ljava/lang/StringBuffer;", (string,))
            .await
    }

    async fn append_long(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i64) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({this:?}, {value})");

        let string = JavaLangString::from_rust_string(jvm, &value.to_string()).await?;
        jvm.invoke_virtual(&this, "append", "(Ljava/lang/String;)Ljava/lang/StringBuffer;", (string,))
            .await
    }

    async fn append_float(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: f32) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({this:?}, {value})");

        let string: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/Float", "toString", "(F)Ljava/lang/String;", (value,))
            .await?;
        jvm.invoke_virtual(&this, "append", "(Ljava/lang/String;)Ljava/lang/StringBuffer;", (string,))
            .await
    }

    async fn append_double(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: f64) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({this:?}, {value})");

        let string: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/Double", "toString", "(D)Ljava/lang/String;", (value,))
            .await?;
        jvm.invoke_virtual(&this, "append", "(Ljava/lang/String;)Ljava/lang/StringBuffer;", (string,))
            .await
    }

    async fn append_char_array_all(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        array: ClassInstanceRef<Array<JavaChar>>,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({this:?}, {array:?})");

        if array.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "str is null").await);
        }
        let length = jvm.array_length(&array).await?;
        let chars = jvm.load_array(&array, 0, length).await?;
        Self::append_utf16(jvm, &mut this, chars).await?;
        Ok(this)
    }

    async fn append_char_array(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        array: ClassInstanceRef<Array<JavaChar>>,
        offset: i32,
        length: i32,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({this:?}, {array:?}, {offset}, {length})");

        if array.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "str is null").await);
        }
        let array_length = jvm.array_length(&array).await? as i32;
        if offset < 0 || length < 0 || offset > array_length - length {
            return Err(jvm
                .exception(
                    "java/lang/IndexOutOfBoundsException",
                    &alloc::format!("offset {offset}, length {length}, array length {array_length}"),
                )
                .await);
        }
        let chars = jvm.load_array(&array, offset as usize, length as usize).await?;
        Self::append_utf16(jvm, &mut this, chars).await?;
        Ok(this)
    }

    async fn insert_string(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        offset: i32,
        string: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::insert({this:?}, {offset}, {string:?})");

        let chars = if string.is_null() {
            "null".encode_utf16().collect()
        } else {
            let value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&string, "value", "[C").await?;
            jvm.load_array(&value, 0, jvm.array_length(&value).await?).await?
        };
        Self::insert_utf16(jvm, &mut this, offset, chars).await?;
        Ok(this)
    }

    async fn insert_object(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        offset: i32,
        object: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::insert({this:?}, {offset}, {object:?})");

        let string: ClassInstanceRef<String> = if object.is_null() {
            JavaLangString::from_rust_string(jvm, "null").await?.into()
        } else {
            jvm.invoke_virtual(&object, "toString", "()Ljava/lang/String;", ()).await?
        };
        jvm.invoke_virtual(&this, "insert", "(ILjava/lang/String;)Ljava/lang/StringBuffer;", (offset, string))
            .await
    }

    async fn insert_boolean(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        offset: i32,
        value: bool,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::insert({this:?}, {offset}, {value})");

        let string = JavaLangString::from_rust_string(jvm, if value { "true" } else { "false" }).await?;
        jvm.invoke_virtual(&this, "insert", "(ILjava/lang/String;)Ljava/lang/StringBuffer;", (offset, string))
            .await
    }

    async fn insert_character(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        offset: i32,
        value: JavaChar,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::insert({this:?}, {offset}, {value})");

        Self::insert_utf16(jvm, &mut this, offset, vec![value]).await?;
        Ok(this)
    }

    async fn insert_integer(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        offset: i32,
        value: i32,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::insert({this:?}, {offset}, {value})");

        let string = JavaLangString::from_rust_string(jvm, &value.to_string()).await?;
        jvm.invoke_virtual(&this, "insert", "(ILjava/lang/String;)Ljava/lang/StringBuffer;", (offset, string))
            .await
    }

    async fn insert_long(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, offset: i32, value: i64) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::insert({this:?}, {offset}, {value})");

        let string = JavaLangString::from_rust_string(jvm, &value.to_string()).await?;
        jvm.invoke_virtual(&this, "insert", "(ILjava/lang/String;)Ljava/lang/StringBuffer;", (offset, string))
            .await
    }

    async fn insert_float(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        offset: i32,
        value: f32,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::insert({this:?}, {offset}, {value})");

        let string: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/Float", "toString", "(F)Ljava/lang/String;", (value,))
            .await?;
        jvm.invoke_virtual(&this, "insert", "(ILjava/lang/String;)Ljava/lang/StringBuffer;", (offset, string))
            .await
    }

    async fn insert_double(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        offset: i32,
        value: f64,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::insert({this:?}, {offset}, {value})");

        let string: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/Double", "toString", "(D)Ljava/lang/String;", (value,))
            .await?;
        jvm.invoke_virtual(&this, "insert", "(ILjava/lang/String;)Ljava/lang/StringBuffer;", (offset, string))
            .await
    }

    async fn insert_char_array(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        offset: i32,
        array: ClassInstanceRef<Array<JavaChar>>,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::insert({this:?}, {offset}, {array:?})");

        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        if offset < 0 || offset > count {
            return Err(jvm
                .exception(
                    "java/lang/StringIndexOutOfBoundsException",
                    &alloc::format!("offset {offset}, length {count}"),
                )
                .await);
        }
        if array.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "str is null").await);
        }
        let chars = jvm.load_array(&array, 0, jvm.array_length(&array).await?).await?;
        Self::insert_utf16(jvm, &mut this, offset, chars).await?;
        Ok(this)
    }

    async fn delete(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, start: i32, end: i32) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::delete({this:?}, {start}, {end})");

        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        let end = end.min(count);
        if start < 0 || start > end {
            return Err(jvm
                .exception(
                    "java/lang/StringIndexOutOfBoundsException",
                    &alloc::format!("start {start}, end {end}, length {count}"),
                )
                .await);
        }

        if start != end {
            let mut value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "value", "[C").await?;
            let tail: Vec<JavaChar> = jvm.load_array(&value, end as usize, (count - end) as usize).await?;
            jvm.store_array(&mut value, start as usize, tail).await?;
            jvm.put_field(&mut this, "count", "I", count - (end - start)).await?;
        }
        Ok(this)
    }

    async fn delete_char_at(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::deleteCharAt({this:?}, {index})");

        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        if index < 0 || index >= count {
            return Err(jvm
                .exception(
                    "java/lang/StringIndexOutOfBoundsException",
                    &alloc::format!("index {index}, length {count}"),
                )
                .await);
        }

        let mut value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "value", "[C").await?;
        let tail: Vec<JavaChar> = jvm.load_array(&value, (index + 1) as usize, (count - index - 1) as usize).await?;
        jvm.store_array(&mut value, index as usize, tail).await?;
        jvm.put_field(&mut this, "count", "I", count - 1).await?;
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
        tracing::debug!("java.lang.StringBuffer::replace({this:?}, {start}, {end}, {string:?})");

        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        let end = end.min(count);
        if start < 0 || start > count || start > end {
            return Err(jvm
                .exception(
                    "java/lang/StringIndexOutOfBoundsException",
                    &alloc::format!("start {start}, end {end}, length {count}"),
                )
                .await);
        }
        if string.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "str is null").await);
        }

        let string_value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&string, "value", "[C").await?;
        let replacement: Vec<JavaChar> = jvm.load_array(&string_value, 0, jvm.array_length(&string_value).await?).await?;
        let replacement_length = replacement.len() as i32;
        let new_count = count + replacement_length - (end - start);
        Self::expand_capacity(jvm, &mut this, new_count).await?;

        let mut value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "value", "[C").await?;
        let tail: Vec<JavaChar> = jvm.load_array(&value, end as usize, (count - end) as usize).await?;
        jvm.store_array(&mut value, (start + replacement_length) as usize, tail).await?;
        jvm.store_array(&mut value, start as usize, replacement).await?;
        jvm.put_field(&mut this, "count", "I", new_count).await?;
        Ok(this)
    }

    async fn substring(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, start: i32) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.StringBuffer::substring({this:?}, {start})");

        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        jvm.invoke_virtual(&this, "substring", "(II)Ljava/lang/String;", (start, count)).await
    }

    async fn substring_range(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        start: i32,
        end: i32,
    ) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.StringBuffer::substring({this:?}, {start}, {end})");

        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        if start < 0 || end > count || start > end {
            return Err(jvm
                .exception(
                    "java/lang/StringIndexOutOfBoundsException",
                    &alloc::format!("start {start}, end {end}, length {count}"),
                )
                .await);
        }

        let value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "value", "[C").await?;
        Ok(jvm.new_class("java/lang/String", "([CII)V", (value, start, end - start)).await?.into())
    }

    async fn reverse(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::reverse({this:?})");

        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        let mut value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "value", "[C").await?;
        let mut chars: Vec<JavaChar> = jvm.load_array(&value, 0, count as usize).await?;
        chars.reverse();
        jvm.store_array(&mut value, 0, chars).await?;
        Ok(this)
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.StringBuffer::toString({this:?})");

        let value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "value", "[C").await?;
        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        Ok(jvm.new_class("java/lang/String", "([CII)V", (value, 0, count)).await?.into())
    }

    async fn capacity(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.lang.StringBuffer::capacity({this:?})");

        let value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "value", "[C").await?;
        Ok(jvm.array_length(&value).await? as i32)
    }

    async fn ensure_capacity(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, minimum_capacity: i32) -> Result<()> {
        tracing::debug!("java.lang.StringBuffer::ensureCapacity({this:?}, {minimum_capacity})");

        if minimum_capacity > 0 {
            Self::expand_capacity(jvm, &mut this, minimum_capacity).await?;
        }
        Ok(())
    }

    async fn set_length(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, new_length: i32) -> Result<()> {
        tracing::debug!("java.lang.StringBuffer::setLength({this:?}, {new_length})");

        if new_length < 0 {
            return Err(jvm.exception("java/lang/StringIndexOutOfBoundsException", &new_length.to_string()).await);
        }

        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        if new_length > count {
            Self::expand_capacity(jvm, &mut this, new_length).await?;
            let mut value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "value", "[C").await?;
            jvm.store_array(&mut value, count as usize, vec![0 as JavaChar; (new_length - count) as usize])
                .await?;
        }
        jvm.put_field(&mut this, "count", "I", new_length).await?;
        Ok(())
    }

    async fn length(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.lang.StringBuffer::length({this:?})");

        jvm.get_field(&this, "count", "I").await
    }

    async fn char_at(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<JavaChar> {
        tracing::debug!("java.lang.StringBuffer::charAt({this:?}, {index})");

        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        if index < 0 || index >= count {
            return Err(jvm
                .exception(
                    "java/lang/StringIndexOutOfBoundsException",
                    &alloc::format!("index {index}, length {count}"),
                )
                .await);
        }

        let value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "value", "[C").await?;
        Ok(jvm.load_array(&value, index as usize, 1).await?[0])
    }

    async fn set_char_at(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32, character: JavaChar) -> Result<()> {
        tracing::debug!("java.lang.StringBuffer::setCharAt({this:?}, {index}, {character})");

        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        if index < 0 || index >= count {
            return Err(jvm
                .exception(
                    "java/lang/StringIndexOutOfBoundsException",
                    &alloc::format!("index {index}, length {count}"),
                )
                .await);
        }

        let mut value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "value", "[C").await?;
        jvm.store_array(&mut value, index as usize, [character]).await
    }

    async fn get_chars(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        src_begin: i32,
        src_end: i32,
        mut destination: ClassInstanceRef<Array<JavaChar>>,
        dst_begin: i32,
    ) -> Result<()> {
        tracing::debug!("java.lang.StringBuffer::getChars({this:?}, {src_begin}, {src_end}, {destination:?}, {dst_begin})");

        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        if src_begin < 0 || src_end > count || src_begin > src_end {
            return Err(jvm
                .exception(
                    "java/lang/StringIndexOutOfBoundsException",
                    &alloc::format!("start {src_begin}, end {src_end}, length {count}"),
                )
                .await);
        }
        if destination.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "dst is null").await);
        }

        let copy_length = src_end - src_begin;
        let destination_length = jvm.array_length(&destination).await? as i32;
        if dst_begin < 0 || dst_begin > destination_length - copy_length {
            return Err(jvm
                .exception(
                    "java/lang/ArrayIndexOutOfBoundsException",
                    &alloc::format!("offset {dst_begin}, length {copy_length}, array length {destination_length}"),
                )
                .await);
        }

        let value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "value", "[C").await?;
        let chars: Vec<JavaChar> = jvm.load_array(&value, src_begin as usize, copy_length as usize).await?;
        jvm.store_array(&mut destination, dst_begin as usize, chars).await
    }

    async fn expand_capacity(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, minimum_capacity: i32) -> Result<()> {
        let value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(this, "value", "[C").await?;
        let old_capacity = jvm.array_length(&value).await? as i32;
        if minimum_capacity <= old_capacity {
            return Ok(());
        }

        let new_capacity = minimum_capacity.max(old_capacity.saturating_mul(2).saturating_add(2));
        let count: i32 = jvm.get_field(this, "count", "I").await?;
        let chars: Vec<JavaChar> = jvm.load_array(&value, 0, count as usize).await?;
        let mut new_value = jvm.instantiate_array("C", new_capacity as usize).await?;
        jvm.store_array(&mut new_value, 0, chars).await?;
        jvm.put_field(this, "value", "[C", new_value).await
    }

    async fn append_utf16(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, chars: Vec<JavaChar>) -> Result<()> {
        let count: i32 = jvm.get_field(this, "count", "I").await?;
        let new_count = count + chars.len() as i32;
        Self::expand_capacity(jvm, this, new_count).await?;

        let mut value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(this, "value", "[C").await?;
        jvm.store_array(&mut value, count as usize, chars).await?;
        jvm.put_field(this, "count", "I", new_count).await
    }

    async fn insert_utf16(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, offset: i32, chars: Vec<JavaChar>) -> Result<()> {
        let count: i32 = jvm.get_field(this, "count", "I").await?;
        if offset < 0 || offset > count {
            return Err(jvm
                .exception(
                    "java/lang/StringIndexOutOfBoundsException",
                    &alloc::format!("offset {offset}, length {count}"),
                )
                .await);
        }

        let inserted_length = chars.len() as i32;
        let new_count = count + inserted_length;
        Self::expand_capacity(jvm, this, new_count).await?;
        let mut value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(this, "value", "[C").await?;
        let tail: Vec<JavaChar> = jvm.load_array(&value, offset as usize, (count - offset) as usize).await?;
        jvm.store_array(&mut value, (offset + inserted_length) as usize, tail).await?;
        jvm.store_array(&mut value, offset as usize, chars).await?;
        jvm.put_field(this, "count", "I", new_count).await
    }
}
