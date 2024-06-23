mod test_helper;

use std::{fs, path::Path};

use jvm::Result;

use test_helper::run_jar;

#[futures_test::test]
async fn test_jar() -> Result<()> {
    let base_path = Path::new("test_data");

    let paths = fs::read_dir(base_path).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        if let Some(x) = path.extension() {
            if x != "jar" {
                continue;
            }
        } else {
            continue;
        }

        let jar_name = path.file_stem().unwrap().to_str().unwrap();
        let expected_path = base_path.join(format!("{}.txt", jar_name));

        let expected = fs::read_to_string(expected_path).unwrap();

        let result = run_jar(&path, &[]).await;
        if let Err(err) = result {
            panic!("Test {} failed with error: {}", jar_name, err);
        } else {
            assert_eq!(
                result.as_ref().unwrap().clone(),
                expected,
                "Test {} failed: {}",
                jar_name,
                result.unwrap()
            );
        }
    }

    Ok(())
}
