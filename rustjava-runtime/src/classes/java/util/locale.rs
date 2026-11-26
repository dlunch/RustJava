use core::iter;

use alloc::{format, string::ToString, vec};

use jvm::{Array, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::lang::{Object, String},
};

// class java.util.Locale
#[java_class(name = "java/util/Locale", implements("java/lang/Cloneable", "java/io/Serializable"), public, final)]
pub struct Locale {
    #[field(public, static, final)]
    ENGLISH: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    FRENCH: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    GERMAN: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    ITALIAN: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    JAPANESE: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    KOREAN: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    CHINESE: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    SIMPLIFIED_CHINESE: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    TRADITIONAL_CHINESE: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    FRANCE: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    GERMANY: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    ITALY: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    JAPAN: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    KOREA: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    CHINA: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    PRC: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    TAIWAN: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    UK: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    US: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    CANADA: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    CANADA_FRENCH: ClassInstanceRef<Self>,
    #[field(private, static)]
    defaultLocale: ClassInstanceRef<Self>,
    #[field(private, final)]
    language: ClassInstanceRef<String>,
    #[field(private, final)]
    country: ClassInstanceRef<String>,
    #[field(private, final)]
    variant: ClassInstanceRef<String>,
}

#[java_methods]
impl Locale {
    #[method(static_initializer)]
    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        tracing::debug!("java.util.Locale::<clinit>()");

        let english = jvm
            .new_class(
                "java/util/Locale",
                "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
                (
                    JavaLangString::from_rust_string(jvm, "en").await?,
                    JavaLangString::from_rust_string(jvm, "").await?,
                    JavaLangString::from_rust_string(jvm, "").await?,
                ),
            )
            .await?;
        jvm.put_static_field("java/util/Locale", "ENGLISH", "Ljava/util/Locale;", english).await?;

        let french = jvm
            .new_class(
                "java/util/Locale",
                "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
                (
                    JavaLangString::from_rust_string(jvm, "fr").await?,
                    JavaLangString::from_rust_string(jvm, "").await?,
                    JavaLangString::from_rust_string(jvm, "").await?,
                ),
            )
            .await?;
        jvm.put_static_field("java/util/Locale", "FRENCH", "Ljava/util/Locale;", french).await?;

        let german = jvm
            .new_class(
                "java/util/Locale",
                "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
                (
                    JavaLangString::from_rust_string(jvm, "de").await?,
                    JavaLangString::from_rust_string(jvm, "").await?,
                    JavaLangString::from_rust_string(jvm, "").await?,
                ),
            )
            .await?;
        jvm.put_static_field("java/util/Locale", "GERMAN", "Ljava/util/Locale;", german).await?;

        let italian = jvm
            .new_class(
                "java/util/Locale",
                "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
                (
                    JavaLangString::from_rust_string(jvm, "it").await?,
                    JavaLangString::from_rust_string(jvm, "").await?,
                    JavaLangString::from_rust_string(jvm, "").await?,
                ),
            )
            .await?;
        jvm.put_static_field("java/util/Locale", "ITALIAN", "Ljava/util/Locale;", italian).await?;

        let japanese = jvm
            .new_class(
                "java/util/Locale",
                "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
                (
                    JavaLangString::from_rust_string(jvm, "ja").await?,
                    JavaLangString::from_rust_string(jvm, "").await?,
                    JavaLangString::from_rust_string(jvm, "").await?,
                ),
            )
            .await?;
        jvm.put_static_field("java/util/Locale", "JAPANESE", "Ljava/util/Locale;", japanese)
            .await?;

        let korean = jvm
            .new_class(
                "java/util/Locale",
                "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
                (
                    JavaLangString::from_rust_string(jvm, "ko").await?,
                    JavaLangString::from_rust_string(jvm, "").await?,
                    JavaLangString::from_rust_string(jvm, "").await?,
                ),
            )
            .await?;
        jvm.put_static_field("java/util/Locale", "KOREAN", "Ljava/util/Locale;", korean).await?;

        let chinese = jvm
            .new_class(
                "java/util/Locale",
                "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
                (
                    JavaLangString::from_rust_string(jvm, "zh").await?,
                    JavaLangString::from_rust_string(jvm, "").await?,
                    JavaLangString::from_rust_string(jvm, "").await?,
                ),
            )
            .await?;
        jvm.put_static_field("java/util/Locale", "CHINESE", "Ljava/util/Locale;", chinese).await?;

