use std::{env, fs, process::Command};

#[test]
fn cli_loads_unicode_and_escaped_classpath_names() {
    let root = env::temp_dir().join(format!("rustjava-unicode-paths-{}", std::process::id()));
    let directory = root.join("\u{d55c}\u{ae00} +%20#");
    fs::create_dir_all(&directory).unwrap();
    let jar_name = "\u{c2e4}\u{d589} +%20#.jar";
    let jar_path = directory.join(jar_name);
    fs::copy("test-data/test.jar", &jar_path).unwrap();
    fs::copy("test-data/Hello.class", directory.join("Hello.class")).unwrap();

    let outputs = [
        (&["-cp", directory.to_str().unwrap(), "Hello"][..], "Hello, world!\n"),
        (&["-cp", jar_name, "JarTest"][..], "test content\n"),
        (&["-cp", jar_path.to_str().unwrap(), "JarTest"][..], "test content\n"),
        (&["-jar", jar_name][..], "test content\n"),
    ]
    .map(|(args, expected)| {
        let output = Command::new(env!("CARGO_BIN_EXE_rust_java"))
            .env_remove("CLASSPATH")
            .current_dir(&directory)
            .args(args)
            .output()
            .unwrap();
        (output, expected)
    });
    fs::remove_dir_all(&root).unwrap();

    for (output, expected) in outputs {
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        assert!(String::from_utf8(output.stdout).unwrap().starts_with(expected));
    }
}

#[test]
fn cli_classpath_options_load_classes_from_directories_and_jars() {
    let class_path = env::join_paths(["missing", "test-data"]).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rust_java"))
        .env_remove("CLASSPATH")
        .arg("-cp")
        .arg(class_path)
        .arg("Hello")
        .output()
        .unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "Hello, world!\n");

    let output = Command::new(env!("CARGO_BIN_EXE_rust_java"))
        .env_remove("CLASSPATH")
        .args(["-classpath", "test-data/test.jar", "JarTest"])
        .output()
        .unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert!(String::from_utf8(output.stdout).unwrap().starts_with("test content\n"));
}

#[test]
fn cli_classpath_loads_from_absolute_entries() {
    let working_directory = env::current_dir().unwrap();
    let class_path = working_directory.join("test-data");
    let output = Command::new(env!("CARGO_BIN_EXE_rust_java"))
        .env_remove("CLASSPATH")
        .arg("-cp")
        .arg(class_path)
        .arg("Hello")
        .output()
        .unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "Hello, world!\n");

    let class_path = working_directory.join("test-data/test.jar");
    let output = Command::new(env!("CARGO_BIN_EXE_rust_java"))
        .env_remove("CLASSPATH")
        .arg("-classpath")
        .arg(class_path)
        .arg("JarTest")
        .output()
        .unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert!(String::from_utf8(output.stdout).unwrap().starts_with("test content\n"));
}

#[test]
fn cli_classpath_uses_environment_and_cli_override() {
    let output = Command::new(env!("CARGO_BIN_EXE_rust_java"))
        .env("CLASSPATH", "test-data")
        .arg("Hello")
        .output()
        .unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "Hello, world!\n");

    let output = Command::new(env!("CARGO_BIN_EXE_rust_java"))
        .env("CLASSPATH", "missing")
        .args(["-cp", "test-data", "Hello"])
        .output()
        .unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "Hello, world!\n");
}

#[test]
fn cli_jar_mode_accepts_but_ignores_classpath_options() {
    let output = Command::new(env!("CARGO_BIN_EXE_rust_java"))
        .env("CLASSPATH", "also-ignored")
        .args(["-cp", "ignored", "-jar", "test-data/test.jar"])
        .output()
        .unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert!(String::from_utf8(output.stdout).unwrap().starts_with("test content\n"));
}

#[test]
fn cli_defaults_classpath_to_current_directory() {
    let output = Command::new(env!("CARGO_BIN_EXE_rust_java"))
        .env_remove("CLASSPATH")
        .current_dir("test-data")
        .arg("Hello")
        .output()
        .unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "Hello, world!\n");
}

#[test]
fn cli_reports_missing_classpath_value() {
    let output = Command::new(env!("CARGO_BIN_EXE_rust_java"))
        .env_remove("CLASSPATH")
        .arg("-cp")
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8(output.stderr).unwrap().contains("Missing class path after -cp"));
}
