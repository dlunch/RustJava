use alloc::{
    format,
    string::{String as RustString, ToString},
    vec,
    vec::Vec,
};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        lang::{Double, Long, Number, String, StringBuffer},
        text::{FieldPosition, ParsePosition},
    },
};

struct DecimalPattern {
    positive_prefix: RustString,
    positive_suffix: RustString,
    negative_prefix: RustString,
    negative_suffix: RustString,
    minimum_integer_digits: i32,
    maximum_fraction_digits: i32,
    minimum_fraction_digits: i32,
    grouping_used: bool,
    grouping_size: i32,
    multiplier: i32,
}

// public class java.text.DecimalFormat
pub struct DecimalFormat;

impl DecimalFormat {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/text/DecimalFormat",
            parent_class: Some("java/text/NumberFormat"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_with_pattern, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "format",
                    "(DLjava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;",
                    Self::format_double,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "format",
                    "(JLjava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;",
                    Self::format_long,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "parse",
                    "(Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/lang/Number;",
                    Self::parse,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("applyPattern", "(Ljava/lang/String;)V", Self::apply_pattern, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("toPattern", "()Ljava/lang/String;", Self::to_pattern, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getMultiplier", "()I", Self::get_multiplier, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("setMultiplier", "(I)V", Self::set_multiplier, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("pattern", "Ljava/lang/String;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("positivePrefix", "Ljava/lang/String;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("positiveSuffix", "Ljava/lang/String;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("negativePrefix", "Ljava/lang/String;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("negativeSuffix", "Ljava/lang/String;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("multiplier", "I", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("groupingSize", "I", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    fn parse_affix(value: &[char]) -> Option<(RustString, bool)> {
        let mut result = RustString::new();
        let mut percent = false;
        let mut quoted = false;
        let mut index = 0;
        while index < value.len() {
            if value[index] == '\'' {
                if index + 1 < value.len() && value[index + 1] == '\'' {
                    result.push('\'');
                    index += 2;
                    continue;
                }
                quoted = !quoted;
                index += 1;
                continue;
            }
            if !quoted && value[index] == '\u{00a4}' {
                result.push('$');
            } else {
                if !quoted && value[index] == '%' {
                    percent = true;
                }
                result.push(value[index]);
            }
            index += 1;
        }
        if quoted { None } else { Some((result, percent)) }
    }

    fn parse_subpattern(value: &[char]) -> Option<(RustString, Vec<char>, RustString, bool)> {
        let mut quoted = false;
        let mut numeric_start = None;
        let mut index = 0;
        while index < value.len() {
            if value[index] == '\'' {
                if index + 1 < value.len() && value[index + 1] == '\'' {
                    index += 2;
                    continue;
                }
                quoted = !quoted;
            } else if !quoted && matches!(value[index], '#' | '0') {
                numeric_start = Some(index);
                break;
            }
            index += 1;
        }
        let numeric_start = numeric_start?;
        let mut numeric_end = numeric_start;
        while numeric_end < value.len() && matches!(value[numeric_end], '#' | '0' | ',' | '.') {
            numeric_end += 1;
        }
        quoted = false;
        index = numeric_end;
        while index < value.len() {
            if value[index] == '\'' {
                if index + 1 < value.len() && value[index + 1] == '\'' {
                    index += 2;
                    continue;
                }
                quoted = !quoted;
            } else if !quoted && matches!(value[index], '#' | '0' | ',' | '.') {
                return None;
            }
            index += 1;
        }
        let (prefix, prefix_percent) = Self::parse_affix(&value[..numeric_start])?;
        let (suffix, suffix_percent) = Self::parse_affix(&value[numeric_end..])?;
        Some((
            prefix,
            value[numeric_start..numeric_end].to_vec(),
            suffix,
            prefix_percent || suffix_percent,
        ))
    }

    fn parse_pattern_value(pattern: &str) -> Option<DecimalPattern> {
        let characters: Vec<char> = pattern.chars().collect();
        let mut quoted = false;
        let mut separator = None;
        let mut index = 0;
        while index < characters.len() {
            if characters[index] == '\'' {
                if index + 1 < characters.len() && characters[index + 1] == '\'' {
                    index += 2;
                    continue;
                }
                quoted = !quoted;
            } else if !quoted && characters[index] == ';' {
                if separator.is_some() {
                    return None;
                }
                separator = Some(index);
            }
            index += 1;
        }
        if quoted || characters.is_empty() {
            return None;
        }

        let positive = &characters[..separator.unwrap_or(characters.len())];
        let (positive_prefix, number, positive_suffix, positive_percent) = Self::parse_subpattern(positive)?;
        if number.iter().filter(|character| **character == '.').count() > 1 {
            return None;
        }
        let decimal_index = number.iter().position(|character| *character == '.').unwrap_or(number.len());
        let integer_pattern = &number[..decimal_index];
        let fraction_pattern = if decimal_index < number.len() {
            &number[decimal_index + 1..]
        } else {
            &[]
        };
        if integer_pattern.is_empty()
            || integer_pattern.iter().any(|character| !matches!(character, '#' | '0' | ','))
            || fraction_pattern.iter().any(|character| !matches!(character, '#' | '0'))
        {
            return None;
        }
        let minimum_integer_digits = integer_pattern.iter().filter(|character| **character == '0').count() as i32;
        let maximum_fraction_digits = fraction_pattern.len() as i32;
        let minimum_fraction_digits = fraction_pattern.iter().filter(|character| **character == '0').count() as i32;
        let grouping_position = integer_pattern.iter().rposition(|character| *character == ',');
        let grouping_size = grouping_position
            .map(|position| {
                integer_pattern[position + 1..]
                    .iter()
                    .filter(|character| matches!(character, '#' | '0'))
                    .count() as i32
            })
            .unwrap_or(0);
        if grouping_position.is_some() && grouping_size == 0 {
            return None;
        }

        let (negative_prefix, negative_suffix, negative_percent) = if let Some(separator) = separator {
            let (prefix, negative_number, suffix, percent) = Self::parse_subpattern(&characters[separator + 1..])?;
            if negative_number != number {
                return None;
            }
            (prefix, suffix, percent)
        } else {
            (format!("-{positive_prefix}"), positive_suffix.clone(), positive_percent)
        };

        Some(DecimalPattern {
            positive_prefix,
            positive_suffix,
            negative_prefix,
            negative_suffix,
            minimum_integer_digits,
            maximum_fraction_digits,
            minimum_fraction_digits,
            grouping_used: grouping_position.is_some(),
            grouping_size,
            multiplier: if positive_percent || negative_percent { 100 } else { 1 },
        })
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let pattern = JavaLangString::from_rust_string(jvm, "#,##0.###").await?;
        jvm.invoke_special(&this, "java/text/DecimalFormat", "<init>", "(Ljava/lang/String;)V", (pattern,))
            .await
    }

    async fn init_with_pattern(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, pattern: ClassInstanceRef<String>) -> Result<()> {
        if pattern.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "pattern").await);
        }
        let _: () = jvm.invoke_special(&this, "java/text/NumberFormat", "<init>", "()V", ()).await?;
        jvm.invoke_virtual(&this, "applyPattern", "(Ljava/lang/String;)V", (pattern,)).await
    }

    async fn apply_pattern(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, pattern: ClassInstanceRef<String>) -> Result<()> {
        if pattern.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "pattern").await);
        }
        let value = JavaLangString::to_rust_string(jvm, &pattern).await?;
        let Some(parsed) = Self::parse_pattern_value(&value) else {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "Malformed pattern").await);
        };

