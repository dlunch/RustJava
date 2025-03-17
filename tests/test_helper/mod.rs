#![allow(dead_code)]

use std::{
    io,
    path::Path,
    str,
    sync::{Arc, Mutex},
};

use jvm::Result;
use rust_java::{StartType, run};

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

pub async fn run_class(path: &Path, class_path: &[&Path], args: &[String]) -> Result<String> {
    let output = Arc::new(Mutex::new(Vec::new()));

    run(Output { output: output.clone() }, StartType::Class(path), args, class_path).await?;

    let result = str::from_utf8(&output.lock().unwrap()).unwrap().to_string();

    Ok(result)
}

pub async fn run_jar(jar_path: &Path, args: &[String]) -> Result<String> {
    let output = Arc::new(Mutex::new(Vec::new()));

    run(Output { output: output.clone() }, StartType::Jar(jar_path), args, &[]).await?;

    let result = str::from_utf8(&output.lock().unwrap()).unwrap().to_string();

    Ok(result)
}
