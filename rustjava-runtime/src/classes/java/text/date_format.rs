use alloc::{format, string::ToString};

use jvm::{Array, ClassInstanceRef, JavaError, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{
        lang::{Object, String, StringBuffer},
        text::{FieldPosition, NumberFormat, ParseException, ParsePosition},
        util::{Calendar, Date, Locale, TimeZone},
    },
};

// public abstract class java.text.DateFormat
#[java_class(name = "java/text/DateFormat", extends = "java/text/Format", public, abstract)]
pub struct DateFormat {
    #[field(protected)]
    calendar: ClassInstanceRef<Calendar>,
    #[field(protected)]
    numberFormat: ClassInstanceRef<NumberFormat>,
    #[field(public, static, final)]
    ERA_FIELD: i32,
    #[field(public, static, final)]
    YEAR_FIELD: i32,
    #[field(public, static, final)]
    MONTH_FIELD: i32,
    #[field(public, static, final)]
    DATE_FIELD: i32,
    #[field(public, static, final)]
    HOUR_OF_DAY1_FIELD: i32,
    #[field(public, static, final)]
    HOUR_OF_DAY0_FIELD: i32,
    #[field(public, static, final)]
    MINUTE_FIELD: i32,
    #[field(public, static, final)]
    SECOND_FIELD: i32,
    #[field(public, static, final)]
    MILLISECOND_FIELD: i32,
    #[field(public, static, final)]
    DAY_OF_WEEK_FIELD: i32,
    #[field(public, static, final)]
    DAY_OF_YEAR_FIELD: i32,
    #[field(public, static, final)]
    DAY_OF_WEEK_IN_MONTH_FIELD: i32,
    #[field(public, static, final)]
    WEEK_OF_YEAR_FIELD: i32,
    #[field(public, static, final)]
    WEEK_OF_MONTH_FIELD: i32,
    #[field(public, static, final)]
    AM_PM_FIELD: i32,
    #[field(public, static, final)]
    HOUR1_FIELD: i32,
    #[field(public, static, final)]
    HOUR0_FIELD: i32,
    #[field(public, static, final)]
    TIMEZONE_FIELD: i32,
    #[field(public, static, final)]
    FULL: i32,
    #[field(public, static, final)]
    LONG: i32,
    #[field(public, static, final)]
    MEDIUM: i32,
    #[field(public, static, final)]
    SHORT: i32,
    #[field(public, static, final)]
    DEFAULT: i32,
}

#[java_methods]
impl DateFormat {
    #[method(static_initializer)]
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