        let positive_prefix = JavaLangString::from_rust_string(jvm, &parsed.positive_prefix).await?;
        let positive_suffix = JavaLangString::from_rust_string(jvm, &parsed.positive_suffix).await?;
        let negative_prefix = JavaLangString::from_rust_string(jvm, &parsed.negative_prefix).await?;
        let negative_suffix = JavaLangString::from_rust_string(jvm, &parsed.negative_suffix).await?;
        jvm.put_field(&mut this, "pattern", "Ljava/lang/String;", pattern).await?;
        jvm.put_field(&mut this, "positivePrefix", "Ljava/lang/String;", positive_prefix).await?;
        jvm.put_field(&mut this, "positiveSuffix", "Ljava/lang/String;", positive_suffix).await?;
        jvm.put_field(&mut this, "negativePrefix", "Ljava/lang/String;", negative_prefix).await?;
        jvm.put_field(&mut this, "negativeSuffix", "Ljava/lang/String;", negative_suffix).await?;
        jvm.put_field(&mut this, "multiplier", "I", parsed.multiplier).await?;
        jvm.put_field(&mut this, "groupingSize", "I", parsed.grouping_size).await?;
        jvm.put_field(&mut this, "groupingUsed", "Z", parsed.grouping_used).await?;
        jvm.put_field(&mut this, "maximumIntegerDigits", "I", 309).await?;
        jvm.put_field(&mut this, "minimumIntegerDigits", "I", parsed.minimum_integer_digits)
            .await?;
        jvm.put_field(&mut this, "maximumFractionDigits", "I", parsed.maximum_fraction_digits)
            .await?;
        jvm.put_field(&mut this, "minimumFractionDigits", "I", parsed.minimum_fraction_digits)
            .await
    }

    async fn append_formatted(
        jvm: &Jvm,
        this: &ClassInstanceRef<Self>,
        negative: bool,
        mut integer: RustString,
        mut fraction: RustString,
        buffer: ClassInstanceRef<StringBuffer>,
        position: ClassInstanceRef<FieldPosition>,
    ) -> Result<ClassInstanceRef<StringBuffer>> {
        if buffer.is_null() || position.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "buffer or position").await);
        }
        let minimum_integer_digits: i32 = jvm.get_field(this, "minimumIntegerDigits", "I").await?;
        let maximum_integer_digits: i32 = jvm.get_field(this, "maximumIntegerDigits", "I").await?;
        let minimum_fraction_digits: i32 = jvm.get_field(this, "minimumFractionDigits", "I").await?;
        let grouping_used: bool = jvm.get_field(this, "groupingUsed", "Z").await?;
        let grouping_size: i32 = jvm.get_field(this, "groupingSize", "I").await?;

        if integer.len() > maximum_integer_digits.max(0) as usize {
            integer = integer[integer.len() - maximum_integer_digits.max(0) as usize..].to_string();
        }
        while integer.len() < minimum_integer_digits.max(0) as usize {
            integer.insert(0, '0');
        }
        if integer.is_empty() {
            integer.push('0');
        }
        if grouping_used && grouping_size > 0 {
            let mut grouped = RustString::new();
            for (index, character) in integer.chars().rev().enumerate() {
                if index > 0 && index % grouping_size as usize == 0 {
                    grouped.push(',');
                }
                grouped.push(character);
            }
            integer = grouped.chars().rev().collect();
        }
        while fraction.ends_with('0') && fraction.len() > minimum_fraction_digits.max(0) as usize {
            fraction.pop();
        }
        while fraction.len() < minimum_fraction_digits.max(0) as usize {
            fraction.push('0');
        }

        let prefix: ClassInstanceRef<String> = jvm
            .get_field(this, if negative { "negativePrefix" } else { "positivePrefix" }, "Ljava/lang/String;")
            .await?;
        let suffix: ClassInstanceRef<String> = jvm
            .get_field(this, if negative { "negativeSuffix" } else { "positiveSuffix" }, "Ljava/lang/String;")
            .await?;
        let prefix = JavaLangString::to_rust_string(jvm, &prefix).await?;
        let suffix = JavaLangString::to_rust_string(jvm, &suffix).await?;
        let mut formatted = format!("{prefix}{integer}");
        if !fraction.is_empty() {
            formatted.push('.');
            formatted.push_str(&fraction);
        }
        formatted.push_str(&suffix);

        let base: i32 = jvm.invoke_virtual(&buffer, "length", "()I", ()).await?;
        let field: i32 = jvm.invoke_virtual(&position, "getField", "()I", ()).await?;
        if field == 0 {
            let begin = base + prefix.encode_utf16().count() as i32;
            let _: () = jvm.invoke_virtual(&position, "setBeginIndex", "(I)V", (begin,)).await?;
            let _: () = jvm
                .invoke_virtual(&position, "setEndIndex", "(I)V", (begin + integer.encode_utf16().count() as i32,))
                .await?;
        } else if field == 1 && !fraction.is_empty() {
            let begin = base + prefix.encode_utf16().count() as i32 + integer.encode_utf16().count() as i32 + 1;
            let _: () = jvm.invoke_virtual(&position, "setBeginIndex", "(I)V", (begin,)).await?;
            let _: () = jvm
                .invoke_virtual(&position, "setEndIndex", "(I)V", (begin + fraction.encode_utf16().count() as i32,))
                .await?;
        }

        let text = JavaLangString::from_rust_string(jvm, &formatted).await?;
        jvm.invoke_virtual(&buffer, "append", "(Ljava/lang/String;)Ljava/lang/StringBuffer;", (text,))
            .await
    }

    async fn format_double(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        value: f64,
        buffer: ClassInstanceRef<StringBuffer>,
        position: ClassInstanceRef<FieldPosition>,
    ) -> Result<ClassInstanceRef<StringBuffer>> {
        if buffer.is_null() || position.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "buffer or position").await);
        }
        if value.is_nan() {
            let text = JavaLangString::from_rust_string(jvm, "NaN").await?;
            return jvm
                .invoke_virtual(&buffer, "append", "(Ljava/lang/String;)Ljava/lang/StringBuffer;", (text,))
                .await;
        }

        let negative = value.is_sign_negative();
        let multiplier: i32 = jvm.get_field(&this, "multiplier", "I").await?;
        let scaled = value.abs() * f64::from(multiplier);
        if scaled.is_infinite() {
            return Self::append_formatted(jvm, &this, negative, "\u{221e}".to_string(), RustString::new(), buffer, position).await;
        }
        let maximum_fraction_digits: i32 = jvm.get_field(&this, "maximumFractionDigits", "I").await?;
        let precision = maximum_fraction_digits.clamp(0, 340) as usize;
        let numeric = format!("{scaled:.precision$}");
        let (integer, fraction) = numeric
            .split_once('.')
            .map(|(integer, fraction)| (integer.to_string(), fraction.to_string()))
            .unwrap_or((numeric, RustString::new()));
        Self::append_formatted(jvm, &this, negative, integer, fraction, buffer, position).await
    }

    async fn format_long(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        value: i64,
        buffer: ClassInstanceRef<StringBuffer>,
        position: ClassInstanceRef<FieldPosition>,
    ) -> Result<ClassInstanceRef<StringBuffer>> {
        let multiplier: i32 = jvm.get_field(&this, "multiplier", "I").await?;
        let scaled = i128::from(value) * i128::from(multiplier);
        let negative = scaled < 0;
        Self::append_formatted(
            jvm,
            &this,
            negative,
            scaled.unsigned_abs().to_string(),
            RustString::new(),
            buffer,
            position,
        )
        .await
    }

    async fn parse(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        source: ClassInstanceRef<String>,
        position: ClassInstanceRef<ParsePosition>,
    ) -> Result<ClassInstanceRef<Number>> {
        if source.is_null() || position.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "source or position").await);
        }
        let source = JavaLangString::to_rust_string(jvm, &source).await?;
        let characters: Vec<char> = source.chars().collect();
        let mut utf16_indices = Vec::with_capacity(characters.len() + 1);
        let mut utf16_index = 0;
        for character in &characters {
            utf16_indices.push(utf16_index);
            utf16_index += character.len_utf16();
        }
        utf16_indices.push(utf16_index);
        let start: i32 = jvm.invoke_virtual(&position, "getIndex", "()I", ()).await?;
        if start < 0 {
            let _: () = jvm.invoke_virtual(&position, "setErrorIndex", "(I)V", (start,)).await?;
            return Ok(ClassInstanceRef::new(None));
        }
        let Some(start_index) = utf16_indices.iter().position(|index| *index == start as usize) else {
            let _: () = jvm.invoke_virtual(&position, "setErrorIndex", "(I)V", (start,)).await?;
            return Ok(ClassInstanceRef::new(None));
        };

        let positive_prefix: ClassInstanceRef<String> = jvm.get_field(&this, "positivePrefix", "Ljava/lang/String;").await?;
        let positive_suffix: ClassInstanceRef<String> = jvm.get_field(&this, "positiveSuffix", "Ljava/lang/String;").await?;
        let negative_prefix: ClassInstanceRef<String> = jvm.get_field(&this, "negativePrefix", "Ljava/lang/String;").await?;
        let negative_suffix: ClassInstanceRef<String> = jvm.get_field(&this, "negativeSuffix", "Ljava/lang/String;").await?;
        let positive_prefix: Vec<char> = JavaLangString::to_rust_string(jvm, &positive_prefix).await?.chars().collect();
        let positive_suffix: Vec<char> = JavaLangString::to_rust_string(jvm, &positive_suffix).await?.chars().collect();
        let negative_prefix: Vec<char> = JavaLangString::to_rust_string(jvm, &negative_prefix).await?.chars().collect();
        let negative_suffix: Vec<char> = JavaLangString::to_rust_string(jvm, &negative_suffix).await?.chars().collect();

        let mut index = start_index;
        let prefix_negative = if characters[index..].starts_with(&negative_prefix) && negative_prefix != positive_prefix {
            index += negative_prefix.len();
            true
        } else if characters[index..].starts_with(&positive_prefix) {
            index += positive_prefix.len();
            false
        } else {
            let _: () = jvm.invoke_virtual(&position, "setErrorIndex", "(I)V", (start,)).await?;
            return Ok(ClassInstanceRef::new(None));
        };

        let parse_integer_only: bool = jvm.get_field(&this, "parseIntegerOnly", "Z").await?;
        let mut normalized = RustString::new();
        let mut digits = 0;
        let mut decimal = false;
        while index < characters.len() {
            match characters[index] {
                '0'..='9' => {
                    normalized.push(characters[index]);
                    digits += 1;
                    index += 1;
                }
                ',' if !decimal => index += 1,
                '.' if !decimal && !parse_integer_only => {
                    normalized.push('.');
                    decimal = true;
                    index += 1;
                }
                _ => break,
            }
        }
        if digits == 0 {
            let _: () = jvm
                .invoke_virtual(&position, "setErrorIndex", "(I)V", (utf16_indices[index] as i32,))
                .await?;
            return Ok(ClassInstanceRef::new(None));
        }
        let negative = if negative_prefix == positive_prefix {
            let positive_matches = characters[index..].starts_with(&positive_suffix);
            let negative_matches = characters[index..].starts_with(&negative_suffix);
            if negative_matches && (!positive_matches || negative_suffix.len() > positive_suffix.len()) {
                index += negative_suffix.len();
                true
            } else if positive_matches {
                index += positive_suffix.len();
                false
            } else {
                let _: () = jvm
                    .invoke_virtual(&position, "setErrorIndex", "(I)V", (utf16_indices[index] as i32,))
                    .await?;
                return Ok(ClassInstanceRef::new(None));
            }
        } else {
            let suffix = if prefix_negative { &negative_suffix } else { &positive_suffix };
            if !characters[index..].starts_with(suffix) {
                let _: () = jvm
                    .invoke_virtual(&position, "setErrorIndex", "(I)V", (utf16_indices[index] as i32,))
                    .await?;
                return Ok(ClassInstanceRef::new(None));
            }
            index += suffix.len();
            prefix_negative
        };
        if negative {
            normalized.insert(0, '-');
        }

        let Ok(mut value) = normalized.parse::<f64>() else {
            let _: () = jvm.invoke_virtual(&position, "setErrorIndex", "(I)V", (start,)).await?;
            return Ok(ClassInstanceRef::new(None));
        };
        let multiplier: i32 = jvm.get_field(&this, "multiplier", "I").await?;
        if multiplier != 0 {
            value /= f64::from(multiplier);
        }
        let _: () = jvm.invoke_virtual(&position, "setIndex", "(I)V", (utf16_indices[index] as i32,)).await?;
        let _: () = jvm.invoke_virtual(&position, "setErrorIndex", "(I)V", (-1,)).await?;

        if multiplier == 1
            && !decimal
            && let Ok(value) = normalized.parse::<i64>()
        {
            let result: ClassInstanceRef<Long> = jvm.new_class("java/lang/Long", "(J)V", (value,)).await?.into();
            return Ok(ClassInstanceRef::new(result.instance));
        }
        if value.is_finite() && value.fract() == 0.0 && (-9_223_372_036_854_775_808.0..9_223_372_036_854_775_808.0).contains(&value) {
            let result: ClassInstanceRef<Long> = jvm.new_class("java/lang/Long", "(J)V", (value as i64,)).await?.into();
            return Ok(ClassInstanceRef::new(result.instance));
        }
        let result: ClassInstanceRef<Double> = jvm.new_class("java/lang/Double", "(D)V", (value,)).await?.into();
        Ok(ClassInstanceRef::new(result.instance))
    }

    async fn to_pattern(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "pattern", "Ljava/lang/String;").await
    }

    async fn get_multiplier(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "multiplier", "I").await
    }

    async fn set_multiplier(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, multiplier: i32) -> Result<()> {
        jvm.put_field(&mut this, "multiplier", "I", multiplier).await
    }
}
