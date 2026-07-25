use alloc::{format, string::String as RustString, vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::{InputStream, OutputStream},
        lang::{Object, String},
    },
};

use super::Hashtable;

// class java.util.Properties
pub struct Properties;

impl Properties {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Properties",
            parent_class: Some("java/util/Hashtable"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(Ljava/util/Properties;)V", Self::init_with_defaults, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "getProperty",
                    "(Ljava/lang/String;)Ljava/lang/String;",
                    Self::get_property,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "getProperty",
                    "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
                    Self::get_property_with_default,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "setProperty",
                    "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
                    Self::set_property,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "load",
                    "(Ljava/io/InputStream;)V",
                    Self::load,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "store",
                    "(Ljava/io/OutputStream;Ljava/lang/String;)V",
                    Self::store,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "propertyNames",
                    "()Ljava/util/Enumeration;",
                    Self::property_names,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
                ),
            ],
            fields: vec![JavaFieldProto::new("defaults", "Ljava/util/Properties;", FieldAccessFlags::PROTECTED)],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Properties::<init>({this:?})");

        let defaults: ClassInstanceRef<Self> = None.into();
        jvm.invoke_special(&this, "java/util/Properties", "<init>", "(Ljava/util/Properties;)V", (defaults,))
            .await
    }

    async fn init_with_defaults(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, defaults: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Properties::<init>({this:?}, {defaults:?})");

        let _: () = jvm.invoke_special(&this, "java/util/Hashtable", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "defaults", "Ljava/util/Properties;", defaults).await
    }

    async fn get_property(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        key: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.Properties::getProperty({this:?}, {key:?})");

        let result: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&this, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (key.clone(),))
            .await?;
        if !result.is_null() && jvm.is_instance(&**result, "java/lang/String") {
            return Ok(ClassInstanceRef::new(result.instance));
        }

        let defaults: ClassInstanceRef<Self> = jvm.get_field(&this, "defaults", "Ljava/util/Properties;").await?;
        if defaults.is_null() {
            Ok(None.into())
        } else {
            jvm.invoke_virtual(&defaults, "getProperty", "(Ljava/lang/String;)Ljava/lang/String;", (key,))
                .await
        }
    }

    async fn get_property_with_default(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        key: ClassInstanceRef<String>,
        default_value: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.Properties::getProperty({this:?}, {key:?}, {default_value:?})");

        let value: ClassInstanceRef<String> = jvm
            .invoke_virtual(&this, "getProperty", "(Ljava/lang/String;)Ljava/lang/String;", (key,))
            .await?;
        if value.is_null() { Ok(default_value) } else { Ok(value) }
    }

    async fn set_property(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        key: ClassInstanceRef<String>,
        value: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Properties::setProperty({this:?}, {key:?}, {value:?})");

        jvm.invoke_virtual(&this, "put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", (key, value))
            .await
    }

    async fn load(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, input: ClassInstanceRef<InputStream>) -> Result<()> {
        tracing::debug!("java.util.Properties::load({this:?}, {input:?})");

        if input.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "inStream").await);
        }

        let mut logical_line = Vec::new();
        let mut skip_lf = false;
        let mut skip_leading_whitespace = false;
        loop {
            let value: i32 = jvm.invoke_virtual(&input, "read", "()I", ()).await?;
            if value < 0 {
                if !logical_line.is_empty() {
                    let trailing_backslashes = logical_line.iter().rev().take_while(|character| **character == b'\\' as JavaChar).count();
                    if trailing_backslashes % 2 == 1 {
                        logical_line.pop();
                    }
                    Self::load_logical_line(jvm, &this, &logical_line).await?;
                }
                return Ok(());
            }

            let byte = value as u8;
            if skip_lf {
                skip_lf = false;
                if byte == b'\n' {
                    continue;
                }
            }

            if byte == b'\n' || byte == b'\r' {
                if byte == b'\r' {
                    skip_lf = true;
                }

                let trailing_backslashes = logical_line.iter().rev().take_while(|character| **character == b'\\' as JavaChar).count();
                let first_non_whitespace = logical_line
                    .iter()
                    .position(|character| !matches!(*character, character if character == b' ' as JavaChar || character == b'\t' as JavaChar || character == 0x0c));
                let is_comment = first_non_whitespace
                    .is_some_and(|position| logical_line[position] == b'#' as JavaChar || logical_line[position] == b'!' as JavaChar);
                if !is_comment && trailing_backslashes % 2 == 1 {
                    logical_line.pop();
                    skip_leading_whitespace = true;
                } else {
                    Self::load_logical_line(jvm, &this, &logical_line).await?;
                    logical_line.clear();
                    skip_leading_whitespace = false;
                }
                continue;
            }

            if skip_leading_whitespace && matches!(byte, b' ' | b'\t' | 0x0c) {
                continue;
            }
            skip_leading_whitespace = false;
            logical_line.push(byte as JavaChar);
        }
    }

    async fn load_logical_line(jvm: &Jvm, this: &ClassInstanceRef<Self>, logical_line: &[JavaChar]) -> Result<()> {
        let mut key_start = 0;
        while key_start < logical_line.len()
            && matches!(logical_line[key_start], character if character == b' ' as JavaChar || character == b'\t' as JavaChar || character == 0x0c)
        {
            key_start += 1;
        }
        if key_start == logical_line.len() || logical_line[key_start] == b'#' as JavaChar || logical_line[key_start] == b'!' as JavaChar {
            return Ok(());
        }

        let mut key_end = key_start;
        let mut preceding_backslash = false;
        while key_end < logical_line.len() {
            let character = logical_line[key_end];
            if !preceding_backslash
                && (character == b'=' as JavaChar
                    || character == b':' as JavaChar
                    || character == b' ' as JavaChar
                    || character == b'\t' as JavaChar
                    || character == 0x0c)
            {
                break;
            }
            if character == b'\\' as JavaChar {
                preceding_backslash = !preceding_backslash;
            } else {
                preceding_backslash = false;
            }
            key_end += 1;
        }

        let mut value_start = key_end;
        while value_start < logical_line.len()
            && matches!(logical_line[value_start], character if character == b' ' as JavaChar || character == b'\t' as JavaChar || character == 0x0c)
        {
            value_start += 1;
        }
        if value_start < logical_line.len() && (logical_line[value_start] == b'=' as JavaChar || logical_line[value_start] == b':' as JavaChar) {
            value_start += 1;
        }
        while value_start < logical_line.len()
            && matches!(logical_line[value_start], character if character == b' ' as JavaChar || character == b'\t' as JavaChar || character == 0x0c)
        {
            value_start += 1;
        }

        let Some(key_chars) = Self::load_convert(&logical_line[key_start..key_end]) else {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "Malformed \\uxxxx encoding").await);
        };
        let Some(value_chars) = Self::load_convert(&logical_line[value_start..]) else {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "Malformed \\uxxxx encoding").await);
        };

        let mut key_value = jvm.instantiate_array("C", key_chars.len()).await?;
        jvm.store_array(&mut key_value, 0, key_chars).await?;
        let key = jvm.new_class("java/lang/String", "([C)V", (key_value,)).await?;
        let mut property_value = jvm.instantiate_array("C", value_chars.len()).await?;
        jvm.store_array(&mut property_value, 0, value_chars).await?;
        let value = jvm.new_class("java/lang/String", "([C)V", (property_value,)).await?;
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(this, "put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", (key, value))
            .await?;
        Ok(())
    }

    async fn store(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        output: ClassInstanceRef<OutputStream>,
        comments: ClassInstanceRef<String>,
    ) -> Result<()> {
        tracing::debug!("java.util.Properties::store({this:?}, {output:?}, {comments:?})");

        if output.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "out").await);
        }

        let mut text = RustString::new();
        if !comments.is_null() {
            let comments_value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&comments, "value", "[C").await?;
            let comments: Vec<JavaChar> = jvm.load_array(&comments_value, 0, jvm.array_length(&comments_value).await?).await?;
            text.push('#');
            let mut index = 0;
            while index < comments.len() {
                let character = comments[index];
                if character == b'\r' as JavaChar || character == b'\n' as JavaChar {
                    if character == b'\r' as JavaChar && index + 1 < comments.len() && comments[index + 1] == b'\n' as JavaChar {
                        index += 1;
                    }
                    text.push('\n');
                    if index + 1 < comments.len() {
                        text.push('#');
                    }
                } else if !(0x20..=0x7e).contains(&character) {
                    text.push_str(&format!("\\u{character:04X}"));
                } else {
                    text.push(char::from_u32(character as u32).unwrap());
                }
                index += 1;
            }
            text.push('\n');
        }
        let date = jvm.new_class("java/util/Date", "()V", ()).await?;
        let date: ClassInstanceRef<String> = jvm.invoke_virtual(&date, "toString", "()Ljava/lang/String;", ()).await?;
        text.push('#');
        text.push_str(&JavaLangString::to_rust_string(jvm, &date).await?);
        text.push('\n');

        let names: ClassInstanceRef<Object> = jvm.invoke_virtual(&this, "keys", "()Ljava/util/Enumeration;", ()).await?;
        while jvm.invoke_virtual::<_, bool>(&names, "hasMoreElements", "()Z", ()).await? {
            let key: ClassInstanceRef<Object> = jvm.invoke_virtual(&names, "nextElement", "()Ljava/lang/Object;", ()).await?;
            let value: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&this, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (key.clone(),))
                .await?;
            if !jvm.is_instance(&**key, "java/lang/String") || value.is_null() || !jvm.is_instance(&**value, "java/lang/String") {
                return Err(jvm
                    .exception("java/lang/ClassCastException", "Properties key and value must be String")
                    .await);
            }

            let key: ClassInstanceRef<String> = ClassInstanceRef::new(key.instance);
            let value: ClassInstanceRef<String> = ClassInstanceRef::new(value.instance);
            let key_value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&key, "value", "[C").await?;
            let value_value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&value, "value", "[C").await?;
            let key_chars: Vec<JavaChar> = jvm.load_array(&key_value, 0, jvm.array_length(&key_value).await?).await?;
            let value_chars: Vec<JavaChar> = jvm.load_array(&value_value, 0, jvm.array_length(&value_value).await?).await?;
            text.push_str(&Self::save_convert(&key_chars, true));
            text.push('=');
            text.push_str(&Self::save_convert(&value_chars, false));
            text.push('\n');
        }

        let bytes = text.into_bytes();
        let mut byte_array = jvm.instantiate_array("B", bytes.len()).await?;
        jvm.store_array(&mut byte_array, 0, bytes.into_iter().map(|byte| byte as i8)).await?;
        let _: () = jvm.invoke_virtual(&output, "write", "([B)V", (byte_array,)).await?;
        jvm.invoke_virtual(&output, "flush", "()V", ()).await
    }

    async fn property_names(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Properties::propertyNames({this:?})");

        let table = Self::enumerate(jvm, &this).await?;
        jvm.invoke_virtual(&table, "keys", "()Ljava/util/Enumeration;", ()).await
    }

    async fn enumerate(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Hashtable>> {
        let table: ClassInstanceRef<Hashtable> = jvm.new_class("java/util/Hashtable", "()V", ()).await?.into();
        let mut layers = Vec::new();
        let mut current = this.clone();
        while !current.is_null() {
            layers.push(current.clone());
            current = jvm.get_field(&current, "defaults", "Ljava/util/Properties;").await?;
        }

        for layer in layers.into_iter().rev() {
            let names: ClassInstanceRef<Object> = jvm.invoke_virtual(&layer, "keys", "()Ljava/util/Enumeration;", ()).await?;
            while jvm.invoke_virtual::<_, bool>(&names, "hasMoreElements", "()Z", ()).await? {
                let key: ClassInstanceRef<Object> = jvm.invoke_virtual(&names, "nextElement", "()Ljava/lang/Object;", ()).await?;
                if key.is_null() || !jvm.is_instance(&**key, "java/lang/String") {
                    return Err(jvm.exception("java/lang/ClassCastException", "Properties key must be String").await);
                }
                let value: ClassInstanceRef<Object> = jvm
                    .invoke_virtual(&layer, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (key.clone(),))
                    .await?;
                let _: ClassInstanceRef<Object> = jvm
                    .invoke_virtual(&table, "put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", (key, value))
                    .await?;
            }
        }
        Ok(table)
    }

    fn load_convert(input: &[JavaChar]) -> Option<Vec<JavaChar>> {
        let mut output = Vec::with_capacity(input.len());
        let mut index = 0;
        while index < input.len() {
            let character = input[index];
            index += 1;
            if character != b'\\' as JavaChar {
                output.push(character);
                continue;
            }
            if index == input.len() {
                output.push(b'\\' as JavaChar);
                break;
            }

            let escaped = input[index];
            index += 1;
            match escaped {
                character if character == b't' as JavaChar => output.push(b'\t' as JavaChar),
                character if character == b'n' as JavaChar => output.push(b'\n' as JavaChar),
                character if character == b'r' as JavaChar => output.push(b'\r' as JavaChar),
                character if character == b'f' as JavaChar => output.push(0x0c),
                character if character == b'u' as JavaChar => {
                    if index + 4 > input.len() {
                        return None;
                    }
                    let mut value = 0u16;
                    for digit in &input[index..index + 4] {
                        value = value.checked_mul(16)?;
                        value = value.checked_add(match *digit {
                            character if (b'0' as JavaChar..=b'9' as JavaChar).contains(&character) => character - b'0' as JavaChar,
                            character if (b'a' as JavaChar..=b'f' as JavaChar).contains(&character) => character - b'a' as JavaChar + 10,
                            character if (b'A' as JavaChar..=b'F' as JavaChar).contains(&character) => character - b'A' as JavaChar + 10,
                            _ => return None,
                        })?;
                    }
                    index += 4;
                    output.push(value);
                }
                _ => output.push(escaped),
            }
        }
        Some(output)
    }

    fn save_convert(input: &[JavaChar], escape_space: bool) -> RustString {
        let mut output = RustString::new();
        for (index, character) in input.iter().copied().enumerate() {
            match character {
                character if character == b' ' as JavaChar && (index == 0 || escape_space) => output.push_str("\\ "),
                character if character == b'\\' as JavaChar => output.push_str("\\\\"),
                character if character == b'\t' as JavaChar => output.push_str("\\t"),
                character if character == b'\n' as JavaChar => output.push_str("\\n"),
                character if character == b'\r' as JavaChar => output.push_str("\\r"),
                0x0c => output.push_str("\\f"),
                character
                    if character == b'=' as JavaChar
                        || character == b':' as JavaChar
                        || character == b'#' as JavaChar
                        || character == b'!' as JavaChar =>
                {
                    output.push('\\');
                    output.push(char::from_u32(character as u32).unwrap());
                }
                character if !(0x20..=0x7e).contains(&character) => output.push_str(&format!("\\u{character:04X}")),
                character => output.push(char::from_u32(character as u32).unwrap()),
            }
        }
        output
    }
}
