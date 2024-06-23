mod test_helper;

use std::{fs, path::Path};

use jvm::Result;

use test_helper::{run_class, run_jar};

// TODO parameterized tests..
#[futures_test::test]
async fn test_class() -> Result<()> {
    let base_path = Path::new("test_data");

    let paths = fs::read_dir(base_path).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        let extension = path.extension();
        if let Some(x) = extension {
            if x != "class" && x != "jar" {
                continue;
            }
        } else {
            continue;
        }

        let name = path.file_stem().unwrap().to_str().unwrap();
        if name.contains('$') {
            continue;
        }

        let expected_path = base_path.join(format!("{}.txt", name));
        let expected = fs::read_to_string(expected_path).unwrap();

        let result = if extension.unwrap().to_str().unwrap() == "jar" {
            run_jar(&path, &[]).await
        } else {
            run_class(name, &[]).await
        };

        if let Err(err) = result {
            panic!("Test {} failed with error: {}", name, err);
        } else {
            assert_eq!(result.as_ref().unwrap().clone(), expected, "Test {} failed: {}", name, result.unwrap());
        }
    }

    Ok(())
}
