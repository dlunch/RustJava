use std::collections::BTreeMap;

use jvm::{runtime::JavaLangString, ClassInstance};

use java_tests::test_jvm;

#[futures_test::test]
async fn test_string() -> anyhow::Result<()> {
    let mut jvm = test_jvm(BTreeMap::new(), |_| {});

    let string = JavaLangString::new(&mut jvm, "test").await?;

    let string = string.to_string(&mut jvm)?;

    assert_eq!(string, "test");

    Ok(())
}

#[futures_test::test]
async fn test_string_concat() -> anyhow::Result<()> {
    let mut jvm = test_jvm(BTreeMap::new(), |_| {});

    let string1 = JavaLangString::new(&mut jvm, "test1").await?;
    let string2 = JavaLangString::new(&mut jvm, "test2").await?;

    let result: Box<dyn ClassInstance> = jvm
        .invoke_virtual(
            &string1.instance,
            "java/lang/String",
            "concat",
            "(Ljava/lang/String;)Ljava/lang/String;",
            (string2.instance,),
        )
        .await?;

    let string = JavaLangString::from_instance(result).to_string(&mut jvm)?;

    assert_eq!(string, "test1test2");

    Ok(())
}
