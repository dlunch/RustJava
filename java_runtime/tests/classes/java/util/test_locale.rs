use java_runtime::classes::java::{lang::String, util::Locale};
use jvm::{Array, ClassInstanceRef, JavaError, Result, runtime::JavaLangString};

use test_utils::test_jvm;

async fn string_array_contains(jvm: &jvm::Jvm, array: &ClassInstanceRef<Array<String>>, expected: &str) -> Result<bool> {
    let len = jvm.array_length(array).await?;
    for value in jvm.load_array::<ClassInstanceRef<String>>(array, 0, len).await? {
        if JavaLangString::to_rust_string(jvm, &value).await? == expected {
            return Ok(true);
        }
    }
    Ok(false)
}

#[tokio::test]
async fn test_locale_constants_and_accessors() -> Result<()> {
    let jvm = test_jvm().await?;

    let english: ClassInstanceRef<Locale> = jvm.get_static_field("java/util/Locale", "ENGLISH", "Ljava/util/Locale;").await?;
    let language = jvm.invoke_virtual(&english, "getLanguage", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &language).await?, "en");
    let country = jvm.invoke_virtual(&english, "getCountry", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &country).await?, "");
    let variant = jvm.invoke_virtual(&english, "getVariant", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &variant).await?, "");

    let english_string = jvm.invoke_virtual(&english, "toString", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &english_string).await?, "en");

    let us: ClassInstanceRef<Locale> = jvm.get_static_field("java/util/Locale", "US", "Ljava/util/Locale;").await?;
    let language = jvm.invoke_virtual(&us, "getLanguage", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &language).await?, "en");
    let country = jvm.invoke_virtual(&us, "getCountry", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &country).await?, "US");

    let us_string = jvm.invoke_virtual(&us, "toString", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &us_string).await?, "en_US");

    Ok(())
}

#[tokio::test]
async fn test_locale_constructors_to_string_equals_and_hash_code() -> Result<()> {
    let jvm = test_jvm().await?;

    let en = JavaLangString::from_rust_string(&jvm, "en").await?;
    let us = JavaLangString::from_rust_string(&jvm, "US").await?;
    let posix = JavaLangString::from_rust_string(&jvm, "POSIX").await?;
    let locale: ClassInstanceRef<Locale> = jvm
        .new_class(
            "java/util/Locale",
            "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
            (en.clone(), us.clone(), posix.clone()),
        )
        .await?
        .into();

    let language = jvm.invoke_virtual(&locale, "getLanguage", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &language).await?, "en");
    let country = jvm.invoke_virtual(&locale, "getCountry", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &country).await?, "US");
    let variant = jvm.invoke_virtual(&locale, "getVariant", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &variant).await?, "POSIX");

    let text = jvm.invoke_virtual(&locale, "toString", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "en_US_POSIX");

    let same: ClassInstanceRef<Locale> = jvm
        .new_class("java/util/Locale", "(Ljava/lang/String;Ljava/lang/String;)V", (en, us))
        .await?
        .into();
    let equals: bool = jvm.invoke_virtual(&locale, "equals", "(Ljava/lang/Object;)Z", (same.clone(),)).await?;
    assert!(!equals);

    let same_with_variant: ClassInstanceRef<Locale> = jvm
        .new_class(
            "java/util/Locale",
            "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
            (
                JavaLangString::from_rust_string(&jvm, "en").await?,
                JavaLangString::from_rust_string(&jvm, "US").await?,
                JavaLangString::from_rust_string(&jvm, "POSIX").await?,
            ),
        )
        .await?
        .into();
    let equals: bool = jvm
        .invoke_virtual(&locale, "equals", "(Ljava/lang/Object;)Z", (same_with_variant.clone(),))
        .await?;
    assert!(equals);

    let hash: i32 = jvm.invoke_virtual(&locale, "hashCode", "()I", ()).await?;
    let same_hash: i32 = jvm.invoke_virtual(&same_with_variant, "hashCode", "()I", ()).await?;
    assert_eq!(hash, same_hash);

    Ok(())
}

#[tokio::test]
async fn test_locale_default_round_trip_and_null_rejection() -> Result<()> {
    let jvm = test_jvm().await?;

    let default: ClassInstanceRef<Locale> = jvm.invoke_static("java/util/Locale", "getDefault", "()Ljava/util/Locale;", ()).await?;
    let language = jvm.invoke_virtual(&default, "getLanguage", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &language).await?, "en");
    let country = jvm.invoke_virtual(&default, "getCountry", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &country).await?, "US");

    let custom: ClassInstanceRef<Locale> = jvm
        .new_class(
            "java/util/Locale",
            "(Ljava/lang/String;Ljava/lang/String;)V",
            (
                JavaLangString::from_rust_string(&jvm, "en").await?,
                JavaLangString::from_rust_string(&jvm, "GB").await?,
            ),
        )
        .await?
        .into();
    let _: () = jvm
        .invoke_static("java/util/Locale", "setDefault", "(Ljava/util/Locale;)V", (custom.clone(),))
        .await?;

    let updated: ClassInstanceRef<Locale> = jvm.invoke_static("java/util/Locale", "getDefault", "()Ljava/util/Locale;", ()).await?;
    let country = jvm.invoke_virtual(&updated, "getCountry", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &country).await?, "GB");

    let err = jvm
        .invoke_static::<_, ()>(
            "java/util/Locale",
            "setDefault",
            "(Ljava/util/Locale;)V",
            (ClassInstanceRef::<Locale>::new(None),),
        )
        .await
        .unwrap_err();
    assert!(matches!(err, JavaError::JavaException(_)));

    Ok(())
}

