mod test_helper;

use std::path::Path;

use jvm::Result;

use test_helper::run_jar;

#[futures_test::test]
async fn test_jar() -> Result<()> {
    run_jar(Path::new("test_data/test.jar"), &[]).await?;

    Ok(())
}
