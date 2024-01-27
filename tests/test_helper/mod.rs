#![allow(dead_code)]

use core::cell::RefCell;
use std::{io, str};

use rust_java::{create_jvm, load_class_file, run_java_main};

thread_local! {
    static OUTPUT: RefCell<Vec<u8>> = RefCell::new(Vec::new());
}

struct Output;

impl io::Write for Output {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        OUTPUT.with_borrow_mut(|x| x.write(buf))
    }

    fn flush(&mut self) -> io::Result<()> {
        io::stdout().flush()
    }
}

pub async fn run_class(main_class_name: &str, classes: &[(&str, &[u8])], args: &[String]) -> anyhow::Result<String> {
    OUTPUT.with_borrow_mut(|x| x.clear());

    let jvm = create_jvm(Output).await?;

    for (name, data) in classes {
        let file_name = name.replace('.', "/") + ".class";
        load_class_file(&jvm, &file_name, data).await?;
    }

    run_java_main(&jvm, main_class_name, args).await?;

    let result = OUTPUT.with_borrow(|output| str::from_utf8(output).unwrap().to_string());

    Ok(result)
}