#[tokio::test]
async fn test_locale_compatibility_edges() -> Result<()> {
    let jvm = test_jvm().await?;

    let english: ClassInstanceRef<Locale> = jvm.get_static_field("java/util/Locale", "ENGLISH", "Ljava/util/Locale;").await?;
    let display: Result<ClassInstanceRef<String>> = jvm
        .invoke_virtual(
            &english,
            "getDisplayLanguage",
            "(Ljava/util/Locale;)Ljava/lang/String;",
            (ClassInstanceRef::<Locale>::new(None),),
        )
        .await;
    assert!(matches!(display, Err(JavaError::JavaException(_))));

    let unsupported: ClassInstanceRef<Locale> = jvm
        .new_class(
            "java/util/Locale",
            "(Ljava/lang/String;Ljava/lang/String;)V",
            (
                JavaLangString::from_rust_string(&jvm, "xx").await?,
                JavaLangString::from_rust_string(&jvm, "YY").await?,
            ),
        )
        .await?
        .into();
    let iso3_language = jvm.invoke_virtual::<_, ClassInstanceRef<String>>(&unsupported, "getISO3Language", "()Ljava/lang/String;", ());
    assert!(matches!(iso3_language.await, Err(JavaError::JavaException(_))));

    Ok(())
}

#[tokio::test]
async fn test_locale_available_locales_and_iso_lists() -> Result<()> {
    let jvm = test_jvm().await?;

    let locales: ClassInstanceRef<Array<Locale>> = jvm
        .invoke_static("java/util/Locale", "getAvailableLocales", "()[Ljava/util/Locale;", ())
        .await?;
    assert_eq!(jvm.array_length(&locales).await?, 21);

    let france: ClassInstanceRef<Locale> = jvm.get_static_field("java/util/Locale", "FRANCE", "Ljava/util/Locale;").await?;
    let germany: ClassInstanceRef<Locale> = jvm.get_static_field("java/util/Locale", "GERMANY", "Ljava/util/Locale;").await?;
    let prc: ClassInstanceRef<Locale> = jvm.get_static_field("java/util/Locale", "PRC", "Ljava/util/Locale;").await?;

    let mut has_france = false;
    let mut has_germany = false;
    let mut has_prc = false;
    for locale in jvm
        .load_array::<ClassInstanceRef<Locale>>(&locales, 0, jvm.array_length(&locales).await?)
        .await?
    {
        let equals_france: bool = jvm.invoke_virtual(&locale, "equals", "(Ljava/lang/Object;)Z", (france.clone(),)).await?;
        let equals_germany: bool = jvm.invoke_virtual(&locale, "equals", "(Ljava/lang/Object;)Z", (germany.clone(),)).await?;
        let equals_prc: bool = jvm.invoke_virtual(&locale, "equals", "(Ljava/lang/Object;)Z", (prc.clone(),)).await?;
        has_france |= equals_france;
        has_germany |= equals_germany;
        has_prc |= equals_prc;
    }
    assert!(has_france);
    assert!(has_germany);
    assert!(has_prc);

    let countries: ClassInstanceRef<Array<String>> = jvm
        .invoke_static("java/util/Locale", "getISOCountries", "()[Ljava/lang/String;", ())
        .await?;
    assert!(string_array_contains(&jvm, &countries, "US").await?);
    assert!(string_array_contains(&jvm, &countries, "FR").await?);

    let languages: ClassInstanceRef<Array<String>> = jvm
        .invoke_static("java/util/Locale", "getISOLanguages", "()[Ljava/lang/String;", ())
        .await?;
    assert!(string_array_contains(&jvm, &languages, "en").await?);
    assert!(string_array_contains(&jvm, &languages, "fr").await?);

    Ok(())
}

#[tokio::test]
async fn test_locale_display_iso3_and_clone_helpers() -> Result<()> {
    let jvm = test_jvm().await?;

    let us: ClassInstanceRef<Locale> = jvm.get_static_field("java/util/Locale", "US", "Ljava/util/Locale;").await?;

    let display_language = jvm.invoke_virtual(&us, "getDisplayLanguage", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &display_language).await?, "English");

    let display_country = jvm.invoke_virtual(&us, "getDisplayCountry", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &display_country).await?, "United States");

    let display_variant = jvm.invoke_virtual(&us, "getDisplayVariant", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &display_variant).await?, "");

    let display_name = jvm.invoke_virtual(&us, "getDisplayName", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &display_name).await?, "English (United States)");

    let iso3_language = jvm.invoke_virtual(&us, "getISO3Language", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &iso3_language).await?, "eng");

    let iso3_country = jvm.invoke_virtual(&us, "getISO3Country", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &iso3_country).await?, "USA");

    let cloned: ClassInstanceRef<Locale> = jvm.invoke_virtual(&us, "clone", "()Ljava/lang/Object;", ()).await?;
    let equals: bool = jvm.invoke_virtual(&us, "equals", "(Ljava/lang/Object;)Z", (cloned,)).await?;
    assert!(equals);

    Ok(())
}