        let simplified_chinese = jvm
            .new_class(
                "java/util/Locale",
                "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
                (
                    JavaLangString::from_rust_string(jvm, "zh").await?,
                    JavaLangString::from_rust_string(jvm, "CN").await?,
                    JavaLangString::from_rust_string(jvm, "").await?,
                ),
            )
            .await?;
        jvm.put_static_field("java/util/Locale", "SIMPLIFIED_CHINESE", "Ljava/util/Locale;", simplified_chinese.clone())
            .await?;

        let traditional_chinese = jvm
            .new_class(
                "java/util/Locale",
                "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
                (
                    JavaLangString::from_rust_string(jvm, "zh").await?,
                    JavaLangString::from_rust_string(jvm, "TW").await?,
                    JavaLangString::from_rust_string(jvm, "").await?,
                ),
            )
            .await?;
        jvm.put_static_field(
            "java/util/Locale",
            "TRADITIONAL_CHINESE",
            "Ljava/util/Locale;",
            traditional_chinese.clone(),
        )
        .await?;

        let france = jvm
            .new_class(
                "java/util/Locale",
                "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
                (
                    JavaLangString::from_rust_string(jvm, "fr").await?,
                    JavaLangString::from_rust_string(jvm, "FR").await?,
                    JavaLangString::from_rust_string(jvm, "").await?,
                ),
            )
            .await?;
        jvm.put_static_field("java/util/Locale", "FRANCE", "Ljava/util/Locale;", france).await?;

        let germany = jvm
            .new_class(
                "java/util/Locale",
                "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
                (
                    JavaLangString::from_rust_string(jvm, "de").await?,
                    JavaLangString::from_rust_string(jvm, "DE").await?,
                    JavaLangString::from_rust_string(jvm, "").await?,
                ),
            )
            .await?;
        jvm.put_static_field("java/util/Locale", "GERMANY", "Ljava/util/Locale;", germany).await?;

        let italy = jvm
            .new_class(
                "java/util/Locale",
                "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
                (
                    JavaLangString::from_rust_string(jvm, "it").await?,
                    JavaLangString::from_rust_string(jvm, "IT").await?,
                    JavaLangString::from_rust_string(jvm, "").await?,
                ),
            )
            .await?;
        jvm.put_static_field("java/util/Locale", "ITALY", "Ljava/util/Locale;", italy).await?;

        let japan = jvm
            .new_class(
                "java/util/Locale",
                "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
                (
                    JavaLangString::from_rust_string(jvm, "ja").await?,
                    JavaLangString::from_rust_string(jvm, "JP").await?,
                    JavaLangString::from_rust_string(jvm, "").await?,
                ),
            )
            .await?;
        jvm.put_static_field("java/util/Locale", "JAPAN", "Ljava/util/Locale;", japan).await?;

        let korea = jvm
            .new_class(
                "java/util/Locale",
                "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
                (
                    JavaLangString::from_rust_string(jvm, "ko").await?,
                    JavaLangString::from_rust_string(jvm, "KR").await?,
                    JavaLangString::from_rust_string(jvm, "").await?,
                ),
            )
            .await?;
        jvm.put_static_field("java/util/Locale", "KOREA", "Ljava/util/Locale;", korea).await?;

        let uk = jvm
            .new_class(
                "java/util/Locale",
                "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
                (
                    JavaLangString::from_rust_string(jvm, "en").await?,
                    JavaLangString::from_rust_string(jvm, "GB").await?,
                    JavaLangString::from_rust_string(jvm, "").await?,
                ),
            )
            .await?;
        jvm.put_static_field("java/util/Locale", "UK", "Ljava/util/Locale;", uk).await?;

        let us = jvm
            .new_class(
                "java/util/Locale",
                "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
                (
                    JavaLangString::from_rust_string(jvm, "en").await?,
                    JavaLangString::from_rust_string(jvm, "US").await?,
                    JavaLangString::from_rust_string(jvm, "").await?,
                ),
            )
            .await?;
        jvm.put_static_field("java/util/Locale", "US", "Ljava/util/Locale;", us.clone()).await?;

        let canada = jvm
            .new_class(
                "java/util/Locale",
                "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
                (
                    JavaLangString::from_rust_string(jvm, "en").await?,
                    JavaLangString::from_rust_string(jvm, "CA").await?,
                    JavaLangString::from_rust_string(jvm, "").await?,
                ),
            )
            .await?;
        jvm.put_static_field("java/util/Locale", "CANADA", "Ljava/util/Locale;", canada).await?;

