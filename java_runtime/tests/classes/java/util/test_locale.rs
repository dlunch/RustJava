use java_runtime::classes::java::{lang::String as JavaString, util::Locale};
use jvm::{ClassInstanceRef, JavaError, Result, runtime::JavaLangString};

use test_utils::test_jvm;

async fn locale_string(jvm: &jvm::Jvm, locale: &ClassInstanceRef<Locale>, method: &str) -> Result<String> {
    let value = jvm.invoke_virtual(locale, method, "()Ljava/lang/String;", ()).await?;
    JavaLangString::to_rust_string(jvm, &value).await
}

#[tokio::test]
async fn test_locale_constants_and_accessors() -> Result<()> {
    let jvm = test_jvm().await?;

    let english: ClassInstanceRef<Locale> = jvm.get_static_field("java/util/Locale", "ENGLISH", "Ljava/util/Locale;").await?;
    assert_eq!(locale_string(&jvm, &english, "getLanguage").await?, "en");
    assert_eq!(locale_string(&jvm, &english, "getCountry").await?, "");
    assert_eq!(locale_string(&jvm, &english, "getVariant").await?, "");

    let english_string = jvm.invoke_virtual(&english, "toString", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &english_string).await?, "en");

    let us: ClassInstanceRef<Locale> = jvm.get_static_field("java/util/Locale", "US", "Ljava/util/Locale;").await?;
    assert_eq!(locale_string(&jvm, &us, "getLanguage").await?, "en");
    assert_eq!(locale_string(&jvm, &us, "getCountry").await?, "US");

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

    assert_eq!(locale_string(&jvm, &locale, "getLanguage").await?, "en");
    assert_eq!(locale_string(&jvm, &locale, "getCountry").await?, "US");
    assert_eq!(locale_string(&jvm, &locale, "getVariant").await?, "POSIX");

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
    assert_eq!(locale_string(&jvm, &default, "getLanguage").await?, "en");
    assert_eq!(locale_string(&jvm, &default, "getCountry").await?, "US");

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
    assert_eq!(locale_string(&jvm, &updated, "getCountry").await?, "GB");

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
    let display: Result<ClassInstanceRef<JavaString>> = jvm
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
    let iso3_language = jvm.invoke_virtual::<_, ClassInstanceRef<JavaString>>(&unsupported, "getISO3Language", "()Ljava/lang/String;", ());
    assert!(matches!(iso3_language.await, Err(JavaError::JavaException(_))));

    Ok(())
}
