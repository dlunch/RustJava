mod test_helper;

use std::{fs, path::Path, process::Command};

use jvm::Result;

use test_helper::{run_class, run_jar};

// TODO parameterized tests..
#[tokio::test]
#[ignore]
async fn test_real_jvm() -> Result<()> {
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

        let (result, expected) = if extension.unwrap().to_str().unwrap() == "jar" {
            let java_result = Command::new("java").arg("-jar").arg(path.to_str().unwrap()).output().unwrap();

            (run_jar(&path, &[]).await, String::from_utf8(java_result.stdout).unwrap())
        } else {
            let java_result = Command::new("java")
                .arg("-cp")
                .arg(base_path.to_str().unwrap())
                .arg(name)
                .output()
                .unwrap();

            (
                run_class(&path, &[Path::new("./test_data/")], &[]).await,
                String::from_utf8(java_result.stdout).unwrap(),
            )
        };

        if let Err(err) = result {
            panic!("Test {name} failed with error: {err}");
        } else {
            assert_eq!(result.as_ref().unwrap().clone(), expected, "Test {} failed: {}", name, result.unwrap());
        }
    }

    Ok(())
}