        let canada_french = jvm
            .new_class(
                "java/util/Locale",
                "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
                (
                    JavaLangString::from_rust_string(jvm, "fr").await?,
                    JavaLangString::from_rust_string(jvm, "CA").await?,
                    JavaLangString::from_rust_string(jvm, "").await?,
                ),
            )
            .await?;
        jvm.put_static_field("java/util/Locale", "CANADA_FRENCH", "Ljava/util/Locale;", canada_french)
            .await?;

        jvm.put_static_field("java/util/Locale", "CHINA", "Ljava/util/Locale;", simplified_chinese.clone())
            .await?;
        jvm.put_static_field("java/util/Locale", "PRC", "Ljava/util/Locale;", simplified_chinese)
            .await?;
        jvm.put_static_field("java/util/Locale", "TAIWAN", "Ljava/util/Locale;", traditional_chinese)
            .await?;
        jvm.put_static_field("java/util/Locale", "defaultLocale", "Ljava/util/Locale;", us)
            .await?;

        Ok(())
    }

    #[method(constructor, public)]
    async fn init_with_language(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, language: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.util.Locale::<init>({this:?}, {language:?})");

        let country: ClassInstanceRef<String> = JavaLangString::from_rust_string(jvm, "").await?.into();
        let variant: ClassInstanceRef<String> = JavaLangString::from_rust_string(jvm, "").await?.into();
        jvm.invoke_special(
            &this,
            "java/util/Locale",
            "<init>",
            "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
            (language, country, variant),
        )
        .await
    }

    #[method(constructor, public)]
    async fn init_with_language_country(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        language: ClassInstanceRef<String>,
        country: ClassInstanceRef<String>,
    ) -> Result<()> {
        tracing::debug!("java.util.Locale::<init>({this:?}, {language:?}, {country:?})");

        let variant: ClassInstanceRef<String> = JavaLangString::from_rust_string(jvm, "").await?.into();
        jvm.invoke_special(
            &this,
            "java/util/Locale",
            "<init>",
            "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
            (language, country, variant),
        )
        .await
    }

    #[method(constructor, public)]
    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        language: ClassInstanceRef<String>,
        country: ClassInstanceRef<String>,
        variant: ClassInstanceRef<String>,
    ) -> Result<()> {
        tracing::debug!("java.util.Locale::<init>({this:?}, {language:?}, {country:?}, {variant:?})");

        if language.is_null() || country.is_null() || variant.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Locale component is null").await);
        }

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        let language = JavaLangString::to_rust_string(jvm, &language).await?.to_ascii_lowercase();
        let country = JavaLangString::to_rust_string(jvm, &country).await?.to_ascii_uppercase();
        let variant = JavaLangString::to_rust_string(jvm, &variant).await?;

        let language = JavaLangString::from_rust_string(jvm, &language).await?;
        let country = JavaLangString::from_rust_string(jvm, &country).await?;
        let variant = JavaLangString::from_rust_string(jvm, &variant).await?;

        jvm.put_field(&mut this, "java/util/Locale", "language", "Ljava/lang/String;", language)
            .await?;
        jvm.put_field(&mut this, "java/util/Locale", "country", "Ljava/lang/String;", country)
            .await?;
        jvm.put_field(&mut this, "java/util/Locale", "variant", "Ljava/lang/String;", variant)
            .await?;

        Ok(())
    }

    #[method(static)]
    async fn get_default(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.util.Locale::getDefault()");

        jvm.get_static_field("java/util/Locale", "defaultLocale", "Ljava/util/Locale;").await
    }

    #[method(static)]
    async fn set_default(jvm: &Jvm, _: &mut RuntimeContext, locale: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Locale::setDefault({locale:?})");

        if locale.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Locale default is null").await);
        }

        jvm.put_static_field("java/util/Locale", "defaultLocale", "Ljava/util/Locale;", locale)
            .await
    }

    #[method(static)]
    async fn get_available_locales(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Array<Self>>> {
        tracing::debug!("java.util.Locale::getAvailableLocales()");

        let fields = [
            "ENGLISH",
            "FRENCH",
            "GERMAN",
            "ITALIAN",
            "JAPANESE",
            "KOREAN",
            "CHINESE",
            "SIMPLIFIED_CHINESE",
            "TRADITIONAL_CHINESE",
            "FRANCE",
            "GERMANY",
            "ITALY",
            "JAPAN",
            "KOREA",
            "CHINA",
            "PRC",
            "TAIWAN",
            "UK",
            "US",
            "CANADA",
            "CANADA_FRENCH",
        ];
        let mut array: ClassInstanceRef<Array<Self>> = jvm.instantiate_array("Ljava/util/Locale;", fields.len()).await?.into();

        for (i, field) in fields.iter().enumerate() {
            let locale: ClassInstanceRef<Self> = jvm.get_static_field("java/util/Locale", field, "Ljava/util/Locale;").await?;
            jvm.store_array(&mut array, i, iter::once(locale)).await?;
        }

        Ok(array)
    }

    #[method(name = "getISOCountries", static)]
    async fn get_iso_countries(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Array<String>>> {
        tracing::debug!("java.util.Locale::getISOCountries()");

        Self::string_array(jvm, &["US", "GB", "CA", "FR", "DE", "IT", "JP", "KR", "CN", "TW"]).await
    }

    #[method(name = "getISOLanguages", static)]
    async fn get_iso_languages(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Array<String>>> {
        tracing::debug!("java.util.Locale::getISOLanguages()");

        Self::string_array(jvm, &["en", "fr", "de", "it", "ja", "ko", "zh"]).await
    }

    async fn string_array(jvm: &Jvm, values: &[&str]) -> Result<ClassInstanceRef<Array<String>>> {
        let mut array: ClassInstanceRef<Array<String>> = jvm.instantiate_array("Ljava/lang/String;", values.len()).await?.into();
        for (i, value) in values.iter().enumerate() {
            let value: ClassInstanceRef<String> = JavaLangString::from_rust_string(jvm, value).await?.into();
            jvm.store_array(&mut array, i, iter::once(value)).await?;
        }
        Ok(array)
    }

    #[method(public)]
    async fn get_language(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.Locale::getLanguage({this:?})");

        jvm.get_field(&this, "java/util/Locale", "language", "Ljava/lang/String;").await
    }

    #[method(public)]
    async fn get_country(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.Locale::getCountry({this:?})");

        jvm.get_field(&this, "java/util/Locale", "country", "Ljava/lang/String;").await
    }

    #[method(public)]
    async fn get_variant(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.Locale::getVariant({this:?})");

        jvm.get_field(&this, "java/util/Locale", "variant", "Ljava/lang/String;").await
    }

    #[method(name = "getISO3Language", public)]
    async fn get_iso3_language(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.Locale::getISO3Language({this:?})");

        let language: ClassInstanceRef<String> = jvm.get_field(&this, "java/util/Locale", "language", "Ljava/lang/String;").await?;
        let language = JavaLangString::to_rust_string(jvm, &language).await?;
        let iso3 = match language.as_str() {
            "" => "",
            "en" => "eng",
            "fr" => "fra",
            "de" => "deu",
            "it" => "ita",
            "ja" => "jpn",
            "ko" => "kor",
            "zh" => "zho",
            _ => return Err(jvm.exception("java/lang/IllegalArgumentException", "Unsupported ISO3 language").await),
        };

        Ok(JavaLangString::from_rust_string(jvm, iso3).await?.into())
    }

    #[method(name = "getISO3Country", public)]
    async fn get_iso3_country(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.Locale::getISO3Country({this:?})");

        let country: ClassInstanceRef<String> = jvm.get_field(&this, "java/util/Locale", "country", "Ljava/lang/String;").await?;
        let country = JavaLangString::to_rust_string(jvm, &country).await?;
        let iso3 = match country.as_str() {
            "" => "",
            "US" => "USA",
            "GB" => "GBR",
            "CA" => "CAN",
            "FR" => "FRA",
            "DE" => "DEU",
            "IT" => "ITA",
            "JP" => "JPN",
            "KR" => "KOR",
            "CN" => "CHN",
            "TW" => "TWN",
            _ => return Err(jvm.exception("java/lang/IllegalArgumentException", "Unsupported ISO3 country").await),
        };

        Ok(JavaLangString::from_rust_string(jvm, iso3).await?.into())
    }

    #[method(name = "getDisplayLanguage", public, final)]
    async fn get_display_language_default(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let default = Self::get_default(jvm, context).await?;
        Self::get_display_language(jvm, context, this, default).await
    }

    #[method(public)]
    async fn get_display_language(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        display_locale: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.Locale::getDisplayLanguage({this:?})");

        if display_locale.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "display locale is null").await);
        }

        let language: ClassInstanceRef<String> = jvm.get_field(&this, "java/util/Locale", "language", "Ljava/lang/String;").await?;
        let language = JavaLangString::to_rust_string(jvm, &language).await?;

        Ok(JavaLangString::from_rust_string(jvm, Self::display_language(&language)).await?.into())
    }

    #[method(name = "getDisplayCountry", public, final)]
    async fn get_display_country_default(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let default = Self::get_default(jvm, context).await?;
        Self::get_display_country(jvm, context, this, default).await
    }

    #[method(public)]
    async fn get_display_country(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        display_locale: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.Locale::getDisplayCountry({this:?})");

        if display_locale.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "display locale is null").await);
        }

        let country: ClassInstanceRef<String> = jvm.get_field(&this, "java/util/Locale", "country", "Ljava/lang/String;").await?;
        let country = JavaLangString::to_rust_string(jvm, &country).await?;

        Ok(JavaLangString::from_rust_string(jvm, Self::display_country(&country)).await?.into())
    }

    #[method(name = "getDisplayVariant", public, final)]
    async fn get_display_variant_default(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let default = Self::get_default(jvm, context).await?;
        Self::get_display_variant(jvm, context, this, default).await
    }

    #[method(public)]
    async fn get_display_variant(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        display_locale: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.Locale::getDisplayVariant({this:?})");

        if display_locale.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "display locale is null").await);
        }

        jvm.get_field(&this, "java/util/Locale", "variant", "Ljava/lang/String;").await
    }

    #[method(name = "getDisplayName", public, final)]
    async fn get_display_name_default(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let default = Self::get_default(jvm, context).await?;
        Self::get_display_name(jvm, context, this, default).await
    }

    #[method(public)]
    async fn get_display_name(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        display_locale: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.Locale::getDisplayName({this:?})");

        if display_locale.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "display locale is null").await);
        }

        let language: ClassInstanceRef<String> = jvm.get_field(&this, "java/util/Locale", "language", "Ljava/lang/String;").await?;
        let country: ClassInstanceRef<String> = jvm.get_field(&this, "java/util/Locale", "country", "Ljava/lang/String;").await?;
        let variant: ClassInstanceRef<String> = jvm.get_field(&this, "java/util/Locale", "variant", "Ljava/lang/String;").await?;

        let language = JavaLangString::to_rust_string(jvm, &language).await?;
        let country = JavaLangString::to_rust_string(jvm, &country).await?;
        let variant = JavaLangString::to_rust_string(jvm, &variant).await?;

        let language = Self::display_language(&language);
        let country = Self::display_country(&country);

        let result = if language.is_empty() {
            if country.is_empty() {
                variant
            } else if variant.is_empty() {
                country.to_string()
            } else {
                format!("{country} ({variant})")
            }
        } else if country.is_empty() && variant.is_empty() {
            language.to_string()
        } else {
            let mut suffixes = vec![];
            if !country.is_empty() {
                suffixes.push(country.to_string());
            }
            if !variant.is_empty() {
                suffixes.push(variant);
            }
            format!("{language} ({})", suffixes.join(", "))
        };

        Ok(JavaLangString::from_rust_string(jvm, &result).await?.into())
    }

    fn display_language(language: &str) -> &str {
        match language {
            "en" => "English",
            "fr" => "French",
            "de" => "German",
            "it" => "Italian",
            "ja" => "Japanese",
            "ko" => "Korean",
            "zh" => "Chinese",
            _ => language,
        }
    }

    fn display_country(country: &str) -> &str {
        match country {
            "US" => "United States",
            "GB" => "United Kingdom",
            "CA" => "Canada",
            "FR" => "France",
            "DE" => "Germany",
            "IT" => "Italy",
            "JP" => "Japan",
            "KR" => "South Korea",
            "CN" => "China",
            "TW" => "Taiwan",
            _ => country,
        }
    }

    #[method(public)]
    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Locale::equals({this:?}, {other:?})");

        if other.is_null() || !jvm.is_instance(&**other, "java/util/Locale") {
            return Ok(false);
        }

        let other: ClassInstanceRef<Self> = ClassInstanceRef::new(other.instance);

        let this_language: ClassInstanceRef<String> = jvm.get_field(&this, "java/util/Locale", "language", "Ljava/lang/String;").await?;
        let this_country: ClassInstanceRef<String> = jvm.get_field(&this, "java/util/Locale", "country", "Ljava/lang/String;").await?;
        let this_variant: ClassInstanceRef<String> = jvm.get_field(&this, "java/util/Locale", "variant", "Ljava/lang/String;").await?;
        let other_language: ClassInstanceRef<String> = jvm.get_field(&other, "java/util/Locale", "language", "Ljava/lang/String;").await?;
        let other_country: ClassInstanceRef<String> = jvm.get_field(&other, "java/util/Locale", "country", "Ljava/lang/String;").await?;
        let other_variant: ClassInstanceRef<String> = jvm.get_field(&other, "java/util/Locale", "variant", "Ljava/lang/String;").await?;

        let this_language = JavaLangString::to_rust_string(jvm, &this_language).await?;
        let this_country = JavaLangString::to_rust_string(jvm, &this_country).await?;
        let this_variant = JavaLangString::to_rust_string(jvm, &this_variant).await?;
        let other_language = JavaLangString::to_rust_string(jvm, &other_language).await?;
        let other_country = JavaLangString::to_rust_string(jvm, &other_country).await?;
        let other_variant = JavaLangString::to_rust_string(jvm, &other_variant).await?;

        Ok(this_language == other_language && this_country == other_country && this_variant == other_variant)
    }

    #[method(public)]
    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.Locale::hashCode({this:?})");

        let language: ClassInstanceRef<String> = jvm.get_field(&this, "java/util/Locale", "language", "Ljava/lang/String;").await?;
        let country: ClassInstanceRef<String> = jvm.get_field(&this, "java/util/Locale", "country", "Ljava/lang/String;").await?;
        let variant: ClassInstanceRef<String> = jvm.get_field(&this, "java/util/Locale", "variant", "Ljava/lang/String;").await?;

        let language = JavaLangString::to_rust_string(jvm, &language).await?;
        let country = JavaLangString::to_rust_string(jvm, &country).await?;
        let variant = JavaLangString::to_rust_string(jvm, &variant).await?;

        Ok(Self::string_hash(&language) ^ Self::string_hash(&country) ^ Self::string_hash(&variant))
    }

    fn string_hash(value: &str) -> i32 {
        value.encode_utf16().fold(0i32, |acc, ch| acc.wrapping_mul(31).wrapping_add(ch as i32))
    }

    #[method(public)]
    async fn clone(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Locale::clone({this:?})");

        let language: ClassInstanceRef<String> = jvm.get_field(&this, "java/util/Locale", "language", "Ljava/lang/String;").await?;
        let country: ClassInstanceRef<String> = jvm.get_field(&this, "java/util/Locale", "country", "Ljava/lang/String;").await?;
        let variant: ClassInstanceRef<String> = jvm.get_field(&this, "java/util/Locale", "variant", "Ljava/lang/String;").await?;

        let language = JavaLangString::to_rust_string(jvm, &language).await?;
        let country = JavaLangString::to_rust_string(jvm, &country).await?;
        let variant = JavaLangString::to_rust_string(jvm, &variant).await?;

        Ok(jvm
            .new_class(
                "java/util/Locale",
                "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
                (
                    JavaLangString::from_rust_string(jvm, &language).await?,
                    JavaLangString::from_rust_string(jvm, &country).await?,
                    JavaLangString::from_rust_string(jvm, &variant).await?,
                ),
            )
            .await?
            .into())
    }

    #[method(public, final)]
    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.Locale::toString({this:?})");

        let language: ClassInstanceRef<String> = jvm.get_field(&this, "java/util/Locale", "language", "Ljava/lang/String;").await?;
        let country: ClassInstanceRef<String> = jvm.get_field(&this, "java/util/Locale", "country", "Ljava/lang/String;").await?;
        let variant: ClassInstanceRef<String> = jvm.get_field(&this, "java/util/Locale", "variant", "Ljava/lang/String;").await?;

        let language = JavaLangString::to_rust_string(jvm, &language).await?;
        let country = JavaLangString::to_rust_string(jvm, &country).await?;
        let variant = JavaLangString::to_rust_string(jvm, &variant).await?;

        let result = if variant.is_empty() {
            if country.is_empty() { language } else { format!("{language}_{country}") }
        } else {
            format!("{language}_{country}_{variant}")
        };

        Ok(JavaLangString::from_rust_string(jvm, &result).await?.into())
    }
}
