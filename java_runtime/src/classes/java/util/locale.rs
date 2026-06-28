use core::iter;

use alloc::{format, string::ToString, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::lang::{Object, String as JavaString},
};

// class java.util.Locale
pub struct Locale;

impl Locale {
    pub fn as_proto() -> RuntimeClassProto {
        let public_static_final = FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL;

        RuntimeClassProto {
            name: "java/util/Locale",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/lang/Cloneable", "java/io/Serializable"],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_with_language, Default::default()),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/lang/String;Ljava/lang/String;)V",
                    Self::init_with_language_country,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
                    Self::init,
                    Default::default(),
                ),
                JavaMethodProto::new("getDefault", "()Ljava/util/Locale;", Self::get_default, MethodAccessFlags::STATIC),
                JavaMethodProto::new("setDefault", "(Ljava/util/Locale;)V", Self::set_default, MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "getAvailableLocales",
                    "()[Ljava/util/Locale;",
                    Self::get_available_locales,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "getISOCountries",
                    "()[Ljava/lang/String;",
                    Self::get_iso_countries,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "getISOLanguages",
                    "()[Ljava/lang/String;",
                    Self::get_iso_languages,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("getLanguage", "()Ljava/lang/String;", Self::get_language, Default::default()),
                JavaMethodProto::new("getCountry", "()Ljava/lang/String;", Self::get_country, Default::default()),
                JavaMethodProto::new("getVariant", "()Ljava/lang/String;", Self::get_variant, Default::default()),
                JavaMethodProto::new("getISO3Language", "()Ljava/lang/String;", Self::get_iso3_language, Default::default()),
                JavaMethodProto::new("getISO3Country", "()Ljava/lang/String;", Self::get_iso3_country, Default::default()),
                JavaMethodProto::new(
                    "getDisplayLanguage",
                    "()Ljava/lang/String;",
                    Self::get_display_language_default,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getDisplayLanguage",
                    "(Ljava/util/Locale;)Ljava/lang/String;",
                    Self::get_display_language,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getDisplayCountry",
                    "()Ljava/lang/String;",
                    Self::get_display_country_default,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getDisplayCountry",
                    "(Ljava/util/Locale;)Ljava/lang/String;",
                    Self::get_display_country,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getDisplayVariant",
                    "()Ljava/lang/String;",
                    Self::get_display_variant_default,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getDisplayVariant",
                    "(Ljava/util/Locale;)Ljava/lang/String;",
                    Self::get_display_variant,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getDisplayName",
                    "()Ljava/lang/String;",
                    Self::get_display_name_default,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getDisplayName",
                    "(Ljava/util/Locale;)Ljava/lang/String;",
                    Self::get_display_name,
                    Default::default(),
                ),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, Default::default()),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, Default::default()),
                JavaMethodProto::new("clone", "()Ljava/lang/Object;", Self::clone, Default::default()),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("ENGLISH", "Ljava/util/Locale;", public_static_final),
                JavaFieldProto::new("FRENCH", "Ljava/util/Locale;", public_static_final),
                JavaFieldProto::new("GERMAN", "Ljava/util/Locale;", public_static_final),
                JavaFieldProto::new("ITALIAN", "Ljava/util/Locale;", public_static_final),
                JavaFieldProto::new("JAPANESE", "Ljava/util/Locale;", public_static_final),
                JavaFieldProto::new("KOREAN", "Ljava/util/Locale;", public_static_final),
                JavaFieldProto::new("CHINESE", "Ljava/util/Locale;", public_static_final),
                JavaFieldProto::new("SIMPLIFIED_CHINESE", "Ljava/util/Locale;", public_static_final),
                JavaFieldProto::new("TRADITIONAL_CHINESE", "Ljava/util/Locale;", public_static_final),
                JavaFieldProto::new("FRANCE", "Ljava/util/Locale;", public_static_final),
                JavaFieldProto::new("GERMANY", "Ljava/util/Locale;", public_static_final),
                JavaFieldProto::new("ITALY", "Ljava/util/Locale;", public_static_final),
                JavaFieldProto::new("JAPAN", "Ljava/util/Locale;", public_static_final),
                JavaFieldProto::new("KOREA", "Ljava/util/Locale;", public_static_final),
                JavaFieldProto::new("CHINA", "Ljava/util/Locale;", public_static_final),
                JavaFieldProto::new("PRC", "Ljava/util/Locale;", public_static_final),
                JavaFieldProto::new("TAIWAN", "Ljava/util/Locale;", public_static_final),
                JavaFieldProto::new("UK", "Ljava/util/Locale;", public_static_final),
                JavaFieldProto::new("US", "Ljava/util/Locale;", public_static_final),
                JavaFieldProto::new("CANADA", "Ljava/util/Locale;", public_static_final),
                JavaFieldProto::new("CANADA_FRENCH", "Ljava/util/Locale;", public_static_final),
                JavaFieldProto::new(
                    "defaultLocale",
                    "Ljava/util/Locale;",
                    FieldAccessFlags::PRIVATE | FieldAccessFlags::STATIC,
                ),
                JavaFieldProto::new("language", "Ljava/lang/String;", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL),
                JavaFieldProto::new("country", "Ljava/lang/String;", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL),
                JavaFieldProto::new("variant", "Ljava/lang/String;", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL),
            ],
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::FINAL,
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        tracing::debug!("java.util.Locale::<clinit>()");

        Self::put_constant(jvm, "ENGLISH", "en", "", "").await?;
        Self::put_constant(jvm, "FRENCH", "fr", "", "").await?;
        Self::put_constant(jvm, "GERMAN", "de", "", "").await?;
        Self::put_constant(jvm, "ITALIAN", "it", "", "").await?;
        Self::put_constant(jvm, "JAPANESE", "ja", "", "").await?;
        Self::put_constant(jvm, "KOREAN", "ko", "", "").await?;
        Self::put_constant(jvm, "CHINESE", "zh", "", "").await?;

        let simplified_chinese = Self::put_constant(jvm, "SIMPLIFIED_CHINESE", "zh", "CN", "").await?;
        let traditional_chinese = Self::put_constant(jvm, "TRADITIONAL_CHINESE", "zh", "TW", "").await?;

        Self::put_constant(jvm, "FRANCE", "fr", "FR", "").await?;
        Self::put_constant(jvm, "GERMANY", "de", "DE", "").await?;
        Self::put_constant(jvm, "ITALY", "it", "IT", "").await?;
        Self::put_constant(jvm, "JAPAN", "ja", "JP", "").await?;
        Self::put_constant(jvm, "KOREA", "ko", "KR", "").await?;
        Self::put_constant(jvm, "UK", "en", "GB", "").await?;
        let us = Self::put_constant(jvm, "US", "en", "US", "").await?;
        Self::put_constant(jvm, "CANADA", "en", "CA", "").await?;
        Self::put_constant(jvm, "CANADA_FRENCH", "fr", "CA", "").await?;

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

    async fn put_constant(jvm: &Jvm, field_name: &str, language: &str, country: &str, variant: &str) -> Result<ClassInstanceRef<Self>> {
        let locale = Self::new_locale(jvm, language, country, variant).await?;
        jvm.put_static_field("java/util/Locale", field_name, "Ljava/util/Locale;", locale.clone())
            .await?;
        Ok(locale)
    }

    async fn new_locale(jvm: &Jvm, language: &str, country: &str, variant: &str) -> Result<ClassInstanceRef<Self>> {
        let language = JavaLangString::from_rust_string(jvm, language).await?;
        let country = JavaLangString::from_rust_string(jvm, country).await?;
        let variant = JavaLangString::from_rust_string(jvm, variant).await?;

        Ok(jvm
            .new_class(
                "java/util/Locale",
                "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
                (language, country, variant),
            )
            .await?
            .into())
    }

    async fn init_with_language(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        language: ClassInstanceRef<JavaString>,
    ) -> Result<()> {
        tracing::debug!("java.util.Locale::<init>({this:?}, {language:?})");

        let country: ClassInstanceRef<JavaString> = JavaLangString::from_rust_string(jvm, "").await?.into();
        let variant: ClassInstanceRef<JavaString> = JavaLangString::from_rust_string(jvm, "").await?.into();
        jvm.invoke_special(
            &this,
            "java/util/Locale",
            "<init>",
            "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
            (language, country, variant),
        )
        .await
    }

    async fn init_with_language_country(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        language: ClassInstanceRef<JavaString>,
        country: ClassInstanceRef<JavaString>,
    ) -> Result<()> {
        tracing::debug!("java.util.Locale::<init>({this:?}, {language:?}, {country:?})");

        let variant: ClassInstanceRef<JavaString> = JavaLangString::from_rust_string(jvm, "").await?.into();
        jvm.invoke_special(
            &this,
            "java/util/Locale",
            "<init>",
            "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
            (language, country, variant),
        )
        .await
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        language: ClassInstanceRef<JavaString>,
        country: ClassInstanceRef<JavaString>,
        variant: ClassInstanceRef<JavaString>,
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

        jvm.put_field(&mut this, "language", "Ljava/lang/String;", language).await?;
        jvm.put_field(&mut this, "country", "Ljava/lang/String;", country).await?;
        jvm.put_field(&mut this, "variant", "Ljava/lang/String;", variant).await?;

        Ok(())
    }

    async fn get_default(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.util.Locale::getDefault()");

        jvm.get_static_field("java/util/Locale", "defaultLocale", "Ljava/util/Locale;").await
    }

    async fn set_default(jvm: &Jvm, _: &mut RuntimeContext, locale: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Locale::setDefault({locale:?})");

        if locale.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "Locale default is null").await);
        }

        jvm.put_static_field("java/util/Locale", "defaultLocale", "Ljava/util/Locale;", locale)
            .await
    }

    async fn get_available_locales(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Array<Self>>> {
        tracing::debug!("java.util.Locale::getAvailableLocales()");

        let fields = ["ENGLISH", "US", "UK", "CANADA", "CANADA_FRENCH", "CHINESE", "CHINA", "TAIWAN"];
        let mut array: ClassInstanceRef<Array<Self>> = jvm.instantiate_array("Ljava/util/Locale;", fields.len()).await?.into();

        for (i, field) in fields.iter().enumerate() {
            let locale: ClassInstanceRef<Self> = jvm.get_static_field("java/util/Locale", field, "Ljava/util/Locale;").await?;
            jvm.store_array(&mut array, i, iter::once(locale)).await?;
        }

        Ok(array)
    }

    async fn get_iso_countries(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Array<JavaString>>> {
        tracing::debug!("java.util.Locale::getISOCountries()");

        Self::string_array(jvm, &["US", "GB", "CA", "FR", "DE", "IT", "JP", "KR", "CN", "TW"]).await
    }

    async fn get_iso_languages(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Array<JavaString>>> {
        tracing::debug!("java.util.Locale::getISOLanguages()");

        Self::string_array(jvm, &["en", "fr", "de", "it", "ja", "ko", "zh"]).await
    }

    async fn string_array(jvm: &Jvm, values: &[&str]) -> Result<ClassInstanceRef<Array<JavaString>>> {
        let mut array: ClassInstanceRef<Array<JavaString>> = jvm.instantiate_array("Ljava/lang/String;", values.len()).await?.into();
        for (i, value) in values.iter().enumerate() {
            let value: ClassInstanceRef<JavaString> = JavaLangString::from_rust_string(jvm, value).await?.into();
            jvm.store_array(&mut array, i, iter::once(value)).await?;
        }
        Ok(array)
    }

    async fn get_language(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<JavaString>> {
        tracing::debug!("java.util.Locale::getLanguage({this:?})");

        jvm.get_field(&this, "language", "Ljava/lang/String;").await
    }

    async fn get_country(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<JavaString>> {
        tracing::debug!("java.util.Locale::getCountry({this:?})");

        jvm.get_field(&this, "country", "Ljava/lang/String;").await
    }

    async fn get_variant(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<JavaString>> {
        tracing::debug!("java.util.Locale::getVariant({this:?})");

        jvm.get_field(&this, "variant", "Ljava/lang/String;").await
    }

    async fn get_iso3_language(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<JavaString>> {
        tracing::debug!("java.util.Locale::getISO3Language({this:?})");

        let language: ClassInstanceRef<JavaString> = jvm.get_field(&this, "language", "Ljava/lang/String;").await?;
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

    async fn get_iso3_country(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<JavaString>> {
        tracing::debug!("java.util.Locale::getISO3Country({this:?})");

        let country: ClassInstanceRef<JavaString> = jvm.get_field(&this, "country", "Ljava/lang/String;").await?;
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

    async fn get_display_language_default(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<JavaString>> {
        let default = Self::get_default(jvm, context).await?;
        Self::get_display_language(jvm, context, this, default).await
    }

    async fn get_display_language(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        display_locale: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<JavaString>> {
        tracing::debug!("java.util.Locale::getDisplayLanguage({this:?})");

        if display_locale.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "display locale is null").await);
        }

        let language: ClassInstanceRef<JavaString> = jvm.get_field(&this, "language", "Ljava/lang/String;").await?;
        let language = JavaLangString::to_rust_string(jvm, &language).await?;

        Ok(JavaLangString::from_rust_string(jvm, Self::display_language(&language)).await?.into())
    }

    async fn get_display_country_default(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<JavaString>> {
        let default = Self::get_default(jvm, context).await?;
        Self::get_display_country(jvm, context, this, default).await
    }

    async fn get_display_country(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        display_locale: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<JavaString>> {
        tracing::debug!("java.util.Locale::getDisplayCountry({this:?})");

        if display_locale.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "display locale is null").await);
        }

        let country: ClassInstanceRef<JavaString> = jvm.get_field(&this, "country", "Ljava/lang/String;").await?;
        let country = JavaLangString::to_rust_string(jvm, &country).await?;

        Ok(JavaLangString::from_rust_string(jvm, Self::display_country(&country)).await?.into())
    }

    async fn get_display_variant_default(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<JavaString>> {
        let default = Self::get_default(jvm, context).await?;
        Self::get_display_variant(jvm, context, this, default).await
    }

    async fn get_display_variant(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        display_locale: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<JavaString>> {
        tracing::debug!("java.util.Locale::getDisplayVariant({this:?})");

        if display_locale.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "display locale is null").await);
        }

        jvm.get_field(&this, "variant", "Ljava/lang/String;").await
    }

    async fn get_display_name_default(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<JavaString>> {
        let default = Self::get_default(jvm, context).await?;
        Self::get_display_name(jvm, context, this, default).await
    }

    async fn get_display_name(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        display_locale: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<JavaString>> {
        tracing::debug!("java.util.Locale::getDisplayName({this:?})");

        if display_locale.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "display locale is null").await);
        }

        let language: ClassInstanceRef<JavaString> = jvm.get_field(&this, "language", "Ljava/lang/String;").await?;
        let country: ClassInstanceRef<JavaString> = jvm.get_field(&this, "country", "Ljava/lang/String;").await?;
        let variant: ClassInstanceRef<JavaString> = jvm.get_field(&this, "variant", "Ljava/lang/String;").await?;

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

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Locale::equals({this:?}, {other:?})");

        if other.is_null() || !jvm.is_instance(&**other, "java/util/Locale") {
            return Ok(false);
        }

        let other: ClassInstanceRef<Self> = ClassInstanceRef::new(other.instance);

        let this_language: ClassInstanceRef<JavaString> = jvm.get_field(&this, "language", "Ljava/lang/String;").await?;
        let this_country: ClassInstanceRef<JavaString> = jvm.get_field(&this, "country", "Ljava/lang/String;").await?;
        let this_variant: ClassInstanceRef<JavaString> = jvm.get_field(&this, "variant", "Ljava/lang/String;").await?;
        let other_language: ClassInstanceRef<JavaString> = jvm.get_field(&other, "language", "Ljava/lang/String;").await?;
        let other_country: ClassInstanceRef<JavaString> = jvm.get_field(&other, "country", "Ljava/lang/String;").await?;
        let other_variant: ClassInstanceRef<JavaString> = jvm.get_field(&other, "variant", "Ljava/lang/String;").await?;

        let this_language = JavaLangString::to_rust_string(jvm, &this_language).await?;
        let this_country = JavaLangString::to_rust_string(jvm, &this_country).await?;
        let this_variant = JavaLangString::to_rust_string(jvm, &this_variant).await?;
        let other_language = JavaLangString::to_rust_string(jvm, &other_language).await?;
        let other_country = JavaLangString::to_rust_string(jvm, &other_country).await?;
        let other_variant = JavaLangString::to_rust_string(jvm, &other_variant).await?;

        Ok(this_language == other_language && this_country == other_country && this_variant == other_variant)
    }

    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.Locale::hashCode({this:?})");

        let language: ClassInstanceRef<JavaString> = jvm.get_field(&this, "language", "Ljava/lang/String;").await?;
        let country: ClassInstanceRef<JavaString> = jvm.get_field(&this, "country", "Ljava/lang/String;").await?;
        let variant: ClassInstanceRef<JavaString> = jvm.get_field(&this, "variant", "Ljava/lang/String;").await?;

        let language = JavaLangString::to_rust_string(jvm, &language).await?;
        let country = JavaLangString::to_rust_string(jvm, &country).await?;
        let variant = JavaLangString::to_rust_string(jvm, &variant).await?;

        Ok(Self::string_hash(&language) ^ Self::string_hash(&country) ^ Self::string_hash(&variant))
    }

    fn string_hash(value: &str) -> i32 {
        value.encode_utf16().fold(0i32, |acc, ch| acc.wrapping_mul(31).wrapping_add(ch as i32))
    }

    async fn clone(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Locale::clone({this:?})");

        let language: ClassInstanceRef<JavaString> = jvm.get_field(&this, "language", "Ljava/lang/String;").await?;
        let country: ClassInstanceRef<JavaString> = jvm.get_field(&this, "country", "Ljava/lang/String;").await?;
        let variant: ClassInstanceRef<JavaString> = jvm.get_field(&this, "variant", "Ljava/lang/String;").await?;

        let language = JavaLangString::to_rust_string(jvm, &language).await?;
        let country = JavaLangString::to_rust_string(jvm, &country).await?;
        let variant = JavaLangString::to_rust_string(jvm, &variant).await?;

        let locale = Self::new_locale(jvm, &language, &country, &variant).await?;
        Ok(ClassInstanceRef::new(locale.instance))
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<JavaString>> {
        tracing::debug!("java.util.Locale::toString({this:?})");

        let language: ClassInstanceRef<JavaString> = jvm.get_field(&this, "language", "Ljava/lang/String;").await?;
        let country: ClassInstanceRef<JavaString> = jvm.get_field(&this, "country", "Ljava/lang/String;").await?;
        let variant: ClassInstanceRef<JavaString> = jvm.get_field(&this, "variant", "Ljava/lang/String;").await?;

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
