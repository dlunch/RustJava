use std::{
    io,
    path::{Path, PathBuf},
};

use clap::{ArgGroup, Parser};

use jvm::Result;
use rust_java::{StartType, run};

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
    #[cfg(not(target_arch = "wasm32"))]
    let runtime = tokio::runtime::Runtime::new().unwrap();
    #[cfg(target_arch = "wasm32")]
    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();

    runtime.block_on(async_main())
}

pub async fn async_main() -> Result<()> {
    let opts = Opts::parse();

    let start_type = if opts.main_class.is_some() {
        StartType::Class(opts.main_class.as_ref().unwrap())
    } else {
        StartType::Jar(opts.jar.as_ref().unwrap())
    };

    run(io::stdout(), start_type, &opts.args, &[Path::new(".")]).await?;

    Ok(())
}
