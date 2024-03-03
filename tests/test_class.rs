mod test_helper;

use std::{fs, path::Path};

use jvm::Result;

use test_helper::run_class;

// TODO parameterized tests..
fn get_test_data() -> Vec<(String, Vec<u8>, String)> {
    let tests = ["Array", "ControlFlow", "Exception", "Field", "Hello", "Method", "MultiArray"];

    let base_path = Path::new("test_data");

    let mut result = Vec::new();
    for test in tests {
        let class_path = base_path.join(format!("{}.class", test));
        let expected_path = base_path.join(format!("{}.txt", test));

        let class = fs::read(class_path).unwrap();
        let expected = fs::read_to_string(expected_path).unwrap();

        result.push((test.to_string(), class, expected));
    }

    result
}

#[futures_test::test]
async fn test_class() -> Result<()> {
    let tests = get_test_data();

    for (name, class, expected) in tests {
        let result = run_class(&name, &[(&name, &class)], &[]).await?;
        assert_eq!(result, expected, "Test failed: {}", name);
    }

    Ok(())
}
