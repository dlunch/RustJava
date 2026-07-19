use std::process::Command;

#[test]
fn cli_classpath_options_load_classes_from_directories_and_jars() {
    let output = Command::new(env!("CARGO_BIN_EXE_rust_java"))
        .env_remove("CLASSPATH")
        .args(["-cp", "missing:test_data", "Hello"])
        .output()
        .unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "Hello, world!\n");

    let output = Command::new(env!("CARGO_BIN_EXE_rust_java"))
        .env_remove("CLASSPATH")
        .args(["-classpath", "test_data/test.jar", "JarTest"])
        .output()
        .unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert!(String::from_utf8(output.stdout).unwrap().starts_with("test content\n"));
}

#[test]
fn cli_classpath_uses_environment_and_cli_override() {
    let output = Command::new(env!("CARGO_BIN_EXE_rust_java"))
        .env("CLASSPATH", "test_data")
        .arg("Hello")
        .output()
        .unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "Hello, world!\n");

    let output = Command::new(env!("CARGO_BIN_EXE_rust_java"))
        .env("CLASSPATH", "missing")
        .args(["-cp", "test_data", "Hello"])
        .output()
        .unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "Hello, world!\n");
}

#[test]
fn cli_jar_mode_accepts_but_ignores_classpath_options() {
    let output = Command::new(env!("CARGO_BIN_EXE_rust_java"))
        .env("CLASSPATH", "also-ignored")
        .args(["-cp", "ignored", "-jar", "test_data/test.jar"])
        .output()
        .unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert!(String::from_utf8(output.stdout).unwrap().starts_with("test content\n"));
}

#[test]
fn cli_defaults_classpath_to_current_directory() {
    let output = Command::new(env!("CARGO_BIN_EXE_rust_java"))
        .env_remove("CLASSPATH")
        .current_dir("test_data")
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
