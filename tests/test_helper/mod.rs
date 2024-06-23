#![allow(dead_code)]

use std::{
    io,
    path::Path,
    str,
    sync::{Arc, Mutex},
};

use jvm::Result;
use rust_java::{create_jvm, get_main_class_name, run_java_main};

struct Output {
    output: Arc<Mutex<Vec<u8>>>,
}

impl io::Write for Output {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.output.lock().unwrap().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub async fn run_class(main_class_name: &str, args: &[String]) -> Result<String> {
    let output = Arc::new(Mutex::new(Vec::new()));
    let jvm = create_jvm(Output { output: output.clone() }, &[Path::new("."), Path::new("./test_data/")]).await?;

    run_java_main(&jvm, main_class_name, args).await?;

    let result = str::from_utf8(&output.lock().unwrap()).unwrap().to_string();

    Ok(result)
}

pub async fn run_jar(jar_path: &Path, args: &[String]) -> Result<String> {
    let output = Arc::new(Mutex::new(Vec::new()));
    let jvm = create_jvm(Output { output: output.clone() }, &[jar_path]).await?;

    let main_class_name = get_main_class_name(&jvm, jar_path).await?;

    run_java_main(&jvm, &main_class_name, args).await?;

    let result = str::from_utf8(&output.lock().unwrap()).unwrap().to_string();

    Ok(result)
}