    #[method(constructor, protected)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/text/Format", "<init>", "()V", ()).await?;
        let calendar: ClassInstanceRef<Calendar> = jvm
            .invoke_static("java/util/Calendar", "getInstance", "()Ljava/util/Calendar;", ())
            .await?;
        let number_format: ClassInstanceRef<NumberFormat> = jvm
            .invoke_static("java/text/NumberFormat", "getInstance", "()Ljava/text/NumberFormat;", ())
            .await?;
        jvm.put_field(&mut this, "java/text/DateFormat", "calendar", "Ljava/util/Calendar;", calendar)
            .await?;
        jvm.put_field(
            &mut this,
            "java/text/DateFormat",
            "numberFormat",
            "Ljava/text/NumberFormat;",
            number_format,
        )
        .await
    }

    #[method(name = "format", public, final)]
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
            "java/text/DateFormat",
            "format",
            "(Ljava/util/Date;Ljava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;",
            (date, buffer, position),
        )
        .await
    }

    #[method(name = "format", public, final)]
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
                "java/text/DateFormat",
                "format",
                "(Ljava/util/Date;Ljava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;",
                (date, buffer, position),
            )
            .await?;
        jvm.invoke_virtual(&buffer, "java/lang/Object", "toString", "()Ljava/lang/String;", ())
            .await
    }

    #[method(public)]
    async fn format(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        date: ClassInstanceRef<Date>,
        buffer: ClassInstanceRef<StringBuffer>,
        position: ClassInstanceRef<FieldPosition>,
    ) -> Result<ClassInstanceRef<StringBuffer>>;

    #[method(public)]
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
                "java/text/DateFormat",
                "parse",
                "(Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/util/Date;",
                (source, position.clone()),
            )
            .await?;
        let index: i32 = jvm.invoke_virtual(&position, "java/text/ParsePosition", "getIndex", "()I", ()).await?;
        if index == 0 {
            let error_index: i32 = jvm
                .invoke_virtual(&position, "java/text/ParsePosition", "getErrorIndex", "()I", ())
                .await?;
            let message = JavaLangString::from_rust_string(jvm, "Unparseable date").await?;
            let exception: ClassInstanceRef<ParseException> = jvm
                .new_class("java/text/ParseException", "(Ljava/lang/String;I)V", (message, error_index))
                .await?
                .into();
            return Err(JavaError::JavaException(exception.into()));
        }
        Ok(date)
    }

    #[method(name = "parse", public)]
    async fn parse_with_position(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        source: ClassInstanceRef<String>,
        position: ClassInstanceRef<ParsePosition>,
    ) -> Result<ClassInstanceRef<Date>>;

    #[method(public)]
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
                "java/text/DateFormat",
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

    #[method(public, static, final)]
    async fn get_instance(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        let locale: ClassInstanceRef<Locale> = jvm.invoke_static("java/util/Locale", "getDefault", "()Ljava/util/Locale;", ()).await?;
        Self::new_formatter(jvm, Some(3), Some(3), locale).await
    }

    #[method(public, static, final)]
    async fn get_time_instance(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        let locale: ClassInstanceRef<Locale> = jvm.invoke_static("java/util/Locale", "getDefault", "()Ljava/util/Locale;", ()).await?;
        Self::new_formatter(jvm, None, Some(2), locale).await
    }

    #[method(name = "getTimeInstance", public, static, final)]
    async fn get_time_instance_with_style(jvm: &Jvm, _: &mut RuntimeContext, style: i32) -> Result<ClassInstanceRef<Self>> {
        let locale: ClassInstanceRef<Locale> = jvm.invoke_static("java/util/Locale", "getDefault", "()Ljava/util/Locale;", ()).await?;
        Self::new_formatter(jvm, None, Some(style), locale).await
    }

    #[method(name = "getTimeInstance", public, static)]
    async fn get_time_instance_with_locale(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        style: i32,
        locale: ClassInstanceRef<Locale>,
    ) -> Result<ClassInstanceRef<Self>> {
        Self::new_formatter(jvm, None, Some(style), locale).await
    }

    #[method(public, static, final)]
    async fn get_date_instance(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        let locale: ClassInstanceRef<Locale> = jvm.invoke_static("java/util/Locale", "getDefault", "()Ljava/util/Locale;", ()).await?;
        Self::new_formatter(jvm, Some(2), None, locale).await
    }

    #[method(name = "getDateInstance", public, static, final)]
    async fn get_date_instance_with_style(jvm: &Jvm, _: &mut RuntimeContext, style: i32) -> Result<ClassInstanceRef<Self>> {
        let locale: ClassInstanceRef<Locale> = jvm.invoke_static("java/util/Locale", "getDefault", "()Ljava/util/Locale;", ()).await?;
        Self::new_formatter(jvm, Some(style), None, locale).await
    }

    #[method(name = "getDateInstance", public, static)]
    async fn get_date_instance_with_locale(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        style: i32,
        locale: ClassInstanceRef<Locale>,
    ) -> Result<ClassInstanceRef<Self>> {
        Self::new_formatter(jvm, Some(style), None, locale).await
    }

    #[method(public, static, final)]
    async fn get_date_time_instance(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        let locale: ClassInstanceRef<Locale> = jvm.invoke_static("java/util/Locale", "getDefault", "()Ljava/util/Locale;", ()).await?;
        Self::new_formatter(jvm, Some(2), Some(2), locale).await
    }

    #[method(name = "getDateTimeInstance", public, static, final)]
    async fn get_date_time_instance_with_styles(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        date_style: i32,
        time_style: i32,
    ) -> Result<ClassInstanceRef<Self>> {
        let locale: ClassInstanceRef<Locale> = jvm.invoke_static("java/util/Locale", "getDefault", "()Ljava/util/Locale;", ()).await?;
        Self::new_formatter(jvm, Some(date_style), Some(time_style), locale).await
    }

    #[method(name = "getDateTimeInstance", public, static)]
    async fn get_date_time_instance_with_locale(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        date_style: i32,
        time_style: i32,
        locale: ClassInstanceRef<Locale>,
    ) -> Result<ClassInstanceRef<Self>> {
        Self::new_formatter(jvm, Some(date_style), Some(time_style), locale).await
    }

    #[method(public, static)]
    async fn get_available_locales(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Array<Locale>>> {
        jvm.invoke_static("java/util/Locale", "getAvailableLocales", "()[Ljava/util/Locale;", ())
            .await
    }

    #[method(public)]
    async fn get_calendar(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Calendar>> {
        jvm.get_field(&this, "java/text/DateFormat", "calendar", "Ljava/util/Calendar;").await
    }

    #[method(public)]
    async fn set_calendar(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, calendar: ClassInstanceRef<Calendar>) -> Result<()> {
        jvm.put_field(&mut this, "java/text/DateFormat", "calendar", "Ljava/util/Calendar;", calendar)
            .await
    }

    #[method(public)]
    async fn get_number_format(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<NumberFormat>> {
        jvm.get_field(&this, "java/text/DateFormat", "numberFormat", "Ljava/text/NumberFormat;")
            .await
    }

    #[method(public)]
    async fn set_number_format(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        number_format: ClassInstanceRef<NumberFormat>,
    ) -> Result<()> {
        jvm.put_field(
            &mut this,
            "java/text/DateFormat",
            "numberFormat",
            "Ljava/text/NumberFormat;",
            number_format,
        )
        .await
    }

    #[method(public)]
    async fn get_time_zone(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<TimeZone>> {
        let calendar: ClassInstanceRef<Calendar> = jvm.get_field(&this, "java/text/DateFormat", "calendar", "Ljava/util/Calendar;").await?;
        if calendar.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "calendar").await);
        }
        jvm.invoke_virtual(&calendar, "java/util/Calendar", "getTimeZone", "()Ljava/util/TimeZone;", ())
            .await
    }

    #[method(public)]
    async fn set_time_zone(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, time_zone: ClassInstanceRef<TimeZone>) -> Result<()> {
        let calendar: ClassInstanceRef<Calendar> = jvm.get_field(&this, "java/text/DateFormat", "calendar", "Ljava/util/Calendar;").await?;
        if calendar.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "calendar").await);
        }
        jvm.invoke_virtual(&calendar, "java/util/Calendar", "setTimeZone", "(Ljava/util/TimeZone;)V", (time_zone,))
            .await
    }

    #[method(public)]
    async fn is_lenient(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let calendar: ClassInstanceRef<Calendar> = jvm.get_field(&this, "java/text/DateFormat", "calendar", "Ljava/util/Calendar;").await?;
        if calendar.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "calendar").await);
        }
        jvm.invoke_virtual(&calendar, "java/util/Calendar", "isLenient", "()Z", ()).await
    }

    #[method(public)]
    async fn set_lenient(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, lenient: bool) -> Result<()> {
        let calendar: ClassInstanceRef<Calendar> = jvm.get_field(&this, "java/text/DateFormat", "calendar", "Ljava/util/Calendar;").await?;
        if calendar.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "calendar").await);
        }
        jvm.invoke_virtual(&calendar, "java/util/Calendar", "setLenient", "(Z)V", (lenient,))
            .await
    }

    #[method(public)]
    async fn clone(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let calendar: ClassInstanceRef<Calendar> = jvm.get_field(&this, "java/text/DateFormat", "calendar", "Ljava/util/Calendar;").await?;
        let number_format: ClassInstanceRef<NumberFormat> = jvm
            .get_field(&this, "java/text/DateFormat", "numberFormat", "Ljava/text/NumberFormat;")
            .await?;
        if calendar.is_null() || number_format.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "calendar or numberFormat").await);
        }
        let mut cloned: ClassInstanceRef<Self> = jvm.shallow_clone(&this)?.into();
        let mut cloned_calendar: ClassInstanceRef<Calendar> = jvm.shallow_clone(&calendar)?.into();
        let fields: ClassInstanceRef<Array<i32>> = jvm.get_field(&calendar, "java/util/Calendar", "fields", "[I").await?;
        let cloned_fields: ClassInstanceRef<Array<i32>> = jvm.shallow_clone(&fields)?.into();
        let time_zone: ClassInstanceRef<TimeZone> = jvm.get_field(&calendar, "java/util/Calendar", "timeZone", "Ljava/util/TimeZone;").await?;
        if time_zone.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "timeZone").await);
        }
        let cloned_time_zone: ClassInstanceRef<TimeZone> = jvm.shallow_clone(&time_zone)?.into();
        jvm.put_field(&mut cloned_calendar, "java/util/Calendar", "fields", "[I", cloned_fields)
            .await?;
        jvm.put_field(
            &mut cloned_calendar,
            "java/util/Calendar",
            "timeZone",
            "Ljava/util/TimeZone;",
            cloned_time_zone,
        )
        .await?;
        let cloned_number_format: ClassInstanceRef<NumberFormat> = jvm.shallow_clone(&number_format)?.into();
        jvm.put_field(&mut cloned, "java/text/DateFormat", "calendar", "Ljava/util/Calendar;", cloned_calendar)
            .await?;
        jvm.put_field(
            &mut cloned,
            "java/text/DateFormat",
            "numberFormat",
            "Ljava/text/NumberFormat;",
            cloned_number_format,
        )
        .await?;
        Ok(ClassInstanceRef::new(cloned.instance))
    }
}
