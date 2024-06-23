mod test_helper;

use std::{fs, path::Path};

use jvm::Result;

use test_helper::run_class;

// TODO parameterized tests..
#[futures_test::test]
async fn test_class() -> Result<()> {
    let base_path = Path::new("test_data");

    let paths = fs::read_dir(base_path).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        if let Some(x) = path.extension() {
            if x != "class" {
                continue;
            }
        } else {
            continue;
        }

        let class_name = path.file_stem().unwrap().to_str().unwrap();
        if class_name.contains('$') {
            continue;
        }

        let expected_path = base_path.join(format!("{}.txt", class_name));
        println!("{:?}", expected_path);

        let expected = fs::read_to_string(expected_path).unwrap();

        let result = run_class(class_name, &[]).await;
        if let Err(err) = result {
            panic!("Test {} failed with error: {}", class_name, err);
        } else {
            assert_eq!(
                result.as_ref().unwrap().clone(),
                expected,
                "Test {} failed: {}",
                class_name,
                result.unwrap()
            );
        }
    }

    Ok(())
}
