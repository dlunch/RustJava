use java_runtime::classes::java::lang::{Character, String};
use jvm::{Array, ClassInstanceRef, JavaChar, JavaError, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_character_value_constants_and_type() -> Result<()> {
    let jvm = test_jvm().await?;

    let value: ClassInstanceRef<Character> = jvm.new_class("java/lang/Character", "(C)V", ('A' as JavaChar,)).await?.into();
    assert_eq!(jvm.invoke_virtual::<_, JavaChar>(&value, "charValue", "()C", ()).await?, 'A' as JavaChar);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&value, "hashCode", "()I", ()).await?, 'A' as i32);

    let text: ClassInstanceRef<String> = jvm.invoke_virtual(&value, "toString", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "A");

    assert_eq!(jvm.get_static_field::<JavaChar>("java/lang/Character", "MIN_VALUE", "C").await?, 0);
    assert_eq!(jvm.get_static_field::<JavaChar>("java/lang/Character", "MAX_VALUE", "C").await?, u16::MAX);
    assert_eq!(jvm.get_static_field::<i32>("java/lang/Character", "MIN_RADIX", "I").await?, 2);
    assert_eq!(jvm.get_static_field::<i32>("java/lang/Character", "MAX_RADIX", "I").await?, 36);

    for (name, expected) in [
        ("UNASSIGNED", 0i8),
        ("UPPERCASE_LETTER", 1),
        ("LOWERCASE_LETTER", 2),
        ("TITLECASE_LETTER", 3),
        ("MODIFIER_LETTER", 4),
        ("OTHER_LETTER", 5),
        ("NON_SPACING_MARK", 6),
        ("ENCLOSING_MARK", 7),
        ("COMBINING_SPACING_MARK", 8),
        ("DECIMAL_DIGIT_NUMBER", 9),
        ("LETTER_NUMBER", 10),
        ("OTHER_NUMBER", 11),
        ("SPACE_SEPARATOR", 12),
        ("LINE_SEPARATOR", 13),
        ("PARAGRAPH_SEPARATOR", 14),
        ("CONTROL", 15),
        ("FORMAT", 16),
        ("PRIVATE_USE", 18),
        ("SURROGATE", 19),
        ("DASH_PUNCTUATION", 20),
        ("START_PUNCTUATION", 21),
        ("END_PUNCTUATION", 22),
        ("CONNECTOR_PUNCTUATION", 23),
        ("OTHER_PUNCTUATION", 24),
        ("MATH_SYMBOL", 25),
        ("CURRENCY_SYMBOL", 26),
        ("MODIFIER_SYMBOL", 27),
        ("OTHER_SYMBOL", 28),
    ] {
        assert_eq!(jvm.get_static_field::<i8>("java/lang/Character", name, "B").await?, expected);
    }

    let typ = jvm.get_static_field("java/lang/Character", "TYPE", "Ljava/lang/Class;").await?;
    let name: ClassInstanceRef<String> = jvm.invoke_virtual(&typ, "getName", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &name).await?, "char");
    assert!(jvm.invoke_virtual::<_, bool>(&typ, "isPrimitive", "()Z", ()).await?);
    assert!(jvm.is_instance(&**value, "java/lang/Comparable"));
    assert!(jvm.is_instance(&**value, "java/io/Serializable"));

    let result: Result<ClassInstanceRef<Character>> = jvm
        .invoke_static("java/lang/Character", "valueOf", "(C)Ljava/lang/Character;", ('A' as JavaChar,))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Character.valueOf(char) must remain outside the Java 1.2 API");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NoSuchMethodError"));

    Ok(())
}

