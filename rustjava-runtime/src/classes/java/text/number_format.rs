use jvm::{Array, ClassInstanceRef, JavaError, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{
        lang::{Number, Object, String, StringBuffer},
        text::{FieldPosition, ParseException, ParsePosition},
        util::Locale,
    },
};

// public abstract class java.text.NumberFormat
#[java_class(name = "java/text/NumberFormat", extends = "java/text/Format", public, abstract)]
pub struct NumberFormat {
    #[field(public, static, final)]
    INTEGER_FIELD: i32,
    #[field(public, static, final)]
    FRACTION_FIELD: i32,
    #[field(private)]
    groupingUsed: bool,
    #[field(private)]
    parseIntegerOnly: bool,
    #[field(private)]
    maximumIntegerDigits: i32,
    #[field(private)]
    minimumIntegerDigits: i32,
    #[field(private)]
    maximumFractionDigits: i32,
    #[field(private)]
    minimumFractionDigits: i32,
}

#[java_methods]
impl NumberFormat {
    #[method(static_initializer)]
    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        jvm.put_static_field("java/text/NumberFormat", "INTEGER_FIELD", "I", 0).await?;
        jvm.put_static_field("java/text/NumberFormat", "FRACTION_FIELD", "I", 1).await
    }

    #[method(constructor, protected)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/text/Format", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/text/NumberFormat", "groupingUsed", "Z", true).await?;
        jvm.put_field(&mut this, "java/text/NumberFormat", "parseIntegerOnly", "Z", false).await?;
        jvm.put_field(&mut this, "java/text/NumberFormat", "maximumIntegerDigits", "I", 40)
            .await?;
        jvm.put_field(&mut this, "java/text/NumberFormat", "minimumIntegerDigits", "I", 1).await?;
        jvm.put_field(&mut this, "java/text/NumberFormat", "maximumFractionDigits", "I", 3)
            .await?;
        jvm.put_field(&mut this, "java/text/NumberFormat", "minimumFractionDigits", "I", 0).await
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
        if object.is_null() || !jvm.is_instance(&**object, "java/lang/Number") {
            return Err(jvm
                .exception("java/lang/IllegalArgumentException", "Cannot format given Object as a Number")
                .await);
        }
        if buffer.is_null() || position.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "buffer or position").await);
        }

        if jvm.is_instance(&**object, "java/lang/Byte")
            || jvm.is_instance(&**object, "java/lang/Short")
            || jvm.is_instance(&**object, "java/lang/Integer")
            || jvm.is_instance(&**object, "java/lang/Long")
        {
            let value: i64 = jvm.invoke_virtual(&object, "java/lang/Number", "longValue", "()J", ()).await?;
            return jvm
                .invoke_virtual(
                    &this,
                    "java/text/NumberFormat",
                    "format",
                    "(JLjava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;",
                    (value, buffer, position),
                )
                .await;
        }

        let value: f64 = jvm.invoke_virtual(&object, "java/lang/Number", "doubleValue", "()D", ()).await?;
        jvm.invoke_virtual(
            &this,
            "java/text/NumberFormat",
            "format",
            "(DLjava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;",
            (value, buffer, position),
        )
        .await
    }

    #[method(name = "format", public, final)]
    async fn format_double(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: f64) -> Result<ClassInstanceRef<String>> {
        let buffer: ClassInstanceRef<StringBuffer> = jvm.new_class("java/lang/StringBuffer", "()V", ()).await?.into();
        let position: ClassInstanceRef<FieldPosition> = jvm.new_class("java/text/FieldPosition", "(I)V", (0,)).await?.into();
        let buffer: ClassInstanceRef<StringBuffer> = jvm
            .invoke_virtual(
                &this,
                "java/text/NumberFormat",
                "format",
                "(DLjava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;",
                (value, buffer, position),
            )
            .await?;
        jvm.invoke_virtual(&buffer, "java/lang/Object", "toString", "()Ljava/lang/String;", ())
            .await
    }

    #[method(name = "format", public, final)]
    async fn format_long(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i64) -> Result<ClassInstanceRef<String>> {
        let buffer: ClassInstanceRef<StringBuffer> = jvm.new_class("java/lang/StringBuffer", "()V", ()).await?.into();
        let position: ClassInstanceRef<FieldPosition> = jvm.new_class("java/text/FieldPosition", "(I)V", (0,)).await?.into();
        let buffer: ClassInstanceRef<StringBuffer> = jvm
            .invoke_virtual(
                &this,
                "java/text/NumberFormat",
                "format",
                "(JLjava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;",
                (value, buffer, position),
            )
            .await?;
        jvm.invoke_virtual(&buffer, "java/lang/Object", "toString", "()Ljava/lang/String;", ())
            .await
    }

    #[method(name = "format", public)]
    async fn format_double_to_buffer(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        value: f64,
        buffer: ClassInstanceRef<StringBuffer>,
        position: ClassInstanceRef<FieldPosition>,
    ) -> Result<ClassInstanceRef<StringBuffer>>;

    #[method(name = "format", public)]
    async fn format_long_to_buffer(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        value: i64,
        buffer: ClassInstanceRef<StringBuffer>,
        position: ClassInstanceRef<FieldPosition>,
    ) -> Result<ClassInstanceRef<StringBuffer>>;

    #[method(public)]
    async fn parse(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        source: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Number>> {
        if source.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "source").await);
        }
        let position: ClassInstanceRef<ParsePosition> = jvm.new_class("java/text/ParsePosition", "(I)V", (0,)).await?.into();
        let result: ClassInstanceRef<Number> = jvm
            .invoke_virtual(
                &this,
                "java/text/NumberFormat",
                "parse",
                "(Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/lang/Number;",
                (source, position.clone()),
            )
            .await?;
        let index: i32 = jvm.invoke_virtual(&position, "java/text/ParsePosition", "getIndex", "()I", ()).await?;
        if index == 0 {
            let error_index: i32 = jvm
                .invoke_virtual(&position, "java/text/ParsePosition", "getErrorIndex", "()I", ())
                .await?;
            let message = JavaLangString::from_rust_string(jvm, "Unparseable number").await?;
            let exception: ClassInstanceRef<ParseException> = jvm
                .new_class("java/text/ParseException", "(Ljava/lang/String;I)V", (message, error_index))
                .await?
                .into();
            return Err(JavaError::JavaException(exception.into()));
        }
        Ok(result)
    }

    #[method(name = "parse", public)]
    async fn parse_with_position(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        source: ClassInstanceRef<String>,
        position: ClassInstanceRef<ParsePosition>,
    ) -> Result<ClassInstanceRef<Number>>;

    #[method(public, final)]
    async fn parse_object(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        source: ClassInstanceRef<String>,
        position: ClassInstanceRef<ParsePosition>,
    ) -> Result<ClassInstanceRef<Object>> {
        let number: ClassInstanceRef<Number> = jvm
            .invoke_virtual(
                &this,
                "java/text/NumberFormat",
                "parse",
                "(Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/lang/Number;",
                (source, position),
            )
            .await?;
        Ok(ClassInstanceRef::new(number.instance))
    }

    #[method(public, static, final)]
    async fn get_instance(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        jvm.invoke_static("java/text/NumberFormat", "getNumberInstance", "()Ljava/text/NumberFormat;", ())
            .await
    }

    #[method(name = "getInstance", public, static)]
    async fn get_instance_with_locale(jvm: &Jvm, _: &mut RuntimeContext, locale: ClassInstanceRef<Locale>) -> Result<ClassInstanceRef<Self>> {
        jvm.invoke_static(
            "java/text/NumberFormat",
            "getNumberInstance",
            "(Ljava/util/Locale;)Ljava/text/NumberFormat;",
            (locale,),
        )
        .await
    }

    #[method(public, static, final)]
    async fn get_number_instance(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        let locale: ClassInstanceRef<Locale> = jvm.invoke_static("java/util/Locale", "getDefault", "()Ljava/util/Locale;", ()).await?;
        jvm.invoke_static(
            "java/text/NumberFormat",
            "getNumberInstance",
            "(Ljava/util/Locale;)Ljava/text/NumberFormat;",
            (locale,),
        )
        .await
    }

    #[method(name = "getNumberInstance", public, static)]
    async fn get_number_instance_with_locale(jvm: &Jvm, _: &mut RuntimeContext, locale: ClassInstanceRef<Locale>) -> Result<ClassInstanceRef<Self>> {
        if locale.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "locale").await);
        }
        let pattern = JavaLangString::from_rust_string(jvm, "#,##0.###").await?;
        Ok(jvm
            .new_class("java/text/DecimalFormat", "(Ljava/lang/String;)V", (pattern,))
            .await?
            .into())
    }

    #[method(public, static, final)]
    async fn get_integer_instance(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        let locale: ClassInstanceRef<Locale> = jvm.invoke_static("java/util/Locale", "getDefault", "()Ljava/util/Locale;", ()).await?;
        jvm.invoke_static(
            "java/text/NumberFormat",
            "getIntegerInstance",
            "(Ljava/util/Locale;)Ljava/text/NumberFormat;",
            (locale,),
        )
        .await
    }

    #[method(name = "getIntegerInstance", public, static)]
    async fn get_integer_instance_with_locale(jvm: &Jvm, _: &mut RuntimeContext, locale: ClassInstanceRef<Locale>) -> Result<ClassInstanceRef<Self>> {
        if locale.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "locale").await);
        }
        let pattern = JavaLangString::from_rust_string(jvm, "#,##0").await?;
        let format: ClassInstanceRef<Self> = jvm
            .new_class("java/text/DecimalFormat", "(Ljava/lang/String;)V", (pattern,))
            .await?
            .into();
        let _: () = jvm
            .invoke_virtual(&format, "java/text/NumberFormat", "setParseIntegerOnly", "(Z)V", (true,))
            .await?;
        Ok(format)
    }

    #[method(public, static, final)]
    async fn get_currency_instance(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        let locale: ClassInstanceRef<Locale> = jvm.invoke_static("java/util/Locale", "getDefault", "()Ljava/util/Locale;", ()).await?;
        jvm.invoke_static(
            "java/text/NumberFormat",
            "getCurrencyInstance",
            "(Ljava/util/Locale;)Ljava/text/NumberFormat;",
            (locale,),
        )
        .await
    }

    #[method(name = "getCurrencyInstance", public, static)]
    async fn get_currency_instance_with_locale(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        locale: ClassInstanceRef<Locale>,
    ) -> Result<ClassInstanceRef<Self>> {
        if locale.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "locale").await);
        }
        let pattern = JavaLangString::from_rust_string(jvm, "\u{00a4}#,##0.00;-\u{00a4}#,##0.00").await?;
        Ok(jvm
            .new_class("java/text/DecimalFormat", "(Ljava/lang/String;)V", (pattern,))
            .await?
            .into())
    }

    #[method(public, static, final)]
    async fn get_percent_instance(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        let locale: ClassInstanceRef<Locale> = jvm.invoke_static("java/util/Locale", "getDefault", "()Ljava/util/Locale;", ()).await?;
        jvm.invoke_static(
            "java/text/NumberFormat",
            "getPercentInstance",
            "(Ljava/util/Locale;)Ljava/text/NumberFormat;",
            (locale,),
        )
        .await
    }

    #[method(name = "getPercentInstance", public, static)]
    async fn get_percent_instance_with_locale(jvm: &Jvm, _: &mut RuntimeContext, locale: ClassInstanceRef<Locale>) -> Result<ClassInstanceRef<Self>> {
        if locale.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "locale").await);
        }
        let pattern = JavaLangString::from_rust_string(jvm, "#,##0%").await?;
        Ok(jvm
            .new_class("java/text/DecimalFormat", "(Ljava/lang/String;)V", (pattern,))
            .await?
            .into())
    }

    #[method(public, static)]
    async fn get_available_locales(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Array<Locale>>> {
        jvm.invoke_static("java/util/Locale", "getAvailableLocales", "()[Ljava/util/Locale;", ())
            .await
    }

    #[method(public)]
    async fn is_grouping_used(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "java/text/NumberFormat", "groupingUsed", "Z").await
    }

    #[method(public)]
    async fn set_grouping_used(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: bool) -> Result<()> {
        jvm.put_field(&mut this, "java/text/NumberFormat", "groupingUsed", "Z", value).await
    }

    #[method(public)]
    async fn is_parse_integer_only(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "java/text/NumberFormat", "parseIntegerOnly", "Z").await
    }

    #[method(public)]
    async fn set_parse_integer_only(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: bool) -> Result<()> {
        jvm.put_field(&mut this, "java/text/NumberFormat", "parseIntegerOnly", "Z", value).await
    }

    #[method(public)]
    async fn get_maximum_integer_digits(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "java/text/NumberFormat", "maximumIntegerDigits", "I").await
    }

    #[method(public)]
    async fn set_maximum_integer_digits(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        let value = value.max(0);
        jvm.put_field(&mut this, "java/text/NumberFormat", "maximumIntegerDigits", "I", value)
            .await?;
        let minimum: i32 = jvm.get_field(&this, "java/text/NumberFormat", "minimumIntegerDigits", "I").await?;
        if minimum > value {
            jvm.put_field(&mut this, "java/text/NumberFormat", "minimumIntegerDigits", "I", value)
                .await?;
        }
        Ok(())
    }

    #[method(public)]
    async fn get_minimum_integer_digits(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "java/text/NumberFormat", "minimumIntegerDigits", "I").await
    }

    #[method(public)]
    async fn set_minimum_integer_digits(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        let value = value.max(0);
        jvm.put_field(&mut this, "java/text/NumberFormat", "minimumIntegerDigits", "I", value)
            .await?;
        let maximum: i32 = jvm.get_field(&this, "java/text/NumberFormat", "maximumIntegerDigits", "I").await?;
        if maximum < value {
            jvm.put_field(&mut this, "java/text/NumberFormat", "maximumIntegerDigits", "I", value)
                .await?;
        }
        Ok(())
    }

    #[method(public)]
    async fn get_maximum_fraction_digits(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "java/text/NumberFormat", "maximumFractionDigits", "I").await
    }

    #[method(public)]
    async fn set_maximum_fraction_digits(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        let value = value.max(0);
        jvm.put_field(&mut this, "java/text/NumberFormat", "maximumFractionDigits", "I", value)
            .await?;
        let minimum: i32 = jvm.get_field(&this, "java/text/NumberFormat", "minimumFractionDigits", "I").await?;
        if minimum > value {
            jvm.put_field(&mut this, "java/text/NumberFormat", "minimumFractionDigits", "I", value)
                .await?;
        }
        Ok(())
    }

    #[method(public)]
    async fn get_minimum_fraction_digits(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "java/text/NumberFormat", "minimumFractionDigits", "I").await
    }

    #[method(public)]
    async fn set_minimum_fraction_digits(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        let value = value.max(0);
        jvm.put_field(&mut this, "java/text/NumberFormat", "minimumFractionDigits", "I", value)
            .await?;
        let maximum: i32 = jvm.get_field(&this, "java/text/NumberFormat", "maximumFractionDigits", "I").await?;
        if maximum < value {
            jvm.put_field(&mut this, "java/text/NumberFormat", "maximumFractionDigits", "I", value)
                .await?;
        }
        Ok(())
    }

    #[method(public)]
    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if other.is_null() || !jvm.is_instance(&**other, "java/text/NumberFormat") {
            return Ok(false);
        }
        let other: ClassInstanceRef<Self> = ClassInstanceRef::new(other.instance);
        for (name, descriptor) in [
            ("groupingUsed", "Z"),
            ("parseIntegerOnly", "Z"),
            ("maximumIntegerDigits", "I"),
            ("minimumIntegerDigits", "I"),
            ("maximumFractionDigits", "I"),
            ("minimumFractionDigits", "I"),
        ] {
            if descriptor == "Z" {
                let value: bool = jvm.get_field(&this, "java/text/NumberFormat", name, descriptor).await?;
                let other_value: bool = jvm.get_field(&other, "java/text/NumberFormat", name, descriptor).await?;
                if value != other_value {
                    return Ok(false);
                }
            } else {
                let value: i32 = jvm.get_field(&this, "java/text/NumberFormat", name, descriptor).await?;
                let other_value: i32 = jvm.get_field(&other, "java/text/NumberFormat", name, descriptor).await?;
                if value != other_value {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    #[method(public)]
    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let maximum_integer_digits: i32 = jvm.get_field(&this, "java/text/NumberFormat", "maximumIntegerDigits", "I").await?;
        let maximum_fraction_digits: i32 = jvm.get_field(&this, "java/text/NumberFormat", "maximumFractionDigits", "I").await?;
        Ok(maximum_integer_digits * 37 + maximum_fraction_digits)
    }
}
