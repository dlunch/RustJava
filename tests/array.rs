mod test_helper;

use jvm::Result;

use test_helper::run_class;

#[futures_test::test]
async fn test_array() -> Result<()> {
    let array = include_bytes!("../test_data/Array.class");

    let result = run_class("Array", &[("Array", array)], &[]).await?;
    assert_eq!(
        result,
        "112344\n123\n12345\n1123412341234\n가\ntrue\ntest한글\n10\n10\n10\n10\n10\n10\n10\n"
    );

    Ok(())
}
