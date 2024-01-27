use std::{
    fs,
    io::{self},
};

use clap::{ArgGroup, Parser};
use futures_executor::block_on;

use rust_java::{create_jvm, load_class_file, load_jar_file, run_java_main};

#[derive(Parser)]
#[clap(group = ArgGroup::new("target").required(true).multiple(false))]
struct Opts {
    #[arg(group = "target", name = "mainclass")]
    main_class: Option<String>,
    #[arg(long, group = "target", name = "jarfile")]
    jar: Option<String>,

    args: Vec<String>,
}

pub fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    block_on(async {
        let jvm = create_jvm(io::stdout()).await?;

        let main_class_name = if let Some(x) = opts.main_class {
            let file_name = x.replace('.', "/") + ".class";
            let data = fs::read(&file_name)?;

            load_class_file(&jvm, &file_name, &data).await?;

            x
        } else if let Some(x) = opts.jar {
            let data = fs::read(&x)?;

            load_jar_file(&jvm, &data).await?
        } else {
            unreachable!() // should be caught by clap
        };

        run_java_main(&jvm, &main_class_name, &opts.args).await?;

        Ok(())
    })
}
