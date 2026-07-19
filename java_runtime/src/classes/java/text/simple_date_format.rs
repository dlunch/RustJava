use alloc::{format, string::String as RustString, vec, vec::Vec};

use chrono::{DateTime, Datelike, NaiveDate, TimeZone as ChronoTimeZone, Timelike, Utc};
use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        lang::{String, StringBuffer},
        text::{FieldPosition, ParsePosition},
        util::{Calendar, Date, Locale, TimeZone},
    },
};

#[derive(Clone)]
enum DateToken {
    Literal(RustString),
    Field(char, usize),
}

// public class java.text.SimpleDateFormat
pub struct SimpleDateFormat;

impl SimpleDateFormat {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/text/SimpleDateFormat",
            parent_class: Some("java/text/DateFormat"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_with_pattern, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/lang/String;Ljava/util/Locale;)V",
                    Self::init_with_pattern_and_locale,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "format",
                    "(Ljava/util/Date;Ljava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;",
                    Self::format,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "parse",
                    "(Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/util/Date;",
                    Self::parse,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("applyPattern", "(Ljava/lang/String;)V", Self::apply_pattern, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("toPattern", "()Ljava/lang/String;", Self::to_pattern, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![JavaFieldProto::new("pattern", "Ljava/lang/String;", FieldAccessFlags::PRIVATE)],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    fn tokenize_pattern(pattern: &str) -> Option<Vec<DateToken>> {
        let characters: Vec<char> = pattern.chars().collect();
        let mut tokens = Vec::new();
        let mut literal = RustString::new();
        let mut quoted = false;
        let mut index = 0;
        while index < characters.len() {
            let character = characters[index];
            if character == '\'' {
                if index + 1 < characters.len() && characters[index + 1] == '\'' {
                    literal.push('\'');
                    index += 2;
                    continue;
                }
                quoted = !quoted;
                index += 1;
                continue;
            }
            if !quoted && character.is_ascii_alphabetic() {
                if !matches!(
                    character,
                    'G' | 'y' | 'M' | 'd' | 'h' | 'H' | 'm' | 's' | 'S' | 'E' | 'D' | 'F' | 'w' | 'W' | 'a' | 'k' | 'K' | 'z'
                ) {
                    return None;
                }
                if !literal.is_empty() {
                    tokens.push(DateToken::Literal(core::mem::take(&mut literal)));
                }
                let mut count = 1;
                while index + count < characters.len() && characters[index + count] == character {
                    count += 1;
                }
                tokens.push(DateToken::Field(character, count));
                index += count;
                continue;
            }
            literal.push(character);
            index += 1;
        }
        if quoted {
            return None;
        }
        if !literal.is_empty() {
            tokens.push(DateToken::Literal(literal));
        }
        Some(tokens)
    }

    fn parse_number(characters: &[char], index: &mut usize, maximum_digits: Option<usize>) -> Option<i32> {
        let start = *index;
        let mut value = 0i32;
        while *index < characters.len() && characters[*index].is_ascii_digit() && maximum_digits.is_none_or(|maximum| *index - start < maximum) {
            value = value.checked_mul(10)?.checked_add(characters[*index].to_digit(10)? as i32)?;
            *index += 1;
        }
        if *index == start { None } else { Some(value) }
    }

    fn starts_with_ignore_ascii_case(characters: &[char], index: usize, value: &str) -> bool {
        let value: Vec<char> = value.chars().collect();
        characters
            .get(index..index + value.len())
            .is_some_and(|candidate| candidate.iter().zip(value).all(|(left, right)| left.eq_ignore_ascii_case(&right)))
    }

    fn parse_timestamp(tokens: &[DateToken], characters: &[char], start: usize, default_offset: i32) -> core::result::Result<(i64, usize), usize> {
        let months = [
            ("January", "Jan"),
            ("February", "Feb"),
            ("March", "Mar"),
            ("April", "Apr"),
            ("May", "May"),
            ("June", "Jun"),
            ("July", "Jul"),
            ("August", "Aug"),
            ("September", "Sep"),
            ("October", "Oct"),
            ("November", "Nov"),
            ("December", "Dec"),
        ];
        let weekdays = [
            ("Sunday", "Sun"),
            ("Monday", "Mon"),
            ("Tuesday", "Tue"),
            ("Wednesday", "Wed"),
            ("Thursday", "Thu"),
            ("Friday", "Fri"),
            ("Saturday", "Sat"),
        ];

        let mut index = start;
        let mut era = 1;
        let mut year = 1970;
        let mut month = 1;
        let mut day = 1;
        let mut ordinal = None;
        let mut month_set = false;
        let mut day_set = false;
        let mut hour = 0;
        let mut twelve_hour = None;
        let mut am_pm = 0;
        let mut minute = 0;
        let mut second = 0;
        let mut millisecond = 0;
        let mut offset = default_offset;

        for token in tokens {
            match token {
                DateToken::Literal(literal) => {
                    let value: Vec<char> = literal.chars().collect();
                    if !characters[index..].starts_with(&value) {
                        return Err(index);
                    }
                    index += value.len();
                }
                DateToken::Field(character, count) => match character {
                    'G' => {
                        if Self::starts_with_ignore_ascii_case(characters, index, "AD") {
                            era = 1;
                            index += 2;
                        } else if Self::starts_with_ignore_ascii_case(characters, index, "BC") {
                            era = 0;
                            index += 2;
                        } else {
                            return Err(index);
                        }
                    }
                    'y' => {
                        let value = Self::parse_number(characters, &mut index, if *count > 1 { Some(*count) } else { None }).ok_or(index)?;
                        year = if *count == 2 {
                            if value <= 69 { 2000 + value } else { 1900 + value }
                        } else {
                            value
                        };
                    }
                    'M' if *count >= 3 => {
                        let mut parsed = None;
                        for (position, (full, short)) in months.iter().enumerate() {
                            let candidate = if *count >= 4 { *full } else { *short };
                            if Self::starts_with_ignore_ascii_case(characters, index, candidate) {
                                parsed = Some((position as i32 + 1, candidate.len()));
                                break;
                            }
                        }
                        let Some((value, length)) = parsed else {
                            return Err(index);
                        };
                        month = value;
                        month_set = true;
                        index += length;
                    }
                    'M' => {
                        month = Self::parse_number(characters, &mut index, if *count > 1 { Some(*count) } else { None }).ok_or(index)?;
                        month_set = true;
                    }
                    'd' => {
                        day = Self::parse_number(characters, &mut index, if *count > 1 { Some(*count) } else { None }).ok_or(index)?;
                        day_set = true;
                    }
                    'H' => hour = Self::parse_number(characters, &mut index, if *count > 1 { Some(*count) } else { None }).ok_or(index)?,
                    'k' => {
                        let value = Self::parse_number(characters, &mut index, if *count > 1 { Some(*count) } else { None }).ok_or(index)?;
                        if !(1..=24).contains(&value) {
                            return Err(index);
                        }
                        hour = value % 24;
                    }
                    'h' | 'K' => {
                        let value = Self::parse_number(characters, &mut index, if *count > 1 { Some(*count) } else { None }).ok_or(index)?;
                        twelve_hour = Some((*character, value));
                    }
                    'm' => minute = Self::parse_number(characters, &mut index, if *count > 1 { Some(*count) } else { None }).ok_or(index)?,
                    's' => second = Self::parse_number(characters, &mut index, if *count > 1 { Some(*count) } else { None }).ok_or(index)?,
                    'S' => millisecond = Self::parse_number(characters, &mut index, Some((*count).max(1))).ok_or(index)?,
                    'E' => {
                        let mut consumed = None;
                        for (full, short) in weekdays {
                            for candidate in [full, short] {
                                if Self::starts_with_ignore_ascii_case(characters, index, candidate) {
                                    consumed = Some(candidate.len());
                                    break;
                                }
                            }
                            if consumed.is_some() {
                                break;
                            }
                        }
                        let Some(length) = consumed else {
                            return Err(index);
                        };
                        index += length;
                    }
                    'D' => ordinal = Some(Self::parse_number(characters, &mut index, if *count > 1 { Some(*count) } else { None }).ok_or(index)?),
                    'F' | 'w' | 'W' => {
                        let _ = Self::parse_number(characters, &mut index, if *count > 1 { Some(*count) } else { None }).ok_or(index)?;
                    }
                    'a' => {
                        if Self::starts_with_ignore_ascii_case(characters, index, "AM") {
                            am_pm = 0;
                            index += 2;
                        } else if Self::starts_with_ignore_ascii_case(characters, index, "PM") {
                            am_pm = 1;
                            index += 2;
                        } else {
                            return Err(index);
                        }
                    }
                    'z' => {
                        if Self::starts_with_ignore_ascii_case(characters, index, "GMT")
                            || Self::starts_with_ignore_ascii_case(characters, index, "UTC")
                        {
                            index += 3;
                        } else {
                            return Err(index);
                        }
                        offset = 0;
                        if index < characters.len() && matches!(characters[index], '+' | '-') {
                            let sign = if characters[index] == '-' { -1 } else { 1 };
                            index += 1;
                            let hours = Self::parse_number(characters, &mut index, Some(2)).ok_or(index)?;
                            if index >= characters.len() || characters[index] != ':' {
                                return Err(index);
                            }
                            index += 1;
                            let minutes = Self::parse_number(characters, &mut index, Some(2)).ok_or(index)?;
                            if hours > 23 || minutes > 59 {
                                return Err(index);
                            }
                            offset = sign * (hours * 60 + minutes) * 60 * 1000;
                        }
                    }
                    _ => return Err(index),
                },
            }
        }

        if let Some((kind, value)) = twelve_hour {
            hour = match kind {
                'h' if (1..=12).contains(&value) => value % 12 + am_pm * 12,
                'K' if (0..=11).contains(&value) => value + am_pm * 12,
                _ => return Err(index),
            };
        }
        if era == 0 {
            year = 1 - year;
        }
        if let Some(ordinal) = ordinal.filter(|_| !month_set && !day_set) {
            let Some(date) = NaiveDate::from_yo_opt(year, ordinal as u32) else {
                return Err(index);
            };
            month = date.month() as i32;
            day = date.day() as i32;
        }
        if !(1..=12).contains(&month)
            || !(1..=31).contains(&day)
            || !(0..=23).contains(&hour)
            || !(0..=59).contains(&minute)
            || !(0..=59).contains(&second)
            || !(0..=999).contains(&millisecond)
        {
            return Err(index);
        }
        let Some(date_time) = Utc
            .with_ymd_and_hms(year, month as u32, day as u32, hour as u32, minute as u32, second as u32)
            .single()
        else {
            return Err(index);
        };
        let Some(timestamp) = date_time
            .timestamp_millis()
            .checked_add(i64::from(millisecond))
            .and_then(|value| value.checked_sub(i64::from(offset)))
        else {
            return Err(index);
        };
        Ok((timestamp, index))
    }

    fn date_field(character: char) -> Option<i32> {
        match character {
            'G' => Some(0),
            'y' => Some(1),
            'M' => Some(2),
            'd' => Some(3),
            'k' => Some(4),
            'H' => Some(5),
            'm' => Some(6),
            's' => Some(7),
            'S' => Some(8),
            'E' => Some(9),
            'D' => Some(10),
            'F' => Some(11),
            'w' => Some(12),
            'W' => Some(13),
            'a' => Some(14),
            'h' => Some(15),
            'K' => Some(16),
            'z' => Some(17),
            _ => None,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let pattern = JavaLangString::from_rust_string(jvm, "M/d/yy h:mm a").await?;
        jvm.invoke_special(&this, "java/text/SimpleDateFormat", "<init>", "(Ljava/lang/String;)V", (pattern,))
            .await
    }

    async fn init_with_pattern(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, pattern: ClassInstanceRef<String>) -> Result<()> {
        let locale: ClassInstanceRef<Locale> = jvm.invoke_static("java/util/Locale", "getDefault", "()Ljava/util/Locale;", ()).await?;
        jvm.invoke_special(
            &this,
            "java/text/SimpleDateFormat",
            "<init>",
            "(Ljava/lang/String;Ljava/util/Locale;)V",
            (pattern, locale),
        )
        .await
    }

    async fn init_with_pattern_and_locale(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        pattern: ClassInstanceRef<String>,
        locale: ClassInstanceRef<Locale>,
    ) -> Result<()> {
        if pattern.is_null() || locale.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "pattern or locale").await);
        }
        let value = JavaLangString::to_rust_string(jvm, &pattern).await?;
        if Self::tokenize_pattern(&value).is_none() {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "Illegal pattern character").await);
        }
        let _: () = jvm.invoke_special(&this, "java/text/DateFormat", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "pattern", "Ljava/lang/String;", pattern).await
    }

    async fn apply_pattern(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, pattern: ClassInstanceRef<String>) -> Result<()> {
        if pattern.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "pattern").await);
        }
        let value = JavaLangString::to_rust_string(jvm, &pattern).await?;
        if Self::tokenize_pattern(&value).is_none() {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "Illegal pattern character").await);
        }
        jvm.put_field(&mut this, "pattern", "Ljava/lang/String;", pattern).await
    }

    async fn to_pattern(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "pattern", "Ljava/lang/String;").await
    }

    async fn format(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        date: ClassInstanceRef<Date>,
        buffer: ClassInstanceRef<StringBuffer>,
        position: ClassInstanceRef<FieldPosition>,
    ) -> Result<ClassInstanceRef<StringBuffer>> {
        if date.is_null() || buffer.is_null() || position.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "date, buffer, or position").await);
        }
        let pattern: ClassInstanceRef<String> = jvm.get_field(&this, "pattern", "Ljava/lang/String;").await?;
        let pattern = JavaLangString::to_rust_string(jvm, &pattern).await?;
        let Some(tokens) = Self::tokenize_pattern(&pattern) else {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "Illegal pattern").await);
        };
        let time: i64 = jvm.invoke_virtual(&date, "getTime", "()J", ()).await?;
        let calendar: ClassInstanceRef<Calendar> = jvm.get_field(&this, "calendar", "Ljava/util/Calendar;").await?;
        let time_zone: ClassInstanceRef<TimeZone> = jvm.invoke_virtual(&calendar, "getTimeZone", "()Ljava/util/TimeZone;", ()).await?;
        let offset: i32 = jvm.invoke_virtual(&time_zone, "getRawOffset", "()I", ()).await?;
        let Some(adjusted) = time.checked_add(i64::from(offset)) else {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "date out of range").await);
        };
        let Some(date_time) = DateTime::<Utc>::from_timestamp_millis(adjusted) else {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "date out of range").await);
        };

        let months = [
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
        ];
        let short_months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
        let weekdays = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
        let short_weekdays = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
        let requested_field: i32 = jvm.invoke_virtual(&position, "getField", "()I", ()).await?;
        let base: i32 = jvm.invoke_virtual(&buffer, "length", "()I", ()).await?;
        let mut field_position_set = false;
        let mut formatted = RustString::new();
        for token in tokens {
            match token {
                DateToken::Literal(value) => formatted.push_str(&value),
                DateToken::Field(character, count) => {
                    let begin = formatted.encode_utf16().count() as i32;
                    match character {
                        'G' => formatted.push_str(if date_time.year() <= 0 { "BC" } else { "AD" }),
                        'y' => {
                            let year = if date_time.year() <= 0 { 1 - date_time.year() } else { date_time.year() };
                            if count == 2 {
                                formatted.push_str(&format!("{:02}", year.rem_euclid(100)));
                            } else {
                                formatted.push_str(&format!("{year:0count$}"));
                            }
                        }
                        'M' if count >= 4 => formatted.push_str(months[date_time.month0() as usize]),
                        'M' if count == 3 => formatted.push_str(short_months[date_time.month0() as usize]),
                        'M' => formatted.push_str(&format!("{:0count$}", date_time.month())),
                        'd' => formatted.push_str(&format!("{:0count$}", date_time.day())),
                        'h' => {
                            let hour = date_time.hour() % 12;
                            formatted.push_str(&format!("{:0count$}", if hour == 0 { 12 } else { hour }));
                        }
                        'H' => formatted.push_str(&format!("{:0count$}", date_time.hour())),
                        'm' => formatted.push_str(&format!("{:0count$}", date_time.minute())),
                        's' => formatted.push_str(&format!("{:0count$}", date_time.second())),
                        'S' => formatted.push_str(&format!("{:0count$}", date_time.timestamp_subsec_millis())),
                        'E' if count >= 4 => formatted.push_str(weekdays[date_time.weekday().num_days_from_sunday() as usize]),
                        'E' => formatted.push_str(short_weekdays[date_time.weekday().num_days_from_sunday() as usize]),
                        'D' => formatted.push_str(&format!("{:0count$}", date_time.ordinal())),
                        'F' | 'W' => formatted.push_str(&format!("{:0count$}", (date_time.day() - 1) / 7 + 1)),
                        'w' => formatted.push_str(&format!("{:0count$}", date_time.iso_week().week())),
                        'a' => formatted.push_str(if date_time.hour() < 12 { "AM" } else { "PM" }),
                        'k' => formatted.push_str(&format!("{:0count$}", if date_time.hour() == 0 { 24 } else { date_time.hour() })),
                        'K' => formatted.push_str(&format!("{:0count$}", date_time.hour() % 12)),
                        'z' => {
                            if offset == 0 {
                                formatted.push_str("GMT");
                            } else {
                                let absolute = offset.unsigned_abs() / 60_000;
                                formatted.push_str(&format!(
                                    "GMT{}{:02}:{:02}",
                                    if offset < 0 { '-' } else { '+' },
                                    absolute / 60,
                                    absolute % 60
                                ));
                            }
                        }
                        _ => return Err(jvm.exception("java/lang/IllegalArgumentException", "Illegal pattern").await),
                    }
                    if !field_position_set && Self::date_field(character) == Some(requested_field) {
                        let _: () = jvm.invoke_virtual(&position, "setBeginIndex", "(I)V", (base + begin,)).await?;
                        let _: () = jvm
                            .invoke_virtual(&position, "setEndIndex", "(I)V", (base + formatted.encode_utf16().count() as i32,))
                            .await?;
                        field_position_set = true;
                    }
                }
            }
        }
        let text = JavaLangString::from_rust_string(jvm, &formatted).await?;
        jvm.invoke_virtual(&buffer, "append", "(Ljava/lang/String;)Ljava/lang/StringBuffer;", (text,))
            .await
    }

    async fn parse(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        source: ClassInstanceRef<String>,
        position: ClassInstanceRef<ParsePosition>,
    ) -> Result<ClassInstanceRef<Date>> {
        if source.is_null() || position.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "source or position").await);
        }
        let pattern: ClassInstanceRef<String> = jvm.get_field(&this, "pattern", "Ljava/lang/String;").await?;
        let pattern = JavaLangString::to_rust_string(jvm, &pattern).await?;
        let Some(tokens) = Self::tokenize_pattern(&pattern) else {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "Illegal pattern").await);
        };
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
        let calendar: ClassInstanceRef<Calendar> = jvm.get_field(&this, "calendar", "Ljava/util/Calendar;").await?;
        let time_zone: ClassInstanceRef<TimeZone> = jvm.invoke_virtual(&calendar, "getTimeZone", "()Ljava/util/TimeZone;", ()).await?;
        let offset: i32 = jvm.invoke_virtual(&time_zone, "getRawOffset", "()I", ()).await?;
        match Self::parse_timestamp(&tokens, &characters, start_index, offset) {
            Ok((timestamp, index)) => {
                let _: () = jvm.invoke_virtual(&position, "setIndex", "(I)V", (utf16_indices[index] as i32,)).await?;
                let _: () = jvm.invoke_virtual(&position, "setErrorIndex", "(I)V", (-1,)).await?;
                Ok(jvm.new_class("java/util/Date", "(J)V", (timestamp,)).await?.into())
            }
            Err(error_index) => {
                let _: () = jvm
                    .invoke_virtual(&position, "setErrorIndex", "(I)V", (utf16_indices[error_index] as i32,))
                    .await?;
                Ok(ClassInstanceRef::new(None))
            }
        }
    }
}
