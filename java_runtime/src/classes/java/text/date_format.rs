use alloc::{format, string::ToString, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, JavaError, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        lang::{Object, String, StringBuffer},
        text::{FieldPosition, NumberFormat, ParseException, ParsePosition},
        util::{Calendar, Date, Locale, TimeZone},
    },
};

// public abstract class java.text.DateFormat
pub struct DateFormat;

impl DateFormat {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/text/DateFormat",
            parent_class: Some("java/text/Format"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PROTECTED),
                JavaMethodProto::new(
                    "format",
                    "(Ljava/lang/Object;Ljava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;",
                    Self::format_object,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::FINAL,
                ),
                JavaMethodProto::new(
                    "format",
                    "(Ljava/util/Date;)Ljava/lang/String;",
                    Self::format_date,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::FINAL,
                ),
                JavaMethodProto::new_abstract(
                    "format",
                    "(Ljava/util/Date;Ljava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;",
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT,
                ),
                JavaMethodProto::new("parse", "(Ljava/lang/String;)Ljava/util/Date;", Self::parse, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new_abstract(
                    "parse",
                    "(Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/util/Date;",
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT,
                ),
                JavaMethodProto::new(
                    "parseObject",
                    "(Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/lang/Object;",
                    Self::parse_object,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "getInstance",
                    "()Ljava/text/DateFormat;",
                    Self::get_instance,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC | MethodAccessFlags::FINAL,
                ),
                JavaMethodProto::new(
                    "getTimeInstance",
                    "()Ljava/text/DateFormat;",
                    Self::get_time_instance,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC | MethodAccessFlags::FINAL,
                ),
                JavaMethodProto::new(
                    "getTimeInstance",
                    "(I)Ljava/text/DateFormat;",
                    Self::get_time_instance_with_style,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC | MethodAccessFlags::FINAL,
                ),
                JavaMethodProto::new(
                    "getTimeInstance",
                    "(ILjava/util/Locale;)Ljava/text/DateFormat;",
                    Self::get_time_instance_with_locale,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "getDateInstance",
                    "()Ljava/text/DateFormat;",
                    Self::get_date_instance,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC | MethodAccessFlags::FINAL,
                ),
                JavaMethodProto::new(
                    "getDateInstance",
                    "(I)Ljava/text/DateFormat;",
                    Self::get_date_instance_with_style,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC | MethodAccessFlags::FINAL,
                ),
                JavaMethodProto::new(
                    "getDateInstance",
                    "(ILjava/util/Locale;)Ljava/text/DateFormat;",
                    Self::get_date_instance_with_locale,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "getDateTimeInstance",
                    "()Ljava/text/DateFormat;",
                    Self::get_date_time_instance,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC | MethodAccessFlags::FINAL,
                ),
                JavaMethodProto::new(
                    "getDateTimeInstance",
                    "(II)Ljava/text/DateFormat;",
                    Self::get_date_time_instance_with_styles,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC | MethodAccessFlags::FINAL,
                ),
                JavaMethodProto::new(
                    "getDateTimeInstance",
                    "(IILjava/util/Locale;)Ljava/text/DateFormat;",
                    Self::get_date_time_instance_with_locale,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "getAvailableLocales",
                    "()[Ljava/util/Locale;",
                    Self::get_available_locales,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("getCalendar", "()Ljava/util/Calendar;", Self::get_calendar, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("setCalendar", "(Ljava/util/Calendar;)V", Self::set_calendar, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "getNumberFormat",
                    "()Ljava/text/NumberFormat;",
                    Self::get_number_format,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "setNumberFormat",
                    "(Ljava/text/NumberFormat;)V",
                    Self::set_number_format,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("getTimeZone", "()Ljava/util/TimeZone;", Self::get_time_zone, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("setTimeZone", "(Ljava/util/TimeZone;)V", Self::set_time_zone, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("isLenient", "()Z", Self::is_lenient, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("setLenient", "(Z)V", Self::set_lenient, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("clone", "()Ljava/lang/Object;", Self::clone, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("calendar", "Ljava/util/Calendar;", FieldAccessFlags::PROTECTED),
                JavaFieldProto::new("numberFormat", "Ljava/text/NumberFormat;", FieldAccessFlags::PROTECTED),
                JavaFieldProto::new(
                    "ERA_FIELD",
                    "I",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "YEAR_FIELD",
                    "I",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "MONTH_FIELD",
                    "I",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "DATE_FIELD",
                    "I",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "HOUR_OF_DAY1_FIELD",
                    "I",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "HOUR_OF_DAY0_FIELD",
                    "I",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "MINUTE_FIELD",
                    "I",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "SECOND_FIELD",
                    "I",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "MILLISECOND_FIELD",
                    "I",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "DAY_OF_WEEK_FIELD",
                    "I",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "DAY_OF_YEAR_FIELD",
                    "I",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "DAY_OF_WEEK_IN_MONTH_FIELD",
                    "I",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "WEEK_OF_YEAR_FIELD",
                    "I",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "WEEK_OF_MONTH_FIELD",
                    "I",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "AM_PM_FIELD",
                    "I",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "HOUR1_FIELD",
                    "I",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "HOUR0_FIELD",
                    "I",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "TIMEZONE_FIELD",
                    "I",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new("FULL", "I", FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new("LONG", "I", FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL),
                JavaFieldProto::new(
                    "MEDIUM",
                    "I",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "SHORT",
                    "I",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "DEFAULT",
                    "I",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
            ],
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::ABSTRACT,
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        for (name, value) in [
            ("ERA_FIELD", 0),
            ("YEAR_FIELD", 1),
            ("MONTH_FIELD", 2),
            ("DATE_FIELD", 3),
            ("HOUR_OF_DAY1_FIELD", 4),
            ("HOUR_OF_DAY0_FIELD", 5),
            ("MINUTE_FIELD", 6),
            ("SECOND_FIELD", 7),
            ("MILLISECOND_FIELD", 8),
            ("DAY_OF_WEEK_FIELD", 9),
            ("DAY_OF_YEAR_FIELD", 10),
            ("DAY_OF_WEEK_IN_MONTH_FIELD", 11),
            ("WEEK_OF_YEAR_FIELD", 12),
            ("WEEK_OF_MONTH_FIELD", 13),
            ("AM_PM_FIELD", 14),
            ("HOUR1_FIELD", 15),
            ("HOUR0_FIELD", 16),
            ("TIMEZONE_FIELD", 17),
            ("FULL", 0),
            ("LONG", 1),
            ("MEDIUM", 2),
            ("SHORT", 3),
            ("DEFAULT", 2),
        ] {
            jvm.put_static_field("java/text/DateFormat", name, "I", value).await?;
        }
        Ok(())
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/text/Format", "<init>", "()V", ()).await?;
        let calendar: ClassInstanceRef<Calendar> = jvm
            .invoke_static("java/util/Calendar", "getInstance", "()Ljava/util/Calendar;", ())
            .await?;
        let number_format: ClassInstanceRef<NumberFormat> = jvm
            .invoke_static("java/text/NumberFormat", "getInstance", "()Ljava/text/NumberFormat;", ())
            .await?;
        jvm.put_field(&mut this, "calendar", "Ljava/util/Calendar;", calendar).await?;
        jvm.put_field(&mut this, "numberFormat", "Ljava/text/NumberFormat;", number_format).await
    }

    async fn format_object(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        object: ClassInstanceRef<Object>,
        buffer: ClassInstanceRef<StringBuffer>,
        position: ClassInstanceRef<FieldPosition>,
    ) -> Result<ClassInstanceRef<StringBuffer>> {
        if object.is_null() || !jvm.is_instance(&**object, "java/util/Date") {
            return Err(jvm
                .exception("java/lang/IllegalArgumentException", "Cannot format given Object as a Date")
                .await);
        }
        let date: ClassInstanceRef<Date> = ClassInstanceRef::new(object.instance);
        jvm.invoke_virtual(
            &this,
            "format",
            "(Ljava/util/Date;Ljava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;",
            (date, buffer, position),
        )
        .await
    }

    async fn format_date(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        date: ClassInstanceRef<Date>,
    ) -> Result<ClassInstanceRef<String>> {
        if date.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "date").await);
        }
        let buffer: ClassInstanceRef<StringBuffer> = jvm.new_class("java/lang/StringBuffer", "()V", ()).await?.into();
        let position: ClassInstanceRef<FieldPosition> = jvm.new_class("java/text/FieldPosition", "(I)V", (0,)).await?.into();
        let buffer: ClassInstanceRef<StringBuffer> = jvm
            .invoke_virtual(
                &this,
                "format",
                "(Ljava/util/Date;Ljava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;",
                (date, buffer, position),
            )
            .await?;
        jvm.invoke_virtual(&buffer, "toString", "()Ljava/lang/String;", ()).await
    }

    async fn parse(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        source: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Date>> {
        if source.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "source").await);
        }
        let position: ClassInstanceRef<ParsePosition> = jvm.new_class("java/text/ParsePosition", "(I)V", (0,)).await?.into();
        let date: ClassInstanceRef<Date> = jvm
            .invoke_virtual(
                &this,
                "parse",
                "(Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/util/Date;",
                (source, position.clone()),
            )
            .await?;
        let index: i32 = jvm.invoke_virtual(&position, "getIndex", "()I", ()).await?;
        if index == 0 {
            let error_index: i32 = jvm.invoke_virtual(&position, "getErrorIndex", "()I", ()).await?;
            let message = JavaLangString::from_rust_string(jvm, "Unparseable date").await?;
            let exception: ClassInstanceRef<ParseException> = jvm
                .new_class("java/text/ParseException", "(Ljava/lang/String;I)V", (message, error_index))
                .await?
                .into();
            return Err(JavaError::JavaException(exception.into()));
        }
        Ok(date)
    }

    async fn parse_object(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        source: ClassInstanceRef<String>,
        position: ClassInstanceRef<ParsePosition>,
    ) -> Result<ClassInstanceRef<Object>> {
        let date: ClassInstanceRef<Date> = jvm
            .invoke_virtual(
                &this,
                "parse",
                "(Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/util/Date;",
                (source, position),
            )
            .await?;
        Ok(ClassInstanceRef::new(date.instance))
    }

    fn date_pattern(style: i32) -> Option<&'static str> {
        match style {
            0 => Some("EEEE, MMMM d, yyyy"),
            1 => Some("MMMM d, yyyy"),
            2 => Some("MMM d, yyyy"),
            3 => Some("M/d/yy"),
            _ => None,
        }
    }

    fn time_pattern(style: i32) -> Option<&'static str> {
        match style {
            0 | 1 => Some("h:mm:ss a z"),
            2 => Some("h:mm:ss a"),
            3 => Some("h:mm a"),
            _ => None,
        }
    }

    async fn new_formatter(
        jvm: &Jvm,
        date_style: Option<i32>,
        time_style: Option<i32>,
        locale: ClassInstanceRef<Locale>,
    ) -> Result<ClassInstanceRef<Self>> {
        if locale.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "locale").await);
        }
        let date_pattern = date_style.and_then(Self::date_pattern);
        let time_pattern = time_style.and_then(Self::time_pattern);
        if (date_style.is_some() && date_pattern.is_none()) || (time_style.is_some() && time_pattern.is_none()) {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "Illegal date style").await);
        }
        let pattern = match (date_pattern, time_pattern) {
            (Some(date), Some(time)) => format!("{date} {time}"),
            (Some(date), None) => date.to_string(),
            (None, Some(time)) => time.to_string(),
            (None, None) => return Err(jvm.exception("java/lang/IllegalArgumentException", "No date or time style").await),
        };
        let pattern = JavaLangString::from_rust_string(jvm, &pattern).await?;
        Ok(jvm
            .new_class("java/text/SimpleDateFormat", "(Ljava/lang/String;Ljava/util/Locale;)V", (pattern, locale))
            .await?
            .into())
    }

    async fn get_instance(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        let locale: ClassInstanceRef<Locale> = jvm.invoke_static("java/util/Locale", "getDefault", "()Ljava/util/Locale;", ()).await?;
        Self::new_formatter(jvm, Some(3), Some(3), locale).await
    }

    async fn get_time_instance(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        let locale: ClassInstanceRef<Locale> = jvm.invoke_static("java/util/Locale", "getDefault", "()Ljava/util/Locale;", ()).await?;
        Self::new_formatter(jvm, None, Some(2), locale).await
    }

    async fn get_time_instance_with_style(jvm: &Jvm, _: &mut RuntimeContext, style: i32) -> Result<ClassInstanceRef<Self>> {
        let locale: ClassInstanceRef<Locale> = jvm.invoke_static("java/util/Locale", "getDefault", "()Ljava/util/Locale;", ()).await?;
        Self::new_formatter(jvm, None, Some(style), locale).await
    }

    async fn get_time_instance_with_locale(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        style: i32,
        locale: ClassInstanceRef<Locale>,
    ) -> Result<ClassInstanceRef<Self>> {
        Self::new_formatter(jvm, None, Some(style), locale).await
    }

    async fn get_date_instance(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        let locale: ClassInstanceRef<Locale> = jvm.invoke_static("java/util/Locale", "getDefault", "()Ljava/util/Locale;", ()).await?;
        Self::new_formatter(jvm, Some(2), None, locale).await
    }

    async fn get_date_instance_with_style(jvm: &Jvm, _: &mut RuntimeContext, style: i32) -> Result<ClassInstanceRef<Self>> {
        let locale: ClassInstanceRef<Locale> = jvm.invoke_static("java/util/Locale", "getDefault", "()Ljava/util/Locale;", ()).await?;
        Self::new_formatter(jvm, Some(style), None, locale).await
    }

    async fn get_date_instance_with_locale(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        style: i32,
        locale: ClassInstanceRef<Locale>,
    ) -> Result<ClassInstanceRef<Self>> {
        Self::new_formatter(jvm, Some(style), None, locale).await
    }

    async fn get_date_time_instance(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        let locale: ClassInstanceRef<Locale> = jvm.invoke_static("java/util/Locale", "getDefault", "()Ljava/util/Locale;", ()).await?;
        Self::new_formatter(jvm, Some(2), Some(2), locale).await
    }

    async fn get_date_time_instance_with_styles(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        date_style: i32,
        time_style: i32,
    ) -> Result<ClassInstanceRef<Self>> {
        let locale: ClassInstanceRef<Locale> = jvm.invoke_static("java/util/Locale", "getDefault", "()Ljava/util/Locale;", ()).await?;
        Self::new_formatter(jvm, Some(date_style), Some(time_style), locale).await
    }

    async fn get_date_time_instance_with_locale(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        date_style: i32,
        time_style: i32,
        locale: ClassInstanceRef<Locale>,
    ) -> Result<ClassInstanceRef<Self>> {
        Self::new_formatter(jvm, Some(date_style), Some(time_style), locale).await
    }

    async fn get_available_locales(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Array<Locale>>> {
        jvm.invoke_static("java/util/Locale", "getAvailableLocales", "()[Ljava/util/Locale;", ())
            .await
    }

    async fn get_calendar(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Calendar>> {
        jvm.get_field(&this, "calendar", "Ljava/util/Calendar;").await
    }

    async fn set_calendar(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, calendar: ClassInstanceRef<Calendar>) -> Result<()> {
        if calendar.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "calendar").await);
        }
        jvm.put_field(&mut this, "calendar", "Ljava/util/Calendar;", calendar).await
    }

    async fn get_number_format(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<NumberFormat>> {
        jvm.get_field(&this, "numberFormat", "Ljava/text/NumberFormat;").await
    }

    async fn set_number_format(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        number_format: ClassInstanceRef<NumberFormat>,
    ) -> Result<()> {
        if number_format.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "numberFormat").await);
        }
        jvm.put_field(&mut this, "numberFormat", "Ljava/text/NumberFormat;", number_format).await
    }

    async fn get_time_zone(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<TimeZone>> {
        let calendar: ClassInstanceRef<Calendar> = jvm.get_field(&this, "calendar", "Ljava/util/Calendar;").await?;
        jvm.invoke_virtual(&calendar, "getTimeZone", "()Ljava/util/TimeZone;", ()).await
    }

    async fn set_time_zone(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, time_zone: ClassInstanceRef<TimeZone>) -> Result<()> {
        let calendar: ClassInstanceRef<Calendar> = jvm.get_field(&this, "calendar", "Ljava/util/Calendar;").await?;
        jvm.invoke_virtual(&calendar, "setTimeZone", "(Ljava/util/TimeZone;)V", (time_zone,))
            .await
    }

    async fn is_lenient(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let calendar: ClassInstanceRef<Calendar> = jvm.get_field(&this, "calendar", "Ljava/util/Calendar;").await?;
        jvm.invoke_virtual(&calendar, "isLenient", "()Z", ()).await
    }

    async fn set_lenient(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, lenient: bool) -> Result<()> {
        let calendar: ClassInstanceRef<Calendar> = jvm.get_field(&this, "calendar", "Ljava/util/Calendar;").await?;
        jvm.invoke_virtual(&calendar, "setLenient", "(Z)V", (lenient,)).await
    }

    async fn clone(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let calendar: ClassInstanceRef<Calendar> = jvm.get_field(&this, "calendar", "Ljava/util/Calendar;").await?;
        let number_format: ClassInstanceRef<NumberFormat> = jvm.get_field(&this, "numberFormat", "Ljava/text/NumberFormat;").await?;
        let mut cloned: ClassInstanceRef<Self> = jvm.shallow_clone(&this)?.into();
        let mut cloned_calendar: ClassInstanceRef<Calendar> = jvm.shallow_clone(&calendar)?.into();
        let fields: ClassInstanceRef<Array<i32>> = jvm.get_field(&calendar, "fields", "[I").await?;
        let cloned_fields: ClassInstanceRef<Array<i32>> = jvm.shallow_clone(&fields)?.into();
        let time_zone: ClassInstanceRef<TimeZone> = jvm.get_field(&calendar, "timeZone", "Ljava/util/TimeZone;").await?;
        let cloned_time_zone: ClassInstanceRef<TimeZone> = jvm.shallow_clone(&time_zone)?.into();
        jvm.put_field(&mut cloned_calendar, "fields", "[I", cloned_fields).await?;
        jvm.put_field(&mut cloned_calendar, "timeZone", "Ljava/util/TimeZone;", cloned_time_zone)
            .await?;
        let cloned_number_format: ClassInstanceRef<NumberFormat> = jvm.shallow_clone(&number_format)?.into();
        jvm.put_field(&mut cloned, "calendar", "Ljava/util/Calendar;", cloned_calendar).await?;
        jvm.put_field(&mut cloned, "numberFormat", "Ljava/text/NumberFormat;", cloned_number_format)
            .await?;
        Ok(ClassInstanceRef::new(cloned.instance))
    }
}
