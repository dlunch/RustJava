use core::char;

use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangClass};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::lang::{Object, String},
};

// public final class java.lang.Character
pub struct Character;

impl Character {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Character",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/io/Serializable", "java/lang/Comparable"],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "(C)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("charValue", "()C", Self::char_value, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("compareTo", "(Ljava/lang/Character;)I", Self::compare_to, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("compareTo", "(Ljava/lang/Object;)I", Self::compare_to_object, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "isLowerCase",
                    "(C)Z",
                    Self::is_lower_case,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "isUpperCase",
                    "(C)Z",
                    Self::is_upper_case,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "isTitleCase",
                    "(C)Z",
                    Self::is_title_case,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("isDigit", "(C)Z", Self::is_digit, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "isDefined",
                    "(C)Z",
                    Self::is_defined,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("isLetter", "(C)Z", Self::is_letter, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "isLetterOrDigit",
                    "(C)Z",
                    Self::is_letter_or_digit,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "isJavaLetter",
                    "(C)Z",
                    Self::is_java_identifier_start,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "isJavaLetterOrDigit",
                    "(C)Z",
                    Self::is_java_identifier_part,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "isJavaIdentifierStart",
                    "(C)Z",
                    Self::is_java_identifier_start,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "isJavaIdentifierPart",
                    "(C)Z",
                    Self::is_java_identifier_part,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "isUnicodeIdentifierStart",
                    "(C)Z",
                    Self::is_unicode_identifier_start,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "isUnicodeIdentifierPart",
                    "(C)Z",
                    Self::is_unicode_identifier_part,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "isIdentifierIgnorable",
                    "(C)Z",
                    Self::is_identifier_ignorable,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("isSpace", "(C)Z", Self::is_space, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "isSpaceChar",
                    "(C)Z",
                    Self::is_space_char,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "isWhitespace",
                    "(C)Z",
                    Self::is_whitespace,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "isISOControl",
                    "(C)Z",
                    Self::is_iso_control,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "toLowerCase",
                    "(C)C",
                    Self::to_lower_case,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "toUpperCase",
                    "(C)C",
                    Self::to_upper_case,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "toTitleCase",
                    "(C)C",
                    Self::to_title_case,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("digit", "(CI)I", Self::digit, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "forDigit",
                    "(II)C",
                    Self::for_digit,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "getNumericValue",
                    "(C)I",
                    Self::get_numeric_value,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("getType", "(C)I", Self::get_type, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
            ],
            fields: vec![
                JavaFieldProto::new(
                    "MIN_VALUE",
                    "C",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "MAX_VALUE",
                    "C",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "MIN_RADIX",
                    "I",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "MAX_RADIX",
                    "I",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "TYPE",
                    "Ljava/lang/Class;",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "UNASSIGNED",
                    "B",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "UPPERCASE_LETTER",
                    "B",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "LOWERCASE_LETTER",
                    "B",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "TITLECASE_LETTER",
                    "B",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "MODIFIER_LETTER",
                    "B",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "OTHER_LETTER",
                    "B",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "NON_SPACING_MARK",
                    "B",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "ENCLOSING_MARK",
                    "B",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "COMBINING_SPACING_MARK",
                    "B",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "DECIMAL_DIGIT_NUMBER",
                    "B",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "LETTER_NUMBER",
                    "B",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "OTHER_NUMBER",
                    "B",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "SPACE_SEPARATOR",
                    "B",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "LINE_SEPARATOR",
                    "B",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "PARAGRAPH_SEPARATOR",
                    "B",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "CONTROL",
                    "B",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "FORMAT",
                    "B",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "PRIVATE_USE",
                    "B",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "SURROGATE",
                    "B",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "DASH_PUNCTUATION",
                    "B",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "START_PUNCTUATION",
                    "B",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "END_PUNCTUATION",
                    "B",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "CONNECTOR_PUNCTUATION",
                    "B",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "OTHER_PUNCTUATION",
                    "B",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "MATH_SYMBOL",
                    "B",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "CURRENCY_SYMBOL",
                    "B",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "MODIFIER_SYMBOL",
                    "B",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "OTHER_SYMBOL",
                    "B",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new("value", "C", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL),
            ],
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::FINAL,
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        jvm.put_static_field("java/lang/Character", "MIN_VALUE", "C", JavaChar::MIN).await?;
        jvm.put_static_field("java/lang/Character", "MAX_VALUE", "C", JavaChar::MAX).await?;
        jvm.put_static_field("java/lang/Character", "MIN_RADIX", "I", 2i32).await?;
        jvm.put_static_field("java/lang/Character", "MAX_RADIX", "I", 36i32).await?;
        jvm.put_static_field(
            "java/lang/Character",
            "TYPE",
            "Ljava/lang/Class;",
            JavaLangClass::from_rust_primitive(jvm, "char").await?,
        )
        .await?;

        for (name, value) in [
            ("UNASSIGNED", 0i8),
            ("UPPERCASE_LETTER", 1),
            ("LOWERCASE_LETTER", 2),
            ("TITLECASE_LETTER", 3),
            ("MODIFIER_LETTER", 4),
            ("OTHER_LETTER", 5),
            ("NON_SPACING_MARK", 6),
            ("ENCLOSING_MARK", 7),
            ("COMBINING_SPACING_MARK", 8),
            ("DECIMAL_DIGIT_NUMBER", 9),
            ("LETTER_NUMBER", 10),
            ("OTHER_NUMBER", 11),
            ("SPACE_SEPARATOR", 12),
            ("LINE_SEPARATOR", 13),
            ("PARAGRAPH_SEPARATOR", 14),
            ("CONTROL", 15),
            ("FORMAT", 16),
            ("PRIVATE_USE", 18),
            ("SURROGATE", 19),
            ("DASH_PUNCTUATION", 20),
            ("START_PUNCTUATION", 21),
            ("END_PUNCTUATION", 22),
            ("CONNECTOR_PUNCTUATION", 23),
            ("OTHER_PUNCTUATION", 24),
            ("MATH_SYMBOL", 25),
            ("CURRENCY_SYMBOL", 26),
            ("MODIFIER_SYMBOL", 27),
            ("OTHER_SYMBOL", 28),
        ] {
            jvm.put_static_field("java/lang/Character", name, "B", value).await?;
        }

        Ok(())
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: JavaChar) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "value", "C", value).await
    }

    async fn char_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<JavaChar> {
        jvm.get_field(&this, "value", "C").await
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let value: JavaChar = jvm.invoke_virtual(&this, "charValue", "()C", ()).await?;
        jvm.invoke_static("java/lang/String", "valueOf", "(C)Ljava/lang/String;", (value,)).await
    }

    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let value: JavaChar = jvm.invoke_virtual(&this, "charValue", "()C", ()).await?;
        Ok(i32::from(value))
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if other.is_null() || !jvm.is_instance(&**other, "java/lang/Character") {
            return Ok(false);
        }

        let this_value: JavaChar = jvm.invoke_virtual(&this, "charValue", "()C", ()).await?;
        let other_value: JavaChar = jvm.invoke_virtual(&other, "charValue", "()C", ()).await?;
        Ok(this_value == other_value)
    }

    async fn compare_to(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<i32> {
        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "other").await);
        }
        if !jvm.is_instance(&**other, "java/lang/Character") {
            return Err(jvm.exception("java/lang/ClassCastException", "not Character").await);
        }

        let this_value: JavaChar = jvm.invoke_virtual(&this, "charValue", "()C", ()).await?;
        let other_value: JavaChar = jvm.invoke_virtual(&other, "charValue", "()C", ()).await?;
        Ok(this_value.cmp(&other_value) as i32)
    }

    async fn compare_to_object(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<i32> {
        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "other").await);
        }
        if !jvm.is_instance(&**other, "java/lang/Character") {
            return Err(jvm.exception("java/lang/ClassCastException", "not Character").await);
        }

        let other = ClassInstanceRef::<Self>::from(other.instance);
        let this_value: JavaChar = jvm.invoke_virtual(&this, "charValue", "()C", ()).await?;
        let other_value: JavaChar = jvm.invoke_virtual(&other, "charValue", "()C", ()).await?;
        Ok(this_value.cmp(&other_value) as i32)
    }

    async fn is_lower_case(_: &Jvm, _: &mut RuntimeContext, value: JavaChar) -> Result<bool> {
        Ok(char::from_u32(u32::from(value)).is_some_and(char::is_lowercase))
    }

    async fn is_upper_case(_: &Jvm, _: &mut RuntimeContext, value: JavaChar) -> Result<bool> {
        Ok(char::from_u32(u32::from(value)).is_some_and(char::is_uppercase))
    }

    async fn is_title_case(_: &Jvm, _: &mut RuntimeContext, value: JavaChar) -> Result<bool> {
        Ok(matches!(
            value,
            0x01c5 | 0x01c8 | 0x01cb | 0x01f2 | 0x1f88..=0x1f8f | 0x1f98..=0x1f9f | 0x1fa8..=0x1faf | 0x1fbc | 0x1fcc | 0x1ffc
        ))
    }

    async fn is_digit(_: &Jvm, _: &mut RuntimeContext, value: JavaChar) -> Result<bool> {
        Ok(Self::decimal_digit_value(value).is_some())
    }

    async fn is_defined(jvm: &Jvm, _: &mut RuntimeContext, value: JavaChar) -> Result<bool> {
        let category: i32 = jvm.invoke_static("java/lang/Character", "getType", "(C)I", (value,)).await?;
        Ok(category != 0)
    }

    async fn is_letter(jvm: &Jvm, _: &mut RuntimeContext, value: JavaChar) -> Result<bool> {
        let category: i32 = jvm.invoke_static("java/lang/Character", "getType", "(C)I", (value,)).await?;
        Ok((1..=5).contains(&category))
    }

    async fn is_letter_or_digit(jvm: &Jvm, _: &mut RuntimeContext, value: JavaChar) -> Result<bool> {
        let category: i32 = jvm.invoke_static("java/lang/Character", "getType", "(C)I", (value,)).await?;
        Ok((1..=5).contains(&category) || category == 9)
    }

    async fn is_java_identifier_start(jvm: &Jvm, _: &mut RuntimeContext, value: JavaChar) -> Result<bool> {
        let category: i32 = jvm.invoke_static("java/lang/Character", "getType", "(C)I", (value,)).await?;
        Ok((1..=5).contains(&category) || matches!(category, 10 | 23 | 26))
    }

    async fn is_java_identifier_part(jvm: &Jvm, _: &mut RuntimeContext, value: JavaChar) -> Result<bool> {
        let category: i32 = jvm.invoke_static("java/lang/Character", "getType", "(C)I", (value,)).await?;
        Ok((1..=10).contains(&category)
            || matches!(category, 23 | 26)
            || matches!(
                value,
                0x0000..=0x0008 | 0x000e..=0x001b | 0x007f..=0x009f | 0x200b..=0x200f | 0x202a..=0x202e | 0x2060..=0x206f | 0xfeff
            ))
    }

    async fn is_unicode_identifier_start(jvm: &Jvm, _: &mut RuntimeContext, value: JavaChar) -> Result<bool> {
        let category: i32 = jvm.invoke_static("java/lang/Character", "getType", "(C)I", (value,)).await?;
        Ok((1..=5).contains(&category) || category == 10)
    }

    async fn is_unicode_identifier_part(jvm: &Jvm, _: &mut RuntimeContext, value: JavaChar) -> Result<bool> {
        let category: i32 = jvm.invoke_static("java/lang/Character", "getType", "(C)I", (value,)).await?;
        Ok((1..=10).contains(&category)
            || category == 23
            || matches!(
                value,
                0x0000..=0x0008 | 0x000e..=0x001b | 0x007f..=0x009f | 0x200b..=0x200f | 0x202a..=0x202e | 0x2060..=0x206f | 0xfeff
            ))
    }

    async fn is_identifier_ignorable(_: &Jvm, _: &mut RuntimeContext, value: JavaChar) -> Result<bool> {
        Ok(matches!(
            value,
            0x0000..=0x0008 | 0x000e..=0x001b | 0x007f..=0x009f | 0x200b..=0x200f | 0x202a..=0x202e | 0x2060..=0x206f | 0xfeff
        ))
    }

    async fn is_space(_: &Jvm, _: &mut RuntimeContext, value: JavaChar) -> Result<bool> {
        Ok(matches!(value, 0x0009 | 0x000a | 0x000c | 0x000d | 0x0020))
    }

    async fn is_space_char(_: &Jvm, _: &mut RuntimeContext, value: JavaChar) -> Result<bool> {
        Ok(matches!(
            value,
            0x0020 | 0x00a0 | 0x1680 | 0x2000..=0x200a | 0x2028..=0x2029 | 0x202f | 0x205f | 0x3000
        ))
    }

    async fn is_whitespace(_: &Jvm, _: &mut RuntimeContext, value: JavaChar) -> Result<bool> {
        let Some(value) = char::from_u32(u32::from(value)) else {
            return Ok(false);
        };
        Ok((value.is_whitespace() && !matches!(value, '\u{00a0}' | '\u{2007}' | '\u{202f}')) || matches!(value, '\u{001c}'..='\u{001f}'))
    }

    async fn is_iso_control(_: &Jvm, _: &mut RuntimeContext, value: JavaChar) -> Result<bool> {
        Ok(matches!(value, 0x0000..=0x001f | 0x007f..=0x009f))
    }

    async fn to_lower_case(_: &Jvm, _: &mut RuntimeContext, value: JavaChar) -> Result<JavaChar> {
        let Some(character) = char::from_u32(u32::from(value)) else {
            return Ok(value);
        };
        let mut mapped = character.to_lowercase();
        let Some(first) = mapped.next() else {
            return Ok(value);
        };
        if mapped.next().is_some() || u32::from(first) > u32::from(JavaChar::MAX) {
            return Ok(value);
        }
        Ok(first as JavaChar)
    }

    async fn to_upper_case(_: &Jvm, _: &mut RuntimeContext, value: JavaChar) -> Result<JavaChar> {
        let Some(character) = char::from_u32(u32::from(value)) else {
            return Ok(value);
        };
        let mut mapped = character.to_uppercase();
        let Some(first) = mapped.next() else {
            return Ok(value);
        };
        if mapped.next().is_some() || u32::from(first) > u32::from(JavaChar::MAX) {
            return Ok(value);
        }
        Ok(first as JavaChar)
    }

    async fn to_title_case(_: &Jvm, _: &mut RuntimeContext, value: JavaChar) -> Result<JavaChar> {
        let title = match value {
            0x01c4..=0x01c6 => 0x01c5,
            0x01c7..=0x01c9 => 0x01c8,
            0x01ca..=0x01cc => 0x01cb,
            0x01f1..=0x01f3 => 0x01f2,
            _ => {
                let Some(character) = char::from_u32(u32::from(value)) else {
                    return Ok(value);
                };
                let mut mapped = character.to_uppercase();
                let Some(first) = mapped.next() else {
                    return Ok(value);
                };
                if mapped.next().is_some() || u32::from(first) > u32::from(JavaChar::MAX) {
                    return Ok(value);
                }
                first as JavaChar
            }
        };
        Ok(title)
    }

    async fn digit(_: &Jvm, _: &mut RuntimeContext, value: JavaChar, radix: i32) -> Result<i32> {
        Ok(Self::digit_value(value, radix))
    }

    pub(crate) fn digit_value(value: JavaChar, radix: i32) -> i32 {
        if !(2..=36).contains(&radix) {
            return -1;
        }

        let numeric = if let Some(value) = Self::decimal_digit_value(value) {
            value
        } else if (b'a' as JavaChar..=b'z' as JavaChar).contains(&value) {
            i32::from(value - b'a' as JavaChar) + 10
        } else if (b'A' as JavaChar..=b'Z' as JavaChar).contains(&value) {
            i32::from(value - b'A' as JavaChar) + 10
        } else if (0xff41..=0xff5a).contains(&value) {
            i32::from(value - 0xff41) + 10
        } else if (0xff21..=0xff3a).contains(&value) {
            i32::from(value - 0xff21) + 10
        } else {
            return -1;
        };

        if numeric < radix { numeric } else { -1 }
    }

    async fn for_digit(_: &Jvm, _: &mut RuntimeContext, digit: i32, radix: i32) -> Result<JavaChar> {
        if !(2..=36).contains(&radix) || !(0..radix).contains(&digit) {
            return Ok(0);
        }

        Ok(if digit < 10 {
            (i32::from(b'0') + digit) as JavaChar
        } else {
            (i32::from(b'a') + digit - 10) as JavaChar
        })
    }

    async fn get_numeric_value(_: &Jvm, _: &mut RuntimeContext, value: JavaChar) -> Result<i32> {
        if let Some(value) = Self::decimal_digit_value(value) {
            return Ok(value);
        }
        if (b'a' as JavaChar..=b'z' as JavaChar).contains(&value) {
            return Ok(i32::from(value - b'a' as JavaChar) + 10);
        }
        if (b'A' as JavaChar..=b'Z' as JavaChar).contains(&value) {
            return Ok(i32::from(value - b'A' as JavaChar) + 10);
        }
        if (0xff41..=0xff5a).contains(&value) {
            return Ok(i32::from(value - 0xff41) + 10);
        }
        if (0xff21..=0xff3a).contains(&value) {
            return Ok(i32::from(value - 0xff21) + 10);
        }

        Ok(match value {
            0x00b2 => 2,
            0x00b3 => 3,
            0x00b9 => 1,
            0x2160..=0x216b => i32::from(value - 0x2160) + 1,
            0x2170..=0x217b => i32::from(value - 0x2170) + 1,
            0x00bc..=0x00be => -2,
            _ => -1,
        })
    }

    async fn get_type(_: &Jvm, _: &mut RuntimeContext, value: JavaChar) -> Result<i32> {
        if char::from_u32(u32::from(value)).is_none() || matches!(value, 0xfdd0..=0xfdef | 0xfffe | 0xffff) {
            return Ok(0);
        }
        if matches!(
            value,
            0x01c5 | 0x01c8 | 0x01cb | 0x01f2 | 0x1f88..=0x1f8f | 0x1f98..=0x1f9f | 0x1fa8..=0x1faf | 0x1fbc | 0x1fcc | 0x1ffc
        ) {
            return Ok(3);
        }

        if matches!(value, 0x2160..=0x2188) {
            return Ok(10);
        }
        if Self::decimal_digit_value(value).is_some() {
            return Ok(9);
        }
        if matches!(value, 0x20dd..=0x20e0) {
            return Ok(7);
        }
        if matches!(value, 0x0300..=0x036f | 0x1ab0..=0x1aff | 0x1dc0..=0x1dff | 0x20d0..=0x20ff | 0xfe00..=0xfe0f | 0xfe20..=0xfe2f) {
            return Ok(6);
        }
        if matches!(value, 0x0903 | 0x093b | 0x093e..=0x0940 | 0x0949..=0x094c) {
            return Ok(8);
        }

        let character = char::from_u32(u32::from(value));
        if character.is_some_and(char::is_uppercase) {
            return Ok(1);
        }
        if character.is_some_and(char::is_lowercase) {
            return Ok(2);
        }
        if character.is_some_and(char::is_alphabetic) {
            return Ok(5);
        }
        if matches!(value, 0x00b2..=0x00b3 | 0x00b9 | 0x00bc..=0x00be | 0x2070 | 0x2074..=0x2079 | 0x2080..=0x2089) {
            return Ok(11);
        }
        if value == 0x2028 {
            return Ok(13);
        }
        if value == 0x2029 {
            return Ok(14);
        }
        if matches!(value, 0x0020 | 0x00a0 | 0x1680 | 0x2000..=0x200a | 0x202f | 0x205f | 0x3000) {
            return Ok(12);
        }
        if matches!(value, 0x0000..=0x001f | 0x007f..=0x009f) {
            return Ok(15);
        }
        if matches!(value, 0x00ad | 0x200b..=0x200f | 0x202a..=0x202e | 0x2060..=0x206f | 0xfeff) {
            return Ok(16);
        }
        if matches!(value, 0xe000..=0xf8ff) {
            return Ok(18);
        }
        if matches!(value, 0x002d | 0x058a | 0x2010..=0x2015 | 0x2e17 | 0x2e1a | 0x2e3a..=0x2e3b | 0x301c | 0x3030) {
            return Ok(20);
        }
        if matches!(value, 0x0028 | 0x005b | 0x007b | 0x0f3a | 0x0f3c | 0x169b | 0x201a | 0x201e | 0x2045) {
            return Ok(21);
        }
        if matches!(value, 0x0029 | 0x005d | 0x007d | 0x0f3b | 0x0f3d | 0x169c | 0x2046) {
            return Ok(22);
        }
        if value == 0x00ab {
            return Ok(21);
        }
        if value == 0x00bb {
            return Ok(22);
        }
        if matches!(value, 0x005f | 0x203f..=0x2040 | 0x2054 | 0xfe33..=0xfe34 | 0xfe4d..=0xfe4f | 0xff3f) {
            return Ok(23);
        }
        if matches!(value, 0x0024 | 0x00a2..=0x00a5 | 0x058f | 0x060b | 0x09f2..=0x09f3 | 0x0af1 | 0x0bf9 | 0x0e3f | 0x17db | 0x20a0..=0x20cf | 0xa838 | 0xfdfc | 0xfe69 | 0xff04 | 0xffe0..=0xffe6)
        {
            return Ok(26);
        }
        if matches!(value, 0x005e | 0x0060 | 0x00a8 | 0x00af | 0x00b4 | 0x00b8 | 0x02b0..=0x02ff) {
            return Ok(27);
        }
        if matches!(value, 0x002b | 0x003c..=0x003e | 0x007c | 0x007e | 0x00ac | 0x00b1 | 0x00d7 | 0x00f7 | 0x2200..=0x22ff) {
            return Ok(25);
        }
        if matches!(value, 0x00a6 | 0x00a9 | 0x00ae | 0x00b0 | 0x2300..=0x23ff | 0x2460..=0x24ff | 0x2600..=0x27bf) {
            return Ok(28);
        }
        if matches!(value, 0x00a1 | 0x00a7 | 0x00b6..=0x00b7 | 0x00bf) {
            return Ok(24);
        }
        if (value <= 0x007e && (value as u8).is_ascii_punctuation()) || matches!(value, 0x2000..=0x206f | 0x3001..=0x303f) {
            return Ok(24);
        }

        Ok(0)
    }

    fn decimal_digit_value(value: JavaChar) -> Option<i32> {
        let zero = match value {
            0x0030..=0x0039 => 0x0030,
            0x0660..=0x0669 => 0x0660,
            0x06f0..=0x06f9 => 0x06f0,
            0x07c0..=0x07c9 => 0x07c0,
            0x0966..=0x096f => 0x0966,
            0x09e6..=0x09ef => 0x09e6,
            0x0a66..=0x0a6f => 0x0a66,
            0x0ae6..=0x0aef => 0x0ae6,
            0x0b66..=0x0b6f => 0x0b66,
            0x0be6..=0x0bef => 0x0be6,
            0x0c66..=0x0c6f => 0x0c66,
            0x0ce6..=0x0cef => 0x0ce6,
            0x0d66..=0x0d6f => 0x0d66,
            0x0e50..=0x0e59 => 0x0e50,
            0x0ed0..=0x0ed9 => 0x0ed0,
            0x0f20..=0x0f29 => 0x0f20,
            0x1040..=0x1049 => 0x1040,
            0x1090..=0x1099 => 0x1090,
            0x17e0..=0x17e9 => 0x17e0,
            0x1810..=0x1819 => 0x1810,
            0x1946..=0x194f => 0x1946,
            0x19d0..=0x19d9 => 0x19d0,
            0x1a80..=0x1a89 => 0x1a80,
            0x1a90..=0x1a99 => 0x1a90,
            0x1b50..=0x1b59 => 0x1b50,
            0x1bb0..=0x1bb9 => 0x1bb0,
            0x1c40..=0x1c49 => 0x1c40,
            0x1c50..=0x1c59 => 0x1c50,
            0xa620..=0xa629 => 0xa620,
            0xa8d0..=0xa8d9 => 0xa8d0,
            0xa900..=0xa909 => 0xa900,
            0xa9d0..=0xa9d9 => 0xa9d0,
            0xa9f0..=0xa9f9 => 0xa9f0,
            0xaa50..=0xaa59 => 0xaa50,
            0xabf0..=0xabf9 => 0xabf0,
            0xff10..=0xff19 => 0xff10,
            _ => return None,
        };
        Some(i32::from(value - zero))
    }
}
