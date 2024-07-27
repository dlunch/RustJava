use std::{
    io,
    path::{Path, PathBuf},
};

use clap::{ArgGroup, Parser};

use jvm::Result;
use rust_java::{run, StartType};

#[derive(Parser)]
#[clap(group = ArgGroup::new("target").required(true).multiple(false))]
struct Opts {
    #[arg(group = "target", name = "mainclass")]
    main_class: Option<PathBuf>,
    #[arg(long, group = "target", name = "jarfile")]
    jar: Option<PathBuf>,

    args: Vec<String>,
}

#[tokio::main]
pub async fn main() -> Result<()> {
    let opts = Opts::parse();

    let start_type = if opts.main_class.is_some() {
        StartType::Class(opts.main_class.as_ref().unwrap())
    } else {
        StartType::Jar(opts.jar.as_ref().unwrap())
    };

    run(io::stdout(), start_type, &opts.args, &[Path::new(".")]).await?;

    Ok(())
}
