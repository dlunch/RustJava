use std::{
    io,
    path::{Path, PathBuf},
};

use clap::{ArgGroup, Parser};
use futures_executor::block_on;

use jvm::Result;
use rust_java::{create_jvm, get_main_class_name, run_java_main};

#[derive(Parser)]
#[clap(group = ArgGroup::new("target").required(true).multiple(false))]
struct Opts {
    #[arg(group = "target", name = "mainclass")]
    main_class: Option<PathBuf>,
    #[arg(long, group = "target", name = "jarfile")]
    jar: Option<PathBuf>,

    args: Vec<String>,
}

pub fn main() -> Result<()> {
    let opts = Opts::parse();

    block_on(async {
        let jvm = create_jvm(io::stdout(), &[Path::new(".")]).await?;

        let main_class_name = if let Some(x) = &opts.main_class {
            x.to_str().unwrap()
        } else if let Some(x) = opts.jar {
            &get_main_class_name(&jvm, &x).await?
        } else {
            unreachable!() // should be caught by clap
        };

        run_java_main(&jvm, main_class_name, &opts.args).await?;

        Ok(())
    })
}
