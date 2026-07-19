use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, JavaError, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        lang::{Number, Object, String, StringBuffer},
        text::{FieldPosition, ParseException, ParsePosition},
        util::Locale,
    },
};

// public abstract class java.text.NumberFormat
pub struct NumberFormat;

impl NumberFormat {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/text/NumberFormat",
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
                    "(D)Ljava/lang/String;",
                    Self::format_double,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::FINAL,
                ),
                JavaMethodProto::new(
                    "format",
                    "(J)Ljava/lang/String;",
                    Self::format_long,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::FINAL,
                ),
                JavaMethodProto::new_abstract(
                    "format",
                    "(DLjava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;",
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT,
                ),
                JavaMethodProto::new_abstract(
                    "format",
                    "(JLjava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;",
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT,
                ),
                JavaMethodProto::new("parse", "(Ljava/lang/String;)Ljava/lang/Number;", Self::parse, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new_abstract(
                    "parse",
                    "(Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/lang/Number;",
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT,
                ),
                JavaMethodProto::new(
                    "parseObject",
                    "(Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/lang/Object;",
                    Self::parse_object,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::FINAL,
                ),
                JavaMethodProto::new(
                    "getInstance",
                    "()Ljava/text/NumberFormat;",
                    Self::get_instance,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC | MethodAccessFlags::FINAL,
                ),
                JavaMethodProto::new(
                    "getInstance",
                    "(Ljava/util/Locale;)Ljava/text/NumberFormat;",
                    Self::get_instance_with_locale,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "getNumberInstance",
                    "()Ljava/text/NumberFormat;",
                    Self::get_number_instance,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC | MethodAccessFlags::FINAL,
                ),
                JavaMethodProto::new(
                    "getNumberInstance",
                    "(Ljava/util/Locale;)Ljava/text/NumberFormat;",
                    Self::get_number_instance_with_locale,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "getIntegerInstance",
                    "()Ljava/text/NumberFormat;",
                    Self::get_integer_instance,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC | MethodAccessFlags::FINAL,
                ),
                JavaMethodProto::new(
                    "getIntegerInstance",
                    "(Ljava/util/Locale;)Ljava/text/NumberFormat;",
                    Self::get_integer_instance_with_locale,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "getCurrencyInstance",
                    "()Ljava/text/NumberFormat;",
                    Self::get_currency_instance,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC | MethodAccessFlags::FINAL,
                ),
                JavaMethodProto::new(
                    "getCurrencyInstance",
                    "(Ljava/util/Locale;)Ljava/text/NumberFormat;",
                    Self::get_currency_instance_with_locale,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "getPercentInstance",
                    "()Ljava/text/NumberFormat;",
                    Self::get_percent_instance,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC | MethodAccessFlags::FINAL,
                ),
                JavaMethodProto::new(
                    "getPercentInstance",
                    "(Ljava/util/Locale;)Ljava/text/NumberFormat;",
                    Self::get_percent_instance_with_locale,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "getAvailableLocales",
                    "()[Ljava/util/Locale;",
                    Self::get_available_locales,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("isGroupingUsed", "()Z", Self::is_grouping_used, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("setGroupingUsed", "(Z)V", Self::set_grouping_used, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("isParseIntegerOnly", "()Z", Self::is_parse_integer_only, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("setParseIntegerOnly", "(Z)V", Self::set_parse_integer_only, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "getMaximumIntegerDigits",
                    "()I",
                    Self::get_maximum_integer_digits,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "setMaximumIntegerDigits",
                    "(I)V",
                    Self::set_maximum_integer_digits,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "getMinimumIntegerDigits",
                    "()I",
                    Self::get_minimum_integer_digits,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "setMinimumIntegerDigits",
                    "(I)V",
                    Self::set_minimum_integer_digits,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "getMaximumFractionDigits",
                    "()I",
                    Self::get_maximum_fraction_digits,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "setMaximumFractionDigits",
                    "(I)V",
                    Self::set_maximum_fraction_digits,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "getMinimumFractionDigits",
                    "()I",
                    Self::get_minimum_fraction_digits,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "setMinimumFractionDigits",
                    "(I)V",
                    Self::set_minimum_fraction_digits,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new(
                    "INTEGER_FIELD",
                    "I",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "FRACTION_FIELD",
                    "I",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new("groupingUsed", "Z", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("parseIntegerOnly", "Z", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("maximumIntegerDigits", "I", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("minimumIntegerDigits", "I", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("maximumFractionDigits", "I", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("minimumFractionDigits", "I", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::ABSTRACT,
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        jvm.put_static_field("java/text/NumberFormat", "INTEGER_FIELD", "I", 0).await?;
        jvm.put_static_field("java/text/NumberFormat", "FRACTION_FIELD", "I", 1).await
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/text/Format", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "groupingUsed", "Z", true).await?;
        jvm.put_field(&mut this, "parseIntegerOnly", "Z", false).await?;
        jvm.put_field(&mut this, "maximumIntegerDigits", "I", 40).await?;
        jvm.put_field(&mut this, "minimumIntegerDigits", "I", 1).await?;
        jvm.put_field(&mut this, "maximumFractionDigits", "I", 3).await?;
        jvm.put_field(&mut this, "minimumFractionDigits", "I", 0).await
    }

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
            let value: i64 = jvm.invoke_virtual(&object, "longValue", "()J", ()).await?;
            return jvm
                .invoke_virtual(
                    &this,
                    "format",
                    "(JLjava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;",
                    (value, buffer, position),
                )
                .await;
        }

        let value: f64 = jvm.invoke_virtual(&object, "doubleValue", "()D", ()).await?;
        jvm.invoke_virtual(
            &this,
            "format",
            "(DLjava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;",
            (value, buffer, position),
        )
        .await
    }

    async fn format_double(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: f64) -> Result<ClassInstanceRef<String>> {
        let buffer: ClassInstanceRef<StringBuffer> = jvm.new_class("java/lang/StringBuffer", "()V", ()).await?.into();
        let position: ClassInstanceRef<FieldPosition> = jvm.new_class("java/text/FieldPosition", "(I)V", (0,)).await?.into();
        let buffer: ClassInstanceRef<StringBuffer> = jvm
            .invoke_virtual(
                &this,
                "format",
                "(DLjava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;",
                (value, buffer, position),
            )
            .await?;
        jvm.invoke_virtual(&buffer, "toString", "()Ljava/lang/String;", ()).await
    }

    async fn format_long(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i64) -> Result<ClassInstanceRef<String>> {
        let buffer: ClassInstanceRef<StringBuffer> = jvm.new_class("java/lang/StringBuffer", "()V", ()).await?.into();
        let position: ClassInstanceRef<FieldPosition> = jvm.new_class("java/text/FieldPosition", "(I)V", (0,)).await?.into();
        let buffer: ClassInstanceRef<StringBuffer> = jvm
            .invoke_virtual(
                &this,
                "format",
                "(JLjava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;",
                (value, buffer, position),
            )
            .await?;
        jvm.invoke_virtual(&buffer, "toString", "()Ljava/lang/String;", ()).await
    }

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
                "parse",
                "(Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/lang/Number;",
                (source, position.clone()),
            )
            .await?;
        let index: i32 = jvm.invoke_virtual(&position, "getIndex", "()I", ()).await?;
        if index == 0 {
            let error_index: i32 = jvm.invoke_virtual(&position, "getErrorIndex", "()I", ()).await?;
            let message = JavaLangString::from_rust_string(jvm, "Unparseable number").await?;
            let exception: ClassInstanceRef<ParseException> = jvm
                .new_class("java/text/ParseException", "(Ljava/lang/String;I)V", (message, error_index))
                .await?
                .into();
            return Err(JavaError::JavaException(exception.into()));
        }
        Ok(result)
    }

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
                "parse",
                "(Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/lang/Number;",
                (source, position),
            )
            .await?;
        Ok(ClassInstanceRef::new(number.instance))
    }

    async fn get_instance(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        jvm.invoke_static("java/text/NumberFormat", "getNumberInstance", "()Ljava/text/NumberFormat;", ())
            .await
    }

    async fn get_instance_with_locale(jvm: &Jvm, _: &mut RuntimeContext, locale: ClassInstanceRef<Locale>) -> Result<ClassInstanceRef<Self>> {
        jvm.invoke_static(
            "java/text/NumberFormat",
            "getNumberInstance",
            "(Ljava/util/Locale;)Ljava/text/NumberFormat;",
            (locale,),
        )
        .await
    }

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

    async fn get_integer_instance_with_locale(jvm: &Jvm, _: &mut RuntimeContext, locale: ClassInstanceRef<Locale>) -> Result<ClassInstanceRef<Self>> {
        if locale.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "locale").await);
        }
        let pattern = JavaLangString::from_rust_string(jvm, "#,##0").await?;
        let format: ClassInstanceRef<Self> = jvm
            .new_class("java/text/DecimalFormat", "(Ljava/lang/String;)V", (pattern,))
            .await?
            .into();
        let _: () = jvm.invoke_virtual(&format, "setParseIntegerOnly", "(Z)V", (true,)).await?;
        Ok(format)
    }

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

    async fn get_available_locales(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Array<Locale>>> {
        jvm.invoke_static("java/util/Locale", "getAvailableLocales", "()[Ljava/util/Locale;", ())
            .await
    }

    async fn is_grouping_used(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "groupingUsed", "Z").await
    }

    async fn set_grouping_used(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: bool) -> Result<()> {
        jvm.put_field(&mut this, "groupingUsed", "Z", value).await
    }

    async fn is_parse_integer_only(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "parseIntegerOnly", "Z").await
    }

    async fn set_parse_integer_only(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: bool) -> Result<()> {
        jvm.put_field(&mut this, "parseIntegerOnly", "Z", value).await
    }

    async fn get_maximum_integer_digits(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "maximumIntegerDigits", "I").await
    }

    async fn set_maximum_integer_digits(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        let value = value.max(0);
        jvm.put_field(&mut this, "maximumIntegerDigits", "I", value).await?;
        let minimum: i32 = jvm.get_field(&this, "minimumIntegerDigits", "I").await?;
        if minimum > value {
            jvm.put_field(&mut this, "minimumIntegerDigits", "I", value).await?;
        }
        Ok(())
    }

    async fn get_minimum_integer_digits(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "minimumIntegerDigits", "I").await
    }

    async fn set_minimum_integer_digits(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        let value = value.max(0);
        jvm.put_field(&mut this, "minimumIntegerDigits", "I", value).await?;
        let maximum: i32 = jvm.get_field(&this, "maximumIntegerDigits", "I").await?;
        if maximum < value {
            jvm.put_field(&mut this, "maximumIntegerDigits", "I", value).await?;
        }
        Ok(())
    }

    async fn get_maximum_fraction_digits(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "maximumFractionDigits", "I").await
    }

    async fn set_maximum_fraction_digits(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        let value = value.max(0);
        jvm.put_field(&mut this, "maximumFractionDigits", "I", value).await?;
        let minimum: i32 = jvm.get_field(&this, "minimumFractionDigits", "I").await?;
        if minimum > value {
            jvm.put_field(&mut this, "minimumFractionDigits", "I", value).await?;
        }
        Ok(())
    }

    async fn get_minimum_fraction_digits(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "minimumFractionDigits", "I").await
    }

    async fn set_minimum_fraction_digits(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        let value = value.max(0);
        jvm.put_field(&mut this, "minimumFractionDigits", "I", value).await?;
        let maximum: i32 = jvm.get_field(&this, "maximumFractionDigits", "I").await?;
        if maximum < value {
            jvm.put_field(&mut this, "maximumFractionDigits", "I", value).await?;
        }
        Ok(())
    }

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
                let value: bool = jvm.get_field(&this, name, descriptor).await?;
                let other_value: bool = jvm.get_field(&other, name, descriptor).await?;
                if value != other_value {
                    return Ok(false);
                }
            } else {
                let value: i32 = jvm.get_field(&this, name, descriptor).await?;
                let other_value: i32 = jvm.get_field(&other, name, descriptor).await?;
                if value != other_value {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let maximum_integer_digits: i32 = jvm.get_field(&this, "maximumIntegerDigits", "I").await?;
        let maximum_fraction_digits: i32 = jvm.get_field(&this, "maximumFractionDigits", "I").await?;
        Ok(maximum_integer_digits * 37 + maximum_fraction_digits)
    }
}
