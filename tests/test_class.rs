mod test_helper;

use std::{fs, path::Path};

use jvm::Result;

use test_helper::run_class;

// TODO parameterized tests..
fn get_test_data() -> Vec<(String, String)> {
    let tests = [
        "Array",
        "ControlFlow",
        "Exception",
        "Field",
        "Hello",
        "Instanceof",
        "Method",
        "MultiArray",
        "OddEven",
        "SuperClass",
        "Switch",
    ];

    let base_path = Path::new("test_data");

    let mut result = Vec::new();
    for test in tests {
        let expected_path = base_path.join(format!("{}.txt", test));

        let expected = fs::read_to_string(expected_path).unwrap();

        result.push((test.to_string(), expected));
    }

    result
}

#[futures_test::test]
async fn test_class() -> Result<()> {
    let tests = get_test_data();

    for (name, expected) in tests {
        let result = run_class(&name, &[]).await;
        if let Err(err) = result {
            panic!("Test {} failed with error: {}", name, err);
        } else {
            assert_eq!(result.as_ref().unwrap().clone(), expected, "Test {} failed: {}", name, result.unwrap());
        }
    }

    Ok(())
}
