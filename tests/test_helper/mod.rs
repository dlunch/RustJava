#![allow(dead_code)]

use std::{
    io, str,
    sync::{Arc, Mutex},
};

use jvm::Result;
use rust_java::{create_jvm, load_class_file, load_jar_file, run_java_main};

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

pub async fn run_class(main_class_name: &str, classes: &[(&str, &[u8])], args: &[String]) -> Result<String> {
    let output = Arc::new(Mutex::new(Vec::new()));
    let jvm = create_jvm(Output { output: output.clone() }).await?;

    for (name, data) in classes {
        let file_name = name.replace('.', "/") + ".class";
        load_class_file(&jvm, &file_name, data).await?;
    }

    run_java_main(&jvm, main_class_name, args).await?;

    let result = str::from_utf8(&output.lock().unwrap()).unwrap().to_string();

    Ok(result)
}

pub async fn run_jar(jar: &[u8], args: &[String]) -> Result<String> {
    let output = Arc::new(Mutex::new(Vec::new()));
    let jvm = create_jvm(Output { output: output.clone() }).await?;

    let main_class_name = load_jar_file(&jvm, jar).await?;

    run_java_main(&jvm, &main_class_name, args).await?;

    let result = str::from_utf8(&output.lock().unwrap()).unwrap().to_string();

    Ok(result)
}