#[tokio::test]
async fn test_character_ascii_non_ascii_and_radix() -> Result<()> {
    let jvm = test_jvm().await?;

    assert!(
        jvm.invoke_static::<_, bool>("java/lang/Character", "isLetter", "(C)Z", ('A' as JavaChar,))
            .await?
    );
    assert!(
        jvm.invoke_static::<_, bool>("java/lang/Character", "isUpperCase", "(C)Z", ('A' as JavaChar,))
            .await?
    );
    assert!(
        jvm.invoke_static::<_, bool>("java/lang/Character", "isLowerCase", "(C)Z", ('z' as JavaChar,))
            .await?
    );
    assert!(
        jvm.invoke_static::<_, bool>("java/lang/Character", "isDigit", "(C)Z", ('7' as JavaChar,))
            .await?
    );
    assert!(
        jvm.invoke_static::<_, bool>("java/lang/Character", "isJavaIdentifierStart", "(C)Z", ('_' as JavaChar,))
            .await?
    );
    assert!(
        jvm.invoke_static::<_, bool>("java/lang/Character", "isJavaIdentifierStart", "(C)Z", ('$' as JavaChar,))
            .await?
    );
    assert!(
        jvm.invoke_static::<_, bool>("java/lang/Character", "isJavaIdentifierPart", "(C)Z", ('7' as JavaChar,))
            .await?
    );
    assert!(
        jvm.invoke_static::<_, bool>("java/lang/Character", "isWhitespace", "(C)Z", ('\n' as JavaChar,))
            .await?
    );
    for value in ['\u{0009}', '\u{000a}', '\u{000c}', '\u{000d}', '\u{0020}'] {
        assert!(
            jvm.invoke_static::<_, bool>("java/lang/Character", "isSpace", "(C)Z", (value as JavaChar,))
                .await?
        );
    }
    for value in ['\u{000b}', '\u{001c}'] {
        assert!(
            !jvm.invoke_static::<_, bool>("java/lang/Character", "isSpace", "(C)Z", (value as JavaChar,))
                .await?
        );
    }
    assert!(
        jvm.invoke_static::<_, bool>("java/lang/Character", "isSpaceChar", "(C)Z", ('\u{00a0}' as JavaChar,))
            .await?
    );
    assert!(
        !jvm.invoke_static::<_, bool>("java/lang/Character", "isWhitespace", "(C)Z", ('\u{00a0}' as JavaChar,))
            .await?
    );

    assert!(
        jvm.invoke_static::<_, bool>("java/lang/Character", "isLetter", "(C)Z", ('é' as JavaChar,))
            .await?
    );
    assert!(
        jvm.invoke_static::<_, bool>("java/lang/Character", "isUpperCase", "(C)Z", ('Ω' as JavaChar,))
            .await?
    );
    assert_eq!(
        jvm.invoke_static::<_, JavaChar>("java/lang/Character", "toLowerCase", "(C)C", ('Ω' as JavaChar,))
            .await?,
        'ω' as JavaChar
    );
    assert_eq!(
        jvm.invoke_static::<_, JavaChar>("java/lang/Character", "toUpperCase", "(C)C", ('ß' as JavaChar,))
            .await?,
        'ß' as JavaChar
    );

    let arabic_three = '٣' as JavaChar;
    assert!(
        jvm.invoke_static::<_, bool>("java/lang/Character", "isDigit", "(C)Z", (arabic_three,))
            .await?
    );
    assert_eq!(
        jvm.invoke_static::<_, i32>("java/lang/Character", "digit", "(CI)I", (arabic_three, 10))
            .await?,
        3
    );
    assert_eq!(
        jvm.invoke_static::<_, i32>("java/lang/Character", "getNumericValue", "(C)I", (arabic_three,))
            .await?,
        3
    );
    assert_eq!(
        jvm.invoke_static::<_, i32>("java/lang/Character", "digit", "(CI)I", ('Ａ' as JavaChar, 16))
            .await?,
        10
    );
    assert_eq!(
        jvm.invoke_static::<_, i32>("java/lang/Character", "digit", "(CI)I", ('g' as JavaChar, 16))
            .await?,
        -1
    );
    assert_eq!(
        jvm.invoke_static::<_, JavaChar>("java/lang/Character", "forDigit", "(II)C", (15, 16))
            .await?,
        'f' as JavaChar
    );
    assert_eq!(
        jvm.invoke_static::<_, JavaChar>("java/lang/Character", "forDigit", "(II)C", (16, 16))
            .await?,
        0
    );

    assert_eq!(
        jvm.invoke_static::<_, i32>("java/lang/Character", "getType", "(C)I", ('A' as JavaChar,))
            .await?,
        1
    );
    assert_eq!(
        jvm.invoke_static::<_, i32>("java/lang/Character", "getType", "(C)I", ('é' as JavaChar,))
            .await?,
        2
    );
    assert_eq!(
        jvm.invoke_static::<_, i32>("java/lang/Character", "getType", "(C)I", (arabic_three,))
            .await?,
        9
    );
    assert_eq!(
        jvm.invoke_static::<_, i32>("java/lang/Character", "getType", "(C)I", ('\u{0301}' as JavaChar,))
            .await?,
        6
    );
    assert_eq!(
        jvm.invoke_static::<_, i32>("java/lang/Character", "getType", "(C)I", ('Ⅷ' as JavaChar,))
            .await?,
        10
    );
    assert!(
        !jvm.invoke_static::<_, bool>("java/lang/Character", "isDefined", "(C)Z", (0x0378 as JavaChar,))
            .await?
    );

    for (value, expected_type) in [('¡', 24), ('«', 21), ('»', 22), ('§', 24)] {
        assert_eq!(
            jvm.invoke_static::<_, i32>("java/lang/Character", "getType", "(C)I", (value as JavaChar,))
                .await?,
            expected_type
        );
        assert!(
            jvm.invoke_static::<_, bool>("java/lang/Character", "isDefined", "(C)Z", (value as JavaChar,))
                .await?
        );
    }
    assert_eq!(
        jvm.invoke_static::<_, i32>("java/lang/Character", "getType", "(C)I", (0xe000 as JavaChar,))
            .await?,
        18
    );
    assert_eq!(
        jvm.invoke_static::<_, i32>("java/lang/Character", "getType", "(C)I", (0xfeff as JavaChar,))
            .await?,
        16
    );
    assert_eq!(
        jvm.invoke_static::<_, i32>("java/lang/Character", "getType", "(C)I", (0xffff as JavaChar,))
            .await?,
        0
    );

    for value in ['£', '‿', 'Ⅰ'] {
        assert!(
            jvm.invoke_static::<_, bool>("java/lang/Character", "isJavaIdentifierStart", "(C)Z", (value as JavaChar,))
                .await?
        );
    }
    for value in ['Ⅰ', '\u{0301}', '\u{0000}'] {
        assert!(
            jvm.invoke_static::<_, bool>("java/lang/Character", "isJavaIdentifierPart", "(C)Z", (value as JavaChar,))
                .await?
        );
    }
    for value in ['£', '‿', 'Ⅰ'] {
        assert!(
            jvm.invoke_static::<_, bool>("java/lang/Character", "isJavaLetter", "(C)Z", (value as JavaChar,))
                .await?
        );
    }
    for value in ['7', 'Ⅰ', '\u{0301}', '\u{0000}'] {
        assert!(
            jvm.invoke_static::<_, bool>("java/lang/Character", "isJavaLetterOrDigit", "(C)Z", (value as JavaChar,))
                .await?
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_character_surrogate_and_compare_errors() -> Result<()> {
    let jvm = test_jvm().await?;
    let surrogate = 0xd800 as JavaChar;

    let value: ClassInstanceRef<Character> = jvm.new_class("java/lang/Character", "(C)V", (surrogate,)).await?.into();
    assert_eq!(jvm.invoke_virtual::<_, JavaChar>(&value, "charValue", "()C", ()).await?, surrogate);
    let text: ClassInstanceRef<String> = jvm.invoke_virtual(&value, "toString", "()Ljava/lang/String;", ()).await?;
    let chars: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&text, "value", "[C").await?;
    assert_eq!(jvm.load_array::<JavaChar>(&chars, 0, 1).await?, [surrogate]);

    for method in [
        "isLowerCase",
        "isUpperCase",
        "isTitleCase",
        "isDigit",
        "isDefined",
        "isLetter",
        "isLetterOrDigit",
        "isJavaIdentifierStart",
        "isJavaIdentifierPart",
        "isUnicodeIdentifierStart",
        "isUnicodeIdentifierPart",
        "isIdentifierIgnorable",
        "isSpace",
        "isSpaceChar",
        "isWhitespace",
        "isISOControl",
    ] {
        assert!(!jvm.invoke_static::<_, bool>("java/lang/Character", method, "(C)Z", (surrogate,)).await?);
    }
    for method in ["toLowerCase", "toUpperCase", "toTitleCase"] {
        assert_eq!(
            jvm.invoke_static::<_, JavaChar>("java/lang/Character", method, "(C)C", (surrogate,))
                .await?,
            surrogate
        );
    }
    assert_eq!(
        jvm.invoke_static::<_, i32>("java/lang/Character", "digit", "(CI)I", (surrogate, 10))
            .await?,
        -1
    );
    assert_eq!(
        jvm.invoke_static::<_, i32>("java/lang/Character", "getNumericValue", "(C)I", (surrogate,))
            .await?,
        -1
    );
    assert_eq!(
        jvm.invoke_static::<_, i32>("java/lang/Character", "getType", "(C)I", (surrogate,))
            .await?,
        0
    );

    let other: ClassInstanceRef<Character> = jvm.new_class("java/lang/Character", "(C)V", ('Z' as JavaChar,)).await?.into();
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&value, "compareTo", "(Ljava/lang/Character;)I", (other.clone(),))
            .await?,
        1
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&other, "compareTo", "(Ljava/lang/Object;)I", (value.clone(),))
            .await?,
        -1
    );

    let null_character: ClassInstanceRef<Character> = None.into();
    let result: Result<i32> = jvm
        .invoke_virtual(&value, "compareTo", "(Ljava/lang/Character;)I", (null_character,))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Character.compareTo(Character) must reject null");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let null_object: ClassInstanceRef<Character> = None.into();
    let result: Result<i32> = jvm.invoke_virtual(&value, "compareTo", "(Ljava/lang/Object;)I", (null_object,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Character.compareTo(Object) must reject null");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let wrong = jvm.new_class("java/lang/Object", "()V", ()).await?;
    let result: Result<i32> = jvm
        .invoke_virtual(&value, "compareTo", "(Ljava/lang/Character;)I", (wrong.clone(),))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("typed Character.compareTo must reject a non-Character instance");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/ClassCastException"));

    let result: Result<i32> = jvm.invoke_virtual(&value, "compareTo", "(Ljava/lang/Object;)I", (wrong,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("raw Character.compareTo must reject a non-Character instance");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/ClassCastException"));

    Ok(())
}
